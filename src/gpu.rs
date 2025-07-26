use crate::cpu::interrupts::InterruptRegisters;
use crate::address_bus::hdma::HDMAState;
use crate::gpu::palettes::Palettes;
use crate::gpu::constants::{GB_SCREEN_HEIGHT, GB_SCREEN_WIDTH, BYTES_PER_COLOR};
use crate::gpu::utils::{get_lcd_enabled_mode, get_window_enabled_mode};
use crate::serializable::Serializable;
use crate::utils::get_t_cycle_increment;
use crate::utils::is_bit_set;
use serializable_derive::Serializable;
use std::io::{Read, Write};

#[derive(Debug, Serializable)]
pub struct GpuRegisters {
    lcdc: u8,
    scy: u8,
    scx: u8,
    wx: u8,
    wy: u8,
    wly: u8,
    ly: u8,
    lyc: u8,
    stat: u8,
    palettes: Palettes,
    cgb_vbk: u8,
    cgb_opri: u8,
    key0: u8
}

#[derive(Debug)]
pub struct Gpu {
    mode: u8,
    mode_clock: u16,
    registers: GpuRegisters,
    frame_buffer: Vec<u8>,
    video_ram: [u8; 0x4000],
    object_attribute_memory: [u8; 0xa0],
    cgb_mode: bool,
    cgb_double_speed: bool,
}

pub struct GpuParams<'a> {
    pub interrupt_registers: &'a mut InterruptRegisters,
    pub hdma: &'a mut HDMAState,
    pub in_color_bios: bool,
    pub renderer: fn(&[u8]),
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

impl Gpu {
    pub fn new() -> Self {
        Gpu {
            mode: 2,
            mode_clock: 0,
            registers: GpuRegisters {
                lcdc: 0,
                scy: 0,
                scx: 0,
                wx: 0,
                wy: 0,
                wly: 0,
                ly: 0,
                lyc: 0,
                stat: 0,
                palettes: Palettes::new(),
                cgb_vbk: 0,
                cgb_opri: 0,
                key0: 0
            },
            frame_buffer: initialize_blank_frame(),
            video_ram: [0; 0x4000],
            object_attribute_memory: [0; 0xa0],
            cgb_mode: false,
            cgb_double_speed: false,
        }
    }

    pub fn reset_frame_buffer(&mut self) {
        self.frame_buffer = initialize_blank_frame();
    }

    pub fn remove_frame_buffer(&mut self) {
        self.frame_buffer.clear();
    }

    fn fire_vblank_interrupt(&mut self, interrupts: &mut InterruptRegisters) {
        interrupts.flags |= 0x1;
    }

    fn lyc_check_enabled(&self) -> bool {
        is_bit_set(self.registers.stat, STAT_INTERRUPT_LYC_CHECK_BIT)
    }

    fn fire_stat_interrupt(&mut self, interrupts: &mut InterruptRegisters) {
        interrupts.flags |= 0x2;
    }

    fn update_mode(&mut self, new_mode: u8, interrupts: &mut InterruptRegisters) {
        self.mode = new_mode;

        let stat = (self.registers.stat & 0b11111100) | new_mode;
        self.registers.stat = stat;

        let fire_interrupt_on_mode_switch = (new_mode == OAM_MODE && is_bit_set(stat, OAM_MODE_STAT_SOURCE_BIT))
            || (new_mode == VBLANK_MODE && is_bit_set(stat, VBLANK_MODE_STAT_SOURCE_BIT))
            || (new_mode == HBLANK_MODE && is_bit_set(stat, HBLANK_MODE_STAT_SOURCE_BIT));

        if fire_interrupt_on_mode_switch {
            self.fire_stat_interrupt(interrupts);
        }
    }

    fn compare_ly_and_lyc(&mut self, interrupts: &mut InterruptRegisters) {
        if self.registers.ly == self.registers.lyc {
            self.registers.stat |= 0b00000100;
            
            if self.lyc_check_enabled() {
                self.fire_stat_interrupt(interrupts);
            }
        }
        else {
            self.registers.stat &= 0b11111011;
        }
    }

    pub fn step(&mut self, params: GpuParams) {
        let lcdc = self.registers.lcdc;
        let lcd_enabled = get_lcd_enabled_mode(lcdc);

        if lcd_enabled {
            self.mode_clock += get_t_cycle_increment(self.cgb_double_speed) as u16;

            match self.mode {
                OAM_MODE => {
                    if self.mode_clock >= OAM_TIME {
                        self.mode_clock = 0;
                        self.update_mode(VRAM_MODE, params.interrupt_registers);
                    }
                }
                VRAM_MODE => {
                    if self.mode_clock >= VRAM_TIME {
                        self.mode_clock = 0;
                        self.update_mode(HBLANK_MODE, params.interrupt_registers);
                        params.hdma.set_hblank_started(true);
                        if !params.in_color_bios {
                            self.write_scanline();
                        }
                     }
                }
                HBLANK_MODE => {
                    if self.mode_clock >= HBLANK_TIME {
                        let wx = self.registers.wx;
                        let wy = self.registers.wy;
                        let window_enabled = get_window_enabled_mode(lcdc);
                        let window_visible = (wx < 7 || wx - 7 < GB_SCREEN_WIDTH as u8) && wy < GB_SCREEN_HEIGHT as u8;

                        if window_enabled && window_visible && self.registers.ly >= wy {
                            self.registers.wly += 1;
                        }

                        if self.registers.ly == FRAME_SCANLINE_COUNT - VBLANK_SCANLINE_COUNT - 1 {
                            self.update_mode(VBLANK_MODE, params.interrupt_registers);
                            (params.renderer)(&self.frame_buffer);
                            self.fire_vblank_interrupt(params.interrupt_registers);
                        }
                        else {
                            self.update_mode(OAM_MODE, params.interrupt_registers);
                        }

                        self.registers.ly += 1;
                        self.mode_clock = 0;

                        self.compare_ly_and_lyc(params.interrupt_registers);
                    }
                }
                VBLANK_MODE => {
                    if self.mode_clock >= SCANLINE_RENDER_TIME {
                        self.mode_clock = 0;
                        self.registers.ly += 1;

                        if self.registers.ly > FRAME_SCANLINE_COUNT - 1 {
                            self.registers.ly = 0;
                            self.registers.wly = 0;
                            self.update_mode(OAM_MODE, params.interrupt_registers);
                        }

                        self.compare_ly_and_lyc(params.interrupt_registers);
                    }
                }
                _ => ()
            }
        }
    }

    pub fn palettes(&mut self) -> &mut Palettes {
        &mut self.registers.palettes
    }

    pub fn palettes_readonly(&self) -> &Palettes {
        &self.registers.palettes
    }

    fn calculate_video_ram_index(&self, index: u16) -> u16 {
        if self.cgb_mode {
            let bank = self.registers.cgb_vbk & 0b1;
            if bank == 1 { index + 0x2000 } else { index }
        } else {
            index
        }
    }
    pub fn get_video_ram_byte(&self, index: u16) -> u8 {
        let calculated_index = self.calculate_video_ram_index(index);
        self.video_ram[calculated_index as usize]
    }

    pub fn set_video_ram_byte(&mut self, index: u16, value: u8) {
        let calculated_index = self.calculate_video_ram_index(index);
        self.video_ram[calculated_index as usize] = value;
    }

    pub fn get_object_attribute_memory_byte(&self, index: u16) -> u8 {
        self.object_attribute_memory[index as usize]
    }

    pub fn set_object_attribute_memory_byte(&mut self, index: u16, value: u8) {
        self.object_attribute_memory[index as usize] = value;
    }

    pub fn cgb_vbk(&self) -> u8 {
        if self.cgb_mode {
            self.registers.cgb_vbk | 0b11111110
        } else {
            0xFF
        }
    }
    pub fn set_cgb_vbk(&mut self, value: u8) {
        if self.cgb_mode {
            self.registers.cgb_vbk = value;
        }
    }

    pub fn cgb_opri(&self) -> u8 {
        if self.cgb_mode {
            self.registers.cgb_opri & 0b1
        } else {
            0xFF
        }
    }
    pub fn set_cgb_opri(&mut self, value: u8) {
        if self.cgb_mode {
            self.registers.cgb_opri = value & 0b1;
        }
    }

    pub fn lcdc(&self) -> u8 {
        self.registers.lcdc
    }
    
    pub fn set_lcdc(&mut self, value: u8) {
        self.registers.lcdc = value;
        let lcd_enabled = get_lcd_enabled_mode(value);
        if !lcd_enabled {
            self.registers.ly = 0;
            self.registers.wly = 0;
            self.mode_clock = 0;
            self.mode = HBLANK_MODE;
            self.registers.stat = (self.registers.stat & 0b11111100) | HBLANK_MODE;
            self.frame_buffer = initialize_blank_frame();
        }
    }

    pub fn key0(&self) -> u8 {
        self.registers.key0
    }

    pub fn set_key0(&mut self, value: u8) {
        self.registers.key0 = value;
    }
    
    pub fn has_dmg_compatability(&self) -> bool {
        self.registers.key0 == 0x04
    }

    pub fn stat(&self) -> u8 {
        self.registers.stat
    }

    pub fn set_stat(&mut self, value: u8) {
        self.registers.stat = value;
    }

    pub fn scy(&self) -> u8 {
        self.registers.scy
    }

    pub fn set_scy(&mut self, value: u8) {
        self.registers.scy = value;
    }

    pub fn scx(&self) -> u8 {
        self.registers.scx
    }

    pub fn set_scx(&mut self, value: u8) {
        self.registers.scx = value;
    }

    pub fn ly(&self) -> u8 {
        self.registers.ly
    }

    pub fn set_ly(&mut self, value: u8) {
        self.registers.ly = value;
    }

    pub fn lyc(&self) -> u8 {
        self.registers.lyc
    }

    pub fn set_lyc(&mut self, value: u8) {
        self.registers.lyc = value;
    }

    pub fn wy(&self) -> u8 {
        self.registers.wy
    }

    pub fn set_wy(&mut self, value: u8) {
        self.registers.wy = value;
    }

    pub fn wx(&self) -> u8 {
        self.registers.wx
    }

    pub fn set_wx(&mut self, value: u8) {
        self.registers.wx = value;
    }

    pub fn set_mode(&mut self, value: u8) {
        self.mode = value;
    }

    pub fn set_cgb_mode(&mut self, cgb_mode: bool) {
        self.cgb_mode = cgb_mode;
    }

    pub fn set_cgb_double_speed(&mut self, cgb_double_speed: bool) {
        self.cgb_double_speed = cgb_double_speed;
    }
}

impl Serializable for Gpu {
    fn serialize(&self, writer: &mut dyn Write)-> std::io::Result<()> {
        self.mode.serialize(writer)?;
        self.mode_clock.serialize(writer)?;
        self.registers.serialize(writer)?;
        self.video_ram.serialize(writer)?;
        self.object_attribute_memory.serialize(writer)?;
        self.cgb_mode.serialize(writer)?;
        self.cgb_double_speed.serialize(writer)?;
        Ok(())
    }

    fn deserialize(&mut self, reader: &mut dyn Read)-> std::io::Result<()> {
        self.mode.deserialize(reader)?;
        self.mode_clock.deserialize(reader)?;
        self.registers.deserialize(reader)?;
        self.video_ram.deserialize(reader)?;
        self.object_attribute_memory.deserialize(reader)?;
        self.cgb_mode.deserialize(reader)?;
        self.cgb_double_speed.deserialize(reader)?;
    
        self.frame_buffer = initialize_blank_frame();

        Ok(())
    }
}

#[cfg(test)]
mod tests;

mod palettes;
mod constants;
mod line_addressing;
mod background;
mod window;
mod prioritization;
mod scanline;
mod sprites;
mod utils;
