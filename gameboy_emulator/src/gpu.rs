use crate::emulator::Emulator;

pub struct GpuState {
    mode: u8,
    line: u8,
    mode_clock: u16
}

const OAM_MODE: u8 = 2;
const OAM_TIME: u16 = 80;

const VRAM_MODE: u8 = 3;
const VRAM_TIME: u16 = 172;

const HBLANK_MODE: u8 = 0;
const HBLANK_TIME: u16 = 204;

const VBLANK_MODE: u8 = 1;

const SCANLINE_RENDER_TIME: u16 = 456;

const FRAME_SCANLINE_COUNT: u8 = 154;
const VBLANK_SCANLINE_COUNT: u8 = 10;

pub fn initialize_gpu() -> GpuState {
    GpuState {
        mode: 2,
        line: 0,
        mode_clock: 0
    }
}

pub fn step(emulator: &mut Emulator) {
    emulator.gpu.mode_clock += emulator.cpu.clock.instruction_clock_cycles as u16;

    match emulator.gpu.mode {
        OAM_MODE => {
            if emulator.gpu.mode_clock >= OAM_TIME {
                emulator.gpu.mode = VRAM_MODE;
                emulator.gpu.mode_clock = 0;
            }
        }
        VRAM_MODE => {
            if emulator.gpu.mode_clock >= VRAM_TIME {
                emulator.gpu.mode = HBLANK_MODE;
                emulator.gpu.mode_clock = 0;
            }
        }
        HBLANK_MODE => {
            if emulator.gpu.mode_clock >= HBLANK_TIME {
                emulator.gpu.line += 1;
                emulator.gpu.mode_clock = 0;

                if emulator.gpu.line == FRAME_SCANLINE_COUNT - VBLANK_SCANLINE_COUNT - 1 {
                    emulator.gpu.mode = VBLANK_MODE;
                }
                else {
                    emulator.gpu.mode = OAM_MODE;
                }
            }
        }
        VBLANK_MODE => {
            if emulator.gpu.mode_clock >= SCANLINE_RENDER_TIME {
                emulator.gpu.mode_clock = 0;
                emulator.gpu.line += 1;

                if emulator.gpu.line == FRAME_SCANLINE_COUNT - 1 {
                    emulator.gpu.mode = OAM_MODE;
                    emulator.gpu.line = 0;
                }
            }
        }
        _ => ()
    }    
}

#[cfg(test)]
mod tests;
