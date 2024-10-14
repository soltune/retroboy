use crate::emulator::{initialize_screenless_emulator, Mode};
use crate::gpu::colors::{Color, Palettes, BLACK, DARK_GRAY, LIGHT_GRAY, WHITE};
use crate::gpu::sprites::Sprite;
use super::*;

const BLACK_TILE: [u8; 16] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
const SAMPLE_TILE_A: [u8; 16] = [0x3C, 0x7E, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7E, 0x5E, 0x7E, 0x0A, 0x7C, 0x56, 0x38, 0x7C];
const SAMPLE_TILE_B: [u8; 16] = [0xFF, 0x0, 0x7E, 0xFF, 0x85, 0x81, 0x89, 0x83, 0x93, 0x85, 0xA5, 0x8B, 0xC9, 0x97, 0x7E, 0xFF];
const WINDOW_TILE: [u8; 16] = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];

const RED: Color = [0xFF, 0x0, 0x0, 0xFF];
const BLUE: Color = [0x0, 0x0, 0xFF, 0xFF];

fn write_tile_to_memory(emulator: &mut Emulator, base_index: u16, index: u16, tile_bytes: [u8; 16]) {
    let offset = index * 16;
    for (tile_byte_index, tile_byte) in tile_bytes.iter().enumerate() {
        emulator.gpu.video_ram[(base_index + offset + tile_byte_index as u16) as usize] = *tile_byte;
    }
}

fn write_tile_to_bg_memory(emulator: &mut Emulator, index: u16, tile_bytes: [u8; 16]) {
    write_tile_to_memory(emulator, 0x1000, index, tile_bytes)
}

fn write_tile_to_bg_memory_in_bank_one(emulator: &mut Emulator, index: u16, tile_bytes: [u8; 16]) {
    write_tile_to_memory(emulator, 0x3000, index, tile_bytes)
}

fn write_tile_to_obj_memory(emulator: &mut Emulator, index: u16, tile_bytes: [u8; 16]) {
    write_tile_to_memory(emulator, 0x0000, index, tile_bytes)
}

fn write_tile_attributes(emulator: &mut Emulator, index: u16, attributes: u8) {
    emulator.gpu.video_ram[(0x3800 + index) as usize] = attributes;
}

fn write_sprite_to_sprite_buffer(emulator: &mut Emulator, sprite: Sprite) {
    emulator.gpu.sprite_buffer.push(sprite);
}

fn write_window_tile_index_to_memory(emulator: &mut Emulator, position_index: u16, tile_index: u8) {
    emulator.gpu.video_ram[(0x1C00 + position_index) as usize] = tile_index;
}

fn initialize_monochrome_palettes(palettes: &mut Palettes) {
    // DMG Palette:
    // Black: color id 0
    // Dark Gray: color id 1
    // Light Gray: color id 2
    // White: color id 3 
    palettes.bgp = 0b00011011;
    palettes.obp0 = 0b00011011;
}

fn initialize_color_palettes(palettes: &mut Palettes) {
    // Red
    palettes.cgb_bcpd[8] = 0b00011111;
    palettes.cgb_bcpd[9] = 0;

    // Green
    palettes.cgb_bcpd[10] = 0b11100000;
    palettes.cgb_bcpd[11] = 0b00000011;

    // Blue
    palettes.cgb_bcpd[12] = 0;
    palettes.cgb_bcpd[13] = 0b01111100;
}

struct FrameBufferAssertion<'a> {
    frame_buffer: &'a Vec<u8>,
    coordinates: (u32, u32)
}

impl <'a> FrameBufferAssertion<'a> {
    pub fn new(frame_buffer: &Vec<u8>) -> FrameBufferAssertion {
        FrameBufferAssertion {
            frame_buffer,
            coordinates: (0, 0),
        }
    }

    pub fn at_starting_coordinates(&mut self, coordinates: (u32, u32)) -> &mut Self {
        self.coordinates = coordinates;
        self
    }

    fn assert_pixel_color(&self, frame_buffer: &Vec<u8>, pixel_position: u32, color: Color) {
        let pixel_index = (pixel_position * 4) as usize;
        assert_eq!(frame_buffer[pixel_index], color[0]);
        assert_eq!(frame_buffer[pixel_index + 1], color[1]);
        assert_eq!(frame_buffer[pixel_index + 2], color[2]);
        assert_eq!(frame_buffer[pixel_index + 3], color[3]);
    }

    pub fn has_pixels(&self, color_pixels: &[Color]) {
        let (x, y) = self.coordinates;
        let pixel_position = y * GB_SCREEN_WIDTH + x;

        for (index, color) in color_pixels.iter().enumerate() {
            self.assert_pixel_color(self.frame_buffer, pixel_position + index as u32, *color);
        }
    }
}

fn assert_that(frame_buffer: &Vec<u8>) -> FrameBufferAssertion {
    FrameBufferAssertion::new(frame_buffer)
}

#[test]
fn should_render_nothing_if_lcd_enable_flag_is_off() {
    let mut emulator = initialize_screenless_emulator();

    initialize_monochrome_palettes(&mut emulator.gpu.registers.palettes);

    write_tile_to_bg_memory(&mut emulator, 0, SAMPLE_TILE_A);
    
    emulator.gpu.registers.ly = 0;
    emulator.gpu.registers.lcdc = 0b00000011;
    
    write_scanline(&mut emulator);

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 0))
        .has_pixels(&[WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE]);
}

#[test]
fn should_render_tile_line() {
    let mut emulator = initialize_screenless_emulator();

    initialize_monochrome_palettes(&mut emulator.gpu.registers.palettes);

    write_tile_to_bg_memory(&mut emulator, 0, SAMPLE_TILE_A);
    
    emulator.gpu.registers.ly = 0;
    emulator.gpu.registers.lcdc = 0b10000011;
    
    write_scanline(&mut emulator);

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 0))
        .has_pixels(&[BLACK, LIGHT_GRAY, WHITE, WHITE, WHITE, WHITE, LIGHT_GRAY, BLACK]);
}

#[test]
fn should_render_multiple_tile_lines() {
    let mut emulator = initialize_screenless_emulator();
    
    initialize_monochrome_palettes(&mut emulator.gpu.registers.palettes);

    write_tile_to_bg_memory(&mut emulator, 0, SAMPLE_TILE_A);

    emulator.gpu.registers.lcdc = 0b10000011;

    for _ in 0..3 {
        write_scanline(&mut emulator);
        emulator.gpu.registers.ly += 1;
    }

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 0))
        .has_pixels(&[BLACK, LIGHT_GRAY, WHITE, WHITE, WHITE, WHITE, LIGHT_GRAY, BLACK]);

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 1))
        .has_pixels(&[BLACK, WHITE, BLACK, BLACK, BLACK, BLACK, WHITE, BLACK]);

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 2))
        .has_pixels(&[BLACK, WHITE, BLACK, BLACK, BLACK, BLACK, WHITE, BLACK]);
}

#[test]
fn should_render_multiple_tile_lines_in_color_mode() {
    let mut emulator = initialize_screenless_emulator();
    
    emulator.mode = Mode::CGB;

    initialize_color_palettes(&mut emulator.gpu.registers.palettes);
    
    write_tile_to_bg_memory(&mut emulator, 0, SAMPLE_TILE_A);

    write_tile_attributes(&mut emulator, 0, 0b00000001);

    emulator.gpu.registers.lcdc = 0b10000011;

    for _ in 0..8 {
        write_scanline(&mut emulator);
        emulator.gpu.registers.ly += 1;
    }

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 0))
        .has_pixels(&[RED, BLUE, BLACK, BLACK, BLACK, BLACK, BLUE, RED]);

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 1))
        .has_pixels(&[RED, BLACK, RED, RED, RED, RED, BLACK, RED]);

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 2))
        .has_pixels(&[RED, BLACK, RED, RED, RED, RED, BLACK, RED]);

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 7))
        .has_pixels(&[RED, BLUE, BLACK, BLACK, BLACK, BLUE, RED, RED]); 
}

#[test]
fn should_render_multiple_tile_lines_in_color_mode_from_bank_one() {
    let mut emulator = initialize_screenless_emulator();
    
    emulator.mode = Mode::CGB;

    initialize_color_palettes(&mut emulator.gpu.registers.palettes);
    
    write_tile_to_bg_memory_in_bank_one(&mut emulator, 0, SAMPLE_TILE_A);

    write_tile_attributes(&mut emulator, 0, 0b00001001);

    emulator.gpu.registers.lcdc = 0b10000011;

    for _ in 0..8 {
        write_scanline(&mut emulator);
        emulator.gpu.registers.ly += 1;
    }

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 0))
        .has_pixels(&[RED, BLUE, BLACK, BLACK, BLACK, BLACK, BLUE, RED]);

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 1))
        .has_pixels(&[RED, BLACK, RED, RED, RED, RED, BLACK, RED]);

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 2))
        .has_pixels(&[RED, BLACK, RED, RED, RED, RED, BLACK, RED]);

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 7))
        .has_pixels(&[RED, BLUE, BLACK, BLACK, BLACK, BLUE, RED, RED]); 
}

#[test]
fn should_flip_background_tile_on_y_axis() {
    let mut emulator = initialize_screenless_emulator();
    
    emulator.mode = Mode::CGB;

    initialize_color_palettes(&mut emulator.gpu.registers.palettes);
    
    write_tile_to_bg_memory(&mut emulator, 0, SAMPLE_TILE_A);

    write_tile_attributes(&mut emulator, 0, 0b01000001);

    emulator.gpu.registers.lcdc = 0b10000011;

    for _ in 0..8 {
        write_scanline(&mut emulator);
        emulator.gpu.registers.ly += 1;
    }

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 0))
        .has_pixels(&[RED, BLUE, BLACK, BLACK, BLACK, BLUE, RED, RED]); 

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 5))
        .has_pixels(&[RED, BLACK, RED, RED, RED, RED, BLACK, RED]);

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 6))
        .has_pixels(&[RED, BLACK, RED, RED, RED, RED, BLACK, RED]);

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 7))
        .has_pixels(&[RED, BLUE, BLACK, BLACK, BLACK, BLACK, BLUE, RED]);
}

#[test]
fn should_flip_background_tile_on_x_axis() {
    let mut emulator = initialize_screenless_emulator();
    
    emulator.mode = Mode::CGB;

    initialize_color_palettes(&mut emulator.gpu.registers.palettes);
    
    write_tile_to_bg_memory(&mut emulator, 0, SAMPLE_TILE_A);

    write_tile_attributes(&mut emulator, 0, 0b00100001);

    emulator.gpu.registers.lcdc = 0b10000011;

    for _ in 0..8 {
        write_scanline(&mut emulator);
        emulator.gpu.registers.ly += 1;
    }

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 7))
        .has_pixels(&[RED, RED, BLUE, BLACK, BLACK, BLACK, BLUE, RED]); 
}

#[test]
fn should_overlay_window_over_multiple_tile_lines() {
    let mut emulator = initialize_screenless_emulator();

    initialize_monochrome_palettes(&mut emulator.gpu.registers.palettes);

    write_tile_to_bg_memory(&mut emulator, 0, SAMPLE_TILE_A);
    write_tile_to_bg_memory(&mut emulator, 1, WINDOW_TILE);
    write_window_tile_index_to_memory(&mut emulator, 0, 1);

    emulator.gpu.registers.wy = 1;
    emulator.gpu.registers.wx = 8;
    emulator.gpu.registers.lcdc = 0b11100011;

    for _ in 0..3 {
        write_scanline(&mut emulator);
        emulator.gpu.registers.ly += 1;
    }

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 0))
        .has_pixels(&[BLACK, LIGHT_GRAY, WHITE, WHITE, WHITE, WHITE, LIGHT_GRAY, BLACK]);

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 1))
        .has_pixels(&[BLACK, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE]);

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 2))
        .has_pixels(&[BLACK, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE]);
}

#[test]
fn should_render_tile_line_in_middle_of_frame() {
    let mut emulator = initialize_screenless_emulator();

    initialize_monochrome_palettes(&mut emulator.gpu.registers.palettes);
    
    write_tile_to_bg_memory(&mut emulator, 1, SAMPLE_TILE_A);
    
    emulator.gpu.video_ram[0x1A10] = 0x1;
    emulator.gpu.registers.ly = 3;
    emulator.gpu.registers.scy = 0x80;
    emulator.gpu.registers.scx = 0x80;
    emulator.gpu.registers.lcdc = 0b10000011;
    
    write_scanline(&mut emulator);

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 3))
        .has_pixels(&[BLACK, WHITE, BLACK, BLACK, BLACK, BLACK, WHITE, BLACK]);
}

#[test]
fn should_render_tile_line_properly_with_greater_scroll_x_value() {
    let mut emulator = initialize_screenless_emulator();
    
    initialize_monochrome_palettes(&mut emulator.gpu.registers.palettes);

    write_tile_to_bg_memory(&mut emulator, 1, SAMPLE_TILE_A);
    
    emulator.gpu.video_ram[0x1A10] = 0x1;
    emulator.gpu.registers.ly = 3;
    emulator.gpu.registers.scy = 0x80;
    emulator.gpu.registers.scx = 0x82;
    emulator.gpu.registers.lcdc = 0b10000011;
    
    write_scanline(&mut emulator);

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 3))
        .has_pixels(&[BLACK, BLACK, BLACK, BLACK, WHITE, BLACK, BLACK, BLACK]);
}

#[test]
fn should_wrap_around_when_rendering_past_max_tile_map_x_value() {
    let mut emulator = initialize_screenless_emulator();

    initialize_monochrome_palettes(&mut emulator.gpu.registers.palettes);
    
    write_tile_to_bg_memory(&mut emulator, 1, SAMPLE_TILE_A);
    
    emulator.gpu.video_ram[0x1800] = 0x1;
    emulator.gpu.registers.ly = 0;
    emulator.gpu.registers.scx = 0xFE;
    emulator.gpu.registers.lcdc = 0b10000011;
    
    write_scanline(&mut emulator);

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 0))
        .has_pixels(&[BLACK, BLACK, BLACK, LIGHT_GRAY, WHITE, WHITE, WHITE, WHITE]);
}

#[test]
fn should_wrap_around_when_rendering_past_max_tile_map_y_value() {
    let mut emulator = initialize_screenless_emulator();

    initialize_monochrome_palettes(&mut emulator.gpu.registers.palettes);
    
    write_tile_to_bg_memory(&mut emulator, 1, SAMPLE_TILE_A);
    
    emulator.gpu.video_ram[0x1800] = 0x1;
    emulator.gpu.registers.ly = 2;
    emulator.gpu.registers.scy = 0xFE;
    emulator.gpu.registers.lcdc = 0b10000011;
    
    write_scanline(&mut emulator);

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 2))
        .has_pixels(&[BLACK, LIGHT_GRAY, WHITE, WHITE, WHITE, WHITE, LIGHT_GRAY, BLACK]);
}

#[test]
fn should_render_tile_line_with_sprite() {
    let mut emulator = initialize_screenless_emulator();

    initialize_monochrome_palettes(&mut emulator.gpu.registers.palettes);

    write_tile_to_bg_memory(&mut emulator, 0, SAMPLE_TILE_A);
    write_tile_to_obj_memory(&mut emulator, 1, SAMPLE_TILE_B);

    write_sprite_to_sprite_buffer(&mut emulator, Sprite {
        y_pos: 0,
        x_pos: 2,
        tile_index: 1,
        priority: false,
        y_flip: false,
        x_flip: false,
        dmg_palette: 0,
        oam_index: 0,
        cgb_from_bank_one: false,
        cgb_palette: 0
    });

    emulator.gpu.registers.ly = 0;
    emulator.gpu.registers.lcdc = 0b10000011;

    write_scanline(&mut emulator);

    let frame_buffer: &Vec<u8> = &emulator.gpu.frame_buffer;

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 0))
        .has_pixels(&[BLACK, LIGHT_GRAY, DARK_GRAY, DARK_GRAY, DARK_GRAY, DARK_GRAY, DARK_GRAY, DARK_GRAY]);
}

#[test]
fn should_render_sprite_with_white_background_if_background_and_window_enabled_is_cleared() {
    let mut emulator = initialize_screenless_emulator();

    initialize_monochrome_palettes(&mut emulator.gpu.registers.palettes);

    write_tile_to_bg_memory(&mut emulator, 0, SAMPLE_TILE_A);
    write_tile_to_obj_memory(&mut emulator, 1, SAMPLE_TILE_B);

    write_sprite_to_sprite_buffer(&mut emulator, Sprite {
        y_pos: 0,
        x_pos: 2,
        tile_index: 1,
        priority: false,
        y_flip: false,
        x_flip: false,
        dmg_palette: 0,
        oam_index: 0,
        cgb_from_bank_one: false,
        cgb_palette: 0
    });

    emulator.gpu.registers.ly = 0;
    emulator.gpu.registers.lcdc = 0b10000010;

    write_scanline(&mut emulator);

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 0))
        .has_pixels(&[WHITE, WHITE, DARK_GRAY, DARK_GRAY, DARK_GRAY, DARK_GRAY, DARK_GRAY, DARK_GRAY]);
}

#[test]
fn should_render_tile_line_with_sprite_having_negative_y_pos() {
    let mut emulator = initialize_screenless_emulator();

    initialize_monochrome_palettes(&mut emulator.gpu.registers.palettes);

    write_tile_to_bg_memory(&mut emulator, 0, SAMPLE_TILE_A);
    write_tile_to_obj_memory(&mut emulator, 1, SAMPLE_TILE_B);

    write_sprite_to_sprite_buffer(&mut emulator, Sprite {
        y_pos: -2,
        x_pos: 2,
        tile_index: 1,
        priority: false,
        y_flip: false,
        x_flip: false,
        dmg_palette: 0,
        oam_index: 0,
        cgb_from_bank_one: false,
        cgb_palette: 0
    });

    emulator.gpu.registers.ly = 0;
    emulator.gpu.registers.lcdc = 0b10000011;

    write_scanline(&mut emulator);

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 0))
        .has_pixels(&[BLACK, LIGHT_GRAY, WHITE, WHITE, WHITE, WHITE, LIGHT_GRAY, DARK_GRAY]);
}

#[test]
fn should_flip_sprite_on_x_axis() {
    let mut emulator = initialize_screenless_emulator();

    initialize_monochrome_palettes(&mut emulator.gpu.registers.palettes);

    write_tile_to_bg_memory(&mut emulator, 0, SAMPLE_TILE_A);
    write_tile_to_obj_memory(&mut emulator, 1, SAMPLE_TILE_B);

    write_sprite_to_sprite_buffer(&mut emulator, Sprite {
        y_pos: -2,
        x_pos: 2,
        tile_index: 1,
        priority: false,
        y_flip: false,
        x_flip: true,
        dmg_palette: 0,
        oam_index: 0,
        cgb_from_bank_one: false,
        cgb_palette: 0
    });

    emulator.gpu.registers.ly = 0;
    emulator.gpu.registers.lcdc = 0b10000011;

    write_scanline(&mut emulator);

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 0))
        .has_pixels(&[BLACK, LIGHT_GRAY, WHITE, WHITE, DARK_GRAY, WHITE, LIGHT_GRAY, BLACK]);
}

#[test]
fn should_flip_sprite_on_y_axis() {
    let mut emulator = initialize_screenless_emulator();

    initialize_monochrome_palettes(&mut emulator.gpu.registers.palettes);

    write_tile_to_bg_memory(&mut emulator, 0, SAMPLE_TILE_A);
    write_tile_to_obj_memory(&mut emulator, 1, SAMPLE_TILE_B);

    write_sprite_to_sprite_buffer(&mut emulator, Sprite {
        y_pos: -2,
        x_pos: 2,
        tile_index: 1,
        priority: false,
        y_flip: true,
        x_flip: false,
        dmg_palette: 0,
        oam_index: 0,
        cgb_from_bank_one: false,
        cgb_palette: 0
    });
    
    emulator.gpu.registers.ly = 0;
    emulator.gpu.registers.lcdc = 0b10000011;

    write_scanline(&mut emulator);

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 0))
        .has_pixels(&[BLACK, LIGHT_GRAY, WHITE, WHITE, DARK_GRAY, WHITE, LIGHT_GRAY, DARK_GRAY]);
}

#[test]
fn should_render_eight_by_sixteen_sprite() {
    let mut emulator = initialize_screenless_emulator();

    initialize_monochrome_palettes(&mut emulator.gpu.registers.palettes);

    write_tile_to_bg_memory(&mut emulator, 0, BLACK_TILE);
    write_tile_to_obj_memory(&mut emulator, 2, SAMPLE_TILE_A);
    write_tile_to_obj_memory(&mut emulator, 3, SAMPLE_TILE_B);

    write_sprite_to_sprite_buffer(&mut emulator, Sprite {
        y_pos: 0,
        x_pos: 2,
        tile_index: 3,
        priority: false,
        y_flip: false,
        x_flip: false,
        dmg_palette: 0,
        oam_index: 0,
        cgb_from_bank_one: false,
        cgb_palette: 0
    });

    emulator.gpu.registers.ly = 0;
    emulator.gpu.registers.lcdc = 0b10000111;

    for _ in 0..9 {
        write_scanline(&mut emulator);
        emulator.gpu.registers.ly += 1;
    }

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 0))
        .has_pixels(&[BLACK, BLACK, BLACK, LIGHT_GRAY, WHITE, WHITE, WHITE, WHITE]);

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 8))
        .has_pixels(&[BLACK, BLACK, DARK_GRAY, DARK_GRAY, DARK_GRAY, DARK_GRAY, DARK_GRAY, DARK_GRAY]);
}

#[test]
fn should_prioritize_non_white_background_colors_when_sprite_priority_flag_set_to_true() {
    let mut emulator = initialize_screenless_emulator();

    initialize_monochrome_palettes(&mut emulator.gpu.registers.palettes);

    write_tile_to_bg_memory(&mut emulator, 0, SAMPLE_TILE_A);
    write_tile_to_obj_memory(&mut emulator, 1, SAMPLE_TILE_B);
    
    write_sprite_to_sprite_buffer(&mut emulator, Sprite {
        y_pos: 0,
        x_pos: 2,
        tile_index: 1,
        priority: true,
        y_flip: false,
        x_flip: false,
        dmg_palette: 0,
        oam_index: 0,
        cgb_from_bank_one: false,
        cgb_palette: 0
    });

    emulator.gpu.registers.ly = 0;
    emulator.gpu.registers.lcdc = 0b10000011;

    write_scanline(&mut emulator);

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 0))
        .has_pixels(&[BLACK, LIGHT_GRAY, DARK_GRAY, DARK_GRAY, DARK_GRAY, DARK_GRAY, LIGHT_GRAY, BLACK]);
}

#[test]
fn should_prioritize_background_colors_when_lcdc_bit_1_is_off() {
    let mut emulator = initialize_screenless_emulator();

    initialize_monochrome_palettes(&mut emulator.gpu.registers.palettes);

    write_tile_to_bg_memory(&mut emulator, 0, SAMPLE_TILE_A);
    write_tile_to_obj_memory(&mut emulator, 1, SAMPLE_TILE_B);

    write_sprite_to_sprite_buffer(&mut emulator, Sprite {
        y_pos: 0,
        x_pos: 2,
        tile_index: 1,
        priority: false,
        y_flip: false,
        x_flip: false,
        dmg_palette: 0,
        oam_index: 0,
        cgb_from_bank_one: false,
        cgb_palette: 0
    });
    
    emulator.gpu.registers.ly = 0;
    emulator.gpu.registers.lcdc = 0b10000001;

    write_scanline(&mut emulator);

    let frame_buffer = &emulator.gpu.frame_buffer;

    assert_that(frame_buffer)
        .at_starting_coordinates((0, 0))
        .has_pixels(&[BLACK, LIGHT_GRAY, WHITE, WHITE, WHITE, WHITE, LIGHT_GRAY, BLACK]);
}