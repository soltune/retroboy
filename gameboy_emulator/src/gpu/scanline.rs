use crate::emulator::Emulator;
use crate::gpu::constants::GB_SCREEN_WIDTH;
use crate::gpu::utils::*;
use crate::gpu::colors::as_bg_color_rgb;
use crate::gpu::sprites::read_sprite_pixel_rgb;
use crate::mmu;

const TILES_PER_ROW: u8 = 32;
const TILE_DATA_LENGTH: u8 = 16;
const TILE_WIDTH: u8 = 8;

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
                let rgb = read_sprite_pixel_rgb(emulator, x, ly)
                    .unwrap_or(as_bg_color_rgb(bit_index, palette, msb_byte, lsb_byte));
                let pixel_index = (ly as u16 * GB_SCREEN_WIDTH + x as u16) as usize;
                emulator.gpu.frame_buffer[pixel_index] = rgb;
            } 
        }
    }
}

#[cfg(test)]
mod tests;