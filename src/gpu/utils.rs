use crate::utils::is_bit_set;
use crate::gpu::GpuState;

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

fn calculate_line_index(tile_data_index: u16, row_offset: u8, y_flip: bool) -> u16 {
    let byte_offset = if y_flip {
        0xF - ((row_offset * 2) + 1)
    }
    else {
        row_offset * 2
    } as u16;

    tile_data_index + byte_offset
}

pub fn get_tile_line_bytes(gpu_state: &GpuState, tile_data_index: u16, row_offset: u8, y_flip: bool) -> (u8, u8) {
    let line_index = calculate_line_index(tile_data_index, row_offset, y_flip);
    let lsb_byte = gpu_state.video_ram[line_index as usize];
    let msb_byte = gpu_state.video_ram[(line_index + 1) as usize];
    (lsb_byte, msb_byte)
}

#[cfg(test)]
mod tests {
    use crate::emulator::initialize_screenless_emulator;
    use super::*;

    #[test]
    fn should_return_obj_size_mode() {
        let mut emulator = initialize_screenless_emulator();
        emulator.gpu.registers.lcdc = 0x04;
        let result = get_obj_size_mode(emulator.gpu.registers.lcdc);
        assert_eq!(result, true); 
    }

    #[test]
    fn should_return_bg_tile_map_mode() {
        let mut emulator = initialize_screenless_emulator();
        emulator.gpu.registers.lcdc = 0x08;
        let result = get_bg_tile_map_mode(emulator.gpu.registers.lcdc);
        assert_eq!(result, true);
    }

    #[test]
    fn should_return_tile_data_mode() {
        let mut emulator = initialize_screenless_emulator();
        emulator.gpu.registers.lcdc = 0x10;
        let result = get_tile_data_addressing_mode(emulator.gpu.registers.lcdc);
        assert_eq!(result, true);
    }

    #[test]
    fn should_return_window_enabled_mode() {
        let mut emulator = initialize_screenless_emulator();
        emulator.gpu.registers.lcdc = 0x20;
        let result = get_window_enabled_mode(emulator.gpu.registers.lcdc);
        assert_eq!(result, true);
    }

    #[test]
    fn should_return_window_tile_map_mode() {
        let mut emulator = initialize_screenless_emulator();
        emulator.gpu.registers.lcdc = 0x40;
        let result = get_window_tile_map_mode(emulator.gpu.registers.lcdc);
        assert_eq!(result, true);
    }

    #[test]
    fn should_return_lcdc_enabled_mode() {
        let mut emulator = initialize_screenless_emulator();
        emulator.gpu.registers.lcdc = 0x80;
        let result = get_lcd_enabled_mode(emulator.gpu.registers.lcdc);
        assert_eq!(result, true);
    }
}