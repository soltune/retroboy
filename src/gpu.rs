use crate::cpu::interrupts::InterruptRegisters;
use crate::address_bus::hdma::HDMAState;
use crate::gpu::palettes::Palettes;
use crate::gpu::constants::{GB_SCREEN_HEIGHT, GB_SCREEN_WIDTH, BYTES_PER_COLOR};
use crate::gpu::utils::{get_lcd_enabled_mode, get_window_enabled_mode};
use crate::serializable::Serializable;
use crate::utils::get_t_cycle_increment;
use crate::utils::is_bit_set;
use getset::{CopyGetters, Getters, MutGetters, Setters};
use std::io::{Read, Write};

#[derive(Debug, CopyGetters, Getters, MutGetters, Setters)]
pub struct Gpu {
    mode: u8,
    mode_clock: u16,
    lcdc: u8,
    #[getset(get_copy = "pub", set = "pub")]
    scy: u8,
    #[getset(get_copy = "pub", set = "pub")]
    scx: u8,
    #[getset(get_copy = "pub", set = "pub")]
    wx: u8,
    #[getset(get_copy = "pub", set = "pub")]
    wy: u8,
    #[getset(get_copy = "pub", set = "pub")]
    wly: u8,
    #[getset(get_copy = "pub", set = "pub")]
    ly: u8,
    #[getset(get_copy = "pub", set = "pub")]
    lyc: u8,
    #[getset(get_copy = "pub", set = "pub")]
    stat: u8,
    #[getset(get = "pub", get_mut = "pub")]
    palettes: Palettes,
    cgb_vbk: u8,
    cgb_opri: u8,
    #[getset(get_copy = "pub", set = "pub")]
    key0: u8,
    frame_buffer: Vec<u8>,
    video_ram: [u8; 0x4000],
    object_attribute_memory: [u8; 0xa0],
    #[getset(set = "pub")]
    cgb_mode: bool,
    #[getset(set = "pub")]
    cgb_double_speed: bool,
    renderer: fn(&[u8])
}

pub struct GpuParams<'a> {
    pub interrupt_registers: &'a mut InterruptRegisters,
    pub hdma: &'a mut HDMAState,
    pub in_color_bios: bool,
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
    pub fn new(renderer: fn(&[u8])) -> Self {
        Gpu {
            mode: 2,
            mode_clock: 0,
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
            key0: 0,
            frame_buffer: initialize_blank_frame(),
            video_ram: [0; 0x4000],
            object_attribute_memory: [0; 0xa0],
            cgb_mode: false,
            cgb_double_speed: false,
            renderer
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
        is_bit_set(self.stat, STAT_INTERRUPT_LYC_CHECK_BIT)
    }

    fn fire_stat_interrupt(&mut self, interrupts: &mut InterruptRegisters) {
        interrupts.flags |= 0x2;
    }

    fn update_mode(&mut self, new_mode: u8, interrupts: &mut InterruptRegisters) {
        self.mode = new_mode;

        let stat = (self.stat & 0b11111100) | new_mode;
        self.stat = stat;

        let fire_interrupt_on_mode_switch = (new_mode == OAM_MODE && is_bit_set(stat, OAM_MODE_STAT_SOURCE_BIT))
            || (new_mode == VBLANK_MODE && is_bit_set(stat, VBLANK_MODE_STAT_SOURCE_BIT))
            || (new_mode == HBLANK_MODE && is_bit_set(stat, HBLANK_MODE_STAT_SOURCE_BIT));

        if fire_interrupt_on_mode_switch {
            self.fire_stat_interrupt(interrupts);
        }
    }

    fn compare_ly_and_lyc(&mut self, interrupts: &mut InterruptRegisters) {
        if self.ly == self.lyc {
            self.stat |= 0b00000100;
            
            if self.lyc_check_enabled() {
                self.fire_stat_interrupt(interrupts);
            }
        }
        else {
            self.stat &= 0b11111011;
        }
    }

    pub fn step(&mut self, params: GpuParams) {
        let lcdc = self.lcdc;
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
                        let wx = self.wx;
                        let wy = self.wy;
                        let window_enabled = get_window_enabled_mode(lcdc);
                        let window_visible = (wx < 7 || wx - 7 < GB_SCREEN_WIDTH as u8) && wy < GB_SCREEN_HEIGHT as u8;

                        if window_enabled && window_visible && self.ly >= wy {
                            self.wly += 1;
                        }

                        if self.ly == FRAME_SCANLINE_COUNT - VBLANK_SCANLINE_COUNT - 1 {
                            self.update_mode(VBLANK_MODE, params.interrupt_registers);
                            (self.renderer)(&self.frame_buffer);
                            self.fire_vblank_interrupt(params.interrupt_registers);
                        }
                        else {
                            self.update_mode(OAM_MODE, params.interrupt_registers);
                        }

                        self.ly += 1;
                        self.mode_clock = 0;

                        self.compare_ly_and_lyc(params.interrupt_registers);
                    }
                }
                VBLANK_MODE => {
                    if self.mode_clock >= SCANLINE_RENDER_TIME {
                        self.mode_clock = 0;
                        self.ly += 1;

                        if self.ly > FRAME_SCANLINE_COUNT - 1 {
                            self.ly = 0;
                            self.wly = 0;
                            self.update_mode(OAM_MODE, params.interrupt_registers);
                        }

                        self.compare_ly_and_lyc(params.interrupt_registers);
                    }
                }
                _ => ()
            }
        }
    }

    fn calculate_video_ram_index(&self, index: u16) -> u16 {
        if self.cgb_mode {
            let bank = self.cgb_vbk & 0b1;
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
            self.cgb_vbk | 0b11111110
        } else {
            0xFF
        }
    }
    pub fn set_cgb_vbk(&mut self, value: u8) {
        if self.cgb_mode {
            self.cgb_vbk = value;
        }
    }

    pub fn cgb_opri(&self) -> u8 {
        if self.cgb_mode {
            self.cgb_opri & 0b1
        } else {
            0xFF
        }
    }
    pub fn set_cgb_opri(&mut self, value: u8) {
        if self.cgb_mode {
            self.cgb_opri = value & 0b1;
        }
    }

    pub fn lcdc(&self) -> u8 {
        self.lcdc
    }
    
    pub fn set_lcdc(&mut self, value: u8) {
        self.lcdc = value;
        let lcd_enabled = get_lcd_enabled_mode(value);
        if !lcd_enabled {
            self.ly = 0;
            self.wly = 0;
            self.mode_clock = 0;
            self.mode = HBLANK_MODE;
            self.stat = (self.stat & 0b11111100) | HBLANK_MODE;
            self.frame_buffer = initialize_blank_frame();
        }
    }
    
    pub fn has_dmg_compatability(&self) -> bool {
        self.key0 == 0x04
    }

    pub fn set_mode(&mut self, value: u8) {
        self.mode = value;
    }
}

impl Serializable for Gpu {
    fn serialize(&self, writer: &mut dyn Write)-> std::io::Result<()> {
        self.mode.serialize(writer)?;
        self.mode_clock.serialize(writer)?;
        self.lcdc.serialize(writer)?;
        self.scy.serialize(writer)?;
        self.scx.serialize(writer)?;
        self.wx.serialize(writer)?;
        self.wy.serialize(writer)?;
        self.wly.serialize(writer)?;
        self.ly.serialize(writer)?;
        self.lyc.serialize(writer)?;
        self.stat.serialize(writer)?;
        self.palettes.serialize(writer)?;
        self.cgb_vbk.serialize(writer)?;
        self.cgb_opri.serialize(writer)?;
        self.video_ram.serialize(writer)?;
        self.object_attribute_memory.serialize(writer)?;
        self.cgb_mode.serialize(writer)?;
        self.cgb_double_speed.serialize(writer)?;
        Ok(())
    }

    fn deserialize(&mut self, reader: &mut dyn Read)-> std::io::Result<()> {
        self.mode.deserialize(reader)?;
        self.mode_clock.deserialize(reader)?;
        self.lcdc.deserialize(reader)?;
        self.scy.deserialize(reader)?;
        self.scx.deserialize(reader)?;
        self.wx.deserialize(reader)?;
        self.wy.deserialize(reader)?;
        self.wly.deserialize(reader)?;
        self.ly.deserialize(reader)?;
        self.lyc.deserialize(reader)?;
        self.stat.deserialize(reader)?;
        self.palettes.deserialize(reader)?;
        self.cgb_vbk.deserialize(reader)?;
        self.cgb_opri.deserialize(reader)?;
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
