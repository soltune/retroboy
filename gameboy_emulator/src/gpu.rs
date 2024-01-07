use crate::mmu::Memory;

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

pub fn step(gpu_state: &mut GpuState, memory: &mut Memory, opcode_clock_cycles: u8) {
    gpu_state.mode_clock += opcode_clock_cycles as u16;

    match gpu_state.mode {
        OAM_MODE => {
            if gpu_state.mode_clock >= OAM_TIME {
                gpu_state.mode = VRAM_MODE;
                gpu_state.mode_clock = 0;
            }
        }
        VRAM_MODE => {
            if gpu_state.mode_clock >= VRAM_TIME {
                gpu_state.mode = HBLANK_MODE;
                gpu_state.mode_clock = 0;
            }
        }
        HBLANK_MODE => {
            if gpu_state.mode_clock >= HBLANK_TIME {
                gpu_state.line += 1;
                gpu_state.mode_clock = 0;

                if gpu_state.line == FRAME_SCANLINE_COUNT - VBLANK_SCANLINE_COUNT - 1 {
                    gpu_state.mode = VBLANK_MODE;
                }
                else {
                    gpu_state.mode = OAM_MODE;
                }
            }
        }
        VBLANK_MODE => {
            if gpu_state.mode_clock >= SCANLINE_RENDER_TIME {
                gpu_state.mode_clock = 0;
                gpu_state.line += 1;

                if gpu_state.line == FRAME_SCANLINE_COUNT - 1 {
                    gpu_state.mode = OAM_MODE;
                    gpu_state.line = 0;
                }
            }
        }
        _ => ()
    }    
}

#[cfg(test)]
mod tests;
