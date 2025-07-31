use crate::apu::{Apu, ApuParams};
use crate::bios::{CGB_BOOT, DMG_BOOTIX};
use crate::cpu::interrupts::{initialize_interrupt_registers, InterruptRegisters};
use crate::cpu::timers::TimerRegisters;
use crate::gpu::{Gpu, GpuParams};
use crate::joypad::{Key, Joypad};
use crate::address_bus::cartridge::{initialize_cartridge_mapper, CartridgeMapper};
use crate::address_bus::cheats::CheatState;
use crate::address_bus::dma::DMAState;
use crate::address_bus::effects::empty_cartridge_effects;
use crate::address_bus::hdma::HDMAState;
use crate::serializable::Serializable;
use crate::serial::Serial;
use crate::utils::is_bit_set;
use crate::address_bus::speed_switch::SpeedSwitch;
use getset::{CopyGetters, Getters, MutGetters, Setters};
use std::io::{self, Read, Write};

pub use crate::address_bus::cartridge::CartridgeHeader;
pub use crate::address_bus::effects::CartridgeEffects;
pub use crate::address_bus::mbc3::RTCState;

#[derive(CopyGetters, Getters, MutGetters, Setters)]
pub struct AddressBus {
    #[getset(get_copy = "pub", set = "pub")]
    in_bios: bool,
    bios: Vec<u8>,
    working_ram: [u8; 0x10000],
    #[getset(get = "pub", get_mut = "pub")]
    zero_page_ram: [u8; 0x80],
    svbk: u8,
    cartridge_mapper: Box<dyn CartridgeMapper>,
    #[getset(get = "pub", get_mut = "pub")]
    processor_test_ram: [u8; 0xFFFF],
    #[getset(get_copy = "pub", set = "pub")]
    processor_test_mode: bool,
    #[getset(get = "pub", get_mut = "pub")]
    interrupts: InterruptRegisters,
    #[getset(get = "pub", get_mut = "pub")]
    timers: TimerRegisters,
    #[getset(get = "pub", get_mut = "pub")]
    gpu: Gpu,
    #[getset(get = "pub", get_mut = "pub")]
    joypad: Joypad,
    #[getset(get = "pub", get_mut = "pub")]
    apu: Apu,
    #[getset(get = "pub", get_mut = "pub")]
    dma: DMAState,
    #[getset(get = "pub", get_mut = "pub")]
    hdma: HDMAState,
    #[getset(get = "pub", get_mut = "pub")]
    serial: Serial,
    #[getset(get = "pub", get_mut = "pub")]
    cheats: CheatState,
    #[getset(get = "pub", get_mut = "pub")]
    speed_switch: SpeedSwitch,
    #[getset(get_copy = "pub")]
    cgb_mode: bool,
}

impl AddressBus {
    pub fn new(renderer: fn(&[u8]), processor_test_mode: bool) -> AddressBus {
        AddressBus {
            in_bios: true,
            bios: [0; 0x100].to_vec(),
            working_ram: [0; 0x10000],
            zero_page_ram: [0; 0x80],
            svbk: 0,
            cartridge_mapper: initialize_cartridge_mapper(empty_cartridge_effects()),
            processor_test_ram: [0; 0xFFFF],
            processor_test_mode,
            interrupts: initialize_interrupt_registers(),
            timers: TimerRegisters::new(),
            gpu: Gpu::new(renderer),
            joypad: Joypad::new(),
            apu: Apu::new(),
            dma: DMAState::new(),
            hdma: HDMAState::new(),
            serial: Serial::new(),
            cheats: CheatState::new(),
            speed_switch: SpeedSwitch::new(),
            cgb_mode: false,
        }
    }

    pub fn load_bios(&mut self, is_cgb: bool) {
        self.bios = if is_cgb {
            CGB_BOOT.to_vec()
        }
        else {
            DMG_BOOTIX.to_vec()
        }
    }

    pub fn get_working_ram_bank(&self) -> u8 {
        if self.cgb_mode {
            let masked_value = self.svbk & 0b111;
            if masked_value == 0 { 1 } else { masked_value }
        }
        else {
            1
        }
    }

    fn address_accessible(&self, address: u16) -> bool {
        let accessing_oam = address >= 0xFE00 && address < 0xFEA0;
        (self.dma.in_progress() && !accessing_oam) || !self.dma.in_progress()
    }

    fn calculate_working_ram_index(&self, address: u16) -> usize {
        let localized_index = address & 0x1FFF;
        if localized_index <= 0xFFF {
            localized_index as usize
        }
        else {
            let bank_number = self.get_working_ram_bank();
            let index = (bank_number as u16 * 0x1000) + (address & 0x0FFF);
            index as usize 
        }
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        if self.processor_test_mode {
            self.processor_test_ram[address as usize]
        }
        else {
            if self.address_accessible(address) {
                self.unsafe_read_byte(address)
            }
            else {
                0xFF
            }
        }
    }

    pub fn unsafe_read_byte(&mut self, address: u16) -> u8 {
        let byte = match address & 0xF000 {
            0x0000 if address <= 0x00FE && self.in_bios => {
                if address == 0x00FE {
                    self.in_bios = false;
                }
                self.bios[address as usize]
            },
            0x0000 if address >= 0x0200 && address <= 0x08FF && self.cgb_mode && self.in_bios => {
                self.bios[address as usize]
            },
            0x0000..=0x7FFF =>
                self.cartridge_mapper.read_rom(address),
            0x8000..=0x9FFF =>
                self.gpu.get_video_ram_byte(address & 0x1FFF),
            0xA000..=0xBFFF =>
                self.cartridge_mapper.read_ram(address & 0x1FFF),
            0xC000..=0xEFFF => {
                let index = self.calculate_working_ram_index(address);
                self.working_ram[index]
            }
            0xF000 => match address & 0x0F00 {
                0x000..=0xD00 => {
                    let index = self.calculate_working_ram_index(address);
                    self.working_ram[index]
                },
                0xE00 if address < 0xFEA0 => self.gpu.get_object_attribute_memory_byte(address & 0xFF),
                0xF00 if address == 0xFFFF => self.interrupts.enabled(),
                0xF00 if address >= 0xFF80 => self.zero_page_ram[(address & 0x7F) as usize],
                _ => match address & 0xFF {
                    0x00 => self.joypad.read_byte(),
                    0x01 => self.serial.data(),
                    0x02 => self.serial.control(),
                    0x10 => self.apu.channel1().sweep().initial_settings() | 0b10000000,
                    0x11 => self.apu.channel1().length().initial_settings() | 0b00111111,
                    0x12 => self.apu.channel1().envelope().initial_settings(),
                    0x14 => self.apu.channel1().period().high() | 0b10111111,
                    0x16 => self.apu.channel2().length().initial_settings() | 0b00111111,
                    0x17 => self.apu.channel2().envelope().initial_settings(),
                    0x19 => self.apu.channel2().period().high() | 0b10111111,
                    0x1A => if self.apu.channel3().dac_enabled() { 0b11111111 } else { 0b01111111 },
                    0x1C => self.apu.channel3().volume() | 0b10011111,
                    0x1E => self.apu.channel3().period().high() | 0b10111111,
                    0x21 => self.apu.channel4().envelope().initial_settings(),
                    0x22 => self.apu.channel4().polynomial(),
                    0x23 => self.apu.channel4().control() | 0b10111111,
                    0x24 => self.apu.master_volume(),
                    0x25 => self.apu.sound_panning(),
                    0x26 => self.apu.audio_master_control(),
                    0x30..=0x3F => self.apu.get_wave_ram_byte((address & 0xF) as u8),
                    0x40 => self.gpu.lcdc(),
                    0x41 => self.gpu.stat(),
                    0x42 => self.gpu.scy(),
                    0x43 => self.gpu.scx(),
                    0x44 => self.gpu.ly(),
                    0x45 => self.gpu.lyc(),
                    0x46 => self.dma.source(),
                    0x47 => self.gpu.palettes().bgp(),
                    0x48 => self.gpu.palettes().obp0(),
                    0x49 => self.gpu.palettes().obp1(),
                    0x4A => self.gpu.wy(),
                    0x4B => self.gpu.wx(),
                    0x4C => self.gpu.key0(),
                    0x4D => self.speed_switch.key1(),
                    0x4F => self.gpu.cgb_vbk(),
                    0x55 => self.hdma.hdma5(),
                    0x68 => self.gpu.palettes().cgb_bcps(),
                    0x69 => self.gpu.palettes().cgb_bcpd(),
                    0x6A => self.gpu.palettes().cgb_ocps(),
                    0x6B => self.gpu.palettes().cgb_ocpd(),
                    0x6C => self.gpu.cgb_opri(),
                    0x70 => if self.cgb_mode { self.svbk } else { 0xFF },
                    0x0F => self.interrupt_flags(),
                    0x04 => self.timers.divider(),
                    0x05 => self.timers.counter(),
                    0x06 => self.timers.modulo(),
                    0x07 => self.timers.control(),
                    _ => 0xFF
                }
            },
            _ => 0x00,
        };
        self.apply_cheat_if_needed(address, byte)
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        if self.processor_test_mode {
            self.processor_test_ram[address as usize] = value;
        }
        else {
            if self.address_accessible(address) {
                self.unsafe_write_byte(address, value);
            } 
        }
    }

    pub fn unsafe_write_byte(&mut self, address: u16, value: u8) {
        let _ = match address & 0xF000 {
            0x0000..=0x7FFF =>
                self.cartridge_mapper.write_rom(address, value),
            0x8000..=0x9FFF =>
                self.gpu.set_video_ram_byte(address & 0x1FFF, value),
            0xA000..=0xBFFF =>
                self.cartridge_mapper.write_ram(address & 0x1FFF, value),
            0xC000..=0xEFFF => {
                let index = self.calculate_working_ram_index(address);
                self.working_ram[index] = value;
            },
            0xF000 => match address & 0x0F00 {
                0x000..=0xD00 => {
                    let index = self.calculate_working_ram_index(address);
                    self.working_ram[index] = value;
                },
                0xE00 if address < 0xFEA0 => self.gpu.set_object_attribute_memory_byte(address & 0xFF, value),
                0xF00 if address == 0xFFFF => { self.interrupts.set_enabled(value); },
                0xF00 if address >= 0xFF80 => self.zero_page_ram[(address & 0x7F) as usize] = value,
                _ => match address & 0xFF {
                    0x00 => self.joypad.write_byte(value),
                    0x01 => { self.serial.set_data(value); },
                    0x02 => self.serial.set_control(value),
                    0x10 => self.apu.set_ch1_sweep_settings(value),
                    0x11 => self.apu.set_ch1_length_settings(value),
                    0x12 => self.apu.set_ch1_envelope_settings(value),
                    0x13 => self.apu.set_ch1_period_low(value),
                    0x14 => self.apu.set_ch1_period_high(value),
                    0x16 => self.apu.set_ch2_length_settings(value),
                    0x17 => self.apu.set_ch2_envelope_settings(value),
                    0x18 => self.apu.set_ch2_period_low(value),
                    0x19 => self.apu.set_ch2_period_high(value),
                    0x1A => self.apu.set_ch3_dac_enabled(value),
                    0x1B => self.apu.set_ch3_length_settings(value),
                    0x1C => self.apu.set_ch3_volume(value),
                    0x1D => self.apu.set_ch3_period_low(value),
                    0x1E => self.apu.set_ch3_period_high(value),
                    0x20 => self.apu.set_ch4_length_settings(value),
                    0x21 => self.apu.set_ch4_envelope_settings(value),
                    0x22 => self.apu.set_ch4_polynomial(value),
                    0x23 => self.apu.set_ch4_control(value),
                    0x24 => self.apu.set_master_volume(value),
                    0x25 => self.apu.set_sound_panning(value),
                    0x26 => self.apu.set_audio_master_control(value),
                    0x30..=0x3F => self.apu.set_wave_ram_byte((address & 0xF) as u8, value),
                    0x40 => self.gpu.set_lcdc(value),
                    0x41 => { self.gpu.set_stat(value); },
                    0x42 => { self.gpu.set_scy(value); },
                    0x43 => { self.gpu.set_scx(value); },
                    0x44 => { self.gpu.set_ly(value); },
                    0x45 => { self.gpu.set_lyc(value); },
                    0x46 => self.dma.start_dma(value),
                    0x47 => { self.gpu.palettes_mut().set_bgp(value); },
                    0x48 => { self.gpu.palettes_mut().set_obp0(value); },
                    0x49 => { self.gpu.palettes_mut().set_obp1(value); },
                    0x4C => { self.gpu.set_key0(value); },
                    0x4D => self.speed_switch.set_key1(value),
                    0x51 => self.hdma.set_hdma1(value),
                    0x52 => self.hdma.set_hdma2(value),
                    0x53 => self.hdma.set_hdma3(value),
                    0x54 => self.hdma.set_hdma4(value),
                    0x55 => self.hdma.set_hdma5(value),
                    0x4A => { self.gpu.set_wy(value); },
                    0x4B => { self.gpu.set_wx(value); },
                    0x4F => self.gpu.set_cgb_vbk(value),
                    0x68 => { self.gpu.palettes_mut().set_cgb_bcps(value); },
                    0x69 => self.gpu.palettes_mut().set_cgb_bcpd(value),
                    0x6A => { self.gpu.palettes_mut().set_cgb_ocps(value); },
                    0x6B => self.gpu.palettes_mut().set_cgb_ocpd(value),
                    0x6C => self.gpu.set_cgb_opri(value),
                    0x70 => {
                        if self.cgb_mode {
                            self.svbk = value;
                        }
                    },
                    0x0F => self.set_interrupt_flags(value),
                    0x04 => self.timers.set_divider(value),
                    0x05 => { self.timers.set_counter(value); },
                    0x06 => { self.timers.set_modulo(value); },
                    0x07 => { self.timers.set_control(value); },
                    _ => ()
                }
            },
            _ => (),
        };
    }

    pub fn load_rom_buffer(&mut self, buffer: Vec<u8>, cartridge_effects: Box<dyn CartridgeEffects>) -> io::Result<CartridgeHeader> {
        let cartridge_result = cartridge::load_rom_buffer(buffer, cartridge_effects); 
        match cartridge_result {
            Ok(mapper) => {
                let cartridge = mapper.get_cartridge();
                let header = cartridge.header.clone();
                self.cartridge_mapper = mapper;
                Ok(header)
            },
            Err(e) => Err(e)
        }
    }

    pub fn get_cartridge_ram(&self) -> Vec<u8> {
        let cartridge = &self.cartridge_mapper.get_cartridge();
        cartridge.ram.clone()
    }

    pub fn set_cartridge_ram(&mut self, buffer: Vec<u8>) {
        self.cartridge_mapper.set_cartridge_ram(buffer);
    }

    pub fn sync(&mut self) {
        let in_color_bios = self.in_bios && self.cgb_mode;

        self.timers.step();
        self.dma_step();
        self.gpu.step(GpuParams {
            hdma: &mut self.hdma,
            in_color_bios,
        });
        self.apu.step(ApuParams {
            in_color_bios,
            divider: self.timers.divider(),
        });
        self.serial.step();
    }

    pub fn set_cgb_mode(&mut self, value: bool) {
        self.cgb_mode = value;
        self.apu.set_cgb_mode(value);
        self.gpu.set_cgb_mode(value);
        self.serial.set_cgb_mode(value);
        self.hdma.set_cgb_mode(value);
        self.speed_switch.set_cgb_mode(value);
    }

    pub fn cartridge_mapper(&self) -> &dyn CartridgeMapper {
        self.cartridge_mapper.as_ref()
    }

    pub fn handle_key_press(&mut self, key: &Key) {
        self.joypad.handle_key_press(key);
    }

    pub fn handle_key_release(&mut self, key: &Key) {
        self.joypad.handle_key_release(key);
    }

    pub fn interrupt_flags(&self) -> u8 {
        let mut flags = 0;
        if self.gpu.vblank_interrupt() {
            flags |= 0x1;
        }
        if self.gpu.stat_interrupt() {
            flags |= 0x2;
        }
        if self.timers.interrupt() {
            flags |= 0x4;
        }
        if self.serial.interrupt() {
            flags |= 0x8;
        }
        if self.joypad.interrupt() {
            flags |= 0x10;
        }
        flags
    }

    pub fn set_interrupt_flags(&mut self, flags: u8) {
        self.gpu.set_vblank_interrupt(is_bit_set(flags, 0));
        self.gpu.set_stat_interrupt(is_bit_set(flags, 1));
        self.timers.set_interrupt(is_bit_set(flags, 2));
        self.serial.set_interrupt(is_bit_set(flags, 3));
        self.joypad.set_interrupt(is_bit_set(flags, 4));
    }
}

impl Serializable for AddressBus {
    fn serialize(&self, writer: &mut dyn Write)-> std::io::Result<()> {
        self.in_bios.serialize(writer)?;
        self.working_ram.serialize(writer)?;
        self.zero_page_ram.serialize(writer)?;
        self.svbk.serialize(writer)?;
        self.cartridge_mapper.serialize(writer)?;
        self.interrupts.serialize(writer)?;
        self.timers.serialize(writer)?;
        self.gpu.serialize(writer)?;
        self.apu.serialize(writer)?;
        self.dma.serialize(writer)?;
        self.hdma.serialize(writer)?;
        self.serial.serialize(writer)?;
        self.speed_switch.serialize(writer)?;
        Ok(())
    }

    fn deserialize(&mut self, reader: &mut dyn Read)-> std::io::Result<()> {
        self.in_bios.deserialize(reader)?;
        self.working_ram.deserialize(reader)?;
        self.zero_page_ram.deserialize(reader)?;
        self.svbk.deserialize(reader)?;
        self.cartridge_mapper.deserialize(reader)?;
        self.interrupts.deserialize(reader)?;
        self.timers.deserialize(reader)?;
        self.gpu.deserialize(reader)?;
        self.apu.deserialize(reader)?;
        self.dma.deserialize(reader)?;
        self.hdma.deserialize(reader)?;
        self.serial.deserialize(reader)?;
        self.speed_switch.deserialize(reader)?;
        Ok(())
    }
}

#[cfg(test)]
pub mod test_utils {
    use crate::address_bus::AddressBus;
    use crate::address_bus::cartridge::*;
    use crate::address_bus::constants::*;

    pub fn build_rom(cartridge_type: u8, rom_size_index: u8, ram_size_index: u8) -> Vec<u8> {
        let mut rom_buffer: Vec<u8> = Vec::new();
        let number_of_banks = as_max_banks(rom_size_index) as u32;
        rom_buffer.resize((0x4000 * number_of_banks) as usize, 0);
        rom_buffer[CARTRIDGE_TYPE_ADDRESS] = cartridge_type; 
        rom_buffer[ROM_SIZE_ADDRESS] = rom_size_index;
        rom_buffer[RAM_SIZE_ADDRESS] = ram_size_index;
        rom_buffer
    }

    pub fn initialize_test_address_bus() -> AddressBus {
        AddressBus::new(|_| {}, false)
    }
}

#[cfg(test)]
mod tests;

pub mod constants;
pub mod effects;
pub mod dma;
pub mod hdma;
pub mod cheats;
pub mod speed_switch;
mod cartridge;
mod huc1;
mod mbc1;
mod mbc3;
mod mbc5;
mod mbc_rom_only;
mod bank_utils;
