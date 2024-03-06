use crate::{emulator::Emulator, mmu, utils::{get_bit, is_bit_set}};

pub struct GpuRegisters {
    pub lcdc: u8,
    pub scy: u8,
    pub scx: u8,
    pub wx: u8,
    pub wy: u8,
    pub palette: u8,
    pub ly: u8,
    pub lyc: u8,
    pub stat: u8
}

pub struct Sprite {
    pub y_pos: u8,
    pub x_pos: u8,
    pub tile_index: u8,
    pub priority: bool,
    pub y_flip: bool,
    pub x_flip: bool,
}

pub struct GpuState {
    pub mode: u8,
    pub mode_clock: u16,
    pub registers: GpuRegisters,
    pub frame_buffer: Vec<u32>
}

const OAM_MODE: u8 = 2;
const OAM_TIME: u16 = 80;

const VRAM_MODE: u8 = 3;
const VRAM_TIME: u16 = 172;

const HBLANK_MODE: u8 = 0;
const HBLANK_TIME: u16 = 204;

const VBLANK_MODE: u8 = 1;

const SCANLINE_RENDER_TIME: u16 = 456;

const FRAME_SCANLINE_COUNT: u8 = 154;
const VBLANK_SCANLINE_COUNT: u8 = 10;

const LCDC_BG_AND_WINDOW_ENABLED_INDEX: u8 = 0;
const LCDC_OBJ_ENABLED_INDEX: u8 = 1;
const LCDC_OBJ_SIZE_INDEX: u8 = 2;
const LCDC_BG_TILE_MAP_INDEX: u8 = 3;
const LCDC_TILE_DATA_INDEX: u8 = 4;
const LCDC_WINDOW_ENABLED_INDEX: u8 = 5;
const LCDC_WINDOW_TILE_MAP_INDEX: u8 = 6;
const LCDC_ENABLED_INDEX: u8 = 7;

const GB_SCREEN_WIDTH: u16 = 160;
const GB_SCREEN_HEIGHT: u16 = 144;

const TILES_PER_ROW: u8 = 32;
const TILE_DATA_LENGTH: u8 = 16;
const TILE_WIDTH: u8 = 8;

const BASE_OAM_ADDRESS: u16 = 0xFE00;
const SPRITE_LIMIT_PER_SCANLINE: usize = 10;
const TOTAL_SPRITES: u16 = 40;

pub fn initialize_gpu() -> GpuState {
    GpuState {
        mode: 2,
        mode_clock: 0,
        registers: GpuRegisters {
            lcdc: 0,
            scy: 0,
            scx: 0,
            wx: 0,
            wy: 0,
            palette: 0,
            ly: 0,
            lyc: 0,
            stat: 0
        },
        frame_buffer: vec![0; (GB_SCREEN_WIDTH * GB_SCREEN_HEIGHT) as usize]
    }
}

pub fn get_bg_and_window_enabled_mode(lcdc: u8) -> bool {
    is_bit_set(lcdc, LCDC_BG_AND_WINDOW_ENABLED_INDEX)
}

pub fn get_obj_enabled_mode(lcdc: u8) -> bool {
    is_bit_set(lcdc, LCDC_OBJ_ENABLED_INDEX) 
}

pub fn get_obj_size_mode(lcdc: u8) -> bool {
    is_bit_set(lcdc, LCDC_OBJ_SIZE_INDEX)
}

pub fn get_bg_tile_map_mode(lcdc: u8) -> bool {
    is_bit_set(lcdc, LCDC_BG_TILE_MAP_INDEX)
}

pub fn get_tile_data_addressing_mode(lcdc: u8) -> bool {
    is_bit_set(lcdc, LCDC_TILE_DATA_INDEX)
}

pub fn get_window_enabled_mode(lcdc: u8) -> bool {
    is_bit_set(lcdc, LCDC_WINDOW_ENABLED_INDEX)
}

pub fn get_window_tile_map_mode(lcdc: u8) -> bool {
    is_bit_set(lcdc, LCDC_WINDOW_TILE_MAP_INDEX)
}

pub fn get_lcd_enabled_mode(lcdc: u8) -> bool {
    is_bit_set(lcdc, LCDC_ENABLED_INDEX)
}

fn as_color_rgb(bit_index: u8, palette: u8, msb_byte: u8, lsb_byte: u8) -> u32 {
    let msb = get_bit(msb_byte, bit_index);
    let lsb = get_bit(lsb_byte, bit_index);
    let color_id = (msb * 2) + lsb;

    let key = match color_id {
        0b11 => (palette & 0b11000000) >> 6,
        0b10 => (palette & 0b00110000) >> 4,
        0b01 => (palette & 0b00001100) >> 2,
        _ => palette & 0b00000011
    };

    match key {
        0b11 => 0x000000,
        0b10 => 0xa9a9a9,
        0b01 => 0xd3d3d3,
        _ => 0xffffff 
    } 
}

fn resolve_tile_index_address(lcdc: u8, tile_map_y: u8, row_tile_offset: u8) -> u16 {
    let tile_map_offest = ((tile_map_y / 8) as u16 * TILES_PER_ROW as u16) + row_tile_offset as u16;
    let tile_map_mode = get_window_tile_map_mode(lcdc);
    if tile_map_mode {
        0x9C00 + tile_map_offest
    }
    else {
        0x9800 + tile_map_offest
    } 
}

fn resolve_tile_data_address(lcdc: u8, index: u8) -> u16 {
    let unsigned_addressing = get_tile_data_addressing_mode(lcdc);
    if unsigned_addressing {
        0x8000 + (index * TILE_DATA_LENGTH) as u16
    }
    else if index >= 128 {
        0x8800 + ((index - 128) * TILE_DATA_LENGTH) as u16
    }
    else {
        0x9000 + (index * TILE_DATA_LENGTH) as u16 
    }
}

fn resolve_line_address(emulator: &Emulator, row_tile_offset: u8) -> u16 {
    let ly = emulator.gpu.registers.ly;
    let scy = emulator.gpu.registers.scy;
    let lcdc = emulator.gpu.registers.lcdc;
    
    let tile_map_y = scy.wrapping_add(ly);
    let tile_index_address = resolve_tile_index_address(lcdc, tile_map_y, row_tile_offset);
    let tile_index = mmu::read_byte(emulator, tile_index_address);
    let tile_data_address = resolve_tile_data_address(lcdc, tile_index);

    let row_offset = tile_map_y % 8;
    tile_data_address + (row_offset * 2) as u16
}

fn within_viewport(scx: u8, leftmost_tile_column: u8) -> bool {
    let rightmost_tile_column = leftmost_tile_column.wrapping_add(TILE_WIDTH);
    let rightmost_viewport_border = scx.wrapping_add(GB_SCREEN_WIDTH as u8);
    rightmost_tile_column >= scx || leftmost_tile_column <= rightmost_viewport_border
}

fn within_scanline(sprite_y_pos: u8, ly: u8) -> bool {
    (sprite_y_pos <= 16 && sprite_y_pos > 0 && sprite_y_pos >= ly) ||
    (sprite_y_pos > 16 && sprite_y_pos >= ly && sprite_y_pos - 16 <= ly)
}

fn pull_sprite(emulator: &Emulator, sprite_number: u16) -> Sprite {
    let sprite_address = BASE_OAM_ADDRESS + (sprite_number * 4);
    let y_pos = mmu::read_byte(emulator, sprite_address);
    let x_pos = mmu::read_byte(emulator, sprite_address + 1);
    let tile_index = mmu::read_byte(emulator, sprite_address + 2);
    Sprite {
        y_pos,
        x_pos,
        tile_index,
        priority: false,
        y_flip: false,
        x_flip: false
    }
}

pub fn collect_scanline_sprites(emulator: &Emulator) -> Vec<Sprite> {
    let mut sprites = Vec::new();
    let ly = emulator.gpu.registers.ly;

    for sprite_number in 0..TOTAL_SPRITES {
        let sprite = pull_sprite(emulator, sprite_number);

        if within_scanline(sprite.y_pos, ly) {
            sprites.push(sprite);

            if sprites.len() == SPRITE_LIMIT_PER_SCANLINE {
                break;
            }
        }
    }

    sprites 
}

pub fn write_scanline(emulator: &mut Emulator) {
    let ly = emulator.gpu.registers.ly;
    let scx = emulator.gpu.registers.scx;
    let palette = emulator.gpu.registers.palette;

    for row_tile_offset in 0..TILES_PER_ROW {
        let line_address = resolve_line_address(emulator, row_tile_offset);
        let tile_map_x = row_tile_offset * 8;
        let lsb_byte = mmu::read_byte(emulator, line_address);
        let msb_byte = mmu::read_byte(emulator, line_address + 1);

        for bit_index in 0..TILE_WIDTH {
            let pixel_x = tile_map_x + bit_index;
            if within_viewport(pixel_x, tile_map_x) {
                let x = pixel_x.wrapping_sub(scx);
                let color_rgb = as_color_rgb(bit_index, palette, msb_byte, lsb_byte);
                let pixel_index = (ly as u16 * GB_SCREEN_WIDTH + x as u16) as usize;
                emulator.gpu.frame_buffer[pixel_index] = color_rgb;
            } 
        }
    }
}

pub fn step(emulator: &mut Emulator) {
    emulator.gpu.mode_clock += emulator.cpu.clock.instruction_clock_cycles as u16;

    match emulator.gpu.mode {
        OAM_MODE => {
            if emulator.gpu.mode_clock >= OAM_TIME {
                emulator.gpu.mode = VRAM_MODE;
                emulator.gpu.mode_clock = 0;
            }
        }
        VRAM_MODE => {
            if emulator.gpu.mode_clock >= VRAM_TIME {
                emulator.gpu.mode = HBLANK_MODE;
                emulator.gpu.mode_clock = 0;
            }
        }
        HBLANK_MODE => {
            if emulator.gpu.mode_clock >= HBLANK_TIME {
                emulator.gpu.registers.ly += 1;
                emulator.gpu.mode_clock = 0;

                if emulator.gpu.registers.ly == FRAME_SCANLINE_COUNT - VBLANK_SCANLINE_COUNT - 1 {
                    emulator.gpu.mode = VBLANK_MODE;
                }
                else {
                    emulator.gpu.mode = OAM_MODE;
                }
            }
        }
        VBLANK_MODE => {
            if emulator.gpu.mode_clock >= SCANLINE_RENDER_TIME {
                emulator.gpu.mode_clock = 0;
                emulator.gpu.registers.ly += 1;

                if emulator.gpu.registers.ly == FRAME_SCANLINE_COUNT - 1 {
                    emulator.gpu.mode = OAM_MODE;
                    emulator.gpu.registers.ly = 0;
                }
            }
        }
        _ => ()
    }    
}

#[cfg(test)]
mod tests;
