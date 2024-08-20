use crate::emulator::Emulator;
use crate::gpu::colors::{as_obj_color_rgb, WHITE, Color};
use crate::mmu;
use crate::utils::is_bit_set;
use crate::gpu::utils::{get_obj_enabled_mode, get_obj_size_mode};

const BASE_OAM_ADDRESS: u16 = 0xFE00;
const BASE_TILE_DATA_ADDRESS: u16 = 0x8000;

const SPRITE_LIMIT_PER_SCANLINE: usize = 10;
const TOTAL_SPRITES: u16 = 40;

const TILE_DATA_BYTE_SIZE: u16 = 16;
const SPRITE_BYTE_SIZE: u16 = 4;

const SPRITE_WIDTH: i16 = 8;

#[derive(Debug)]
pub struct Sprite {
    pub y_pos: i16,
    pub x_pos: i16,
    pub tile_index: u8,
    pub priority: bool,
    pub y_flip: bool,
    pub x_flip: bool,
    pub dmg_palette: bool,
    pub address: u16
}

impl Sprite {
    fn has_higher_priority_than(&self, compared_sprite: &Sprite) -> bool {
        let has_lower_x = self.x_pos < compared_sprite.x_pos;
        let has_same_x = self.x_pos == compared_sprite.x_pos;
        let located_earlier_in_oam = self.address < compared_sprite.address;
        has_lower_x || (has_same_x && located_earlier_in_oam)
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

fn calculate_sprite_address(sprite_number: u16) -> u16 {
    BASE_OAM_ADDRESS + (sprite_number * SPRITE_BYTE_SIZE)
}

fn calculate_tile_data_address(tile_index: u16) -> u16 {
    BASE_TILE_DATA_ADDRESS + (tile_index * TILE_DATA_BYTE_SIZE)
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
    let sprite_address = calculate_sprite_address(sprite_number);

    let y_pos = mmu::read_byte(emulator, sprite_address);
    let x_pos = mmu::read_byte(emulator, sprite_address + 1);
    let tile_index = mmu::read_byte(emulator, sprite_address + 2);
    let attributes = mmu::read_byte(emulator, sprite_address + 3);
    
    Sprite {
        y_pos: (y_pos as i16 - 16),
        x_pos: (x_pos as i16 - 8),
        tile_index,
        priority: is_bit_set(attributes, 7),
        y_flip: is_bit_set(attributes, 6),
        x_flip: is_bit_set(attributes, 5),
        dmg_palette: is_bit_set(attributes, 4),
        address: sprite_address
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

pub fn calculate_sprite_pixel_color(emulator: &Emulator, sprite: &Sprite, x: u8, y: u8, bg_color: Color) -> Option<Color> {
    let y_int = y as i16;
    let x_int  = x as i16;

    let lcdc = emulator.gpu.registers.lcdc;
    let eight_by_sixteen_mode = get_obj_size_mode(lcdc);

    let calculated_index = calculate_tile_index(&sprite, y_int, eight_by_sixteen_mode);
    let tile_data_address = calculate_tile_data_address(calculated_index as u16);
    let row_offset = ((y_int - sprite.y_pos) % 8) as u8;
    let tile_data_byte_offset = (if sprite.y_flip { 0xF - ((row_offset * 2) + 1) } else { row_offset * 2 }) as u16;
    let line_address = tile_data_address + tile_data_byte_offset;
    let column_offset = x_int - sprite.x_pos;

    if column_offset >= 0 {
        let lsb_byte = mmu::read_byte(&emulator, line_address);
        let msb_byte = mmu::read_byte(&emulator, line_address + 1);
        let palette = get_sprite_palette(sprite.dmg_palette, emulator.gpu.registers.obp0, emulator.gpu.registers.obp1);

        if (sprite.priority && bg_color == WHITE) || !sprite.priority {
            as_obj_color_rgb(column_offset as u8, palette, msb_byte, lsb_byte, sprite.x_flip) 
        }
        else {
           None
        }
    }
    else {
        None
    } 
}

fn resolve_highest_priority_pixel_color(emulator: &Emulator, sprites: Vec<&Sprite>, x: u8, y: u8, bg_color: Color) -> Option<Color> {
    let mut maybe_highest_priority: Option<(&Sprite, Option<Color>)> = None;

    for sprite in sprites {
        match maybe_highest_priority {
            Some(highest_priority) => {
                let current_highest_priority_sprite = highest_priority.0;
                let maybe_current_highest_priority_color = highest_priority.1;

                let maybe_color = calculate_sprite_pixel_color(emulator, sprite, x, y, bg_color);
 
                match (maybe_color, maybe_current_highest_priority_color) {
                    (Some(color), Some(_)) if sprite.has_higher_priority_than(current_highest_priority_sprite) => {
                        maybe_highest_priority = Some((sprite, Some(color)));
                    }
                    (Some(color), None) => {
                        maybe_highest_priority = Some((sprite, Some(color)));
                    }
                    _ => {}
                }
            }
            None => {
                let maybe_color = calculate_sprite_pixel_color(emulator, sprite, x, y, bg_color);
                maybe_highest_priority = Some((sprite, maybe_color));
            }
        }
    }

    maybe_highest_priority.map(|(_, color)| color).flatten()
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

pub fn read_sprite_pixel_color(emulator: &Emulator, x: u8, y: u8, bg_color: Color) -> Option<Color> {
    let lcdc = emulator.gpu.registers.lcdc;

    let eight_by_sixteen_mode = get_obj_size_mode(lcdc);
    let sprites_enabled = get_obj_enabled_mode(lcdc);

    let possible_sprites = lookup_possible_sprites(emulator, x, y, eight_by_sixteen_mode);
    
    if sprites_enabled {
        resolve_highest_priority_pixel_color(emulator, possible_sprites, x, y, bg_color)
    }
    else {
        None
    }
}
