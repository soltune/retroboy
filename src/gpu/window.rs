use crate::emulator::Emulator;
use crate::gpu::colors::{as_bg_color_rgb, Color};
use crate::gpu::line_addressing::{resolve_window_tile_index_address, resolve_tile_data_address};
use crate::gpu::utils::{get_window_enabled_mode, get_bg_and_window_enabled_mode};
use crate::mmu;

fn resolve_line_address(emulator: &Emulator, y: u8, column_tile_offset: u8, row_tile_offset: u8) -> u16 {
    let lcdc = emulator.gpu.registers.lcdc;
    let tile_index_address = resolve_window_tile_index_address(lcdc, column_tile_offset, row_tile_offset);
    let tile_index = mmu::read_byte(emulator, tile_index_address);
    let tile_data_address = resolve_tile_data_address(lcdc, tile_index);
    tile_data_address + ((y % 8) * 2) as u16
}

pub fn read_window_color(emulator: &Emulator, x: u8, y: u8) -> Option<Color> {
    let wx = emulator.gpu.registers.wx;
    let wy = emulator.gpu.registers.wy;
    let lcdc = emulator.gpu.registers.lcdc;
    let palette = emulator.gpu.registers.palette;

    let x_int = x as i16;
    let wx_int = wx as i16;

    let background_and_window_enabled = get_bg_and_window_enabled_mode(lcdc);
    let window_enabled = get_window_enabled_mode(lcdc);

    if background_and_window_enabled && window_enabled && x_int >= wx_int - 7 && y >= wy {
        let column_tile_offset = (y - wy) / 8;
        let row_tile_offset = ((x_int - (wx_int - 7)) / 8) as u8;
        let line_address = resolve_line_address(emulator, y, column_tile_offset, row_tile_offset);
        let lsb_byte = mmu::read_byte(emulator, line_address);
        let msb_byte = mmu::read_byte(emulator, line_address + 1);

        let bit_index = ((x_int - (wx_int - 7)) % 8) as u8;

        Some(as_bg_color_rgb(bit_index, palette, msb_byte, lsb_byte))
    }  
    else {
        None
    }
}
