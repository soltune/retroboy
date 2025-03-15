use crate::mmu::cartridge::{Cartridge, CartridgeMapper};

#[derive(Debug)]
pub struct MBCRomOnly {
    cartridge: Cartridge
}

pub fn initialize_mbc_rom_only(cartridge: Cartridge) -> MBCRomOnly {
    MBCRomOnly {
        cartridge
    }
}

impl CartridgeMapper for MBCRomOnly {
    fn read_rom(&self, address: u16) -> u8 {
        self.cartridge.rom[address as usize]
    }

    fn write_rom(&mut self, _: u16, _: u8) {
        ()
    }

    fn read_ram(&self, _: u16) -> u8 {
        0xFF
    }

    fn write_ram(&mut self, _: u16, _: u8) {
        ()
    }

    fn get_cartridge(&self) -> &Cartridge {
        &self.cartridge
    }

    fn set_cartridge_ram(&mut self, _: Vec<u8>) {
        ()
    }

    fn get_ram_bank(&self) -> u8 {
        0
    }
}
