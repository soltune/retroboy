use crate::emulator::Emulator;
use crate::mmu;

#[derive(Debug)]
pub struct DmaState {
    pub source: u16,
    pub offset: u8,
    pub delay: u8,
    pub in_progress: bool
}

pub const DMA_TRANSFER_BYTES: u8 = 160;

pub fn initialize_dma() -> DmaState {
    DmaState {
        source: 0x0,
        offset: 0x0,
        delay: 0,
        in_progress: false
    }
}

pub fn start_dma(emulator: &mut Emulator, source: u8) {
    emulator.dma.source = (source as u16) << 8;

    if !emulator.dma.in_progress {
        emulator.dma.offset = 0x0;
        emulator.dma.delay = 2;
        emulator.dma.in_progress = true;
    }
}

pub fn get_source(emulator: &Emulator) -> u8 {
    (emulator.dma.source >> 8) as u8
}

fn transfer_byte(emulator: &mut Emulator) {
    let address = emulator.dma.source + (emulator.dma.offset as u16);
    emulator.memory.object_attribute_memory[emulator.dma.offset as usize] = mmu::read_byte(emulator, address);
}

pub fn step(emulator: &mut Emulator) {
    if emulator.dma.in_progress {
        if emulator.dma.delay > 0 {
            emulator.dma.delay -= 1;
        }
        else {
            transfer_byte(emulator);
    
            emulator.dma.offset += 1;
    
            if emulator.dma.offset == DMA_TRANSFER_BYTES {
                emulator.dma.in_progress = false;
            } 
        }
    }
}

#[cfg(test)]
mod tests;
