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

pub fn read_byte(memory: &Memory, address: u16) -> u8 {
    match address & 0xF000 {
        0x0000 => {
            if address < 0x0100 && memory.in_bios {
                memory.bios[address as usize]
            }
            else {
                memory.rom[address as usize]
            }
        },
        0x1000 | 0x2000 | 0x3000 | 0x4000 |
        0x5000 | 0x6000 | 0x7000 =>
            // TODO: Implement bank switching
            memory.rom[address as usize],
        0x8000 | 0x9000 =>
            memory.video_ram[(address & 0x1FFF) as usize],
        0xA000 | 0xB000 =>
            memory.external_ram[(address & 0x1FFF) as usize],
        0xC000 | 0xD000 | 0xE000 =>
            memory.working_ram[(address & 0x1FFF) as usize],
        0xF000 =>
            match address & 0x0F00 {
                0x000 | 0x100 | 0x200 | 0x300 |
                0x400 | 0x500 | 0x600 | 0x700 |
                0x800 | 0x900 | 0xA00 | 0xB00 |
                0xC00 | 0xD00 =>
                    memory.working_ram[(address & 0x1FFF) as usize],
                0xE00 =>
                    if address < 0xFEA0 {
                        memory.object_attribute_memory[(address & 0xFF) as usize]
                    }
                    else {
                        0
                    }
                0xF00 =>
                    if address >= 0xFF80 {
                        memory.zero_page_ram[(address & 0x7F) as usize]
                    }
                    else {
                        0
                    },
                _ => 0x00
            },
        _ => 0x00
    }
}

pub fn read_word(memory: &Memory, address: u16) -> u16 {
    let first_byte = read_byte(memory, address) as u16;
    let second_byte = read_byte(memory, address + 1) as u16;
    first_byte + (second_byte << 8)
}

pub fn load_rom_buffer(memory: & mut Memory, buffer: Vec<u8>) {
    memory.rom[..0x8000].copy_from_slice(&buffer[..0x8000]);
}

#[cfg(test)]
mod tests;
