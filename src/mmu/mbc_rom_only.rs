use crate::mmu::cartridge::Cartridge;

pub fn write_rom(_: &mut Cartridge, _: u16, _: u8) {
    ()
}

pub fn read_rom(cartridge: &Cartridge, address: u16) -> u8 {
    cartridge.rom[address as usize]
}

pub fn write_ram(_: &mut Cartridge, _: u16, _: u8) {
    ()
}

pub fn read_ram(_: &Cartridge, _: u16) -> u8 {
    0xFF
}
