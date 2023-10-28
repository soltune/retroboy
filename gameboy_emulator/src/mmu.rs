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

impl Memory {
    pub fn new() -> Memory {
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
    
    pub fn read_byte(&self, address: u16) -> u8 {
        match address & 0xF000 {
            0x0000 => {
                if address < 0x0100 && self.in_bios {
                    self.bios[address as usize]
                }
                else {
                    self.rom[address as usize]
                }
            },
            0x1000 | 0x2000 | 0x3000 | 0x4000 |
            0x5000 | 0x6000 | 0x7000 =>
                // I will implement bank switching later.
                self.rom[address as usize],
            0x8000 | 0x9000 =>
                self.video_ram[(address & 0x1FFF) as usize],
            0xA000 | 0xB000 =>
                self.external_ram[(address & 0x1FFF) as usize],
            0xC000 | 0xD000 | 0xE000 =>
                self.working_ram[(address & 0x1FFF) as usize],
            0xF000 =>
                match address & 0x0F00 {
                    0x000 | 0x100 | 0x200 | 0x300 |
                    0x400 | 0x500 | 0x600 | 0x700 |
                    0x800 | 0x900 | 0xA00 | 0xB00 |
                    0xC00 | 0xD00 =>
                        self.working_ram[(address & 0x1FFF) as usize],
                    0xE00 =>
                        if address < 0xFEA0 {
                            self.object_attribute_memory[(address & 0xFF) as usize]
                        }
                        else {
                            0
                        }
                    0xF00 =>
                        if address >= 0xFF80 {
                            self.zero_page_ram[(address & 0x7F) as usize]
                        }
                        else {
                            0
                        },
                    _ => 0x00
                },
            _ => 0x00
        }
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let first_byte = self.read_byte(address) as u16;
        let second_byte = self.read_byte(address + 1) as u16;
        first_byte + (second_byte << 8)
    }

    pub fn load_rom_buffer(& mut self, buffer: Vec<u8>) {
        self.rom[..0x8000].copy_from_slice(&buffer[..0x8000])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_memory_data() -> Memory {
        let mut memory = Memory::new();

        memory.bios[0] = 0xAF;
        memory.bios[1] = 0xF1;
        memory.bios[2] = 0x03;
        memory.bios[3] = 0x4D;

        memory.rom[0] = 0x1E;
        memory.rom[1] = 0xF2;
        memory.rom[2] = 0x01;
        memory.rom[3] = 0x09;

        memory.rom[0x20AF] = 0x11;
        memory.rom[0x20B0] = 0x17;
        memory.rom[0x20B1] = 0xEE;

        memory.rom[0x5ACC] = 0x13;
        memory.rom[0x5ACD] = 0x9C;
        memory.rom[0x5ACE] = 0x55;

        memory.video_ram[0] = 0xB1;
        memory.video_ram[1] = 0xD2;
        memory.video_ram[2] = 0xAA;

        memory.external_ram[0] = 0xC2;
        memory.external_ram[1] = 0x22;
        memory.external_ram[2] = 0x35;

        memory.working_ram[0] = 0xF1;
        memory.working_ram[1] = 0x22;
        memory.working_ram[2] = 0x2B;

        memory.working_ram[0x15F0] = 0x2B;
        memory.working_ram[0x15F1] = 0x7C;

        memory.object_attribute_memory[0x7A] = 0x44;
        memory.object_attribute_memory[0x7B] = 0x45;
        memory.object_attribute_memory[0x7C] = 0x9B;

        memory.zero_page_ram[0x20] = 0xBB;
        memory.zero_page_ram[0x21] = 0x44;
        memory.zero_page_ram[0x5B] = 0x5F;

        memory.in_bios = false;

        return memory;
    }

    #[test]
    fn reads_from_bios() {
        let mut memory = setup_test_memory_data();
        memory.in_bios = true;
        assert_eq!(memory.read_byte(0x02), 0x03);
    }

    #[test]
    fn reads_from_rom_in_bank_zero() {
        let memory = setup_test_memory_data();
        assert_eq!(memory.read_byte(0x02), 0x01);
    }

    #[test]
    fn reads_from_rom_in_bank_zero_scenario_two() {
        let memory = setup_test_memory_data();
        assert_eq!(memory.read_byte(0x20B1), 0xEE);
    }

    #[test]
    fn reads_from_rom_in_subsequent_bank() {
        let memory = setup_test_memory_data();
        assert_eq!(memory.read_byte(0x5ACE), 0x55);
    }

    #[test]
    fn reads_from_video_ram() {
        let memory = setup_test_memory_data();
        assert_eq!(memory.read_byte(0x8002), 0xAA);
    }

    #[test]
    fn reads_from_external_ram() {
        let memory = setup_test_memory_data();
        assert_eq!(memory.read_byte(0xA001), 0x22);
    }

    #[test]
    fn reads_from_working_ram() {
        let memory = setup_test_memory_data();
        assert_eq!(memory.read_byte(0xC002), 0x2B);
    }

    #[test]
    fn reads_from_working_ram_shadow() {
        let memory = setup_test_memory_data();
        assert_eq!(memory.read_byte(0xE002), 0x2B);
    }

    #[test]
    fn reads_from_working_ram_shadow_scenario_two() {
        let memory = setup_test_memory_data();
        assert_eq!(memory.read_byte(0xF5F0), 0x2B);
    }

    #[test]
    fn reads_from_object_attribute_memory() {
        let memory = setup_test_memory_data();
        assert_eq!(memory.read_byte(0xFE7B), 0x45);
    }

    #[test]
    fn reads_zero_values_outside_of_object_attribute_memory() {
        let memory = setup_test_memory_data();
        assert_eq!(memory.read_byte(0xFEEE), 0x00);
    }

    #[test]
    fn reads_from_zero_page_ram() {
        let memory = setup_test_memory_data();
        assert_eq!(memory.read_byte(0xFFA0), 0xBB);
    }

    #[test]
    fn reads_word_from_memory() {
        let memory = setup_test_memory_data();
        assert_eq!(memory.read_word(0x20AF), 0x1711);
    }

    #[test]
    fn loads_rom_buffer_into_memory() {
        let mut memory = setup_test_memory_data();

        let mut rom_buffer = vec![0; 0xA000];
        rom_buffer[0] = 0xA0;
        rom_buffer[1] = 0xCC;
        rom_buffer[2] = 0x3B;
        rom_buffer[3] = 0x4C;
        rom_buffer[0x7FFF] = 0xD4;
        rom_buffer[0x8000] = 0xBB;
        rom_buffer[0x8001] = 0xD1;

        memory.load_rom_buffer(rom_buffer);

        assert_eq!(memory.read_byte(0x0000), 0xA0);
        assert_eq!(memory.read_byte(0x0001), 0xCC);
        assert_eq!(memory.read_byte(0x0002), 0x3B);
        assert_eq!(memory.read_byte(0x0003), 0x4C);
        assert_eq!(memory.read_byte(0x7FFF), 0xD4);
        assert_eq!(memory.read_byte(0x8000), 0xB1);
    }
}
