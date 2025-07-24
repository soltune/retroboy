use crate::address_bus::cartridge::{Cartridge, CartridgeMapper, CartridgeMapperSnapshot, MBCSnapshot};

#[derive(Debug)]
pub struct MBCRomOnlyCartridgeMapper {
    cartridge: Cartridge
}

pub fn initialize_mbc_rom_only_mapper(cartridge: Cartridge) -> MBCRomOnlyCartridgeMapper {
    MBCRomOnlyCartridgeMapper {
        cartridge
    }
}

impl CartridgeMapper for MBCRomOnlyCartridgeMapper {
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

    fn get_snapshot(&self) -> CartridgeMapperSnapshot {
        CartridgeMapperSnapshot {
            ram: self.cartridge.ram.clone(),
            mbc: MBCSnapshot::RomOnly
        }
    }

    fn apply_snapshot(&mut self, snapshot: CartridgeMapperSnapshot) {
        self.cartridge.ram = snapshot.ram;
    }
}
