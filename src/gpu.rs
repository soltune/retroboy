use crate::emulator::Emulator;
use crate::emulator::Mode;
use crate::cpu::hdma;
use crate::gpu::colors::{initialize_palettes, Palettes};
use crate::gpu::constants::{GB_SCREEN_HEIGHT, GB_SCREEN_WIDTH, BYTES_PER_COLOR};
use crate::gpu::scanline::write_scanline;
use crate::gpu::sprites::{collect_scanline_sprites, Sprite};
use crate::gpu::utils::get_lcd_enabled_mode;
use crate::utils::get_t_cycle_increment;
use crate::utils::is_bit_set;

#[derive(Debug)]
pub struct GpuRegisters {
    pub lcdc: u8,
    pub scy: u8,
    pub scx: u8,
    pub wx: u8,
    pub wy: u8,
    pub ly: u8,
    pub lyc: u8,
    pub stat: u8,
    pub palettes: Palettes,
    pub cgb_vbk: u8,
    pub cgb_opri: u8,
    pub key0: u8
}

#[derive(Debug)]
pub struct GpuState {
    pub mode: u8,
    pub mode_clock: u16,
    pub registers: GpuRegisters,
    pub frame_buffer: Vec<u8>,
    pub sprite_buffer: Vec<Sprite>,
    pub video_ram: [u8; 0x4000],
    pub object_attribute_memory: [u8; 0xa0]
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

const STAT_INTERRUPT_LYC_CHECK_BIT: u8 = 6;
const OAM_MODE_STAT_SOURCE_BIT: u8 = 5;
const VBLANK_MODE_STAT_SOURCE_BIT: u8 = 4;
const HBLANK_MODE_STAT_SOURCE_BIT: u8 = 3;

fn initialize_blank_frame() -> Vec<u8> {
    vec![0xFF; (GB_SCREEN_WIDTH * GB_SCREEN_HEIGHT * BYTES_PER_COLOR) as usize]
}

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
            ly: 0,
            lyc: 0,
            stat: 0,
            palettes: initialize_palettes(),
            cgb_vbk: 0,
            cgb_opri: 0,
            key0: 0
        },
        frame_buffer: initialize_blank_frame(),
        sprite_buffer: Vec::new(),
        video_ram: [0; 0x4000],
        object_attribute_memory: [0; 0xa0]
    }
}

fn fire_vblank_interrupt(emulator: &mut Emulator) {
    emulator.interrupts.flags |= 0x1;
}

fn lyc_check_enabled(emulator: &Emulator) -> bool {
    is_bit_set(emulator.gpu.registers.stat, STAT_INTERRUPT_LYC_CHECK_BIT) 
}

fn fire_stat_interrupt(emulator: &mut Emulator) {
    emulator.interrupts.flags |= 0x2;
}

fn update_mode(emulator: &mut Emulator, new_mode: u8) {
    emulator.gpu.mode = new_mode;

    let stat = (emulator.gpu.registers.stat & 0b11111100) | new_mode;
    emulator.gpu.registers.stat = stat;

    let fire_interrupt_on_mode_switch = (new_mode == OAM_MODE && is_bit_set(stat, OAM_MODE_STAT_SOURCE_BIT))
        || (new_mode == VBLANK_MODE && is_bit_set(stat, VBLANK_MODE_STAT_SOURCE_BIT))
        || (new_mode == HBLANK_MODE && is_bit_set(stat, HBLANK_MODE_STAT_SOURCE_BIT));

    if fire_interrupt_on_mode_switch {
        fire_stat_interrupt(emulator);
    }
}

fn compare_ly_and_lyc(emulator: &mut Emulator) {
    if emulator.gpu.registers.ly == emulator.gpu.registers.lyc {
        emulator.gpu.registers.stat = emulator.gpu.registers.stat | 0b00000100;
        
        if lyc_check_enabled(emulator) {
            fire_stat_interrupt(emulator);
        }
    }
    else {
        emulator.gpu.registers.stat = emulator.gpu.registers.stat & 0b11111011;
    }
}

pub fn step(emulator: &mut Emulator) {
    let lcdc = emulator.gpu.registers.lcdc;
    let lcd_enabled = get_lcd_enabled_mode(lcdc);

    if lcd_enabled {
        let double_speed_mode = emulator.speed_switch.cgb_double_speed;
        emulator.gpu.mode_clock += get_t_cycle_increment(double_speed_mode) as u16;
    
        match emulator.gpu.mode {
            OAM_MODE => {
                if emulator.gpu.mode_clock >= OAM_TIME {
                    emulator.gpu.sprite_buffer = collect_scanline_sprites(emulator);
                    emulator.gpu.mode_clock = 0;
                    update_mode(emulator, VRAM_MODE);
                }
            }
            VRAM_MODE => {
                if emulator.gpu.mode_clock >= VRAM_TIME {
                    emulator.gpu.mode_clock = 0;
                    update_mode(emulator, HBLANK_MODE);
                    hdma::set_hblank_started(emulator, true);
                    write_scanline(emulator);
                }
            }
            HBLANK_MODE => {
                if emulator.gpu.mode_clock >= HBLANK_TIME {
                    if emulator.gpu.registers.ly == FRAME_SCANLINE_COUNT - VBLANK_SCANLINE_COUNT - 1 {
                        update_mode(emulator, VBLANK_MODE);
                        (emulator.render)(&emulator.gpu.frame_buffer);
                        fire_vblank_interrupt(emulator);
                    }
                    else {
                        update_mode(emulator, OAM_MODE);
                    }
    
                    emulator.gpu.registers.ly += 1;
                    emulator.gpu.mode_clock = 0;
    
                    compare_ly_and_lyc(emulator);
                }
            }
            VBLANK_MODE => {
                if emulator.gpu.mode_clock >= SCANLINE_RENDER_TIME {
                    emulator.gpu.mode_clock = 0;
                    emulator.gpu.registers.ly += 1;
    
                    if emulator.gpu.registers.ly > FRAME_SCANLINE_COUNT - 1 {
                        emulator.gpu.registers.ly = 0;
                        update_mode(emulator, OAM_MODE);
                    }
    
                    compare_ly_and_lyc(emulator);
                }
            }
            _ => ()
        }   
    }
}

pub fn get_cgb_bcpd(emulator: &Emulator) -> u8 {
    if emulator.mode == Mode::CGB {
        colors::get_cgb_bcpd(&emulator.gpu.registers.palettes)
    }
    else {
        0xFF
    }
}

pub fn set_cgb_bcpd(emulator: &mut Emulator, value: u8) {
    if emulator.mode == Mode::CGB {
        colors::set_cgb_bcpd(&mut emulator.gpu.registers.palettes, value);
    }
}

pub fn get_cgb_bcps(emulator: &Emulator) -> u8 {
    if emulator.mode == Mode::CGB {
        colors::get_cgb_bcps(&emulator.gpu.registers.palettes)
    }
    else {
        0xFF
    }
}

pub fn set_cgb_bcps(emulator: &mut Emulator, value: u8) {
    if emulator.mode == Mode::CGB {
        colors::set_cgb_bcps(&mut emulator.gpu.registers.palettes, value);
    }
}

pub fn get_cgb_ocpd(emulator: &Emulator) -> u8 {
    if emulator.mode == Mode::CGB {
        colors::get_cgb_ocpd(&emulator.gpu.registers.palettes)
    }
    else {
        0xFF
    }
}

pub fn set_cgb_ocpd(emulator: &mut Emulator, value: u8) {
    if emulator.mode == Mode::CGB {
        colors::set_cgb_ocpd(&mut emulator.gpu.registers.palettes, value);
    }
}

pub fn get_cgb_ocps(emulator: &Emulator) -> u8 {
    if emulator.mode == Mode::CGB {
        colors::get_cgb_ocps(&emulator.gpu.registers.palettes)
    }
    else {
        0xFF
    }
}

pub fn set_cgb_ocps(emulator: &mut Emulator, value: u8) {
    if emulator.mode == Mode::CGB {
        colors::set_cgb_ocps(&mut emulator.gpu.registers.palettes, value);
    }
}

fn calculate_video_ram_index(emulator: &Emulator, index: u16) -> u16 {
    if emulator.mode == Mode::CGB {
        let bank = emulator.gpu.registers.cgb_vbk & 0b1;
        if bank == 1 { index + 0x2000 } else { index }
    }
    else {
        index
    }
}

pub fn get_video_ram_byte(emulator: &Emulator, index: u16) -> u8 {
    let calculated_index = calculate_video_ram_index(emulator, index);
    emulator.gpu.video_ram[calculated_index as usize]
}

pub fn set_video_ram_byte(emulator: &mut Emulator, index: u16, value: u8) {
    let calculated_index = calculate_video_ram_index(emulator, index);
    emulator.gpu.video_ram[calculated_index as usize] = value;
}

pub fn get_object_attribute_memory_byte(emulator: &Emulator, index: u16) -> u8 {
    emulator.gpu.object_attribute_memory[index as usize]
}

pub fn set_object_attribute_memory_byte(emulator: &mut Emulator, index: u16, value: u8) {
    emulator.gpu.object_attribute_memory[index as usize] = value;
}

pub fn get_cgb_vbk(emulator: &Emulator) -> u8 {
    if emulator.mode == Mode::CGB {
        emulator.gpu.registers.cgb_vbk | 0b11111110
    }
    else {
        0xFF
    }
}

pub fn set_cgb_vbk(emulator: &mut Emulator, value: u8) {
    if emulator.mode == Mode::CGB {
        emulator.gpu.registers.cgb_vbk = value;
    }
}

pub fn get_cgb_opri(emulator: &Emulator) -> u8 {
    if emulator.mode == Mode::CGB {
        emulator.gpu.registers.cgb_opri & 0b1
    }
    else {
        0xFF
    }
}

pub fn set_cgb_opri(emulator: &mut Emulator, value: u8) {
    if emulator.mode == Mode::CGB {
        emulator.gpu.registers.cgb_opri = value & 0b1;
    }
}

pub fn get_lcdc(emulator: &Emulator) -> u8 {
    emulator.gpu.registers.lcdc
}

pub fn set_lcdc(emulator: &mut Emulator, value: u8) {
    emulator.gpu.registers.lcdc = value;
    let lcd_enabled = get_lcd_enabled_mode(emulator.gpu.registers.lcdc);
    if !lcd_enabled {
        emulator.gpu.registers.ly = 0;
        emulator.gpu.mode_clock = 0;
        emulator.gpu.registers.stat = 0;
        emulator.gpu.mode = HBLANK_MODE;
        emulator.gpu.frame_buffer = initialize_blank_frame();
        emulator.gpu.sprite_buffer = Vec::new();
    }
}

pub fn set_key0(emulator: &mut Emulator, value: u8) {
    emulator.gpu.registers.key0 = value;
}

pub fn get_key0(emulator: &Emulator) -> u8 {
    emulator.gpu.registers.key0
}

pub fn has_dmg_compatability(emulator: &Emulator) -> bool {
    emulator.gpu.registers.key0 == 0x04
}

#[cfg(test)]
mod tests;

mod colors;
mod constants;
mod line_addressing;
mod background;
mod window;
mod prioritization;
pub mod scanline;
pub mod sprites;
pub mod utils;