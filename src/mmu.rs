use crate::bios::{CGB_BOOT, DMG_BOOTIX};
use crate::{apu, dma, gpu};
use crate::cpu::hdma;
use crate::emulator::{is_cgb, Emulator};
pub use crate::mmu::cartridge::CartridgeHeader;
use crate::mmu::cartridge::{Cartridge, initialize_cartridge};
use crate::speed_switch;
use crate::keys;
use std::io;

pub struct Memory {
    pub in_bios: bool,
    pub bios: Vec<u8>,
    pub working_ram: [u8; 0x10000],
    pub zero_page_ram: [u8; 0x80],
    pub svbk: u8,
    pub cartridge: Cartridge,
    pub processor_test_ram: [u8; 0xFFFF]
}

pub fn initialize_memory() -> Memory {
    Memory {
        in_bios: true,
        bios: [0; 0x100].to_vec(),
        working_ram: [0; 0x10000],
        zero_page_ram: [0; 0x80],
        svbk: 0,
        cartridge: initialize_cartridge(),
        processor_test_ram: [0; 0xFFFF]
    }
}

pub fn load_bios(emulator: &mut Emulator) {
    emulator.memory.bios = if is_cgb(emulator) {
        CGB_BOOT.to_vec()
    }
    else {
        DMG_BOOTIX.to_vec()
    }
}   

fn address_accessible(emulator: &Emulator, address: u16) -> bool {
    let accessing_oam = address >= 0xFE00 && address < 0xFEA0;
    (emulator.dma.in_progress && !accessing_oam) || !emulator.dma.in_progress
}

fn calculate_working_ram_index(emulator: &Emulator, address: u16) -> usize {
    let localized_index = address & 0x1FFF;
    if localized_index <= 0xFFF {
        localized_index as usize
    }
    else {
        let masked_value = emulator.memory.svbk & 0b111;
        let bank_number = if is_cgb(emulator) {
            if masked_value == 0 { 1 } else { masked_value }
        }
        else {
            1
        };
        let index = (bank_number as u16 * 0x1000) + (address & 0x0FFF);
        index as usize 
    }
}

pub fn read_byte(emulator: &mut Emulator, address: u16) -> u8 {
    if emulator.processor_test_mode {
        emulator.memory.processor_test_ram[address as usize]
    }
    else {
        if address_accessible(emulator, address) {
            match address & 0xF000 {
                0x0000 if address <= 0x00FE && emulator.memory.in_bios => {
                    if address == 0x00FE {
                        emulator.memory.in_bios = false;
                    }
                    emulator.memory.bios[address as usize]
                },
                0x0000 if address >= 0x0200 && address <= 0x08FF && is_cgb(emulator) && emulator.memory.in_bios => {
                    emulator.memory.bios[address as usize]
                },
                0x0000..=0x7FFF =>
                    cartridge::read_rom(&emulator.memory.cartridge, address),
                0x8000..=0x9FFF =>
                    gpu::get_video_ram_byte(emulator, address & 0x1FFF),
                0xA000..=0xBFFF =>
                    cartridge::read_ram(&emulator.memory.cartridge, address & 0x1FFF),
                0xC000..=0xEFFF => {
                    let index = calculate_working_ram_index(emulator, address);
                    emulator.memory.working_ram[index]
                }
                0xF000 => match address & 0x0F00 {
                    0x000..=0xD00 => {
                        let index = calculate_working_ram_index(emulator, address);
                        emulator.memory.working_ram[index]
                    },
                    0xE00 if address < 0xFEA0 => gpu::get_object_attribute_memory_byte(emulator, address & 0xFF),
                    0xF00 if address == 0xFFFF => emulator.interrupts.enabled,
                    0xF00 if address >= 0xFF80 => emulator.memory.zero_page_ram[(address & 0x7F) as usize],
                    _ => match address & 0xFF {
                        0x00 => keys::read_joyp_byte(&emulator.keys),
                        0x10 => emulator.apu.channel1.sweep.initial_settings | 0b10000000,
                        0x11 => emulator.apu.channel1.length.initial_settings | 0b00111111,
                        0x12 => emulator.apu.channel1.envelope.initial_settings,
                        0x14 => emulator.apu.channel1.period.high | 0b10111111,
                        0x16 => emulator.apu.channel2.length.initial_settings | 0b00111111,
                        0x17 => emulator.apu.channel2.envelope.initial_settings,
                        0x19 => emulator.apu.channel2.period.high | 0b10111111,
                        0x1A => if emulator.apu.channel3.dac_enabled { 0b11111111 } else { 0b01111111 },
                        0x1C => emulator.apu.channel3.volume | 0b10011111,
                        0x1E => emulator.apu.channel3.period.high | 0b10111111,
                        0x21 => emulator.apu.channel4.envelope.initial_settings,
                        0x22 => emulator.apu.channel4.polynomial,
                        0x23 => emulator.apu.channel4.control | 0b10111111,
                        0x24 => emulator.apu.master_volume,
                        0x25 => emulator.apu.sound_panning,
                        0x26 => apu::get_audio_master_control(&emulator),
                        0x30..=0x3F => apu::get_wave_ram_byte(&emulator, (address & 0xF) as u8),
                        0x40 => gpu::get_lcdc(emulator),
                        0x41 => emulator.gpu.registers.stat,
                        0x42 => emulator.gpu.registers.scy,
                        0x43 => emulator.gpu.registers.scx,
                        0x44 => emulator.gpu.registers.ly,
                        0x45 => emulator.gpu.registers.lyc,
                        0x46 => dma::get_source(emulator),
                        0x47 => emulator.gpu.registers.palettes.bgp,
                        0x48 => emulator.gpu.registers.palettes.obp0,
                        0x49 => emulator.gpu.registers.palettes.obp1,
                        0x4A => emulator.gpu.registers.wy,
                        0x4B => emulator.gpu.registers.wx,
                        0x4C => gpu::get_key0(emulator),
                        0x4D => speed_switch::get_key1(emulator),
                        0x4F => gpu::get_cgb_vbk(emulator),
                        0x55 => hdma::get_hdma5(emulator),
                        0x68 => gpu::get_cgb_bcps(emulator),
                        0x69 => gpu::get_cgb_bcpd(emulator),
                        0x6A => gpu::get_cgb_ocps(emulator),
                        0x6B => gpu::get_cgb_ocpd(emulator),
                        0x6C => gpu::get_cgb_opri(emulator),
                        0x70 => if is_cgb(emulator) { emulator.memory.svbk } else { 0xFF },
                        0x0F => emulator.interrupts.flags,
                        0x04 => emulator.timers.divider,
                        0x05 => emulator.timers.counter,
                        0x06 => emulator.timers.modulo,
                        0x07 => emulator.timers.control,
                        _ => 0xFF
                    }
                },
                _ => 0x00,
            }
        }
        else {
            0xFF
        }
    }
}

pub fn write_byte(emulator: &mut Emulator, address: u16, value: u8) {
    if emulator.processor_test_mode {
        emulator.memory.processor_test_ram[address as usize] = value;
    }
    else {
        if address_accessible(emulator, address) {
            match address & 0xF000 {
                0x0000..=0x7FFF =>
                    cartridge::write_rom(&mut emulator.memory.cartridge, address, value),
                0x8000..=0x9FFF =>
                    gpu::set_video_ram_byte(emulator, address & 0x1FFF, value),
                0xA000..=0xBFFF =>
                    cartridge::write_ram(&mut emulator.memory.cartridge, address & 0x1FFF, value),
                0xC000..=0xEFFF => {
                    let index = calculate_working_ram_index(emulator, address);
                    emulator.memory.working_ram[index] = value;
                },
                0xF000 => match address & 0x0F00 {
                    0x000..=0xD00 => {
                        let index = calculate_working_ram_index(emulator, address);
                        emulator.memory.working_ram[index] = value;
                    },
                    0xE00 if address < 0xFEA0 => gpu::set_object_attribute_memory_byte(emulator, address & 0xFF, value),
                    0xF00 if address == 0xFFFF => emulator.interrupts.enabled = value,
                    0xF00 if address >= 0xFF80 => emulator.memory.zero_page_ram[(address & 0x7F) as usize] = value,
                    _ => match address & 0xFF {
                        0x00 => keys::write_joyp_byte(&mut emulator.keys, value),
                        0x10 => apu::set_ch1_sweep_settings(emulator, value),
                        0x11 => apu::set_ch1_length_settings(emulator, value),
                        0x12 => apu::set_ch1_envelope_settings(emulator, value),
                        0x13 => apu::set_ch1_period_low(emulator, value),
                        0x14 => apu::set_ch1_period_high(emulator, value),
                        0x16 => apu::set_ch2_length_settings(emulator, value),
                        0x17 => apu::set_ch2_envelope_settings(emulator, value),
                        0x18 => apu::set_ch2_period_low(emulator, value),
                        0x19 => apu::set_ch2_period_high(emulator, value),
                        0x1A => apu::set_ch3_dac_enabled(emulator, value),
                        0x1B => apu::set_ch3_length_settings(emulator, value),
                        0x1C => apu::set_ch3_volume(emulator, value),
                        0x1D => apu::set_ch3_period_low(emulator, value),
                        0x1E => apu::set_ch3_period_high(emulator, value),
                        0x20 => apu::set_ch4_length_settings(emulator, value),
                        0x21 => apu::set_ch4_envelope_settings(emulator, value),
                        0x22 => apu::set_ch4_polynomial(emulator, value),
                        0x23 => apu::set_ch4_control(emulator, value),
                        0x24 => apu::set_master_volume(emulator, value),
                        0x25 => apu::set_sound_panning(emulator, value),
                        0x26 => apu::set_audio_master_control(emulator, value),
                        0x30..=0x3F => apu::set_wave_ram_byte(emulator, (address & 0xF) as u8, value),
                        0x40 => gpu::set_lcdc(emulator, value),
                        0x41 => emulator.gpu.registers.stat = value,
                        0x42 => emulator.gpu.registers.scy = value,
                        0x43 => emulator.gpu.registers.scx = value,
                        0x44 => emulator.gpu.registers.ly = value,
                        0x45 => emulator.gpu.registers.lyc = value,
                        0x46 => dma::start_dma(emulator, value),
                        0x47 => emulator.gpu.registers.palettes.bgp = value,
                        0x48 => emulator.gpu.registers.palettes.obp0 = value,
                        0x49 => emulator.gpu.registers.palettes.obp1 = value,
                        0x4C => gpu::set_key0(emulator, value),
                        0x4D => speed_switch::set_key1(emulator, value),
                        0x51 => hdma::set_hdma1(emulator, value),
                        0x52 => hdma::set_hdma2(emulator, value),
                        0x53 => hdma::set_hdma3(emulator, value),
                        0x54 => hdma::set_hdma4(emulator, value),
                        0x55 => hdma::set_hdma5(emulator, value),
                        0x4A => emulator.gpu.registers.wy = value,
                        0x4B => emulator.gpu.registers.wx = value,
                        0x4F => gpu::set_cgb_vbk(emulator, value),
                        0x68 => gpu::set_cgb_bcps(emulator, value),
                        0x69 => gpu::set_cgb_bcpd(emulator, value),
                        0x6A => gpu::set_cgb_ocps(emulator, value),
                        0x6B => gpu::set_cgb_ocpd(emulator, value),
                        0x6C => gpu::set_cgb_opri(emulator, value),
                        0x70 => {
                            if is_cgb(emulator) {
                                emulator.memory.svbk = value;
                            }
                        },
                        0x0F => emulator.interrupts.flags = value,
                        0x04 => {
                            emulator.timers.divider = value;
                            emulator.timers.divider_clock = 0;
                            emulator.timers.m_cycles_clock = 0;
                        },
                        0x05 => emulator.timers.counter = value,
                        0x06 => emulator.timers.modulo = value,
                        0x07 => emulator.timers.control = value,
                        _ => ()
                    }
                },
                _ => (),
            }
        }
    }
}

pub fn load_rom_buffer(memory: &mut Memory, buffer: Vec<u8>) -> io::Result<CartridgeHeader> {
    let cartridge_result = cartridge::load_rom_buffer(buffer);
    match cartridge_result {
        Ok(cartridge) => {
            let header = cartridge.header.clone();
            memory.cartridge = cartridge;
            Ok(header)
        },
        Err(e) => Err(e)
    }
}

pub fn get_cartridge_ram(memory: &Memory) -> Vec<u8> {
    cartridge::get_cartridge_ram(&memory.cartridge)
}

pub fn set_cartridge_ram(memory: &mut Memory, buffer: Vec<u8>) {
    cartridge::set_cartridge_ram(&mut memory.cartridge, buffer);
}

#[cfg(test)]
mod tests;

mod cartridge;
mod mbc1;
mod mbc3;
mod mbc_rom_only;