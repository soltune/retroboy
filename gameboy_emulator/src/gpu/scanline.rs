use crate::emulator::Emulator;
use crate::gpu::constants::GB_SCREEN_WIDTH;
use crate::gpu::sprites::read_sprite_pixel_rgb;
use crate::gpu::background::read_bg_rgb;
use crate::gpu::window::read_window_rgb;
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

            let bg_rgb = read_window_rgb(emulator, x, y)
                .unwrap_or(read_bg_rgb(emulator, x, y));

            let sprite_rgb = read_sprite_pixel_rgb(emulator, viewport_x, ly, bg_rgb);

            let rgb = sprite_rgb.unwrap_or(bg_rgb);

            let pixel_index = (ly as u16 * GB_SCREEN_WIDTH + viewport_x as u16) as usize;
            emulator.gpu.frame_buffer[pixel_index] = rgb;
        } 
    }
}

#[cfg(test)]
mod tests;