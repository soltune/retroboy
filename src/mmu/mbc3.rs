use crate::mmu::cartridge::Cartridge;
use crate::mmu::cartridge::{CART_TYPE_MBC3_RAM, CART_TYPE_MBC3_RAM_BATTERY, CART_TYPE_MBC3_TIMER_RAM_BATTERY, CART_TYPE_MBC3_TIMER_BATTERY};

#[derive(Debug)]
pub struct RTC {
    pub halted: bool,
    pub seconds: u8,
    pub minutes: u8,
    pub hours: u8,
    pub days_lower: u8,
    pub days_upper: u8
}

#[derive(Debug)]
pub struct MBC3 {
    pub rom_bank_number: u8,
    pub ram_rtc_enabled: bool,
    pub ram_rtc_selection: u8,
    pub rtc: RTC,
    pub rtc_latch: u8,
    pub get_next_rtc: fn() -> RTC
}

pub fn empty_clock() -> RTC {
    RTC {
        halted: false,
        seconds: 0,
        minutes: 0,
        hours: 0,
        days_lower: 0,
        days_upper: 0
    }
}

pub fn initialize_mbc3(get_next_rtc: fn() -> RTC) -> MBC3 {
    MBC3 {
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

pub fn write_rom(cartridge: &mut Cartridge, address: u16, value: u8) {
    match address {
        0x0000..=0x1FFF => {
            if ram_supported(cartridge) || timer_supported(cartridge) {
                cartridge.mbc3.ram_rtc_enabled = (value & 0xF) == 0x0A;
            }
        },
        0x2000..=0x3FFF => {
            let value = if value == 0 { 1 } else { value };
            cartridge.mbc3.rom_bank_number = value & 0x7F;
        },
        0x4000..=0x5FFF => {
            if value <= 0x03 || (value >= 0x08 && value <= 0x0C) {
                cartridge.mbc3.ram_rtc_selection = value;
            }
        },
        0x6000..=0x7FFF => {
            if timer_supported(cartridge) {
                if cartridge.mbc3.rtc_latch == 0x00 && value == 0x01 {
                    cartridge.mbc3.rtc = (cartridge.mbc3.get_next_rtc)();
                }
                cartridge.mbc3.rtc_latch = value;
            }
        },
        _ => panic!("Invalid ROM address: {:#X}", address),
    } 
}

pub fn read_rom(cartridge: &Cartridge, address: u16) -> u8 {
    match address {
        0x0000..=0x3FFF =>
            cartridge.rom[address as usize],
        0x4000..=0x7FFF => {
            let base_location = cartridge.mbc3.rom_bank_number as u32 * 0x4000;
            let calculated_address = base_location + ((address & 0x3FFF) as u32);
            cartridge.rom[calculated_address as usize]
        },
        _ => panic!("Invalid ROM address: {:#X}", address),
    }
}

pub fn write_ram(cartridge: &mut Cartridge, address: u16, value: u8) {
    if cartridge.mbc3.ram_rtc_enabled {
        let ram_rtc_selection = cartridge.mbc3.ram_rtc_selection;

        match ram_rtc_selection {
            0x00..=0x03 if ram_supported(cartridge) => {
                let calculated_address = (cartridge.mbc3.ram_rtc_selection as u16 * 0x2000) + address;
                cartridge.ram[calculated_address as usize] = value;
            },
            0x08..=0x0C if timer_supported(cartridge) => {
                match ram_rtc_selection {
                    0x08 => cartridge.mbc3.rtc.seconds = value,
                    0x09 => cartridge.mbc3.rtc.minutes = value,
                    0x0A => cartridge.mbc3.rtc.hours = value,
                    0x0B => cartridge.mbc3.rtc.days_lower = value,
                    0x0C => cartridge.mbc3.rtc.days_upper = value,
                    _ => ()
                }
            }
            _ =>
                ()
        }
    }
}

pub fn read_ram(cartridge: &Cartridge, address: u16) -> u8 {
    if cartridge.mbc3.ram_rtc_enabled {
        let ram_rtc_selection = cartridge.mbc3.ram_rtc_selection;

        match ram_rtc_selection {
            0x00..=0x03 if ram_supported(cartridge) => {
                let calculated_address = (cartridge.mbc3.ram_rtc_selection as u16 * 0x2000) + address;
                cartridge.ram[calculated_address as usize]
            },
            0x08..=0x0C if timer_supported(cartridge) => {
                match ram_rtc_selection {
                    0x08 => cartridge.mbc3.rtc.seconds,
                    0x09 => cartridge.mbc3.rtc.minutes,
                    0x0A => cartridge.mbc3.rtc.hours,
                    0x0B => cartridge.mbc3.rtc.days_lower,
                    0x0C => cartridge.mbc3.rtc.days_upper,
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

#[cfg(test)]
mod tests {
    use crate::emulator::initialize_screenless_emulator;
    use crate::mmu::cartridge::{CART_TYPE_MBC3, CART_TYPE_MBC3_RAM, CART_TYPE_MBC3_TIMER_BATTERY, CART_TYPE_MBC3_TIMER_RAM_BATTERY};
    use crate::mmu::{self, load_rom_buffer};
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
    fn reads_from_bank_0() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.in_bios = false;

        let mut rom_buffer = vec![0; 0x40000];
        rom_buffer[0] = 0xB1;
        rom_buffer[1] = 0xD2;

        load_rom_buffer(&mut emulator.memory, rom_buffer).unwrap();

        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC3;

        assert_eq!(mmu::read_byte(&mut emulator, 0x1), 0xD2);
    }

    #[test]
    fn reads_from_bank_2() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.in_bios = false;

        let mut rom_buffer = vec![0; 0x40000];
        rom_buffer[0x8000] = 0xB1;
        rom_buffer[0x8001] = 0xD2;

        load_rom_buffer(&mut emulator.memory, rom_buffer).unwrap();

        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC3;
        emulator.memory.cartridge.mbc3.rom_bank_number = 2;

        assert_eq!(mmu::read_byte(&mut emulator, 0x4001), 0xD2);
    }

    #[test]
    fn reads_from_ram_bank_1() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.in_bios = false;

        let rom_buffer = vec![0; 0x40000];
        load_rom_buffer(&mut emulator.memory, rom_buffer).unwrap();

        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC3_RAM;
        emulator.memory.cartridge.ram[0x2001] = 0xDD;
        emulator.memory.cartridge.mbc3.ram_rtc_enabled = true;
        emulator.memory.cartridge.mbc3.ram_rtc_selection = 1;

        assert_eq!(mmu::read_byte(&mut emulator, 0xA001), 0xDD);
    }

    #[test]
    fn does_not_read_from_ram_if_not_supported() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.in_bios = false;

        let rom_buffer = vec![0; 0x40000];
        load_rom_buffer(&mut emulator.memory, rom_buffer).unwrap();

        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC3_TIMER_BATTERY;
        emulator.memory.cartridge.ram[0x2001] = 0xDD;
        emulator.memory.cartridge.mbc3.ram_rtc_enabled = true;
        emulator.memory.cartridge.mbc3.ram_rtc_selection = 1;

        assert_eq!(mmu::read_byte(&mut emulator, 0xA001), 0xFF);
    }

    #[test]
    fn writes_to_ram_bank_1() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.in_bios = false;

        let rom_buffer = vec![0; 0x40000];
        load_rom_buffer(&mut emulator.memory, rom_buffer).unwrap();

        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC3_RAM;
        emulator.memory.cartridge.mbc3.ram_rtc_enabled = true;
        emulator.memory.cartridge.mbc3.ram_rtc_selection = 1;

        mmu::write_byte(&mut emulator, 0xA001, 0xDD);

        assert_eq!(emulator.memory.cartridge.ram[0x2001], 0xDD);
    }

    #[test]
    fn does_not_write_to_ram_if_not_supported() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.in_bios = false;

        let rom_buffer = vec![0; 0x40000];
        load_rom_buffer(&mut emulator.memory, rom_buffer).unwrap();

        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC3_TIMER_BATTERY;
        emulator.memory.cartridge.mbc3.ram_rtc_enabled = true;
        emulator.memory.cartridge.mbc3.ram_rtc_selection = 1;

        mmu::write_byte(&mut emulator, 0xA001, 0xDD);

        assert_eq!(emulator.memory.cartridge.ram[0x2001], 0x00);
    }

    #[test]
    fn enables_ram_or_rtc() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.in_bios = false;

        let rom_buffer = vec![0; 0x40000];
        load_rom_buffer(&mut emulator.memory, rom_buffer).unwrap();

        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC3_RAM;

        mmu::write_byte(&mut emulator, 0x0, 0x0A);

        assert_eq!(emulator.memory.cartridge.mbc3.ram_rtc_enabled, true);
    }

    #[test]
    fn does_not_enable_ram_or_rtc_if_neither_ram_nor_timer_supported() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.in_bios = false;

        let rom_buffer = vec![0; 0x40000];
        load_rom_buffer(&mut emulator.memory, rom_buffer).unwrap();

        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC3;

        mmu::write_byte(&mut emulator, 0x0, 0x0A);

        assert_eq!(emulator.memory.cartridge.mbc3.ram_rtc_enabled, false);
    }

    #[test]
    fn enables_ram_or_rtc_if_timer_supported() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.in_bios = false;

        let rom_buffer = vec![0; 0x40000];
        load_rom_buffer(&mut emulator.memory, rom_buffer).unwrap();

        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC3_TIMER_BATTERY;

        mmu::write_byte(&mut emulator, 0x0, 0x0A);

        assert_eq!(emulator.memory.cartridge.mbc3.ram_rtc_enabled, true);
    }

    #[test]
    fn changes_rom_bank_number_to_bank_2() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.in_bios = false;

        let rom_buffer = vec![0; 0x40000];
        load_rom_buffer(&mut emulator.memory, rom_buffer).unwrap();

        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC3;
        emulator.memory.cartridge.mbc3.rom_bank_number = 1;

        mmu::write_byte(&mut emulator, 0x2000, 0x02);

        assert_eq!(emulator.memory.cartridge.mbc3.rom_bank_number, 2);
    }

    #[test]
    fn changes_rom_bank_number_to_bank_1_when_writing_0() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.in_bios = false;

        let rom_buffer = vec![0; 0x40000];
        load_rom_buffer(&mut emulator.memory, rom_buffer).unwrap();

        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC3;
        emulator.memory.cartridge.mbc3.rom_bank_number = 2;

        mmu::write_byte(&mut emulator, 0x2000, 0x00);

        assert_eq!(emulator.memory.cartridge.mbc3.rom_bank_number, 1);
    }

    #[test]
    fn writes_ram_rtc_selection() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.in_bios = false;

        let rom_buffer = vec![0; 0x40000];
        load_rom_buffer(&mut emulator.memory, rom_buffer).unwrap();

        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC3_RAM;
        emulator.memory.cartridge.mbc3.ram_rtc_selection = 0;

        mmu::write_byte(&mut emulator, 0x4000, 0x01);

        assert_eq!(emulator.memory.cartridge.mbc3.ram_rtc_selection, 1);
    }

    #[test]
    fn reads_from_minutes_rtc_register() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.in_bios = false;

        let rom_buffer = vec![0; 0x40000];
        load_rom_buffer(&mut emulator.memory, rom_buffer).unwrap();

        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC3_TIMER_RAM_BATTERY;
        emulator.memory.cartridge.mbc3.ram_rtc_enabled = true;
        emulator.memory.cartridge.mbc3.ram_rtc_selection = 0x09;
        emulator.memory.cartridge.mbc3.rtc.minutes = 0x02;

        assert_eq!(mmu::read_byte(&mut emulator, 0xA000), 0x02);
    }

    #[test]
    fn does_not_read_from_minutes_rtc_register_if_not_supported() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.in_bios = false;

        let rom_buffer = vec![0; 0x40000];
        load_rom_buffer(&mut emulator.memory, rom_buffer).unwrap();

        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC3_RAM;
        emulator.memory.cartridge.mbc3.ram_rtc_enabled = true;
        emulator.memory.cartridge.mbc3.ram_rtc_selection = 0x09;
        emulator.memory.cartridge.mbc3.rtc.minutes = 0x02;

        assert_eq!(mmu::read_byte(&mut emulator, 0xA000), 0xFF);
    }

    #[test]
    fn writes_to_minutes_rtc_register() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.in_bios = false;

        let rom_buffer = vec![0; 0x40000];
        load_rom_buffer(&mut emulator.memory, rom_buffer).unwrap();

        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC3_TIMER_RAM_BATTERY;
        emulator.memory.cartridge.mbc3.ram_rtc_enabled = true;
        emulator.memory.cartridge.mbc3.ram_rtc_selection = 0x09;

        mmu::write_byte(&mut emulator, 0xA000, 0x02);

        assert_eq!(emulator.memory.cartridge.mbc3.rtc.minutes, 0x02);
    }

    #[test]
    fn does_not_write_to_minutes_rtc_register_if_not_supported() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.in_bios = false;

        let rom_buffer = vec![0; 0x40000];
        load_rom_buffer(&mut emulator.memory, rom_buffer).unwrap();

        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC3_RAM;
        emulator.memory.cartridge.mbc3.ram_rtc_enabled = true;
        emulator.memory.cartridge.mbc3.ram_rtc_selection = 0x09;

        mmu::write_byte(&mut emulator, 0xA000, 0x02);

        assert_eq!(emulator.memory.cartridge.mbc3.rtc.minutes, 0x00);
    }

    #[test]
    fn latches_rtc() {
        let mut emulator = initialize_screenless_emulator();
        emulator.memory.in_bios = false;

        let rom_buffer = vec![0; 0x40000];
        load_rom_buffer(&mut emulator.memory, rom_buffer).unwrap();

        emulator.memory.cartridge.header.type_code = CART_TYPE_MBC3_TIMER_RAM_BATTERY;
        emulator.memory.cartridge.mbc3.ram_rtc_enabled = true;
        emulator.memory.cartridge.mbc3.get_next_rtc = fake_clock;

        mmu::write_byte(&mut emulator, 0x6000, 0x00);
        mmu::write_byte(&mut emulator, 0x6000, 0x01);

        assert_eq!(emulator.memory.cartridge.mbc3.rtc.seconds, 10);
        assert_eq!(emulator.memory.cartridge.mbc3.rtc.minutes, 10);
        assert_eq!(emulator.memory.cartridge.mbc3.rtc.hours, 10);
        assert_eq!(emulator.memory.cartridge.mbc3.rtc.days_lower, 0);
        assert_eq!(emulator.memory.cartridge.mbc3.rtc.days_upper, 0);
    }
}