use crate::emulator::{Emulator, Mode};
use crate::gpu::colors::{Color, as_bg_color_rgb, as_cgb_bg_color_rgb};
use crate::gpu::line_addressing::{calculate_bg_tile_map_index, calculate_tile_data_index};
use crate::gpu::utils::get_bg_and_window_enabled_mode;

fn calculate_line_index(emulator: &Emulator, y: u8, column_tile_offset: u8, row_tile_offset: u8) -> u16 {
    let lcdc = emulator.gpu.registers.lcdc;
    let tile_map_index = calculate_bg_tile_map_index(lcdc, column_tile_offset, row_tile_offset);
    let tile_index = emulator.gpu.video_ram[tile_map_index as usize];
    let tile_data_index = calculate_tile_data_index(lcdc, tile_index);
    tile_data_index + ((y % 8) * 2) as u16
}

pub fn read_bg_color(emulator: &Emulator, x: u8, y: u8) -> Color {
    let lcdc = emulator.gpu.registers.lcdc;
    let palette = emulator.gpu.registers.palettes.bgp;

    let background_and_window_enabled = get_bg_and_window_enabled_mode(lcdc);

    if background_and_window_enabled {
        let column_tile_offset = y / 8;
        let row_tile_offset = x / 8;
        let line_index = calculate_line_index(emulator, y, column_tile_offset, row_tile_offset);
        let lsb_byte = emulator.gpu.video_ram[line_index as usize];
        let msb_byte = emulator.gpu.video_ram[(line_index + 1) as usize];
    
        let bit_index = x % 8;
    
        if emulator.mode == Mode::CGB {
            // TODO: As I progress with implementing CGB support, I need to support looking up the palette
            // number from the attribute of the tile. For now, always set to 0.
            let palette_number = 0;
            as_cgb_bg_color_rgb(&emulator.gpu.registers.palettes, bit_index, palette_number, msb_byte, lsb_byte)
        }
        else {
            as_bg_color_rgb(bit_index, palette, msb_byte, lsb_byte)
        }
    }
    else {
        [0xFF, 0xFF, 0xFF, 0xFF]
    }
}
