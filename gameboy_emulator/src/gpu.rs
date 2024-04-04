use crate::emulator::Emulator;
use crate::gpu::constants::{GB_SCREEN_HEIGHT, GB_SCREEN_WIDTH};
use crate::gpu::scanline::write_scanline;
use crate::gpu::sprites::{collect_scanline_sprites, Sprite};

pub struct GpuRegisters {
    pub lcdc: u8,
    pub scy: u8,
    pub scx: u8,
    pub wx: u8,
    pub wy: u8,
    pub palette: u8,
    pub ly: u8,
    pub lyc: u8,
    pub stat: u8,
    pub obp0: u8,
    pub obp1: u8
}

pub struct GpuState {
    pub mode: u8,
    pub mode_clock: u16,
    pub registers: GpuRegisters,
    pub frame_buffer: Vec<u32>,
    pub sprite_buffer: Vec<Sprite>
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
        mode_clock: 0,
        registers: GpuRegisters {
            lcdc: 0,
            scy: 0,
            scx: 0,
            wx: 0,
            wy: 0,
            palette: 0,
            ly: 0,
            lyc: 0,
            stat: 0,
            obp0: 0,
            obp1: 0
        },
        frame_buffer: vec![0; (GB_SCREEN_WIDTH * GB_SCREEN_HEIGHT) as usize],
        sprite_buffer: Vec::new()
    }
}

pub fn step(emulator: &mut Emulator) {
    emulator.gpu.mode_clock += emulator.cpu.clock.instruction_clock_cycles as u16;

    match emulator.gpu.mode {
        OAM_MODE => {
            if emulator.gpu.mode_clock >= OAM_TIME {
                emulator.gpu.sprite_buffer = collect_scanline_sprites(emulator);
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
                write_scanline(emulator);
                emulator.gpu.registers.ly += 1;
                emulator.gpu.mode_clock = 0;

                if emulator.gpu.registers.ly == FRAME_SCANLINE_COUNT - VBLANK_SCANLINE_COUNT - 1 {
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
                emulator.gpu.registers.ly += 1;

                if emulator.gpu.registers.ly == FRAME_SCANLINE_COUNT - 1 {
                    emulator.gpu.mode = OAM_MODE;
                    emulator.gpu.registers.ly = 0;
                }
            }
        }
        _ => ()
    }    
}

#[cfg(test)]
mod tests;

mod colors;
mod constants;
mod line_addressing;
mod background;
mod window;
pub mod scanline;
pub mod sprites;
pub mod utils;