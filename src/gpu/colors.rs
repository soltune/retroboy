use crate::utils::{get_bit, is_bit_set};
use bincode::{Encode, Decode};

// Each RGBA color is represented in four bytes.
pub type Color = [u8; 4];

pub const BLACK: Color = [0x0, 0x0, 0x0, 0xFF];
pub const DARK_GRAY: Color = [0xA9, 0xA9, 0xA9, 0xFF];
pub const LIGHT_GRAY: Color = [0xD3, 0xD3, 0xD3, 0xFF];
pub const WHITE: Color = [0xFF, 0xFF, 0xFF, 0xFF];

pub const COLORS_PER_PALETTE: usize = 4;
pub const CGB_PALETTES: usize = 8;

const MONOCHROME_COLORS: [Color; 4] = [WHITE, LIGHT_GRAY, DARK_GRAY, BLACK];

#[derive(Clone, Debug, Encode, Decode)]
pub struct Palettes {
    pub bgp: u8,
    pub obp0: u8,
    pub obp1: u8,
    pub cgb_bcpd: [u8; COLORS_PER_PALETTE * CGB_PALETTES * 2],
    pub cgb_ocpd: [u8; COLORS_PER_PALETTE * CGB_PALETTES * 2],
    pub cgb_bcps: u8,
    pub cgb_ocps: u8
}

pub fn initialize_palettes() -> Palettes {
    Palettes {
        bgp: 0,
        obp0: 0,
        obp1: 0,
        cgb_bcpd: [0; COLORS_PER_PALETTE * CGB_PALETTES * 2],
        cgb_ocpd: [0; COLORS_PER_PALETTE * CGB_PALETTES * 2],
        cgb_bcps: 0,
        cgb_ocps: 0
    }
}

pub fn calculate_color_id(bit_index: u8, msb_byte: u8, lsb_byte: u8, x_flip: bool) -> u8 {
    let calculated_index = if x_flip { bit_index } else { 7 - bit_index };
    let msb = get_bit(msb_byte, calculated_index);
    let lsb = get_bit(lsb_byte, calculated_index);
    (msb * 2) + lsb
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

fn resolve_dmg_object_palette(palettes: &Palettes, palette_number: u8) -> u8 {
    if palette_number == 0 {
        palettes.obp0
    }
    else {
        palettes.obp1
    }
}

fn as_dmg_bg_color_key(palettes: &Palettes, color_id: u8) -> u8 {
    let palette = palettes.bgp;
    as_bg_color_key(color_id, palette)
}

fn as_dmg_obj_color_key(palettes: &Palettes, palette_number: u8, color_id: u8) -> Option<u8> {
    let palette = resolve_dmg_object_palette(palettes, palette_number);
    as_obj_color_key(color_id, palette)
}

pub fn as_dmg_bg_color_rgb(palettes: &Palettes, color_id: u8) -> Color {
    let key = as_dmg_bg_color_key(palettes, color_id);
    MONOCHROME_COLORS[key as usize]
}

pub fn as_dmg_obj_color_rgb(palettes: &Palettes, palette_number: u8, color_id: u8) -> Option<Color> {
    let maybe_key = as_dmg_obj_color_key(palettes, palette_number, color_id);
    maybe_key.map(|key| MONOCHROME_COLORS[key as usize])
}

fn calculate_cgb_palette_data_index(palette_number: u8, color_id: u8) -> usize {
    ((palette_number as usize * COLORS_PER_PALETTE) + color_id as usize) * 2
}

fn rgb555_as_color(rgb555: u16) -> Color {
    let red = rgb555 & 0b11111;
    let green = (rgb555 >> 5) & 0b11111;
    let blue = (rgb555 >> 10) & 0b11111;

    // Takes the five bits of each color channel and scales them to eight bits.
    let scaled_red = (red * 0xFF) / 31;
    let scaled_green = (green * 0xFF) / 31;
    let scaled_blue = (blue * 0xFF) / 31;

    [scaled_red as u8, scaled_green as u8, scaled_blue as u8, 0xFF]
}

fn lookup_cgb_background_palette(palettes: &Palettes, palette_number: u8, color_id: u8) -> u16 {
    let index = calculate_cgb_palette_data_index(palette_number, color_id);
    palettes.cgb_bcpd[index & !1] as u16 | ((palettes.cgb_bcpd[index | 1] as u16) << 8)
}

fn lookup_cgb_object_palette(palettes: &Palettes, palette_number: u8, color_id: u8) -> u16 {
    let index = calculate_cgb_palette_data_index(palette_number, color_id);
    palettes.cgb_ocpd[index & !1] as u16 | ((palettes.cgb_ocpd[index | 1] as u16) << 8)
}

pub fn as_cgb_bg_color_rgb(palettes: &Palettes, palette_number: u8, color_id: u8, dmg_compatible: bool) -> Color {
    let palette = if dmg_compatible {
        let key = as_dmg_bg_color_key(palettes, color_id);
        lookup_cgb_background_palette(palettes, palette_number, key)
    }
    else {
        lookup_cgb_background_palette(palettes, palette_number, color_id)
    };
    rgb555_as_color(palette)
}

pub fn as_cgb_obj_color_rgb(palettes: &Palettes, palette_number: u8, color_id: u8, dmg_compatible: bool) -> Option<Color> {
    let maybe_palette = match color_id {
        0b00 => None,
        _ => {
            if dmg_compatible {
                let maybe_key = as_dmg_obj_color_key(palettes, palette_number, color_id);
                maybe_key.map(|key| lookup_cgb_object_palette(palettes, palette_number, key)) 
            }
            else {
                Some(lookup_cgb_object_palette(palettes, palette_number, color_id))
            }
        }
    };
    maybe_palette.map(rgb555_as_color)
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
    let address = (palettes.cgb_bcps & 0b00111111) as usize;
    palettes.cgb_bcpd[address]
}

pub fn set_cgb_bcpd(palettes: &mut Palettes, value: u8) {
    let address = (palettes.cgb_bcps & 0b00111111) as usize;
    palettes.cgb_bcpd[address] = value;

    let should_auto_increment = is_bit_set(palettes.cgb_bcps, 7);
    
    palettes.cgb_bcps = if should_auto_increment {
        (palettes.cgb_bcps + 1) | 0x80
    }
    else {
        palettes.cgb_bcps
    };
}

pub fn get_cgb_ocpd(palettes: &Palettes) -> u8 {
    let address = (palettes.cgb_ocps & 0b00111111) as usize;
    palettes.cgb_ocpd[address]
}

pub fn set_cgb_ocpd(palettes: &mut Palettes, value: u8) {
    let address = (palettes.cgb_ocps & 0b00111111) as usize;
    palettes.cgb_ocpd[address] = value;

    let should_auto_increment = is_bit_set(palettes.cgb_ocps, 7);

    palettes.cgb_ocps = if should_auto_increment {
        (palettes.cgb_ocps + 1) | 0x80
    }
    else {
        palettes.cgb_ocps
    }; 
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_background_palettes(palettes: &mut Palettes) {
        palettes.cgb_bcpd[0] = 0x11;
        palettes.cgb_bcpd[1] = 0x11;
        palettes.cgb_bcpd[2] = 0x22;
        palettes.cgb_bcpd[3] = 0x22;
        palettes.cgb_bcpd[4] = 0x33;
        palettes.cgb_bcpd[5] = 0x33;
        palettes.cgb_bcpd[6] = 0x44;
        palettes.cgb_bcpd[7] = 0x44;
        palettes.cgb_bcpd[8] = 0x55;
        palettes.cgb_bcpd[9] = 0x55;
        palettes.cgb_bcpd[10] = 0x66;
        palettes.cgb_bcpd[11] = 0x66;
        palettes.cgb_bcpd[12] = 0x77;
        palettes.cgb_bcpd[13] = 0x77;
        palettes.cgb_bcpd[14] = 0x88;
        palettes.cgb_bcpd[15] = 0x88;
        palettes.cgb_bcpd[16] = 0x99;
        palettes.cgb_bcpd[17] = 0x99;
        palettes.cgb_bcpd[18] = 0xAA;
        palettes.cgb_bcpd[19] = 0xAA;
        palettes.cgb_bcpd[20] = 0xBB;
        palettes.cgb_bcpd[21] = 0xBB;
        palettes.cgb_bcpd[22] = 0xCC;
        palettes.cgb_bcpd[23] = 0xCC;
        palettes.cgb_bcpd[24] = 0xDD;
        palettes.cgb_bcpd[25] = 0xDD;
        palettes.cgb_bcpd[26] = 0xEE;
        palettes.cgb_bcpd[27] = 0xEE;
        palettes.cgb_bcpd[28] = 0xFF;
        palettes.cgb_bcpd[29] = 0xFF;
    }

    fn setup_test_object_palettes(palettes: &mut Palettes) {
        palettes.cgb_ocpd[0] = 0x11;
        palettes.cgb_ocpd[1] = 0x11;
        palettes.cgb_ocpd[2] = 0x22;
        palettes.cgb_ocpd[3] = 0x22;
        palettes.cgb_ocpd[4] = 0x33;
        palettes.cgb_ocpd[5] = 0x33;
        palettes.cgb_ocpd[6] = 0x44;
        palettes.cgb_ocpd[7] = 0x44;
        palettes.cgb_ocpd[8] = 0x55;
        palettes.cgb_ocpd[9] = 0x55;
        palettes.cgb_ocpd[10] = 0x66;
        palettes.cgb_ocpd[11] = 0x66;
        palettes.cgb_ocpd[12] = 0x77;
        palettes.cgb_ocpd[13] = 0x77;
        palettes.cgb_ocpd[14] = 0x88;
        palettes.cgb_ocpd[15] = 0x88;
        palettes.cgb_ocpd[16] = 0x99;
        palettes.cgb_ocpd[17] = 0x99;
        palettes.cgb_ocpd[18] = 0xAA;
        palettes.cgb_ocpd[19] = 0xAA;
        palettes.cgb_ocpd[20] = 0xBB;
        palettes.cgb_ocpd[21] = 0xBB;
        palettes.cgb_ocpd[22] = 0xCC;
        palettes.cgb_ocpd[23] = 0xCC;
        palettes.cgb_ocpd[24] = 0xDD;
        palettes.cgb_ocpd[25] = 0xDD;
        palettes.cgb_ocpd[26] = 0xEE;
        palettes.cgb_ocpd[27] = 0xEE;
        palettes.cgb_ocpd[28] = 0xFF;
        palettes.cgb_ocpd[29] = 0xFF;
    }

    #[test]
    fn should_lookup_background_palette() {
        let mut palettes = initialize_palettes();

        setup_test_background_palettes(&mut palettes);

        let palette_number = 3;
        let color_id = 1;
        let palette = lookup_cgb_background_palette(&palettes, palette_number, color_id);

        assert_eq!(palette, 0xEEEE);
    }

    #[test]
    fn should_lookup_object_palette() {
        let mut palettes = initialize_palettes();

        setup_test_object_palettes(&mut palettes);

        let palette_number = 2;
        let color_id = 2;
        let palette = lookup_cgb_object_palette(&palettes, palette_number, color_id);

        assert_eq!(palette, 0xBBBB);
    }

    #[test]
    fn should_lookup_background_palette_byte_via_bcpd() {
        let mut palettes = initialize_palettes();

        setup_test_background_palettes(&mut palettes);
        palettes.cgb_bcpd[2] = 0x22;
        palettes.cgb_bcpd[3] = 0xBB;

        set_cgb_bcps(&mut palettes, 0b00000011);
        let palette_byte = get_cgb_bcpd(&palettes);

        assert_eq!(palette_byte, 0xBB);
    }

    #[test]
    fn should_lookup_object_palette_byte_via_ocpd() {
        let mut palettes = initialize_palettes();

        setup_test_object_palettes(&mut palettes);
        palettes.cgb_ocpd[2] = 0x22;
        palettes.cgb_ocpd[3] = 0xBB;

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

        assert_eq!(palettes.cgb_bcpd[3], 0xBB);
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

        let color_id = calculate_color_id(0, msb_byte, lsb_byte, false);
        let color = as_cgb_bg_color_rgb(&palettes, palette_number, color_id, false);

        // Palette will be 0b1110111011101110.
        assert_eq!(color, [0x73, 0xBD, 0xDE, 0xFF]);
    }

    #[test]
    fn should_calculate_object_color() {
        let mut palettes = initialize_palettes();

        setup_test_object_palettes(&mut palettes);

        let palette_number = 3;

        let msb_byte = 0b00101010;
        let lsb_byte = 0b11010101;

        let color_id = calculate_color_id(0, msb_byte, lsb_byte, false);
        let color = as_cgb_obj_color_rgb(&palettes, palette_number, color_id, false);

        // Palette will be 0b1110111011101110.
        assert_eq!(color, Some([0x73, 0xBD, 0xDE, 0xFF])); 
    }

    #[test]
    fn should_return_none_when_calculating_object_color_with_color_id_0() {
        let mut palettes = initialize_palettes();

        setup_test_object_palettes(&mut palettes);

        let palette_number = 3;

        // Bit 7 on both bytes set to 0 which will result in a color id of 0.
        let msb_byte = 0b00101010;
        let lsb_byte = 0b01010101;

        let color_id = calculate_color_id(0, msb_byte, lsb_byte, false);
        let color = as_cgb_obj_color_rgb(&palettes, palette_number, color_id, false);

        assert_eq!(color, None);
    }
}