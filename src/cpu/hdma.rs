use crate::cpu::microops;
use crate::emulator::{is_cgb, Emulator};
use crate::mmu;
use crate::utils::is_bit_set;

#[derive(Debug, PartialEq, Eq)]
pub enum VRAMTransferMode {
    GeneralPurpose,
    HBlank
}

#[derive(Debug)]
pub struct HDMAState {
    pub hdma1: u8,
    pub hdma2: u8,
    pub hdma3: u8,
    pub hdma4: u8,
    pub offset: u16,
    pub transfer_length: u8,
    pub transfer_mode: VRAMTransferMode,
    pub in_progress: bool,
    pub completed: bool,
    pub hblank_started: bool
}

pub fn initialize_hdma() -> HDMAState {
    HDMAState {
        hdma1: 0x0,
        hdma2: 0x0,
        hdma3: 0x0,
        hdma4: 0x0,
        offset: 0x0,
        transfer_length: 0x0,
        transfer_mode: VRAMTransferMode::GeneralPurpose,
        in_progress: false,
        completed: true,
        hblank_started: false
    }
}

const VRAM_TRANSFER_INDEX: u8 = 7;
const BLOCK_SIZE: u8 = 16;

pub fn set_hdma1(emulator: &mut Emulator, value: u8) {
    if is_cgb(emulator) {
        emulator.hdma.hdma1 = value;
    }
}

pub fn set_hdma2(emulator: &mut Emulator, value: u8) {
    if is_cgb(emulator) {
        emulator.hdma.hdma2 = value;
    }
}

pub fn set_hdma3(emulator: &mut Emulator, value: u8) {
    if is_cgb(emulator) {
        emulator.hdma.hdma3 = value;
    }
}

pub fn set_hdma4(emulator: &mut Emulator, value: u8) {
    if is_cgb(emulator) {
        emulator.hdma.hdma4 = value;
    }
}

pub fn set_hdma5(emulator: &mut Emulator, value: u8) {
    if is_cgb(emulator) {
        if emulator.hdma.in_progress && !is_bit_set(value, VRAM_TRANSFER_INDEX) {
            emulator.hdma.in_progress = false;
        }
        else {
            emulator.hdma.transfer_length = value & 0b01111111;
   
            let transfer_bit_set = is_bit_set(value, VRAM_TRANSFER_INDEX);
            let mode = if transfer_bit_set { VRAMTransferMode::HBlank } else { VRAMTransferMode::GeneralPurpose };
            emulator.hdma.transfer_mode = mode;
   
            emulator.hdma.in_progress = true;
            emulator.hdma.completed = false;
        }
    }
}

pub fn get_hdma5(emulator: &Emulator) -> u8 {
    if is_cgb(emulator) {
        if emulator.hdma.in_progress {
            emulator.hdma.transfer_length & 0b01111111
        } else if emulator.hdma.completed {
            0xFF
        } else {
            0b10000000 | (emulator.hdma.transfer_length & 0b01111111)
        }
    }
    else {
        0xFF
    }
}

fn get_vram_dma_source(emulator: &Emulator) -> u16 {
    ((emulator.hdma.hdma1 as u16) << 8) | ((emulator.hdma.hdma2 as u16) & 0b11110000)
}

fn get_vram_dma_destination(emulator: &Emulator) -> u16 {
    let offset = (((emulator.hdma.hdma3 as u16) & 0b00011111) << 8) |
        ((emulator.hdma.hdma4 as u16) & 0b11110000);

    0x8000 + offset
}

pub fn set_hblank_started(emulator: &mut Emulator, value: bool) {
    if emulator.hdma.in_progress {
        emulator.hdma.hblank_started = value; 
    }
}

fn transfer_block(emulator: &mut Emulator, source: u16, destination: u16) {
    for _ in (0..BLOCK_SIZE).step_by(2) {
        for _ in 0..2 {
            let offset = emulator.hdma.offset;
            let source_byte = mmu::read_byte(emulator, source + offset);
            mmu::write_byte(emulator, destination + offset, source_byte);
            emulator.hdma.offset += 1;
        }

        // Takes one machine cycle (or two "fast" machine cycles in double speed mode)
        // to transfer two bytes during VRAM DMA.
        let cycle_count = if emulator.speed_switch.cgb_double_speed { 2 } else { 1 };
        microops::step_machine_cycles(emulator, cycle_count);
    }

    if emulator.hdma.transfer_length == 0 {
        emulator.hdma.completed = true;
        emulator.hdma.in_progress = false;
        emulator.hdma.offset = 0;
    }
    else {
        emulator.hdma.transfer_length -= 1;
    }
}

pub fn step(emulator: &mut Emulator) {
    if is_cgb(emulator) && emulator.hdma.in_progress {
        let source = get_vram_dma_source(emulator);
        let destination = get_vram_dma_destination(emulator);

        let is_hblank_mode = emulator.hdma.transfer_mode == VRAMTransferMode::HBlank;
        if is_hblank_mode && emulator.hdma.hblank_started {
            transfer_block(emulator, source, destination);
            emulator.hdma.hblank_started = false;
        }
        else if !is_hblank_mode {
            while !emulator.hdma.completed {
                transfer_block(emulator, source, destination);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::{initialize_screenless_emulator, Mode};
    use crate::mmu;
    use crate::mmu::constants::*;
    use crate::mmu::effects::empty_cartridge_effects;
    use crate::mmu::test_utils::*;
    use super::*;

    #[test]
    fn should_calculate_vram_dma_source() {
        let mut emulator = initialize_screenless_emulator();
        emulator.mode = Mode::CGB;
        set_hdma1(&mut emulator, 0x71);
        set_hdma2(&mut emulator, 0xA2);
        let vram_source = get_vram_dma_source(&emulator);
        assert_eq!(vram_source, 0x71A0);
    }

    #[test]
    fn should_calculate_vram_dma_destination() {
        let mut emulator = initialize_screenless_emulator();
        emulator.mode = Mode::CGB;
        set_hdma3(&mut emulator, 0x71);
        set_hdma4(&mut emulator, 0xA2);
        let vram_destination = get_vram_dma_destination(&emulator);
        assert_eq!(vram_destination, 0x91A0);
    }

    #[test]
    fn should_get_general_purpose_for_vram_transfer_mode() {
        let mut emulator = initialize_screenless_emulator();
        emulator.mode = Mode::CGB;
        set_hdma5(&mut emulator, 0x71);
        assert_eq!(emulator.hdma.transfer_mode, VRAMTransferMode::GeneralPurpose);
    }

    #[test]
    fn should_get_hblank_for_vram_transfer_mode() {
        let mut emulator = initialize_screenless_emulator();
        emulator.mode = Mode::CGB;
        set_hdma5(&mut emulator, 0xF1);
        assert_eq!(emulator.hdma.transfer_mode, VRAMTransferMode::HBlank);
    }

    #[test]
    fn should_get_hdma5_when_transfer_is_not_active() {
        let mut emulator = initialize_screenless_emulator();
        emulator.mode = Mode::CGB;
        let hdma5 = get_hdma5(&emulator);
        assert_eq!(hdma5, 0xFF);
    }

    #[test]
    fn should_get_hdma5_when_transfer_is_active() {
        let mut emulator = initialize_screenless_emulator();
        emulator.mode = Mode::CGB;
        emulator.hdma.in_progress = true;
        emulator.hdma.transfer_length = 0b01010110;
        let hdma5 = get_hdma5(&emulator);
        assert_eq!(hdma5, 0b01010110);
    }

    #[test]
    fn should_terminate_active_vram_dma_transfer() {
        let mut emulator = initialize_screenless_emulator();
        emulator.mode = Mode::CGB;
        emulator.hdma.in_progress = true;
        emulator.hdma.completed = false;
        emulator.hdma.transfer_length = 0b01010110;
        set_hdma5(&mut emulator, 0x0);
        assert_eq!(emulator.hdma.in_progress, false);
        let hdma5 = get_hdma5(&emulator);
        assert_eq!(hdma5, 0b11010110);
    }

    #[test]
    fn should_transfer_sixteen_bytes_in_hblank_mode() {
        let mut emulator = initialize_screenless_emulator();
        emulator.mode = Mode::CGB;

        let mut test_instructions = build_rom(CART_TYPE_MBC1, ROM_SIZE_64KB, RAM_SIZE_2KB);
        for i in 0..32 {
            test_instructions[0x71A0 + i] = 0xA1;
        }

        mmu::load_rom_buffer(&mut emulator.memory, test_instructions, empty_cartridge_effects()).unwrap();

        set_hdma1(&mut emulator, 0x71);
        set_hdma2(&mut emulator, 0xA2);
        set_hdma3(&mut emulator, 0x71);
        set_hdma4(&mut emulator, 0xA2);
        set_hdma5(&mut emulator, 0x81); // transfer length of 0x20

        emulator.gpu.mode = 0; // mode 0 = hblank mode
        set_hblank_started(&mut emulator, true);

        step(&mut emulator);

        for i in 0..16 {
            assert_eq!(emulator.gpu.video_ram[0x11A0 + i], 0xA1);
        }
        
        for i in 16..32 {
            assert_eq!(emulator.gpu.video_ram[0x11A0 + i], 0x0);
        }

        assert_eq!(emulator.hdma.hblank_started, false);
        assert_eq!(emulator.cpu.clock.total_clock_cycles, 32);
    }

    #[test]
    fn should_transfer_all_bytes_in_general_purpose_mode() {
        let mut emulator = initialize_screenless_emulator();
        emulator.mode = Mode::CGB;

        let mut test_instructions = build_rom(CART_TYPE_MBC1, ROM_SIZE_64KB, RAM_SIZE_2KB);
        for i in 0..32 {
            test_instructions[0x71A0 + i] = 0xA1;
        }

        mmu::load_rom_buffer(&mut emulator.memory, test_instructions, empty_cartridge_effects()).unwrap();

        set_hdma1(&mut emulator, 0x71);
        set_hdma2(&mut emulator, 0xA2);
        set_hdma3(&mut emulator, 0x71);
        set_hdma4(&mut emulator, 0xA2);
        set_hdma5(&mut emulator, 0x01); // transfer length of 0x20

        step(&mut emulator);

        for i in 0..32 {
            assert_eq!(emulator.gpu.video_ram[0x11A0 + i], 0xA1);
        }

        assert_eq!(emulator.cpu.clock.total_clock_cycles, 64);
    }
}
