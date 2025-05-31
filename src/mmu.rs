use crate::bios::{CGB_BOOT, DMG_BOOTIX};
use crate::{cheats, dma, gpu, serial};
use crate::cpu::hdma;
use crate::emulator::{is_cgb, Emulator};
use crate::mmu::cartridge::{initialize_cartridge_mapper, CartridgeMapper, CartridgeMapperSnapshot};
use crate::mmu::effects::empty_cartridge_effects;
use crate::speed_switch;
use crate::keys;
use bincode::{Decode, Encode};
use std::io;

pub use crate::mmu::cartridge::CartridgeHeader;
pub use crate::mmu::effects::CartridgeEffects;
pub use crate::mmu::mbc3::RTCState;

pub struct Memory {
    pub in_bios: bool,
    pub bios: Vec<u8>,
    pub working_ram: [u8; 0x10000],
    pub zero_page_ram: [u8; 0x80],
    pub svbk: u8,
    pub cartridge_mapper: Box<dyn CartridgeMapper>,
    pub processor_test_ram: [u8; 0xFFFF]
}

#[derive(Clone, Encode, Decode)]
pub struct MemorySnapshot {
    pub in_bios: bool,
    pub working_ram: [u8; 0x10000],
    pub zero_page_ram: [u8; 0x80],
    pub svbk: u8,
    pub cartridge: CartridgeMapperSnapshot
}

pub fn initialize_memory() -> Memory {
    Memory {
        in_bios: true,
        bios: [0; 0x100].to_vec(),
        working_ram: [0; 0x10000],
        zero_page_ram: [0; 0x80],
        svbk: 0,
        cartridge_mapper: initialize_cartridge_mapper(empty_cartridge_effects()),
        processor_test_ram: [0; 0xFFFF]
    }
}

pub fn as_snapshot(memory: &Memory) -> MemorySnapshot {
    MemorySnapshot {
        in_bios: memory.in_bios,
        working_ram: memory.working_ram,
        zero_page_ram: memory.zero_page_ram,
        svbk: memory.svbk,
        cartridge: memory.cartridge_mapper.get_snapshot()
    }
}

pub fn apply_snapshot(emulator: &mut Emulator, snapshot: MemorySnapshot) {
    emulator.memory.in_bios = snapshot.in_bios;
    emulator.memory.working_ram = snapshot.working_ram;
    emulator.memory.zero_page_ram = snapshot.zero_page_ram;
    emulator.memory.svbk = snapshot.svbk;
    emulator.memory.cartridge_mapper.apply_snapshot(snapshot.cartridge);
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

pub fn get_working_ram_bank(emulator: &Emulator) -> u8 {
    if is_cgb(emulator) {
        let masked_value = emulator.memory.svbk & 0b111;
        if masked_value == 0 { 1 } else { masked_value }
    }
    else {
        1
    }
}

fn calculate_working_ram_index(emulator: &Emulator, address: u16) -> usize {
    let localized_index = address & 0x1FFF;
    if localized_index <= 0xFFF {
        localized_index as usize
    }
    else {
        let bank_number = get_working_ram_bank(emulator);
        let index = (bank_number as u16 * 0x1000) + (address & 0x0FFF);
        index as usize 
    }
}

pub fn read_byte(emulator: &mut Emulator, address: u16) -> u8 {
    if emulator.processor_test_mode {
        emulator.memory.processor_test_ram[address as usize]
    }
    else {
        let byte = if address_accessible(emulator, address) {
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
                    emulator.memory.cartridge_mapper.read_rom(address),
                0x8000..=0x9FFF =>
                    gpu::get_video_ram_byte(emulator, address & 0x1FFF),
                0xA000..=0xBFFF =>
                    emulator.memory.cartridge_mapper.read_ram(address & 0x1FFF),
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
                        0x01 => serial::get_data(emulator),
                        0x02 => serial::get_control(emulator),
                        0x10 => emulator.apu.channel1_readonly().sweep_readonly().initial_settings() | 0b10000000,
                        0x11 => emulator.apu.channel1_readonly().length_readonly().initial_settings() | 0b00111111,
                        0x12 => emulator.apu.channel1_readonly().envelope_readonly().initial_settings(),
                        0x14 => emulator.apu.channel1_readonly().period_readonly().high() | 0b10111111,
                        0x16 => emulator.apu.channel2_readonly().length_readonly().initial_settings() | 0b00111111,
                        0x17 => emulator.apu.channel2_readonly().envelope_readonly().initial_settings(),
                        0x19 => emulator.apu.channel2_readonly().period_readonly().high() | 0b10111111,
                        0x1A => if emulator.apu.channel3_readonly().dac_enabled() { 0b11111111 } else { 0b01111111 },
                        0x1C => emulator.apu.channel3_readonly().volume() | 0b10011111,
                        0x1E => emulator.apu.channel3_readonly().period_readonly().high() | 0b10111111,
                        0x21 => emulator.apu.channel4_readonly().envelope_readonly().initial_settings(),
                        0x22 => emulator.apu.channel4_readonly().polynomial(),
                        0x23 => emulator.apu.channel4_readonly().control() | 0b10111111,
                        0x24 => emulator.apu.master_volume(),
                        0x25 => emulator.apu.sound_panning(),
                        0x26 => {
                            let result= emulator.apu.audio_master_control();
                            result
                        },
                        0x30..=0x3F => emulator.apu.get_wave_ram_byte((address & 0xF) as u8),
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
        };

        cheats::apply_cheat_if_needed(emulator, address, byte)
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
                    emulator.memory.cartridge_mapper.write_rom(address, value),
                0x8000..=0x9FFF =>
                    gpu::set_video_ram_byte(emulator, address & 0x1FFF, value),
                0xA000..=0xBFFF =>
                    emulator.memory.cartridge_mapper.write_ram(address & 0x1FFF, value),
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
                        0x01 => serial::set_data(emulator, value),
                        0x02 => serial::set_control(emulator, value),
                        0x10 => emulator.apu.set_ch1_sweep_settings(value),
                        0x11 => emulator.apu.set_ch1_length_settings(value),
                        0x12 => emulator.apu.set_ch1_envelope_settings(value),
                        0x13 => emulator.apu.set_ch1_period_low(value),
                        0x14 => emulator.apu.set_ch1_period_high(value),
                        0x16 => emulator.apu.set_ch2_length_settings(value),
                        0x17 => emulator.apu.set_ch2_envelope_settings(value),
                        0x18 => emulator.apu.set_ch2_period_low(value),
                        0x19 => emulator.apu.set_ch2_period_high(value),
                        0x1A => emulator.apu.set_ch3_dac_enabled(value),
                        0x1B => emulator.apu.set_ch3_length_settings(value),
                        0x1C => emulator.apu.set_ch3_volume(value),
                        0x1D => emulator.apu.set_ch3_period_low(value),
                        0x1E => emulator.apu.set_ch3_period_high(value),
                        0x20 => emulator.apu.set_ch4_length_settings(value),
                        0x21 => emulator.apu.set_ch4_envelope_settings(value),
                        0x22 => emulator.apu.set_ch4_polynomial(value),
                        0x23 => emulator.apu.set_ch4_control(value),
                        0x24 => emulator.apu.set_master_volume(value),
                        0x25 => emulator.apu.set_sound_panning(value),
                        0x26 => emulator.apu.set_audio_master_control(value),
                        0x30..=0x3F => emulator.apu.set_wave_ram_byte((address & 0xF) as u8, value),
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

pub fn load_rom_buffer(memory: &mut Memory, buffer: Vec<u8>, cartridge_effects: Box<dyn CartridgeEffects>) -> io::Result<CartridgeHeader> {
    let cartridge_result = cartridge::load_rom_buffer(buffer, cartridge_effects); 
    match cartridge_result {
        Ok(mapper) => {
            let cartridge = mapper.get_cartridge();
            let header = cartridge.header.clone();
            memory.cartridge_mapper = mapper;
            Ok(header)
        },
        Err(e) => Err(e)
    }
}

pub fn get_cartridge_ram(memory: &Memory) -> Vec<u8> {
    let cartridge = &memory.cartridge_mapper.get_cartridge();
    cartridge.ram.clone()
}

pub fn set_cartridge_ram(memory: &mut Memory, buffer: Vec<u8>) {
    memory.cartridge_mapper.set_cartridge_ram(buffer);
}

#[cfg(test)]
pub mod test_utils {
    use crate::mmu::cartridge::*;
    use crate::mmu::constants::*;

    pub fn build_rom(cartridge_type: u8, rom_size_index: u8, ram_size_index: u8) -> Vec<u8> {
        let mut rom_buffer: Vec<u8> = Vec::new();
        let number_of_banks = as_max_banks(rom_size_index) as u32;
        rom_buffer.resize((0x4000 * number_of_banks) as usize, 0);
        rom_buffer[CARTRIDGE_TYPE_ADDRESS] = cartridge_type; 
        rom_buffer[ROM_SIZE_ADDRESS] = rom_size_index;
        rom_buffer[RAM_SIZE_ADDRESS] = ram_size_index;
        rom_buffer
    }
}

#[cfg(test)]
mod tests;

pub mod constants;
pub mod effects;
mod cartridge;
mod huc1;
mod mbc1;
mod mbc3;
mod mbc5;
mod mbc_rom_only;
mod bank_utils;