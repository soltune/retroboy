use crate::mmu::bank_utils::{banked_read, banked_write};
use crate::mmu::cartridge::{Cartridge, CartridgeMapper, CartridgeMapperSnapshot, MBCSnapshot};
use crate::mmu::constants::*;
use bincode::{Decode, Encode};

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
pub enum MBCMode {
    ROM,
    RAM
}

#[derive(Clone, Debug, Encode, Decode)]
pub struct MBC1State {
    ram_enabled: bool,
    rom_bank_number: u8,
    ram_bank_number: u8,
    mode: MBCMode,
}

#[derive(Debug)]
pub struct MBC1CartridgeMapper {
    cartridge: Cartridge,
    state: MBC1State
}

pub fn initialize_mbc1() -> MBC1State {
    MBC1State {
        ram_enabled: false,
        rom_bank_number: 1,
        ram_bank_number: 0,
        mode: MBCMode::ROM,
    }
}

pub fn initialize_mbc1_mapper(cartridge: Cartridge) -> MBC1CartridgeMapper {
    MBC1CartridgeMapper {
        cartridge,
        state: initialize_mbc1()
    }
}

fn ram_supported(cartridge: &Cartridge) -> bool {
    cartridge.header.type_code == CART_TYPE_MBC1_WITH_RAM ||
    cartridge.header.type_code == CART_TYPE_MBC1_WITH_RAM_PLUS_BATTERY
}

fn battery_supported(cartridge: &Cartridge) -> bool {
    cartridge.header.type_code == CART_TYPE_MBC1_WITH_RAM_PLUS_BATTERY
}

impl CartridgeMapper for MBC1CartridgeMapper {
    fn read_rom(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF =>
                self.cartridge.rom[address as usize],
            0x4000..=0x7FFF => {
                banked_read(&self.cartridge.rom, 0x4000, address, self.state.rom_bank_number as u16)
            },
            _ => panic!("Invalid ROM address: {:#X}", address),
        }
    }
    
    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                if ram_supported(&self.cartridge) {
                    self.state.ram_enabled = (value & 0xF) == 0x0A;
                }
            },
            0x2000..=0x3FFF => {
                let masked_value = value & 0x1F;
                let mut bank_value = if masked_value == 0 { 1 as u8 } else { masked_value };
    
                let max_bank_mask = ((self.cartridge.header.max_banks - 1) & 0x1F) as u8;
                bank_value &= max_bank_mask;
    
                self.state.rom_bank_number = (self.state.rom_bank_number & 0x60) + (bank_value & 0x1F);
            },
            0x4000..=0x5FFF => {
                if self.state.mode == MBCMode::RAM {
                    self.state.ram_bank_number = value & 0x3;
                } else if self.cartridge.header.max_banks >= 64 {
                    self.state.rom_bank_number = ((value & 0x3) << 5) + (self.state.rom_bank_number & 0x1F);
                }
            },
            0x6000..=0x7FFF => {
                if ram_supported(&self.cartridge) {
                    self.state.mode = if value == 1 { MBCMode::RAM } else { MBCMode::ROM };
                }
            },
            _ => panic!("Invalid ROM address: {:#X}", address),
        }
    }
    
    fn read_ram(&self, address: u16) -> u8 {
        if self.state.ram_enabled && self.cartridge.header.max_ram_banks > 0 {
            banked_read(&self.cartridge.ram, 0x2000, address, self.state.ram_bank_number as u16)
        } else {
            0xFF
        }
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if self.state.ram_enabled && self.cartridge.header.max_ram_banks > 0 {
            banked_write(&mut self.cartridge.ram, 0x2000, address, self.state.ram_bank_number as u16, value);
            if battery_supported(&self.cartridge) {
                self.cartridge.effects.save_ram(&self.cartridge.header.title, &self.cartridge.ram);
            }
        }
    }

    fn get_cartridge(&self) -> &Cartridge {
        &self.cartridge
    }

    fn set_cartridge_ram(&mut self, ram: Vec<u8>) {
        self.cartridge.ram = ram;
    }
    
    fn get_ram_bank(&self) -> u8 {
        self.state.ram_bank_number
    }

    fn get_snapshot(&self) -> CartridgeMapperSnapshot {
        CartridgeMapperSnapshot {
            ram: self.cartridge.ram.clone(),
            mbc: MBCSnapshot::MBC1(self.state.clone())
        }
    }

    fn apply_snapshot(&mut self, snapshot: CartridgeMapperSnapshot) {
        if let MBCSnapshot::MBC1(mbc1_state) = snapshot.mbc {
            self.state = mbc1_state;
        } else {
            panic!("Invalid snapshot type for MBC1");
        }
        
        self.cartridge.ram = snapshot.ram;
    }
}

#[cfg(test)]
mod tests {
    use crate::mmu::cartridge::*;
    use crate::mmu::cartridge::test_utils::*;
    use crate::mmu::constants::*;
    use crate::mmu::effects::empty_cartridge_effects;
    use crate::mmu::test_utils::*;

    #[test]
    fn enable_external_ram_if_correct_cartridge_type() {
        let mut mapper = build_cartridge_mapper(CART_TYPE_MBC1_WITH_RAM, ROM_SIZE_64KB, RAM_SIZE_2KB);

        mapper.write_rom(0x0000, 0xA);

        mapper.write_ram(0x0000, 0xC2);
        assert_eq!(mapper.read_ram(0x000), 0xC2);
    }

    #[test]
    fn enable_external_ram_if_correct_cartridge_type_scenario_two() {
        let mut mapper = build_cartridge_mapper(CART_TYPE_MBC1_WITH_RAM_PLUS_BATTERY, ROM_SIZE_64KB, RAM_SIZE_2KB);

        mapper.write_rom(0x0000, 0xA);

        mapper.write_ram(0x0000, 0xC2);
        assert_eq!(mapper.read_ram(0x000), 0xC2);
    }

    #[test]
    fn enable_external_ram_if_lower_nibble_is_equal_to_a() {
        let mut mapper = build_cartridge_mapper(CART_TYPE_MBC1_WITH_RAM, ROM_SIZE_64KB, RAM_SIZE_2KB);

        mapper.write_rom(0x0000, 0x1A);

        mapper.write_ram(0x0000, 0xC2);
        assert_eq!(mapper.read_ram(0x000), 0xC2);
    }

    #[test]
    fn not_enable_external_ram_if_incorrect_cartridge_type() {
        let mut mapper = build_cartridge_mapper(CART_TYPE_MBC1, ROM_SIZE_64KB, RAM_SIZE_2KB);

        mapper.write_rom(0x0000, 0xA);

        mapper.write_ram(0x0000, 0xC2);
        assert_eq!(mapper.read_ram(0x000), 0xFF);
    }

    #[test]
    fn disable_external_ram_if_correct_cartridge_type() {
        let mut mapper = build_cartridge_mapper(CART_TYPE_MBC1_WITH_RAM, ROM_SIZE_64KB, RAM_SIZE_2KB);

        mapper.write_rom(0x0000, 0xA);

        mapper.write_ram(0x0000, 0xC2);
        assert_eq!(mapper.read_ram(0x000), 0xC2);

        mapper.write_rom(0x0000, 0xB);

        mapper.write_ram(0x0000, 0xD2);
        assert_eq!(mapper.read_ram(0x000), 0xFF);
    }

    #[test]
    fn set_rom_bank_number() {
        let mut rom = build_rom(CART_TYPE_MBC1, ROM_SIZE_128KB, RAM_SIZE_2KB);
        rom[0xC005] = 0xA1;
        let mut mapper = load_rom_buffer(rom, empty_cartridge_effects()).unwrap(); 

        mapper.write_rom(0x2000, 0x3);
        
        let byte = mapper.read_rom(0x4005);
        assert_eq!(byte, 0xA1);        
    }

    #[test]
    fn correctly_sets_lower_and_upper_bits_of_the_rom_bank_number() {
        let mut rom = build_rom(CART_TYPE_MBC1, ROM_SIZE_2MB, RAM_SIZE_2KB);
        rom[0x110005] = 0xA1;
        let mut mapper = load_rom_buffer(rom, empty_cartridge_effects()).unwrap();

        mapper.write_rom(0x4000, 0x02);
        mapper.write_rom(0x2000, 0x4);

        let byte = mapper.read_rom(0x4005);
        assert_eq!(byte, 0xA1);
    }

    #[test]
    fn reads_bank_zero_as_bank_one() {
        let mut rom = build_rom(CART_TYPE_MBC1, ROM_SIZE_64KB, RAM_SIZE_2KB);
        rom[0x4005] = 0xCC;
        let mut mapper = load_rom_buffer(rom, empty_cartridge_effects()).unwrap();

        mapper.write_rom(0x2000, 0x0);

        let byte = mapper.read_rom(0x4005);
        assert_eq!(byte, 0xCC);
    }

    #[test]
    fn masks_bank_number_to_required_number_of_bits() {
        let mut rom = build_rom(CART_TYPE_MBC1, ROM_SIZE_256KB, RAM_SIZE_2KB);
        rom[0] = 0xB1;
        rom[1] = 0xD2;
        rom[0x8000] = 0xBB;
        rom[0x8001] = 0xD1;
        let mut mapper = load_rom_buffer(rom, empty_cartridge_effects()).unwrap();

        mapper.write_rom(0x2000, 0x12);

        let byte = mapper.read_rom(0x4001);
        assert_eq!(byte, 0xD1);
    }

    #[test]
    fn sets_ram_bank_number() {
        let mut mapper = build_cartridge_mapper(CART_TYPE_MBC1_WITH_RAM, ROM_SIZE_64KB, RAM_SIZE_32KB);

        // Enable RAM
        mapper.write_rom(0x0000, 0xA);

        // Switch to RAM mode
        mapper.write_rom(0x6000, 0x1);

        // Set RAM to bank 3
        mapper.write_rom(0x4000, 0x3);

        mapper.write_ram(0x0005, 0xCC);
        let first_byte = mapper.read_ram(0x0005);
        assert_eq!(first_byte, 0xCC);

        // Set RAM to bank 0
        mapper.write_rom(0x4000, 0);
        let second_byte = mapper.read_ram(0x0005);
        assert_eq!(second_byte, 0x00);
    }

    #[test]
    fn only_allow_reading_from_ram_if_it_is_enabled() {
        let mut mapper = build_cartridge_mapper(CART_TYPE_MBC1_WITH_RAM, ROM_SIZE_64KB, RAM_SIZE_32KB);

        // Try writing to RAM even though RAM is not enabled
        mapper.write_ram(0x0005, 0xCC);
        let first_byte = mapper.read_ram(0x0005);
        assert_eq!(first_byte, 0xFF);

        // Enable RAM
        mapper.write_rom(0x0000, 0xA);

        mapper.write_ram(0x0005, 0xCC);
        let second_byte = mapper.read_ram(0x0005);
        assert_eq!(second_byte, 0xCC);
    }
}