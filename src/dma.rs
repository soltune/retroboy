use crate::dma;
use crate::dma::oam::{initialize_oam_dma, OAMDMAState};
use crate::dma::vram::{initialize_vram_dma, VRAMDMAState};
use crate::emulator::Emulator;

#[derive(Debug)]
pub struct DMAState {
    pub oam: OAMDMAState,
    pub vram: VRAMDMAState
}


pub fn initialize_dma() -> DMAState {
    DMAState {
        oam: initialize_oam_dma(),
        vram: initialize_vram_dma()
    }
}

pub fn step(emulator: &mut Emulator) {
    dma::oam::step(emulator);
    dma::vram::step(emulator);
}

pub mod oam;
pub mod vram;