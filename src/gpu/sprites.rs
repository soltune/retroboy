use crate::emulator::{Emulator, Mode};
use crate::gpu::has_dmg_compatability;
use crate::gpu::colors::{as_cgb_obj_color_rgb, as_obj_color_rgb, Color};
use crate::gpu::prioritization::SpritePixel;
use crate::gpu::utils::{get_obj_enabled_mode, get_obj_size_mode, get_tile_line_bytes};
use crate::utils::is_bit_set;

const SPRITE_LIMIT_PER_SCANLINE: usize = 10;
const TOTAL_SPRITES: u16 = 40;

const TILE_DATA_BYTE_SIZE: u16 = 16;
const SPRITE_BYTE_SIZE: u16 = 4;

const SPRITE_WIDTH: i16 = 8;

const CGB_OPRI_PRIORITY_BIT: u8 = 0;

#[derive(Debug)]
pub struct Sprite {
    pub y_pos: i16,
    pub x_pos: i16,
    pub tile_index: u8,
    pub priority: bool,
    pub y_flip: bool,
    pub x_flip: bool,
    pub dmg_palette: bool,
    pub oam_index: u16,
    pub cgb_from_bank_one: bool,
    pub cgb_palette: u8
}

impl Sprite {
    fn has_higher_priority_than(&self, compared_sprite: &Sprite, oam_location_prioritization: bool) -> bool {
        let has_lower_x = self.x_pos < compared_sprite.x_pos;
        let has_same_x = self.x_pos == compared_sprite.x_pos;
        let located_earlier_in_oam = self.oam_index < compared_sprite.oam_index;
        (oam_location_prioritization && located_earlier_in_oam) ||
            (!oam_location_prioritization && (has_lower_x || (has_same_x && located_earlier_in_oam)))
    }
}

fn within_scanline(sprite_y_pos: i16, y_int: i16, eight_by_sixteen_mode: bool) -> bool {
    let sprite_height = if eight_by_sixteen_mode { 16 } else { 8 };
    let last_row = sprite_y_pos + sprite_height;
    y_int >= sprite_y_pos && y_int < last_row && last_row >= 0
}

fn sprite_overlaps_coordinates(sprite_x_pos: i16, sprite_y_pos: i16, x_int: i16, y_int: i16, eight_by_sixteen_mode: bool) -> bool {
    within_scanline(sprite_y_pos, y_int, eight_by_sixteen_mode)
        && x_int >= sprite_x_pos && x_int < sprite_x_pos + SPRITE_WIDTH
}

fn calculate_oam_index(sprite_number: u16) -> u16 {
    sprite_number * SPRITE_BYTE_SIZE
}

fn calculate_tile_data_index(tile_index: u16) -> u16 {
    tile_index * TILE_DATA_BYTE_SIZE
}

fn get_sprite_palette(dmg_palette: bool, obp0: u8, obp1: u8) -> u8 {
    if dmg_palette {
        obp1
    }
    else {
        obp0
    }
}

fn pull_sprite(emulator: &Emulator, sprite_number: u16) -> Sprite {
    let oam_index = calculate_oam_index(sprite_number);

    let y_pos = emulator.gpu.object_attribute_memory[oam_index as usize];
    let x_pos = emulator.gpu.object_attribute_memory[(oam_index + 1) as usize];
    let tile_index = emulator.gpu.object_attribute_memory[(oam_index + 2) as usize];
    let attributes = emulator.gpu.object_attribute_memory[(oam_index + 3) as usize];
    
    Sprite {
        y_pos: (y_pos as i16 - 16),
        x_pos: (x_pos as i16 - 8),
        tile_index,
        priority: is_bit_set(attributes, 7),
        y_flip: is_bit_set(attributes, 6),
        x_flip: is_bit_set(attributes, 5),
        dmg_palette: is_bit_set(attributes, 4),
        oam_index,
        cgb_from_bank_one: is_bit_set(attributes, 3),
        cgb_palette: attributes & 0b111
    }
}

pub fn collect_scanline_sprites(emulator: &Emulator) -> Vec<Sprite> {
    let mut sprites = Vec::new();
    let ly = emulator.gpu.registers.ly;
    let lcdc = emulator.gpu.registers.lcdc;

    let eight_by_sixteen_mode = get_obj_size_mode(lcdc);

    for sprite_number in 0..TOTAL_SPRITES {
        let sprite = pull_sprite(emulator, sprite_number);

        let y_int = ly as i16;

        if within_scanline(sprite.y_pos, y_int, eight_by_sixteen_mode) {
            sprites.push(sprite);

            if sprites.len() == SPRITE_LIMIT_PER_SCANLINE {
                break;
            }
        }
    }

    sprites 
}

fn lookup_possible_sprites(emulator: &Emulator, x: u8, y: u8, eight_by_sixteen_mode: bool) -> Vec<&Sprite> {
    let mut found_sprites = Vec::new();

    for sprite_number in 0..TOTAL_SPRITES {
        let sprite_number_usize = sprite_number as usize;

        if sprite_number_usize < emulator.gpu.sprite_buffer.len() {
            let sprite = &emulator.gpu.sprite_buffer[sprite_number_usize];

            let x_int  = x as i16;
            let y_int = y as i16;
            
            if sprite_overlaps_coordinates(sprite.x_pos, sprite.y_pos, x_int, y_int, eight_by_sixteen_mode) {
                found_sprites.push(sprite);
            }   
        }
    }

    found_sprites
}

pub fn calculate_sprite_pixel_color(emulator: &Emulator, sprite: &Sprite, x: u8, y: u8) -> Option<Color> {
    let y_int = y as i16;
    let x_int  = x as i16;

    let lcdc = emulator.gpu.registers.lcdc;
    let eight_by_sixteen_mode = get_obj_size_mode(lcdc);

    let calculated_index = calculate_tile_index(&sprite, y_int, eight_by_sixteen_mode);
    let tile_data_index = calculate_tile_data_index(calculated_index as u16);
    let row_offset = ((y_int - sprite.y_pos) % 8) as u8;
    let column_offset = x_int - sprite.x_pos;

    if column_offset >= 0 {
        if emulator.mode == Mode::CGB {
            let (lsb_byte, msb_byte) = get_tile_line_bytes(&emulator.gpu, tile_data_index, row_offset, sprite.y_flip, sprite.cgb_from_bank_one);
            let palette_number = if has_dmg_compatability(emulator) {
                if sprite.dmg_palette { 1 } else { 0 }
            }
            else {
                sprite.cgb_palette
            };
            as_cgb_obj_color_rgb(&emulator.gpu.registers.palettes, column_offset as u8, palette_number, msb_byte, lsb_byte, sprite.x_flip)
        }
        else {
            let (lsb_byte, msb_byte) = get_tile_line_bytes(&emulator.gpu, tile_data_index, row_offset, sprite.y_flip, false);
            let palette = get_sprite_palette(sprite.dmg_palette, emulator.gpu.registers.palettes.obp0, emulator.gpu.registers.palettes.obp1);
            as_obj_color_rgb(column_offset as u8, palette, msb_byte, lsb_byte, sprite.x_flip) 
        }
    }
    else {
        None
    } 
}

fn resolve_highest_priority_sprite<'a>(emulator: &Emulator, sprites: Vec<&'a Sprite>, x: u8, y: u8) -> Option<(&'a Sprite, Option<Color>)> {
    let mut maybe_highest_priority: Option<(&'a Sprite, Option<Color>)> = None;
    let cgb_mode = emulator.mode == Mode::CGB;
    let oam_location_prioritization = cgb_mode && !is_bit_set(emulator.gpu.registers.cgb_opri, CGB_OPRI_PRIORITY_BIT);

    for sprite in sprites {
        match maybe_highest_priority {
            Some(highest_priority) => {
                let current_highest_priority_sprite = highest_priority.0;
                let maybe_current_highest_priority_color = highest_priority.1;

                let maybe_color = calculate_sprite_pixel_color(emulator, sprite, x, y);
 
                match (maybe_color, maybe_current_highest_priority_color) {
                    (Some(color), Some(_)) if sprite.has_higher_priority_than(current_highest_priority_sprite, oam_location_prioritization) => {
                        maybe_highest_priority = Some((sprite, Some(color)));
                    }
                    (Some(color), None) => {
                        maybe_highest_priority = Some((sprite, Some(color)));
                    }
                    _ => {}
                }
            }
            None => {
                let maybe_color = calculate_sprite_pixel_color(emulator, sprite, x, y);
                maybe_highest_priority = Some((sprite, maybe_color));
            }
        }
    }

    maybe_highest_priority
}

fn calculate_tile_index(sprite: &Sprite, y_int: i16, eight_by_sixteen_mode: bool) -> u8 {
    if eight_by_sixteen_mode && (y_int - sprite.y_pos) >= 8 {
        if sprite.y_flip { sprite.tile_index & 0xFE } else { sprite.tile_index | 0x01 }
    }
    else if eight_by_sixteen_mode {
        if sprite.y_flip { sprite.tile_index | 0x01 } else { sprite.tile_index & 0xFE }
    }
    else {
        sprite.tile_index
    }
}

pub fn read_sprite_pixel_color(emulator: &Emulator, x: u8, y: u8) -> Option<SpritePixel> {
    let lcdc = emulator.gpu.registers.lcdc;

    let eight_by_sixteen_mode = get_obj_size_mode(lcdc);
    let sprites_enabled = get_obj_enabled_mode(lcdc);

    let possible_sprites = lookup_possible_sprites(emulator, x, y, eight_by_sixteen_mode);
    
    if sprites_enabled {
        match resolve_highest_priority_sprite(emulator, possible_sprites, x, y) {
            Some((highest_priority_sprite, maybe_color)) => {
                let prioritize_bg = highest_priority_sprite.priority;
                maybe_color.map(|color| SpritePixel { color, prioritize_bg })
            },
            _ => None
        }
    }
    else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::initialize_screenless_emulator;
    use super::*;

    fn write_sprite(emulator: &mut Emulator, sprit_number: u8, y_pos: u8, x_pos: u8, attributes: u8) {
        let index = (sprit_number * 4) as usize;
        emulator.gpu.object_attribute_memory[index] = y_pos;
        emulator.gpu.object_attribute_memory[index + 1] = x_pos;
        emulator.gpu.object_attribute_memory[index + 2] = 0x0;
        emulator.gpu.object_attribute_memory[index + 3] = attributes;
    }

    #[test]
    fn should_get_ten_sprites_from_oam_memory() {
        let mut emulator = initialize_screenless_emulator();
        
        emulator.gpu.registers.ly = 0;

        write_sprite(&mut emulator, 0, 0, 0, 0);
        write_sprite(&mut emulator, 1, 16, 0, 0);
        write_sprite(&mut emulator, 2, 44, 0, 0);
        write_sprite(&mut emulator, 3, 9, 0x1F, 0);
        write_sprite(&mut emulator, 4, 14, 0x2A, 0);
        write_sprite(&mut emulator, 5, 16, 0x60, 0);
        write_sprite(&mut emulator, 6, 0, 0xFF, 0);
        write_sprite(&mut emulator, 7, 10, 0x3F, 0);
        write_sprite(&mut emulator, 8, 16, 0x4A, 0);
        write_sprite(&mut emulator, 9, 14, 0x51, 0);
        write_sprite(&mut emulator, 10, 8, 0x22, 0);
        write_sprite(&mut emulator, 11, 11, 0x1B, 0);
        write_sprite(&mut emulator, 12, 13, 0x14, 0);
        write_sprite(&mut emulator, 13, 16, 0x55, 0);
        write_sprite(&mut emulator, 14, 14, 0x22, 0);
        write_sprite(&mut emulator, 15, 15, 0x23, 0);

        let sprites = collect_scanline_sprites(&emulator);

        assert_eq!(sprites.len(), 10);
        assert_eq!(sprites[0].y_pos, 0);
        assert_eq!(sprites[1].y_pos, -7);
        assert_eq!(sprites[2].y_pos, -2);
        assert_eq!(sprites[3].y_pos, 0);
        assert_eq!(sprites[4].y_pos, -6);
        assert_eq!(sprites[5].y_pos, 0);
        assert_eq!(sprites[6].y_pos, -2);
        assert_eq!(sprites[7].y_pos, -5);
        assert_eq!(sprites[8].y_pos, -3);
        assert_eq!(sprites[9].y_pos, 0);
    }

    #[test]
    fn should_parse_sprite_attributes_correctly() {
        let mut emulator = initialize_screenless_emulator();
        
        write_sprite(&mut emulator, 0, 16, 0, 0b11000000);
        
        let sprites = collect_scanline_sprites(&emulator);

        assert_eq!(sprites[0].priority, true);
        assert_eq!(sprites[0].y_flip, true);
        assert_eq!(sprites[0].x_flip, false);
        assert_eq!(sprites[0].dmg_palette, false);
    }
}