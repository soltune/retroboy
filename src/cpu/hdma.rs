use crate::emulator::{is_cgb, Emulator};
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
    pub offset: u8,
    pub transfer_length: u8,
    pub transfer_mode: VRAMTransferMode,
    pub in_progress: bool
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
        in_progress: false
    }
}

const VRAM_TRANSFER_INDEX: u8 = 7;

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
       }
    }
}

pub fn get_hdma5(emulator: &Emulator) -> u8 {
    if is_cgb(emulator) {
        if emulator.hdma.in_progress {
            emulator.hdma.transfer_length & 0b01111111
        } else if emulator.hdma.transfer_length == 0 {
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

fn calculate_vram_dma_transfer_length(emulator: &Emulator) -> u16 {
    (emulator.hdma.transfer_length as u16 + 1) * 0x10
}

pub fn step(emulator: &mut Emulator) {
    // TODO: Step logic here
}

#[cfg(test)]
mod tests {
    use crate::emulator::{initialize_screenless_emulator, Mode};
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
    fn should_calculate_vram_dma_transfer_length() {
        let mut emulator = initialize_screenless_emulator();
        emulator.mode = Mode::CGB;
        set_hdma5(&mut emulator, 0xF1);
        let length = calculate_vram_dma_transfer_length(&emulator);
        assert_eq!(length, 0x720);
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
        emulator.hdma.transfer_length = 0b01010110;
        set_hdma5(&mut emulator, 0x0);
        assert_eq!(emulator.hdma.in_progress, false);
        let hdma5 = get_hdma5(&emulator);
        assert_eq!(hdma5, 0b11010110);
    }
}
