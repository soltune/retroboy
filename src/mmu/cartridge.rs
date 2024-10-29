use core::panic;
use std::io;

use crate::mmu::mbc1;
use crate::mmu::mbc1::{MBC1, initialize_mbc1};
use crate::mmu::mbc_rom_only;

#[derive(Debug)]
pub struct CartridgeHeader {
    pub sgb_support: bool,
    pub type_code: u8,
    pub max_banks: u16
}

#[derive(Debug)]
pub struct Cartridge {
    pub rom: Vec<u8>,
    pub ram: [u8; 0x8000],
    pub header: CartridgeHeader,
    pub mbc1: MBC1
}

const ENTRY_POINT_ADDRESS: usize = 0x100;
const SGB_SUPPORT_ADDRESS: usize = 0x146;
const CARTRIDGE_TYPE_ADDRESS: usize = 0x147;
const ROM_SIZE_ADDRESS: usize = 0x148;

pub const CART_TYPE_ROM_ONLY: u8 = 0;
pub const CART_TYPE_MBC1: u8 = 1;
pub const CART_TYPE_MBC1_WITH_RAM: u8 = 2;
pub const CART_TYPE_MBC1_WITH_RAM_PLUS_BATTERY: u8 = 3;

pub const SUPPORTED_CARTRIDGE_TYPES: [u8; 4] = [CART_TYPE_ROM_ONLY,
    CART_TYPE_MBC1,
    CART_TYPE_MBC1_WITH_RAM,
    CART_TYPE_MBC1_WITH_RAM_PLUS_BATTERY];

pub fn initialize_cartridge() -> Cartridge {
    Cartridge {
        rom: Vec::new(),
        ram: [0; 0x8000],
        header: CartridgeHeader {
            sgb_support: false,
            type_code: 0,
            max_banks: 0,
        },
        mbc1: initialize_mbc1()
    }
}

fn cartridge_type_supported(type_code: u8) -> bool {
    SUPPORTED_CARTRIDGE_TYPES.contains(&type_code)
}

fn as_max_banks(rom_size_index: u8) -> u16 {
    (2 as u16).pow(rom_size_index as u32 + 1)
}

pub fn load_rom_buffer(buffer: Vec<u8>) -> io::Result<Cartridge> {
    if buffer.len() > ENTRY_POINT_ADDRESS {
        let type_code = buffer[CARTRIDGE_TYPE_ADDRESS];
        let sgb_support = buffer[SGB_SUPPORT_ADDRESS] == 0x03;
        let rom_size = buffer[ROM_SIZE_ADDRESS];

        if cartridge_type_supported(type_code) {
            let cartridge = Cartridge {
                rom: buffer,
                ram: [0; 0x8000],
                header: CartridgeHeader {
                    sgb_support,
                    type_code,
                    max_banks: as_max_banks(rom_size),
                },
                mbc1: initialize_mbc1()
            };

            Ok(cartridge)
        } else {
            let error_message = format!("Unsupported cartridge type {type_code}.");
            Err(io::Error::new(io::ErrorKind::Other, error_message))
        }
    } else {
        let error_message = "Buffer is too small to contain a valid ROM.";
        Err(io::Error::new(io::ErrorKind::Other, error_message))
    }
}

pub fn read_rom(cartridge: &Cartridge, address: u16) -> u8 {
    match cartridge.header.type_code {
        CART_TYPE_ROM_ONLY =>
            mbc_rom_only::read_rom(cartridge, address),
        CART_TYPE_MBC1 | CART_TYPE_MBC1_WITH_RAM | CART_TYPE_MBC1_WITH_RAM_PLUS_BATTERY =>
            mbc1::read_rom(cartridge, address),
        _ =>
            panic!("Unsupported cartridge type: {}", cartridge.header.type_code),
    }
 }

pub fn write_rom(cartridge: &mut Cartridge, address: u16, value: u8) {
    match cartridge.header.type_code {
        CART_TYPE_ROM_ONLY =>
            mbc_rom_only::write_rom(cartridge, address, value),
        CART_TYPE_MBC1 | CART_TYPE_MBC1_WITH_RAM | CART_TYPE_MBC1_WITH_RAM_PLUS_BATTERY =>
            mbc1::write_rom(cartridge, address, value),
        _ =>
            panic!("Unsupported cartridge type: {}", cartridge.header.type_code),
    }
}

pub fn read_ram(cartridge: &Cartridge, address: u16) -> u8 {
    match cartridge.header.type_code {
        CART_TYPE_ROM_ONLY =>
            mbc_rom_only::read_ram(cartridge, address),
        CART_TYPE_MBC1 | CART_TYPE_MBC1_WITH_RAM | CART_TYPE_MBC1_WITH_RAM_PLUS_BATTERY =>
            mbc1::read_ram(cartridge, address),
        _ =>
            panic!("Unsupported cartridge type: {}", cartridge.header.type_code),
    }
}

pub fn write_ram(cartridge: &mut Cartridge, address: u16, value: u8) {
    match cartridge.header.type_code {
        CART_TYPE_ROM_ONLY =>
            mbc_rom_only::write_ram(cartridge, address, value),
        CART_TYPE_MBC1 | CART_TYPE_MBC1_WITH_RAM | CART_TYPE_MBC1_WITH_RAM_PLUS_BATTERY =>
            mbc1::write_ram(cartridge, address, value),
        _ =>
            panic!("Unsupported cartridge type: {}", cartridge.header.type_code),
    }
}
