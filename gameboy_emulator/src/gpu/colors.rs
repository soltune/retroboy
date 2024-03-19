use crate::utils::get_bit;

const BLACK: u32 = 0x000000;
const DARK_GRAY: u32 = 0xA9A9A9;
const LIGHT_GRAY: u32 = 0xD3D3D3;
const WHITE: u32 = 0xFFFFFF;

fn calculate_color_id(bit_index: u8, msb_byte: u8, lsb_byte: u8) -> u8 {
    let msb = get_bit(msb_byte, bit_index);
    let lsb = get_bit(lsb_byte, bit_index);
    (msb * 2) + lsb
}

fn decode_color_key(color_key: u8) -> u32 {
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

pub fn as_bg_color_rgb(bit_index: u8, palette: u8, msb_byte: u8, lsb_byte: u8) -> u32 {
    let color_id = calculate_color_id(bit_index, msb_byte, lsb_byte);
    let key = as_bg_color_key(color_id, palette); 
    decode_color_key(key)
}

pub fn as_obj_color_rgb(bit_index: u8, palette: u8, msb_byte: u8, lsb_byte: u8) -> Option<u32> {
    let color_id = calculate_color_id(bit_index, msb_byte, lsb_byte);
    let maybe_key = as_obj_color_key(color_id, palette); 
    maybe_key.map(decode_color_key)
}
