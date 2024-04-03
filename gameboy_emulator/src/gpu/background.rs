use crate::emulator::Emulator;
use crate::gpu::colors::as_bg_color_rgb;
use crate::gpu::line_addressing::{resolve_bg_tile_index_address, resolve_tile_data_address};
use crate::mmu;

fn resolve_line_address(emulator: &Emulator, y: u8, column_tile_offset: u8, row_tile_offset: u8) -> u16 {
    let lcdc = emulator.gpu.registers.lcdc;
    let tile_index_address = resolve_bg_tile_index_address(lcdc, column_tile_offset, row_tile_offset);
    let tile_index = mmu::read_byte(emulator, tile_index_address);
    let tile_data_address = resolve_tile_data_address(lcdc, tile_index);
    tile_data_address + ((y % 8) * 2) as u16
}

pub fn read_bg_rgb(emulator: &Emulator, x: u8, y: u8) -> u32 {
    let palette = emulator.gpu.registers.palette;

    let column_tile_offset = y / 8;
    let row_tile_offset = x / 8;
    let line_address = resolve_line_address(emulator, y, column_tile_offset, row_tile_offset);
    let lsb_byte = mmu::read_byte(emulator, line_address);
    let msb_byte = mmu::read_byte(emulator, line_address + 1);

    let bit_index = x % 8;

    as_bg_color_rgb(bit_index, palette, msb_byte, lsb_byte)
}
