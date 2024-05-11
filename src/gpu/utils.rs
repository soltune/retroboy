use crate::utils::is_bit_set;

const LCDC_BG_AND_WINDOW_ENABLED_INDEX: u8 = 0;
const LCDC_OBJ_ENABLED_INDEX: u8 = 1;
const LCDC_OBJ_SIZE_INDEX: u8 = 2;
const LCDC_BG_TILE_MAP_INDEX: u8 = 3;
const LCDC_TILE_DATA_INDEX: u8 = 4;
const LCDC_WINDOW_ENABLED_INDEX: u8 = 5;
const LCDC_WINDOW_TILE_MAP_INDEX: u8 = 6;
const LCDC_ENABLED_INDEX: u8 = 7;

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

#[cfg(test)]
mod tests;