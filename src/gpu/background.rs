use crate::emulator::{Emulator, Mode};
use crate::gpu::colors::{as_bg_color_rgb, as_cgb_bg_color_rgb};
use crate::gpu::line_addressing::{calculate_bg_tile_map_index, calculate_tile_data_index, get_cgb_tile_attributes};
use crate::gpu::prioritization::BackgroundPixel;
use crate::gpu::utils::get_tile_line_bytes;

pub fn read_bg_color(emulator: &Emulator, x: u8, y: u8) -> BackgroundPixel {
    let lcdc = emulator.gpu.registers.lcdc;

    let column_tile_offset = y / 8;
    let row_tile_offset = x / 8;

    let tile_map_index = calculate_bg_tile_map_index(lcdc, column_tile_offset, row_tile_offset);
    let tile_index = emulator.gpu.video_ram[tile_map_index as usize];
    let tile_data_index = calculate_tile_data_index(lcdc, tile_index);

    let row_offset = y % 8;
    let bit_index = x % 8;

    if emulator.mode == Mode::CGB {
        let attributes = get_cgb_tile_attributes(emulator, tile_map_index);
        let (lsb_byte, msb_byte) = get_tile_line_bytes(&emulator.gpu, tile_data_index, row_offset, attributes.y_flip, attributes.from_bank_one);
        let color = as_cgb_bg_color_rgb(&emulator.gpu.registers.palettes, bit_index, attributes.palette_number, msb_byte, lsb_byte, attributes.x_flip);
        BackgroundPixel { color, prioritize_bg: attributes.priority }
    }
    else {
        let palette = emulator.gpu.registers.palettes.bgp;
        let (lsb_byte, msb_byte) = get_tile_line_bytes(&emulator.gpu, tile_data_index, row_offset, false, false);
        let color = as_bg_color_rgb(bit_index, palette, msb_byte, lsb_byte);
        BackgroundPixel { color, prioritize_bg: false }
    }
}