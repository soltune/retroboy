use crate::emulator::Emulator;
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
    pub zero_page_ram: [u8; 0x80],
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
        zero_page_ram: [0; 0x80],
    }
}

pub fn read_byte(emulator: &Emulator, address: u16) -> u8 {
    let memory = &emulator.memory;
    match address & 0xF000 {
        0x0000 if address < 0x0100 && memory.in_bios => memory.bios[address as usize],
        0x0000..=0x0FFF => memory.rom[address as usize],
        0x1000..=0x7FFF => memory.rom[address as usize],
        0x8000..=0x9FFF => memory.video_ram[(address & 0x1FFF) as usize],
        0xA000..=0xBFFF => memory.external_ram[(address & 0x1FFF) as usize],
        0xC000..=0xEFFF => memory.working_ram[(address & 0x1FFF) as usize],
        0xF000 => match address & 0x0F00 {
            0x000..=0xD00 => memory.working_ram[(address & 0x1FFF) as usize],
            0xE00 if address < 0xFEA0 => memory.object_attribute_memory[(address & 0xFF) as usize],
            0xF00 if address == 0xFFFF => emulator.interrupts.enabled,
            0xF00 if address >= 0xFF80 => memory.zero_page_ram[(address & 0x7F) as usize],
            _ => match address & 0xFF {
                0x40 => emulator.gpu.registers.lcdc,
                0x41 => emulator.gpu.registers.stat,
                0x42 => emulator.gpu.registers.scy,
                0x43 => emulator.gpu.registers.scx,
                0x44 => emulator.gpu.registers.ly,
                0x45 => emulator.gpu.registers.lyc,
                0x47 => emulator.gpu.registers.palette,
                0x4A => emulator.gpu.registers.wy,
                0x4B => emulator.gpu.registers.wx,
                0x0F => emulator.interrupts.flags,
                0x04 => emulator.timers.divider,
                0x05 => emulator.timers.counter,
                0x06 => emulator.timers.modulo,
                0x07 => emulator.timers.control,
                _ => 0x00
            }
        },
        _ => 0x00,
    }
}

pub fn write_byte(emulator: &mut Emulator, address: u16, value: u8) {
    let memory = &mut emulator.memory;
    match address & 0xF000 {
        0x0000 if address < 0x0100 && memory.in_bios => memory.bios[address as usize] = value,
        // You can't actually write to ROM. The next couple lines will probably change when
        // MBC support is implemented.
        0x0000..=0x0FFF => memory.rom[address as usize] = value,
        0x1000..=0x7FFF => memory.rom[address as usize] = value,
        0x8000..=0x9FFF => memory.video_ram[(address & 0x1FFF) as usize] = value,
        0xA000..=0xBFFF => memory.external_ram[(address & 0x1FFF) as usize] = value,
        0xC000..=0xEFFF => memory.working_ram[(address & 0x1FFF) as usize] = value,
        0xF000 => match address & 0x0F00 {
            0x000..=0xD00 => memory.working_ram[(address & 0x1FFF) as usize] = value,
            0xE00 if address < 0xFEA0 => memory.object_attribute_memory[(address & 0xFF) as usize]= value,
            0xF00 if address == 0xFFFF => emulator.interrupts.enabled = value,
            0xF00 if address >= 0xFF80 => memory.zero_page_ram[(address & 0x7F) as usize] = value,
            _ => match address & 0xFF {
                0x40 => emulator.gpu.registers.lcdc = value,
                0x41 => emulator.gpu.registers.stat = value,
                0x42 => emulator.gpu.registers.scy = value,
                0x43 => emulator.gpu.registers.scx = value,
                0x44 => emulator.gpu.registers.ly = value,
                0x45 => emulator.gpu.registers.lyc = value,
                0x47 => emulator.gpu.registers.palette = value,
                0x4A => emulator.gpu.registers.wy = value,
                0x4B => emulator.gpu.registers.wx = value,
                0x0F => emulator.interrupts.flags = value,
                0x04 => emulator.timers.divider = value,
                0x05 => emulator.timers.counter = value,
                0x06 => emulator.timers.modulo = value,
                0x07 => emulator.timers.control = value,
                _ => ()
            }
        },
        _ => (),
    }
}

pub fn read_word(emulator: &Emulator, address: u16) -> u16 {
    let first_byte = read_byte(&emulator, address) as u16;
    let second_byte = read_byte(&emulator, address + 1) as u16;
    first_byte + (second_byte << 8)
}

pub fn write_word(emulator: &mut Emulator, address: u16, value: u16) {
    let first_byte = value & 0xFF;
    let second_byte = value >> 8;
    write_byte(emulator, address, first_byte as u8);
    write_byte(emulator, address + 1, second_byte as u8);
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
