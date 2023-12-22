use std::io;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;

#[derive(Debug)]
pub struct Memory {
    pub in_bios: bool,
    pub bios: [u8; 0x100],
    pub rom: [u8; 0x8000],
    pub video_ram: [u8; 0x2000],
    pub object_attribute_memory: [u8; 0xa0],
    pub working_ram: [u8; 0x3e00],
    pub external_ram: [u8; 0x2000],
    pub zero_page_ram: [u8; 0x80]
}

pub enum MemoryLocation {
    Bios,
    Rom,
    VideoRam,
    ObjectAttributeMemory,
    WorkingRam,
    ExternalRam,
    ZeroPageRam,
    Forbidden
}

pub fn initialize_memory() -> Memory {
    Memory {
        in_bios: true,
        bios: [0; 0x100],
        rom: [0; 0x8000],
        video_ram: [0; 0x2000],
        object_attribute_memory: [0; 0xa0],
        working_ram: [0; 0x3e00],
        external_ram: [0; 0x2000],
        zero_page_ram: [0; 0x80]
    }
}

fn get_memory_location(address: u16, in_bios: bool) -> (MemoryLocation, u16) {
    match address & 0xF000 {
        0x0000 if address < 0x0100 && in_bios => (MemoryLocation::Bios, address),
        0x0000..=0x0FFF => (MemoryLocation::Rom, address),
        0x1000..=0x7FFF => (MemoryLocation::Rom, address),
        0x8000..=0x9FFF => (MemoryLocation::VideoRam, address & 0x1FFF),
        0xA000..=0xBFFF => (MemoryLocation::ExternalRam, address & 0x1FFF),
        0xC000..=0xEFFF => (MemoryLocation::WorkingRam, address & 0x1FFF),
        0xF000 => match address & 0x0F00 {
            0x000..=0xD00 => (MemoryLocation::WorkingRam, address & 0x1FFF),
            0xE00 if address < 0xFEA0 => (MemoryLocation::ObjectAttributeMemory, address & 0xFF),
            0xF00 if address >= 0xFF80 => (MemoryLocation::ZeroPageRam, address & 0x7F),
            _ => (MemoryLocation::Forbidden, address),
        },
        _ => (MemoryLocation::Forbidden, address),
    }
}

pub fn read_byte(memory: &Memory, address: u16) -> u8 {
    let (location, localized_address) = get_memory_location(address, memory.in_bios);
    match location {
        MemoryLocation::Bios =>
            memory.bios[localized_address as usize],
        MemoryLocation::Rom =>
            memory.rom[localized_address as usize],
        MemoryLocation::VideoRam =>
            memory.video_ram[localized_address as usize],
        MemoryLocation::ObjectAttributeMemory =>
            memory.object_attribute_memory[localized_address as usize],
        MemoryLocation::WorkingRam =>
            memory.working_ram[localized_address as usize],
        MemoryLocation::ExternalRam =>
            memory.external_ram[localized_address as usize],
        MemoryLocation::ZeroPageRam =>
            memory.zero_page_ram[localized_address as usize],
        MemoryLocation::Forbidden =>
            0x00
    }
}

pub fn write_byte(memory: &mut Memory, address: u16, value: u8) {
    let (location, localized_address) = get_memory_location(address, memory.in_bios);
    match location {
        MemoryLocation::Bios =>
            memory.bios[localized_address as usize] = value,
        MemoryLocation::Rom =>
            memory.rom[localized_address as usize] = value,
        MemoryLocation::VideoRam =>
            memory.video_ram[localized_address as usize] = value,
        MemoryLocation::ObjectAttributeMemory =>
            memory.object_attribute_memory[localized_address as usize] = value,
        MemoryLocation::WorkingRam =>
            memory.working_ram[localized_address as usize] = value,
        MemoryLocation::ExternalRam =>
            memory.external_ram[localized_address as usize] = value,
        MemoryLocation::ZeroPageRam =>
            memory.zero_page_ram[localized_address as usize] = value,
        MemoryLocation::Forbidden =>
            ()
    }
}

pub fn read_word(memory: &Memory, address: u16) -> u16 {
    let first_byte = read_byte(memory, address) as u16;
    let second_byte = read_byte(memory, address + 1) as u16;
    first_byte + (second_byte << 8)
}

pub fn write_word(memory: &mut Memory, address: u16, value: u16) {
    let first_byte = value & 0xFF;
    let second_byte = value >> 8;
    write_byte(memory, address, first_byte as u8);
    write_byte(memory, address + 1, second_byte as u8);
}

pub fn load_rom_buffer(mut memory: Memory, buffer: Vec<u8>) -> Memory {
    let slice_length = std::cmp::min(buffer.len(), 0x8000);
    memory.rom[..slice_length].copy_from_slice(&buffer[..slice_length]);
    memory
}

pub fn load_rom_by_filepath(memory: Memory, filepath: &str) -> io::Result<Memory> {
    let f = File::open(filepath)?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;
    let loaded_memory = load_rom_buffer(memory, buffer);
    
    Ok(loaded_memory)
}

#[cfg(test)]
mod tests;
