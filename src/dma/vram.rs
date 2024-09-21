use crate::emulator::Emulator;
use crate::utils::is_bit_set;

#[derive(Debug, PartialEq, Eq)]
pub enum VRAMTransferMode {
    GeneralPurpose,
    HBlank
}

#[derive(Debug)]
pub struct VRAMDMAState {
    pub hdma1: u8,
    pub hdma2: u8,
    pub hdma3: u8,
    pub hdma4: u8,
    pub hdma5: u8
}

pub fn initialize_vram_dma() -> VRAMDMAState {
    VRAMDMAState {
        hdma1: 0x0,
        hdma2: 0x0,
        hdma3: 0x0,
        hdma4: 0x0,
        hdma5: 0x0
    }
}

pub fn set_hdma1(emulator: &mut Emulator, value: u8) {
    emulator.dma.vram.hdma1 = value;
}

pub fn set_hdma2(emulator: &mut Emulator, value: u8) {
    emulator.dma.vram.hdma2 = value;
}

pub fn get_vram_dma_source(emulator: &Emulator) -> u16 {
    ((emulator.dma.vram.hdma1 as u16) << 8) | ((emulator.dma.vram.hdma2 as u16) & 0b11110000)
}

pub fn set_hdma3(emulator: &mut Emulator, value: u8) {
    emulator.dma.vram.hdma3 = value;
}

pub fn set_hdma4(emulator: &mut Emulator, value: u8) {
    emulator.dma.vram.hdma4 = value;
}

pub fn set_hdma5(emulator: &mut Emulator, value: u8) {
    emulator.dma.vram.hdma5 = value;
}

pub fn get_vram_dma_destination(emulator: &Emulator) -> u16 {
    let offset = (((emulator.dma.vram.hdma3 as u16) & 0b00011111) << 8) |
        ((emulator.dma.vram.hdma4 as u16) & 0b11110000);

    0x8000 + offset
}

const VRAM_TRANSFER_INDEX: u8 = 7;

pub fn get_vram_dma_transfer_mode(emulator: &Emulator) -> VRAMTransferMode {
    let transfer_bit_set = is_bit_set(emulator.dma.vram.hdma5, VRAM_TRANSFER_INDEX);
    if transfer_bit_set { VRAMTransferMode::HBlank } else { VRAMTransferMode::GeneralPurpose }
}

pub fn calculate_vram_dma_transfer_length(emulator: &Emulator) -> u16 {
    let length = (emulator.dma.vram.hdma5 & 0b01111111) as u16;
    (length + 1) * 0x10
}

pub fn step(emulator: &mut Emulator) {
    // TODO: Step logic here
}

#[cfg(test)]
mod tests {
    use crate::emulator::initialize_screenless_emulator;
    use super::*;

    #[test]
    fn should_calculate_vram_dma_source() {
        let mut emulator = initialize_screenless_emulator();
        set_hdma1(&mut emulator, 0x71);
        set_hdma2(&mut emulator, 0xA2);
        let vram_source = get_vram_dma_source(&emulator);
        assert_eq!(vram_source, 0x71A0);
    }

    #[test]
    fn should_calculate_vram_dma_destination() {
        let mut emulator = initialize_screenless_emulator();
        set_hdma3(&mut emulator, 0x71);
        set_hdma4(&mut emulator, 0xA2);
        let vram_destination = get_vram_dma_destination(&emulator);
        assert_eq!(vram_destination, 0x91A0);
    }

    #[test]
    fn should_get_general_purpose_for_vram_transfer_mode() {
        let mut emulator = initialize_screenless_emulator();
        set_hdma5(&mut emulator, 0x71);
        let mode = get_vram_dma_transfer_mode(&emulator);
        assert_eq!(mode, VRAMTransferMode::GeneralPurpose);
    }

    #[test]
    fn should_get_hblank_for_vram_transfer_mode() {
        let mut emulator = initialize_screenless_emulator();
        set_hdma5(&mut emulator, 0xF1);
        let mode = get_vram_dma_transfer_mode(&emulator);
        assert_eq!(mode, VRAMTransferMode::HBlank);
    }

    #[test]
    fn should_calculate_vram_dma_transfer_length() {
        let mut emulator = initialize_screenless_emulator();
        set_hdma5(&mut emulator, 0xF1);
        let length = calculate_vram_dma_transfer_length(&emulator);
        assert_eq!(length, 0x720);
    }
}
