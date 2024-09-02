use crate::emulator::Emulator;
use crate::gpu::colors::{as_bg_color_rgb, Color};
use crate::gpu::line_addressing::{calculate_window_tile_map_index, calculate_tile_data_index};
use crate::gpu::utils::{get_window_enabled_mode, get_bg_and_window_enabled_mode};

fn calculate_line_index(emulator: &Emulator, y: u8, column_tile_offset: u8, row_tile_offset: u8) -> u16 {
    let lcdc = emulator.gpu.registers.lcdc;
    let tile_map_index = calculate_window_tile_map_index(lcdc, column_tile_offset, row_tile_offset);
    let tile_index = emulator.gpu.video_ram[tile_map_index as usize];
    let tile_data_index = calculate_tile_data_index(lcdc, tile_index);
    tile_data_index + ((y % 8) * 2) as u16
}

pub fn read_window_color(emulator: &Emulator, x: u8, y: u8) -> Option<Color> {
    let wx = emulator.gpu.registers.wx;
    let wy = emulator.gpu.registers.wy;
    let lcdc = emulator.gpu.registers.lcdc;
    let palette = emulator.gpu.registers.palettes.bgp;

    let x_int = x as i16;
    let wx_int = wx as i16;

    let background_and_window_enabled = get_bg_and_window_enabled_mode(lcdc);
    let window_enabled = get_window_enabled_mode(lcdc);

    if background_and_window_enabled && window_enabled && x_int >= wx_int - 7 && y >= wy {
        let column_tile_offset = (y - wy) / 8;
        let row_tile_offset = ((x_int - (wx_int - 7)) / 8) as u8;
        let line_index = calculate_line_index(emulator, y, column_tile_offset, row_tile_offset);
        let lsb_byte = emulator.gpu.video_ram[line_index as usize];
        let msb_byte = emulator.gpu.video_ram[(line_index + 1) as usize];

        let bit_index = ((x_int - (wx_int - 7)) % 8) as u8;

        Some(as_bg_color_rgb(bit_index, palette, msb_byte, lsb_byte))
    }  
    else {
        None
    }
}
