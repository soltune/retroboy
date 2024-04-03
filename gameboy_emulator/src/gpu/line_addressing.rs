use crate::gpu::utils::*;
 
const TILES_PER_ROW: u8 = 32;
const TILE_DATA_LENGTH: u8 = 16;

fn resolve_tile_index_address(tile_map_mode: bool, tile_map_offset: u16) -> u16 {
    if tile_map_mode {
        0x9C00 + tile_map_offset
    }
    else {
        0x9800 + tile_map_offset
    }
}

fn calculate_tile_offset(column_tile_offset: u8, row_tile_offset: u8) -> u16 {
    column_tile_offset as u16 * TILES_PER_ROW as u16 + row_tile_offset as u16
}

pub fn resolve_bg_tile_index_address(lcdc: u8, column_tile_offset: u8, row_tile_offset: u8) -> u16 {
    let tile_map_offset = calculate_tile_offset(column_tile_offset, row_tile_offset);
    let tile_map_mode = get_bg_tile_map_mode(lcdc);
    resolve_tile_index_address(tile_map_mode, tile_map_offset)
}

pub fn resolve_window_tile_index_address(lcdc: u8, column_tile_offset: u8, row_tile_offset: u8) -> u16 {
    let tile_map_offset = calculate_tile_offset(column_tile_offset, row_tile_offset);
    let tile_map_mode = get_window_tile_map_mode(lcdc);
    resolve_tile_index_address(tile_map_mode, tile_map_offset)
}

pub fn resolve_tile_data_address(lcdc: u8, index: u8) -> u16 {
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

