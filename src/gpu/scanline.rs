use crate::emulator::Emulator;
use crate::gpu::constants::{GB_SCREEN_WIDTH, BYTES_PER_COLOR};
use crate::gpu::sprites::read_sprite_pixel_color;
use crate::gpu::background::read_bg_color;
use crate::gpu::window::read_window_color;
use crate::gpu::utils::get_lcd_enabled_mode;

pub fn write_scanline(emulator: &mut Emulator) {
    let ly = emulator.gpu.registers.ly;
    let scx = emulator.gpu.registers.scx;
    let scy = emulator.gpu.registers.scy;
    let lcdc = emulator.gpu.registers.lcdc;

    let y = scy.wrapping_add(ly);

    let lcd_enabled = get_lcd_enabled_mode(lcdc);

    if lcd_enabled {
        for viewport_x in 0..GB_SCREEN_WIDTH as u8 {
            let x = scx.wrapping_add(viewport_x);

            let bg_color = read_window_color(emulator, x, y)
                .unwrap_or(read_bg_color(emulator, x, y));

            let sprite_color = read_sprite_pixel_color(emulator, viewport_x, ly, bg_color);

            let color = sprite_color.unwrap_or(bg_color);

            let pixel_position = ly as u32 * GB_SCREEN_WIDTH + viewport_x as u32;
            let pixel_index = (pixel_position * BYTES_PER_COLOR) as usize;

            emulator.gpu.frame_buffer[pixel_index] = color[0];
            emulator.gpu.frame_buffer[pixel_index + 1] = color[1];
            emulator.gpu.frame_buffer[pixel_index + 2] = color[2];
            emulator.gpu.frame_buffer[pixel_index + 3] = color[3];
        } 
    }
}

#[cfg(test)]
mod tests;