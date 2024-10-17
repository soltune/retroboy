use crate::emulator::{Emulator, Mode, in_color_bios};
use crate::gpu::constants::{GB_SCREEN_WIDTH, BYTES_PER_COLOR};
use crate::gpu::sprites::read_sprite_pixel_color;
use crate::gpu::background::read_bg_color;
use crate::gpu::prioritization::resolve_highest_priority_pixel;
use crate::gpu::window::read_window_color;
use crate::gpu::utils::{get_bg_and_window_enabled_mode, get_lcd_enabled_mode};

pub fn write_scanline(emulator: &mut Emulator) {
    let ly = emulator.gpu.registers.ly;
    let scx = emulator.gpu.registers.scx;
    let scy = emulator.gpu.registers.scy;
    let lcdc = emulator.gpu.registers.lcdc;

    let y = scy.wrapping_add(ly);

    let lcd_enabled = !in_color_bios(emulator) && get_lcd_enabled_mode(lcdc);

    if lcd_enabled {
        for viewport_x in 0..GB_SCREEN_WIDTH as u8 {
            let x = scx.wrapping_add(viewport_x);

            let bg_pixel = read_window_color(emulator, viewport_x, ly)
                .unwrap_or(read_bg_color(emulator, x, y));

            let maybe_sprite_pixel = read_sprite_pixel_color(emulator, viewport_x, ly);

            let cgb_mode = emulator.mode == Mode::CGB;
            let lcdc_bg_and_window_priority = get_bg_and_window_enabled_mode(lcdc);
            let color = resolve_highest_priority_pixel(cgb_mode, lcdc_bg_and_window_priority, bg_pixel, maybe_sprite_pixel);

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