use crate::emulator::Emulator;
use crate::gpu::constants::GB_SCREEN_WIDTH;
use crate::gpu::sprites::read_sprite_pixel_rgb;
use crate::gpu::background::read_bg_rgb;
use crate::gpu::window::read_window_rgb;
use crate::gpu::utils::get_lcd_enabled_mode;

const TILE_WIDTH: u8 = 8;

fn within_viewport(scx: u8, leftmost_tile_column: u8) -> bool {
    let rightmost_tile_column = leftmost_tile_column.wrapping_add(TILE_WIDTH);
    let rightmost_viewport_border = scx.wrapping_add(GB_SCREEN_WIDTH as u8);
    rightmost_tile_column >= scx || leftmost_tile_column <= rightmost_viewport_border
}

pub fn write_scanline(emulator: &mut Emulator) {
    let ly = emulator.gpu.registers.ly;
    let scx = emulator.gpu.registers.scx;
    let scy = emulator.gpu.registers.scy;
    let lcdc = emulator.gpu.registers.lcdc;

    let y = scy.wrapping_add(ly);

    let lcd_enabled = get_lcd_enabled_mode(lcdc);

    if lcd_enabled {
        for x in 0..GB_SCREEN_WIDTH as u8 {
            let leftmost_tile_column = (x / 8) * 8;
            if within_viewport(scx, leftmost_tile_column) {
                let viewport_x = x.wrapping_sub(scx);
                let rgb = read_window_rgb(emulator, x, y)
                    .or(read_sprite_pixel_rgb(emulator, viewport_x, ly))
                    .unwrap_or(read_bg_rgb(emulator, x, y));
                let pixel_index = (ly as u16 * GB_SCREEN_WIDTH + viewport_x as u16) as usize;
                emulator.gpu.frame_buffer[pixel_index] = rgb;
            }
        } 
    }
}

#[cfg(test)]
mod tests;