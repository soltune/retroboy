use crate::emulator::Emulator;
use crate::gpu::colors::as_obj_color_rgb;
use crate::mmu;
use crate::utils::is_bit_set;

const BASE_OAM_ADDRESS: u16 = 0xFE00;
const BASE_TILE_DATA_ADDRESS: u16 = 0x8000;

const SPRITE_LIMIT_PER_SCANLINE: usize = 10;
const TOTAL_SPRITES: u16 = 40;

const TILE_DATA_BYTE_SIZE: u16 = 16;
const SPRITE_BYTE_SIZE: u16 = 4;

const SPRITE_HEIGHT: i16 = 8;
const SPRITE_WIDTH: i16 = 8;

pub struct Sprite {
    pub y_pos: i16,
    pub x_pos: i16,
    pub tile_index: u8,
    pub priority: bool,
    pub y_flip: bool,
    pub x_flip: bool,
    pub dmg_palette: bool
}

fn within_scanline(sprite_y_pos: i16, y: i16) -> bool {
    let last_row = sprite_y_pos + SPRITE_HEIGHT;
    y >= sprite_y_pos && y < last_row && last_row >= 0
}

fn should_render(sprite_x_pos: i16, sprite_y_pos: i16, x: i16, y: i16) -> bool {
    within_scanline(sprite_y_pos, y) && x >= sprite_x_pos && x < sprite_x_pos + SPRITE_WIDTH
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
        dmg_palette: is_bit_set(attributes, 4)
    }
}

pub fn collect_scanline_sprites(emulator: &Emulator) -> Vec<Sprite> {
    let mut sprites = Vec::new();
    let ly = emulator.gpu.registers.ly;

    for sprite_number in 0..TOTAL_SPRITES {
        let sprite = pull_sprite(emulator, sprite_number);

        let y_int = ly as i16;

        if within_scanline(sprite.y_pos, y_int) {
            sprites.push(sprite);

            if sprites.len() == SPRITE_LIMIT_PER_SCANLINE {
                break;
            }
        }
    }

    sprites 
}

fn lookup_sprite(emulator: &Emulator, x: u8, y: u8) -> Option<&Sprite> {
    let mut maybe_found_sprite: Option<&Sprite> = None;

    for sprite_number in 0..TOTAL_SPRITES {
        let sprite_number_usize = sprite_number as usize;

        if sprite_number_usize < emulator.gpu.sprite_buffer.len() {
            let sprite = &emulator.gpu.sprite_buffer[sprite_number_usize];

            let x_int  = x as i16;
            let y_int = y as i16;
            
            if should_render(sprite.x_pos, sprite.y_pos, x_int, y_int) {
                maybe_found_sprite = Some(sprite);
                break;
            }   
        }
    }

    maybe_found_sprite
}

pub fn read_sprite_pixel_rgb(emulator: &Emulator, x: u8, y: u8) -> Option<u32> {
    let maybe_found_sprite = lookup_sprite(emulator, x, y);

    match maybe_found_sprite {
        Some(sprite) => {
            let tile_data_address = calculate_tile_data_address(sprite.tile_index as u16);

            let y_int = y as i16;
            let x_int  = x as i16;

            let row_offset = y_int - sprite.y_pos;
            let line_address = tile_data_address + (row_offset * 2) as u16;
            let column_offset = x_int - sprite.x_pos;

            if column_offset >= 0 {
                let lsb_byte = mmu::read_byte(&emulator, line_address);
                let msb_byte = mmu::read_byte(&emulator, line_address + 1);
                let palette = get_sprite_palette(sprite.dmg_palette, emulator.gpu.registers.obp0, emulator.gpu.registers.obp1);
                as_obj_color_rgb(column_offset as u8, palette, msb_byte, lsb_byte) 
            }
            else {
                None
            }
        }
        None => None
    }
}
