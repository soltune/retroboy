use crate::utils::{get_bit, is_bit_set};

// Each RGBA color is represented in four bytes.
pub type Color = [u8; 4];

pub const BLACK: Color = [0x0, 0x0, 0x0, 0xFF];
pub const DARK_GRAY: Color = [0xA9, 0xA9, 0xA9, 0xFF];
pub const LIGHT_GRAY: Color = [0xD3, 0xD3, 0xD3, 0xFF];
pub const WHITE: Color = [0xFF, 0xFF, 0xFF, 0xFF];

pub const COLORS_PER_PALETTE: usize = 4;
pub const CGB_PALETTES: usize = 8;

#[derive(Debug)]
pub struct Palettes {
    pub bgp: u8,
    pub obp0: u8,
    pub obp1: u8,
    pub cgb_bcpd: [u16; COLORS_PER_PALETTE * CGB_PALETTES],
    pub cgb_ocpd: [u16; COLORS_PER_PALETTE * CGB_PALETTES],
    pub cgb_bcps: u8,
    pub cgb_ocps: u8
}

pub fn initialize_palettes() -> Palettes {
    Palettes {
        bgp: 0,
        obp0: 0,
        obp1: 0,
        cgb_bcpd: [0; COLORS_PER_PALETTE * CGB_PALETTES],
        cgb_ocpd: [0; COLORS_PER_PALETTE * CGB_PALETTES],
        cgb_bcps: 0,
        cgb_ocps: 0
    }
}

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

fn calculate_palette_data_index(palette_number: u8, color_id: u8) -> usize {
    (palette_number as usize * COLORS_PER_PALETTE) + color_id as usize
}

fn calculate_palette_data_index_by_address(spec_address: u8) -> usize {
    let palette_number = spec_address / 8;
    let color_id = (spec_address % 8) / 2;
    calculate_palette_data_index(palette_number, color_id)
}

fn rgb555_as_color(rgb555: u16) -> Color {
    let red = ((rgb555 & 0b1111100000000000) >> 11) as u16;
    let green = ((rgb555 & 0b0000011111000000) >> 6) as u16;
    let blue = ((rgb555 & 0b0000000000111110) >> 1) as u16;
    
    // Takes the five bits of each color channel and scales them to eight bits.
    let scaled_red = (red * 0xFF) / 31;
    let scaled_green = (green * 0xFF) / 31;
    let scaled_blue = (blue * 0xFF) / 31;

    [scaled_red as u8, scaled_green as u8, scaled_blue as u8, 0xFF]
}

fn lookup_background_palette(palettes: &Palettes, palette_number: u8, color_id: u8) -> u16 {
    let index = calculate_palette_data_index(palette_number, color_id);
    palettes.cgb_bcpd[index]
}

fn lookup_object_palette(palettes: &Palettes, palette_number: u8, color_id: u8) -> u16 {
    let index = calculate_palette_data_index(palette_number, color_id);
    palettes.cgb_ocpd[index]
}

pub fn as_cgb_bg_color_rgb(palettes: &Palettes, bit_index: u8, palette_number: u8, msb_byte: u8, lsb_byte: u8, x_flip: bool) -> Color {
    let color_id = calculate_color_id(bit_index, msb_byte, lsb_byte, x_flip);
    let palette = lookup_background_palette(palettes, palette_number, color_id);
    rgb555_as_color(palette)
}

pub fn as_cgb_obj_color_rgb(palettes: &Palettes, bit_index: u8, palette_number: u8, msb_byte: u8, lsb_byte: u8, x_flip: bool) -> Option<Color> {
    let color_id = calculate_color_id(bit_index, msb_byte, lsb_byte, x_flip);
    match color_id {
        0b00 => None,
        _ => {
            let palette = lookup_object_palette(palettes, palette_number, color_id);
            Some(rgb555_as_color(palette))
        }
    }
}

fn lookup_palette_byte_by_spec_register(spec_register: u8, palette_data: &[u16]) -> u8 {
    let address = spec_register & 0b00111111;
    let palette_data_index = calculate_palette_data_index_by_address(address); 
    let palette = palette_data[palette_data_index];
    let is_lower_byte = address % 2 == 0;

    // Palettes are stored in little-endian format.
    if is_lower_byte {
        ((palette & 0xFF00) >> 8) as u8
    } else {
        (palette & 0x00FF) as u8
    }
}

fn store_palette_byte_by_spec_register(spec_register: u8, palette_data: &mut [u16], value: u8) -> u8 {
    let address = spec_register & 0b00111111;
    let palette_data_index = calculate_palette_data_index_by_address(address); 
    let palette = palette_data[palette_data_index]; 
    let is_lower_byte = address % 2 == 0;

    // Palettes are stored in little-endian format.
    let updated_word = if is_lower_byte {
        ((value as u16) << 8) | (palette & 0x00FF)
    } else {
        (value as u16) | (palette & 0xFF00)
    };

    palette_data[palette_data_index] = updated_word;

    let should_auto_increment = is_bit_set(spec_register, 7);
    if should_auto_increment {
        (spec_register & 0b11000000) | ((address + 1) & 0b00111111)
    }
    else {
        spec_register
    } 
}

pub fn get_cgb_bcps(palettes: &Palettes) -> u8 {
    palettes.cgb_bcps
}

pub fn set_cgb_bcps(palettes: &mut Palettes, value: u8) {
    palettes.cgb_bcps = value;
}

pub fn get_cgb_ocps(palettes: &Palettes) -> u8 {
    palettes.cgb_ocps
}

pub fn set_cgb_ocps(palettes: &mut Palettes, value: u8) {
    palettes.cgb_ocps = value;
}

pub fn get_cgb_bcpd(palettes: &Palettes) -> u8 {
    lookup_palette_byte_by_spec_register(palettes.cgb_bcps, &palettes.cgb_bcpd)
}

pub fn set_cgb_bcpd(palettes: &mut Palettes, value: u8) {
    let new_bcps = store_palette_byte_by_spec_register(palettes.cgb_bcps, &mut palettes.cgb_bcpd, value);
    palettes.cgb_bcps = new_bcps;
}

pub fn get_cgb_ocpd(palettes: &Palettes) -> u8 {
    lookup_palette_byte_by_spec_register(palettes.cgb_ocps, &palettes.cgb_ocpd)
}

pub fn set_cgb_ocpd(palettes: &mut Palettes, value: u8) {
    let new_ocps = store_palette_byte_by_spec_register(palettes.cgb_ocps, &mut palettes.cgb_ocpd, value);
    palettes.cgb_ocps = new_ocps; 
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_background_palettes(palettes: &mut Palettes) {
        palettes.cgb_bcpd[0] = 0x1111;
        palettes.cgb_bcpd[1] = 0x2222;
        palettes.cgb_bcpd[2] = 0x3333;
        palettes.cgb_bcpd[3] = 0x4444;
        palettes.cgb_bcpd[4] = 0x5555;
        palettes.cgb_bcpd[5] = 0x6666;
        palettes.cgb_bcpd[6] = 0x7777;
        palettes.cgb_bcpd[7] = 0x8888;
        palettes.cgb_bcpd[8] = 0x9999;
        palettes.cgb_bcpd[9] = 0xAAAA;
        palettes.cgb_bcpd[10] = 0xBBBB;
        palettes.cgb_bcpd[11] = 0xCCCC;
        palettes.cgb_bcpd[12] = 0xDDDD;
        palettes.cgb_bcpd[13] = 0xEEEE;
        palettes.cgb_bcpd[14] = 0xFFFF;
    }

    fn setup_test_object_palettes(palettes: &mut Palettes) {
        palettes.cgb_ocpd[0] = 0x1111;
        palettes.cgb_ocpd[1] = 0x2222;
        palettes.cgb_ocpd[2] = 0x3333;
        palettes.cgb_ocpd[3] = 0x4444;
        palettes.cgb_ocpd[4] = 0x5555;
        palettes.cgb_ocpd[5] = 0x6666;
        palettes.cgb_ocpd[6] = 0x7777;
        palettes.cgb_ocpd[7] = 0x8888;
        palettes.cgb_ocpd[8] = 0x9999;
        palettes.cgb_ocpd[9] = 0xAAAA;
        palettes.cgb_ocpd[10] = 0xBBBB;
        palettes.cgb_ocpd[11] = 0xCCCC;
        palettes.cgb_ocpd[12] = 0xDDDD;
        palettes.cgb_ocpd[13] = 0xEEEE;
        palettes.cgb_ocpd[14] = 0xFFFF;
    }

    #[test]
    fn should_lookup_background_palette() {
        let mut palettes = initialize_palettes();

        setup_test_background_palettes(&mut palettes);

        let palette_number = 3;
        let color_id = 1;
        let palette = lookup_background_palette(&palettes, palette_number, color_id);

        assert_eq!(palette, 0xEEEE);
    }

    #[test]
    fn should_lookup_object_palette() {
        let mut palettes = initialize_palettes();

        setup_test_object_palettes(&mut palettes);

        let palette_number = 2;
        let color_id = 2;
        let palette = lookup_object_palette(&palettes, palette_number, color_id);

        assert_eq!(palette, 0xBBBB);
    }

    #[test]
    fn should_lookup_background_palette_byte_via_bcpd() {
        let mut palettes = initialize_palettes();

        setup_test_background_palettes(&mut palettes);
        palettes.cgb_bcpd[1] = 0x22BB;

        set_cgb_bcps(&mut palettes, 0b00000011);
        let palette_byte = get_cgb_bcpd(&palettes);

        assert_eq!(palette_byte, 0xBB);
    }

    #[test]
    fn should_lookup_object_palette_byte_via_ocpd() {
        let mut palettes = initialize_palettes();

        setup_test_object_palettes(&mut palettes);
        palettes.cgb_ocpd[1] = 0x22BB;

        set_cgb_ocps(&mut palettes, 0b00000011);
        let palette_byte = get_cgb_ocpd(&palettes);

        assert_eq!(palette_byte, 0xBB);
    }

    #[test]
    fn should_write_palette_byte_via_bcpd() {
        let mut palettes = initialize_palettes();

        setup_test_background_palettes(&mut palettes);

        set_cgb_bcps(&mut palettes, 0b00000011);
        set_cgb_bcpd(&mut palettes, 0xBB);

        assert_eq!(palettes.cgb_bcpd[1], 0x22BB);
    }

    #[test]
    fn should_auto_increment_bcps_address_when_writing_palette_byte() {
        let mut palettes = initialize_palettes();

        setup_test_background_palettes(&mut palettes);

        // Set bit 7 on BCPS to 1 in order to trigger auto-increment when writing byte.
        set_cgb_bcps(&mut palettes, 0b10000011);
        set_cgb_bcpd(&mut palettes, 0xBB);

        assert_eq!(palettes.cgb_bcps, 0b10000100);
    }

    #[test]
    fn should_auto_increment_ocps_address_when_writing_palette_byte() {
        let mut palettes = initialize_palettes();

        setup_test_object_palettes(&mut palettes);

        // Set bit 7 on OCPS to 1 in order to trigger auto-increment when writing byte.
        set_cgb_ocps(&mut palettes, 0b10000011);
        set_cgb_ocpd(&mut palettes, 0xBB);

        assert_eq!(palettes.cgb_ocps, 0b10000100); 
    }

    #[test]
    fn should_not_auto_increment_bcps_address_when_writing_palette_byte_if_auto_incrementing_is_disabled() {
        let mut palettes = initialize_palettes();

        setup_test_background_palettes(&mut palettes);

        // Set bit 7 on BCPS to 0 in order to disable auto-increment when writing byte.
        set_cgb_bcps(&mut palettes, 0b00000011);
        set_cgb_bcpd(&mut palettes, 0xBB);

        assert_eq!(palettes.cgb_bcps, 0b00000011);
    }

    #[test]
    fn should_not_auto_increment_ocps_address_when_writing_palette_byte_if_auto_incrementing_is_disabled() {
        let mut palettes = initialize_palettes();

        setup_test_object_palettes(&mut palettes);

        // Set bit 7 on OCPS to 0 in order to disable auto-increment when writing byte.
        set_cgb_ocps(&mut palettes, 0b00000011);
        set_cgb_ocpd(&mut palettes, 0xBB);

        assert_eq!(palettes.cgb_ocps, 0b00000011);
    }

    #[test]
    fn should_calculate_background_color() {
        let mut palettes = initialize_palettes();

        setup_test_background_palettes(&mut palettes);

        let palette_number = 3;

        let msb_byte = 0b00101010;
        let lsb_byte = 0b11010101;

        let color = as_cgb_bg_color_rgb(&palettes, 0, palette_number, msb_byte, lsb_byte, false);

        // Palette will be 0b1110111011101110.
        assert_eq!(color, [0xEE, 0xDE, 0xBD, 0xFF]);
    }

    #[test]
    fn should_calculate_object_color() {
        let mut palettes = initialize_palettes();

        setup_test_object_palettes(&mut palettes);

        let palette_number = 3;

        let msb_byte = 0b00101010;
        let lsb_byte = 0b11010101;

        let color = as_cgb_obj_color_rgb(&palettes, 0, palette_number, msb_byte, lsb_byte, false);

        // Palette will be 0b1110111011101110.
        assert_eq!(color, Some([0xEE, 0xDE, 0xBD, 0xFF])); 
    }

    #[test]
    fn should_return_none_when_calculating_object_color_with_color_id_0() {
        let mut palettes = initialize_palettes();

        setup_test_object_palettes(&mut palettes);

        let palette_number = 3;

        // Bit 7 on both bytes set to 0 which will result in a color id of 0.
        let msb_byte = 0b00101010;
        let lsb_byte = 0b01010101;

        let color = as_cgb_obj_color_rgb(&palettes, 0, palette_number, msb_byte, lsb_byte, false);

        assert_eq!(color, None);
    }
}