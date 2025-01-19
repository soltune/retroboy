use crate::mmu::cartridge::{Cartridge, CartridgeMapper};
use crate::mmu::constants::*;
use crate::mmu::rtc::{empty_clock, RTC};

#[derive(Debug)]
pub struct MBC3 {
    cartridge: Cartridge,
    rom_bank_number: u8,
    ram_rtc_enabled: bool,
    ram_rtc_selection: u8,
    rtc: RTC,
    rtc_latch: u8,
    get_next_rtc: fn() -> RTC
}

pub fn initialize_mbc3(cartridge: Cartridge, get_next_rtc: fn() -> RTC) -> MBC3 {
    MBC3 {
        cartridge,
        rom_bank_number: 1,
        ram_rtc_enabled: false,
        ram_rtc_selection: 0,
        rtc: empty_clock(),
        rtc_latch: 0xFF,
        get_next_rtc
    }
}

fn ram_supported(cartridge: &Cartridge) -> bool {
    cartridge.header.type_code == CART_TYPE_MBC3_RAM ||
    cartridge.header.type_code == CART_TYPE_MBC3_RAM_BATTERY ||
    cartridge.header.type_code == CART_TYPE_MBC3_TIMER_RAM_BATTERY
}

fn timer_supported(cartridge: &Cartridge) -> bool {
    cartridge.header.type_code == CART_TYPE_MBC3_TIMER_RAM_BATTERY ||
    cartridge.header.type_code == CART_TYPE_MBC3_TIMER_BATTERY
}

impl CartridgeMapper for MBC3 {
    fn read_rom(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF =>
                self.cartridge.rom[address as usize],
            0x4000..=0x7FFF => {
                let base_location = self.rom_bank_number as u32 * 0x4000;
                let calculated_address = base_location + ((address & 0x3FFF) as u32);
                self.cartridge.rom[calculated_address as usize]
            },
            _ => panic!("Invalid ROM address: {:#X}", address),
        }
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                if ram_supported(&self.cartridge) || timer_supported(&self.cartridge) {
                    self.ram_rtc_enabled = (value & 0xF) == 0x0A;
                }
            },
            0x2000..=0x3FFF => {
                let value = if value == 0 { 1 } else { value };
                self.rom_bank_number = value & 0x7F;
            },
            0x4000..=0x5FFF => {
                if value <= 0x03 || (value >= 0x08 && value <= 0x0C) {
                    self.ram_rtc_selection = value;
                }
            },
            0x6000..=0x7FFF => {
                if timer_supported(&self.cartridge) {
                    if self.rtc_latch == 0x00 && value == 0x01 {
                        self.rtc = (self.get_next_rtc)();
                    }
                    self.rtc_latch = value;
                }
            },
            _ => panic!("Invalid ROM address: {:#X}", address),
        } 
    }
    
    fn read_ram(&self, address: u16) -> u8 {
        if self.ram_rtc_enabled {
            let ram_rtc_selection = self.ram_rtc_selection;
    
            match ram_rtc_selection {
                0x00..=0x03 if ram_supported(&self.cartridge) => {
                    let calculated_address = (self.ram_rtc_selection as u16 * 0x2000) + address;
                    self.cartridge.ram[calculated_address as usize]
                },
                0x08..=0x0C if timer_supported(&self.cartridge) => {
                    match ram_rtc_selection {
                        0x08 => self.rtc.seconds,
                        0x09 => self.rtc.minutes,
                        0x0A => self.rtc.hours,
                        0x0B => self.rtc.days_lower,
                        0x0C => self.rtc.days_upper,
                        _ => 0xFF
                    }
                }
                _ =>
                    0xFF
            }
        }
        else {
            0xFF
        }
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if self.ram_rtc_enabled {
            let ram_rtc_selection = self.ram_rtc_selection;
    
            match ram_rtc_selection {
                0x00..=0x03 if ram_supported(&self.cartridge) => {
                    let calculated_address = (self.ram_rtc_selection as u16 * 0x2000) + address;
                    self.cartridge.ram[calculated_address as usize] = value;
                },
                0x08..=0x0C if timer_supported(&self.cartridge) => {
                    match ram_rtc_selection {
                        0x08 => self.rtc.seconds = value,
                        0x09 => self.rtc.minutes = value,
                        0x0A => self.rtc.hours = value,
                        0x0B => self.rtc.days_lower = value,
                        0x0C => self.rtc.days_upper = value,
                        _ => ()
                    }
                }
                _ =>
                    ()
            }
        }
    }

    fn get_cartridge(&self) -> &Cartridge {
        &self.cartridge
    }

    fn set_cartridge_ram(&mut self, ram: Vec<u8>) {
        self.cartridge.ram = ram;
    }
}

#[cfg(test)]
mod tests {
    use crate::mmu::cartridge::*;
    use crate::mmu::cartridge::test_utils::*;
    use crate::mmu::constants::*;
    use crate::mmu::test_utils::*;
    use super::*;

    fn fake_clock() -> RTC {
        RTC {
            halted: false,
            seconds: 10,
            minutes: 10,
            hours: 10,
            days_lower: 0,
            days_upper: 0
        }
    }

    #[test]
    fn reads_from_rom_bank_0() {
        let mut rom = build_rom(CART_TYPE_MBC3, ROM_SIZE_64KB, RAM_SIZE_2KB);
        rom[0] = 0xB1;
        rom[1] = 0xD2;
        let mapper = load_rom_buffer(rom, empty_clock).unwrap(); 

        let byte = mapper.read_rom(0x1);
        assert_eq!(byte, 0xD2);      
    }

    #[test]
    fn reads_from_rom_bank_2() {
        let mut rom = build_rom(CART_TYPE_MBC3, ROM_SIZE_64KB, RAM_SIZE_2KB);
        rom[0x8000] = 0xB1;
        rom[0x8001] = 0xD2;
        let mut mapper = load_rom_buffer(rom, empty_clock).unwrap(); 

        mapper.write_rom(0x2000, 2);

        let byte = mapper.read_rom(0x4001);
        assert_eq!(byte, 0xD2);
    }

    #[test]
    fn reads_from_ram_bank_1() {
        let mut mapper = build_cartridge_mapper(CART_TYPE_MBC3_RAM, ROM_SIZE_64KB, RAM_SIZE_32KB);

        let mut ram = vec![0; 0x8000];
        ram[0x2001] = 0xDD;
        mapper.set_cartridge_ram(ram);

        // Enables RAM
        mapper.write_rom(0x0000, 0xA);

        // Sets RAM bank to 1
        mapper.write_rom(0x4000, 0x1);

        let byte = mapper.read_ram(0x0001);
        assert_eq!(byte, 0xDD);
    }

    #[test]
    fn does_not_read_from_ram_if_not_supported() {
        let mut mapper = build_cartridge_mapper(CART_TYPE_MBC3_TIMER_BATTERY, ROM_SIZE_64KB, RAM_SIZE_32KB);

        let mut ram = vec![0; 0x8000];
        ram[0x2001] = 0xDD;
        mapper.set_cartridge_ram(ram);

        // Enables RAM
        mapper.write_rom(0x0000, 0xA);

        // Sets RAM bank to 1
        mapper.write_rom(0x4000, 0x1);

        let byte = mapper.read_ram(0x0001);
        assert_eq!(byte, 0xFF);
    }

    #[test]
    fn writes_to_ram_bank_1() {
        let mut mapper = build_cartridge_mapper(CART_TYPE_MBC3_RAM, ROM_SIZE_64KB, RAM_SIZE_32KB);

        // Enables RAM
        mapper.write_rom(0x0000, 0xA);

        // Sets RAM bank to 1
        mapper.write_rom(0x4000, 0x1);

        mapper.write_ram(0x0001, 0xDD);

        let byte = mapper.read_ram(0x001);
        assert_eq!(byte, 0xDD);
    }

    #[test]
    fn does_not_write_to_ram_if_not_supported() {
        let mut mapper = build_cartridge_mapper(CART_TYPE_MBC3_TIMER_BATTERY, ROM_SIZE_64KB, RAM_SIZE_32KB);

        // Enables RAM
        mapper.write_rom(0x0000, 0xA);

        // Sets RAM bank to 1
        mapper.write_rom(0x4000, 0x1);

        mapper.write_ram(0x0001, 0xDD);

        let byte = mapper.read_ram(0x001);
        assert_eq!(byte, 0xFF);
    }

    #[test]
    fn changes_rom_bank_number_to_bank_1_when_writing_0() {
        let mut rom = build_rom(CART_TYPE_MBC3, ROM_SIZE_64KB, RAM_SIZE_2KB);
        rom[0x4000] = 0xB1;
        rom[0x4001] = 0xD2;
        let mut mapper = load_rom_buffer(rom, empty_clock).unwrap(); 

        mapper.write_rom(0x2000, 2);

        mapper.write_rom(0x2000, 0);

        let byte = mapper.read_rom(0x4001);
        assert_eq!(byte, 0xD2);
    }

    #[test]
    fn reads_from_minutes_rtc_register() {
        let mut mapper = build_cartridge_mapper_with_rtc(CART_TYPE_MBC3_TIMER_RAM_BATTERY,
            ROM_SIZE_64KB,
            RAM_SIZE_2KB,
            fake_clock);

        mapper.write_rom(0x0000, 0xA);

        mapper.write_rom(0x4000, 0x9);

        // Latch RTC
        mapper.write_rom(0x6000, 0x0);
        mapper.write_rom(0x6000, 0x1);

        let byte = mapper.read_ram(0x0000);
        assert_eq!(byte, 0x0A);
    }

    #[test]
    fn does_not_read_from_minutes_rtc_register_if_not_supported() {
        let mut mapper = build_cartridge_mapper_with_rtc(CART_TYPE_MBC3_RAM_BATTERY,
            ROM_SIZE_64KB,
            RAM_SIZE_2KB,
            fake_clock);

        mapper.write_rom(0x0000, 0xA);

        mapper.write_rom(0x4000, 0x9);

        // Latch RTC
        mapper.write_rom(0x6000, 0x0);
        mapper.write_rom(0x6000, 0x1);

        let byte = mapper.read_ram(0x0000);
        assert_eq!(byte, 0xFF);
    }

    #[test]
    fn writes_to_minutes_rtc_register() {
        let mut mapper = build_cartridge_mapper_with_rtc(CART_TYPE_MBC3_TIMER_RAM_BATTERY,
            ROM_SIZE_64KB,
            RAM_SIZE_2KB,
            fake_clock);

        mapper.write_rom(0x0000, 0xA);

        mapper.write_rom(0x4000, 0x9);

        // Latch RTC
        mapper.write_rom(0x6000, 0x0);
        mapper.write_rom(0x6000, 0x1);

        mapper.write_ram(0x0000, 0x2);
        let byte = mapper.read_ram(0x0000);
        assert_eq!(byte, 0x02);
    }

    #[test]
    fn does_not_write_to_minutes_rtc_register_if_not_supported() {
        let mut mapper = build_cartridge_mapper_with_rtc(CART_TYPE_MBC3_RAM_BATTERY,
            ROM_SIZE_64KB,
            RAM_SIZE_2KB,
            fake_clock);

        mapper.write_rom(0x0000, 0xA);

        mapper.write_rom(0x4000, 0x9);

        // Latch RTC
        mapper.write_rom(0x6000, 0x0);
        mapper.write_rom(0x6000, 0x1);

        mapper.write_ram(0x0000, 0x2);
        let byte = mapper.read_ram(0x0000);
        assert_eq!(byte, 0xFF);
    }
}