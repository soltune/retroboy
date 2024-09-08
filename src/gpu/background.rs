use crate::emulator::{Emulator, Mode};
use crate::gpu::colors::{Color, as_bg_color_rgb, as_cgb_bg_color_rgb};
use crate::gpu::line_addressing::{calculate_bg_tile_map_index, calculate_tile_data_index, get_cgb_tile_attributes};
use crate::gpu::utils::get_bg_and_window_enabled_mode;

pub fn read_bg_color(emulator: &Emulator, x: u8, y: u8) -> Color {
    let lcdc = emulator.gpu.registers.lcdc;

    let background_and_window_enabled = get_bg_and_window_enabled_mode(lcdc);

    if background_and_window_enabled {
        let column_tile_offset = y / 8;
        let row_tile_offset = x / 8;

        let tile_map_index = calculate_bg_tile_map_index(lcdc, column_tile_offset, row_tile_offset);
        let tile_index = emulator.gpu.video_ram[tile_map_index as usize];
        let tile_data_index = calculate_tile_data_index(lcdc, tile_index);
        let line_index = tile_data_index + ((y % 8) * 2) as u16;
        
        let lsb_byte = emulator.gpu.video_ram[line_index as usize];
        let msb_byte = emulator.gpu.video_ram[(line_index + 1) as usize];
    
        let bit_index = x % 8;
    
        if emulator.mode == Mode::CGB {
            let attributes = get_cgb_tile_attributes(emulator, tile_map_index);
            as_cgb_bg_color_rgb(&emulator.gpu.registers.palettes, bit_index, attributes.palette_number, msb_byte, lsb_byte, attributes.x_flip)
        }
        else {
            let palette = emulator.gpu.registers.palettes.bgp;
            as_bg_color_rgb(bit_index, palette, msb_byte, lsb_byte)
        }
    }
    else {
        [0xFF, 0xFF, 0xFF, 0xFF]
    }
}
