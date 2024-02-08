use crate::{emulator::Emulator, utils::is_bit_set};

pub struct GpuRegisters {
    pub lcdc: u8,
    pub scy: u8,
    pub scx: u8,
    pub wx: u8,
    pub wy: u8,
    pub palette: u8,
    pub ly: u8,
    pub lyc: u8,
    pub stat: u8
}

pub struct GpuState {
    pub mode: u8,
    pub mode_clock: u16,
    pub registers: GpuRegisters
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

const LCDC_BG_AND_WINDOW_ENABLED_INDEX: u8 = 0;
const LCDC_OBJ_ENABLED_INDEX: u8 = 1;
const LCDC_OBJ_SIZE_INDEX: u8 = 2;
const LCDC_BG_TILE_MAP_INDEX: u8 = 3;
const LCDC_TILE_DATA_INDEX: u8 = 4;
const LCDC_WINDOW_ENABLED_INDEX: u8 = 5;
const LCDC_WINDOW_TILE_MAP_INDEX: u8 = 6;
const LCDC_ENABLED_INDEX: u8 = 7;

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
            stat: 0
        }
    }
}

pub fn get_bg_and_window_enabled_mode(lcdc: u8) -> bool {
    is_bit_set(lcdc, LCDC_BG_AND_WINDOW_ENABLED_INDEX)
}

pub fn get_obj_enabled_mode(lcdc: u8) -> bool {
    is_bit_set(lcdc, LCDC_OBJ_ENABLED_INDEX) 
}

pub fn get_obj_size_mode(lcdc: u8) -> bool {
    is_bit_set(lcdc, LCDC_OBJ_SIZE_INDEX)
}

pub fn get_bg_tile_map_mode(lcdc: u8) -> bool {
    is_bit_set(lcdc, LCDC_BG_TILE_MAP_INDEX)
}

pub fn get_tile_data_addressing_mode(lcdc: u8) -> bool {
    is_bit_set(lcdc, LCDC_TILE_DATA_INDEX)
}

pub fn get_window_enabled_mode(lcdc: u8) -> bool {
    is_bit_set(lcdc, LCDC_WINDOW_ENABLED_INDEX)
}

pub fn get_window_tile_map_mode(lcdc: u8) -> bool {
    is_bit_set(lcdc, LCDC_WINDOW_TILE_MAP_INDEX)
}

pub fn get_lcd_enabled_mode(lcdc: u8) -> bool {
    is_bit_set(lcdc, LCDC_ENABLED_INDEX)
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
