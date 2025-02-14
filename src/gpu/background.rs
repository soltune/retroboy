use crate::emulator::{is_cgb, Emulator};
use crate::gpu::has_dmg_compatability;
use crate::gpu::colors::{as_dmg_bg_color_rgb, as_cgb_bg_color_rgb, calculate_color_id};
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

    if is_cgb(emulator) {
        let attributes = get_cgb_tile_attributes(emulator, tile_map_index);
        let (lsb_byte, msb_byte) = get_tile_line_bytes(&emulator.gpu, tile_data_index, row_offset, attributes.y_flip, attributes.from_bank_one);

        let dmg_compatible = has_dmg_compatability(emulator);
        let palette_number = if dmg_compatible { 0 } else { attributes.palette_number };
        let color_id = calculate_color_id(bit_index, msb_byte, lsb_byte, attributes.x_flip);
        let color = as_cgb_bg_color_rgb(&emulator.gpu.registers.palettes, palette_number, color_id, dmg_compatible);

        BackgroundPixel { color, color_id, prioritize_bg: attributes.priority }
    }
    else {
        let (lsb_byte, msb_byte) = get_tile_line_bytes(&emulator.gpu, tile_data_index, row_offset, false, false);

        let color_id = calculate_color_id(bit_index, msb_byte, lsb_byte, false);
        let color = as_dmg_bg_color_rgb(&emulator.gpu.registers.palettes, color_id);
        
        BackgroundPixel { color, color_id, prioritize_bg: false }
    }
}