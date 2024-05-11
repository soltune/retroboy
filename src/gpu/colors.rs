use crate::utils::get_bit;

// Each RGBA color is represented in four bytes.
pub type Color = [u8; 4];

pub const BLACK: Color = [0x0, 0x0, 0x0, 0xFF];
pub const DARK_GRAY: Color = [0xA9, 0xA9, 0xA9, 0xFF];
pub const LIGHT_GRAY: Color = [0xD3, 0xD3, 0xD3, 0xFF];
pub const WHITE: Color = [0xFF, 0xFF, 0xFF, 0xFF];

fn calculate_color_id(bit_index: u8, msb_byte: u8, lsb_byte: u8, x_flip: bool) -> u8 {
    let calculated_index = if x_flip { bit_index } else { 7 - bit_index };
    let msb = get_bit(msb_byte, calculated_index);
    let lsb = get_bit(lsb_byte, calculated_index);
    (msb * 2) + lsb
}

fn decode_color_key(color_key: u8) -> Color {
    match color_key {
        0b11 => BLACK,
        0b10 => DARK_GRAY,
        0b01 => LIGHT_GRAY,
        _ => WHITE
    }
}

fn as_bg_color_key(color_id: u8, palette: u8) -> u8 {
    match color_id {
        0b11 => (palette & 0b11000000) >> 6,
        0b10 => (palette & 0b00110000) >> 4,
        0b01 => (palette & 0b00001100) >> 2,
        _ => palette & 0b00000011
    }
}

fn as_obj_color_key(color_id: u8, palette: u8) -> Option<u8> {
    match color_id {
        0b11 => Some((palette & 0b11000000) >> 6),
        0b10 => Some((palette & 0b00110000) >> 4),
        0b01 => Some((palette & 0b00001100) >> 2),
        _ => None 
    }
}

pub fn as_bg_color_rgb(bit_index: u8, palette: u8, msb_byte: u8, lsb_byte: u8) -> Color {
    let color_id = calculate_color_id(bit_index, msb_byte, lsb_byte, false);
    let key = as_bg_color_key(color_id, palette); 
    decode_color_key(key)
}

pub fn as_obj_color_rgb(bit_index: u8, palette: u8, msb_byte: u8, lsb_byte: u8, x_flip: bool) -> Option<Color> {
    let color_id = calculate_color_id(bit_index, msb_byte, lsb_byte, x_flip);
    let maybe_key = as_obj_color_key(color_id, palette); 
    maybe_key.map(decode_color_key)
}
