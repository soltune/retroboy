use crate::mmu::cartridge::{Cartridge, CART_TYPE_MBC1_WITH_RAM, CART_TYPE_MBC1_WITH_RAM_PLUS_BATTERY};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum MBCMode {
    ROM,
    RAM
}

#[derive(Debug)]
pub struct MBC1 {
    pub ram_enabled: bool,
    pub rom_bank_number: u8,
    pub ram_bank_number: u8,
    pub mode: MBCMode,
}

pub fn initialize_mbc1() -> MBC1 {
    MBC1 {
        ram_enabled: false,
        rom_bank_number: 1,
        ram_bank_number: 0,
        mode: MBCMode::ROM,
    }
}

fn ram_supported(cartridge: &Cartridge) -> bool {
    cartridge.header.type_code == CART_TYPE_MBC1_WITH_RAM ||
    cartridge.header.type_code == CART_TYPE_MBC1_WITH_RAM_PLUS_BATTERY
}

pub fn write_rom(cartridge: &mut Cartridge, address: u16, value: u8) {
    match address {
        0x0000..=0x1FFF => {
            if ram_supported(cartridge) {
                cartridge.mbc1.ram_enabled = value & 0x0F == 0x0A;
            }
        },
        0x2000..=0x3FFF => {
            let masked_value = value & 0x1F;
            let mut bank_value = if masked_value == 0 { 1 as u8 } else { masked_value };

            let max_bank_mask = ((cartridge.header.max_banks - 1) & 0x1F) as u8;
            bank_value &= max_bank_mask;

            cartridge.mbc1.rom_bank_number = (cartridge.mbc1.rom_bank_number & 0x60) + (bank_value & 0x1F);
        },
        0x4000..=0x5FFF => {
            if cartridge.mbc1.mode == MBCMode::RAM {
                cartridge.mbc1.ram_bank_number = value & 0x3;
            } else if cartridge.header.max_banks >= 64 {
                cartridge.mbc1.rom_bank_number = ((value & 0x3) << 5) + (cartridge.mbc1.rom_bank_number & 0x1F);
            }
        },
        0x6000..=0x7FFF => {
            cartridge.mbc1.mode = if value & 0x01 == 0 { MBCMode::ROM } else { MBCMode::RAM };
        },
        _ => panic!("Invalid ROM address: {:#X}", address),
    }
}

pub fn read_rom(cartridge: &Cartridge, address: u16) -> u8 {
    match address {
        0x0000..=0x3FFF =>
            cartridge.rom[address as usize],
        0x4000..=0x7FFF => {
            let base_location = cartridge.mbc1.rom_bank_number as u32 * 0x4000;
            let calculated_address = base_location + ((address & 0x3FFF) as u32);
            cartridge.rom[calculated_address as usize]
        },
        _ => panic!("Invalid ROM address: {:#X}", address),
    }
}

pub fn write_ram(cartridge: &mut Cartridge, address: u16, value: u8) {
    let calculated_address = (cartridge.mbc1.ram_bank_number as u16 * 0x2000) + address;
    if cartridge.mbc1.ram_enabled {
        cartridge.ram[calculated_address as usize] = value;
    }
}

pub fn read_ram(cartridge: &Cartridge, address: u16) -> u8 {
    let calculated_address = (cartridge.mbc1.ram_bank_number as u16 * 0x2000) + address;
    cartridge.ram[calculated_address as usize]
}

#[cfg(test)]
mod tests {
    use crate::emulator::initialize_screenless_emulator;
    use crate::mmu::{self, load_rom_buffer};
    use crate::mmu::cartridge::CART_TYPE_MBC1;
    use super::*;

    #[test]
    fn enable_external_ram_if_correct_cartridge_type() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC1_WITH_RAM;
        mmu::write_byte(&mut emulator, 0x0000, 0xA);
        assert_eq!(emulator.memory.cartridge.mbc1.ram_enabled, true);
    }

    #[test]
    fn enable_external_ram_if_correct_cartridge_type_scenario_two() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC1_WITH_RAM_PLUS_BATTERY;
        mmu::write_byte(&mut emulator, 0x0000, 0xA);
        assert_eq!(emulator.memory.cartridge.mbc1.ram_enabled, true);
    }

    #[test]
    fn enable_external_ram_if_lower_nibble_is_equal_to_a() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC1_WITH_RAM;
        mmu::write_byte(&mut emulator, 0x0000, 0x1A);
        assert_eq!(emulator.memory.cartridge.mbc1.ram_enabled, true);
    }

    #[test]
    fn not_enable_external_ram_if_incorrect_cartridge_type() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC1;
        mmu::write_byte(&mut emulator, 0x0000, 0xA);
        assert_eq!(emulator.memory.cartridge.mbc1.ram_enabled, false); 
    }

    #[test]
    fn disable_external_ram_if_correct_cartridge_type() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC1_WITH_RAM;
        emulator.memory.cartridge.mbc1.ram_enabled = true;
        mmu::write_byte(&mut emulator, 0x0000, 0xB);
        assert_eq!(emulator.memory.cartridge.mbc1.ram_enabled, false);
    }

    #[test]
    fn set_rom_bank_number() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC1;
        emulator.memory.cartridge.header.max_banks = 8;
        mmu::write_byte(&mut emulator, 0x2000, 0x4);
        assert_eq!(emulator.memory.cartridge.mbc1.rom_bank_number, 0x04);
    }

    #[test]
    fn sets_the_lower_five_bits_of_the_rom_bank_number() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC1;
        emulator.memory.cartridge.header.max_banks = 128;
        emulator.memory.cartridge.mbc1.mode = MBCMode::ROM;
        emulator.memory.cartridge.mbc1.rom_bank_number = 0x41;
        mmu::write_byte(&mut emulator, 0x2000, 0x4);
        assert_eq!(emulator.memory.cartridge.mbc1.rom_bank_number, 0x44);
    }

    #[test]
    fn masks_bank_number_to_required_number_of_bits() {
        let mut emulator = initialize_screenless_emulator();

        let mut rom_buffer = vec![0; 0x40000];
        rom_buffer[0] = 0xB1;
        rom_buffer[1] = 0xD2;
        rom_buffer[0x8000] = 0xBB;
        rom_buffer[0x8001] = 0xD1;

        load_rom_buffer(&mut emulator.memory, rom_buffer).unwrap();

        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC1;
        emulator.memory.cartridge.header.max_banks = 16;
        emulator.memory.cartridge.mbc1.mode = MBCMode::ROM;

        // The ROM is 256 KB, so 0x12 is too big and it will be masked
        // to the required number of bits with a result of 0x2 for the
        // bank number.
        mmu::write_byte(&mut emulator, 0x2000, 0x12);

        assert_eq!(emulator.memory.cartridge.mbc1.rom_bank_number, 0x2);
        assert_eq!(mmu::read_byte(&mut emulator, 0x4001), 0xD1);
    }

    #[test]
    fn treats_setting_bank_zero_as_bank_one() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC1;
        emulator.memory.cartridge.header.max_banks = 8;
        emulator.memory.cartridge.mbc1.mode = MBCMode::ROM;
        mmu::write_byte(&mut emulator, 0x2000, 0x0);
        assert_eq!(emulator.memory.cartridge.mbc1.rom_bank_number, 0x1);
    }

    #[test]
    fn sets_ram_bank_number() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC1;
        emulator.memory.cartridge.header.max_banks = 8;
        emulator.memory.cartridge.mbc1.mode = MBCMode::RAM;
        mmu::write_byte(&mut emulator, 0x4000, 0x2);
        assert_eq!(emulator.memory.cartridge.mbc1.ram_bank_number, 0x2);
    }

    #[test]
    fn sets_high_two_bits_of_rom_bank_number() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC1;
        emulator.memory.cartridge.header.max_banks = 128;
        emulator.memory.cartridge.mbc1.mode = MBCMode::ROM;
        emulator.memory.cartridge.mbc1.rom_bank_number = 0x41;
        mmu::write_byte(&mut emulator, 0x4000, 0x3);
        assert_eq!(emulator.memory.cartridge.mbc1.rom_bank_number, 0x61);
    }

    #[test]
    fn switch_mbc_mode_from_rom_mode_to_ram_mode() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC1_WITH_RAM;
        emulator.memory.cartridge.mbc1.ram_enabled = true;
        emulator.memory.cartridge.mbc1.mode = MBCMode::ROM;
        mmu::write_byte(&mut emulator, 0x6010, 0x01);
        assert_eq!(emulator.memory.cartridge.mbc1.mode, MBCMode::RAM); 
    }

    #[test]
    fn switch_mbc_mode_from_ram_mode_to_rom_mode() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC1_WITH_RAM;
        emulator.memory.cartridge.mbc1.ram_enabled = true;
        emulator.memory.cartridge.mbc1.mode = MBCMode::RAM;
        mmu::write_byte(&mut emulator, 0x6010, 0x00);
        assert_eq!(emulator.memory.cartridge.mbc1.mode, MBCMode::ROM); 
    }

    #[test]
    fn reads_from_different_rom_bank() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC1;
        emulator.memory.cartridge.mbc1.mode = MBCMode::ROM;
        emulator.memory.cartridge.mbc1.rom_bank_number = 3;
        emulator.memory.cartridge.rom.resize(0x16000, 0);
        emulator.memory.cartridge.rom[0xC005] = 0xA1;
        let result = mmu::read_byte(&mut emulator, 0x4005);
        assert_eq!(result, 0xA1);
    }

    #[test]
    fn reads_from_different_ram_bank() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC1_WITH_RAM;
        emulator.memory.cartridge.mbc1.mode = MBCMode::RAM;
        emulator.memory.cartridge.mbc1.ram_bank_number = 3;
        emulator.memory.cartridge.mbc1.ram_enabled = true;
        emulator.memory.cartridge.ram[0x6005] = 0xA1;
        let result = mmu::read_byte(&mut emulator, 0xA005);
        assert_eq!(result, 0xA1);
    }
}