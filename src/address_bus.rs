use crate::apu::{Apu, ApuParams};
use crate::bios::{CGB_BOOT, DMG_BOOTIX};
use crate::cpu::interrupts::{initialize_interrupt_registers, InterruptRegisters};
use crate::cpu::timers::{TimerRegisters, TimerParams};
use crate::gpu::{Gpu, GpuParams};
use crate::joypad::{Key, Joypad};
use crate::address_bus::cartridge::{initialize_cartridge_mapper, CartridgeMapper, CartridgeMapperSnapshot};
use crate::address_bus::cheats::CheatState;
use crate::address_bus::dma::DMAState;
use crate::address_bus::effects::empty_cartridge_effects;
use crate::address_bus::hdma::HDMAState;
use crate::serial::{Serial, SerialParams};
use crate::address_bus::speed_switch::SpeedSwitch;
use bincode::{Decode, Encode};
use std::io;

pub use crate::address_bus::cartridge::CartridgeHeader;
pub use crate::address_bus::effects::CartridgeEffects;
pub use crate::address_bus::mbc3::RTCState;

pub struct AddressBus {
    in_bios: bool,
    bios: Vec<u8>,
    working_ram: [u8; 0x10000],
    zero_page_ram: [u8; 0x80],
    svbk: u8,
    cartridge_mapper: Box<dyn CartridgeMapper>,
    processor_test_ram: [u8; 0xFFFF],
    processor_test_mode: bool,
    interrupts: InterruptRegisters,
    timers: TimerRegisters,
    gpu: Gpu,
    joypad: Joypad,
    apu: Apu,
    dma: DMAState,
    hdma: HDMAState,
    serial: Serial,
    cheats: CheatState,
    speed_switch: SpeedSwitch,
    cgb_mode: bool,
    renderer: fn(&[u8])
}

#[derive(Clone, Encode, Decode)]
pub struct MemorySnapshot {
    pub in_bios: bool,
    pub working_ram: [u8; 0x10000],
    pub zero_page_ram: [u8; 0x80],
    pub svbk: u8,
    pub cartridge: CartridgeMapperSnapshot
}

impl AddressBus {
    pub fn new(renderer: fn(&[u8])) -> AddressBus {
        AddressBus {
            in_bios: true,
            bios: [0; 0x100].to_vec(),
            working_ram: [0; 0x10000],
            zero_page_ram: [0; 0x80],
            svbk: 0,
            cartridge_mapper: initialize_cartridge_mapper(empty_cartridge_effects()),
            processor_test_ram: [0; 0xFFFF],
            processor_test_mode: false,
            interrupts: initialize_interrupt_registers(),
            timers: TimerRegisters::new(),
            gpu: Gpu::new(),
            joypad: Joypad::new(),
            apu: Apu::new(),
            dma: DMAState::new(),
            hdma: HDMAState::new(),
            serial: Serial::new(),
            cheats: CheatState::new(),
            speed_switch: SpeedSwitch::new(),
            cgb_mode: false,
            renderer
        }
    }

    pub fn as_memory_snapshot(&self) -> MemorySnapshot {
        MemorySnapshot {
            in_bios: self.in_bios,
            working_ram: self.working_ram,
            zero_page_ram: self.zero_page_ram,
            svbk: self.svbk,
            cartridge: self.cartridge_mapper.get_snapshot()
        }
    }

    pub fn apply_memory_snapshot(&mut self, snapshot: MemorySnapshot) {
        self.in_bios = snapshot.in_bios;
        self.working_ram = snapshot.working_ram;
        self.zero_page_ram = snapshot.zero_page_ram;
        self.svbk = snapshot.svbk;
        self.cartridge_mapper.apply_snapshot(snapshot.cartridge);
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
                0xF00 if address == 0xFFFF => self.interrupts.enabled,
                0xF00 if address >= 0xFF80 => self.zero_page_ram[(address & 0x7F) as usize],
                _ => match address & 0xFF {
                    0x00 => self.joypad.read_byte(),
                    0x01 => self.serial.data(),
                    0x02 => self.serial.control(),
                    0x10 => self.apu.channel1_readonly().sweep_readonly().initial_settings() | 0b10000000,
                    0x11 => self.apu.channel1_readonly().length_readonly().initial_settings() | 0b00111111,
                    0x12 => self.apu.channel1_readonly().envelope_readonly().initial_settings(),
                    0x14 => self.apu.channel1_readonly().period_readonly().high() | 0b10111111,
                    0x16 => self.apu.channel2_readonly().length_readonly().initial_settings() | 0b00111111,
                    0x17 => self.apu.channel2_readonly().envelope_readonly().initial_settings(),
                    0x19 => self.apu.channel2_readonly().period_readonly().high() | 0b10111111,
                    0x1A => if self.apu.channel3_readonly().dac_enabled() { 0b11111111 } else { 0b01111111 },
                    0x1C => self.apu.channel3_readonly().volume() | 0b10011111,
                    0x1E => self.apu.channel3_readonly().period_readonly().high() | 0b10111111,
                    0x21 => self.apu.channel4_readonly().envelope_readonly().initial_settings(),
                    0x22 => self.apu.channel4_readonly().polynomial(),
                    0x23 => self.apu.channel4_readonly().control() | 0b10111111,
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
                    0x47 => self.gpu.palettes_readonly().bgp(),
                    0x48 => self.gpu.palettes_readonly().obp0(),
                    0x49 => self.gpu.palettes_readonly().obp1(),
                    0x4A => self.gpu.wy(),
                    0x4B => self.gpu.wx(),
                    0x4C => self.gpu.key0(),
                    0x4D => self.speed_switch.key1(),
                    0x4F => self.gpu.cgb_vbk(),
                    0x55 => self.hdma.hdma5(),
                    0x68 => self.gpu.palettes_readonly().cgb_bcps(),
                    0x69 => self.gpu.palettes_readonly().cgb_bcpd(),
                    0x6A => self.gpu.palettes_readonly().cgb_ocps(),
                    0x6B => self.gpu.palettes_readonly().cgb_ocpd(),
                    0x6C => self.gpu.cgb_opri(),
                    0x70 => if self.cgb_mode { self.svbk } else { 0xFF },
                    0x0F => self.interrupts.flags,
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
        match address & 0xF000 {
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
                0xF00 if address == 0xFFFF => self.interrupts.enabled = value,
                0xF00 if address >= 0xFF80 => self.zero_page_ram[(address & 0x7F) as usize] = value,
                _ => match address & 0xFF {
                    0x00 => self.joypad.write_byte(value),
                    0x01 => self.serial.set_data(value),
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
                    0x41 => self.gpu.set_stat(value),
                    0x42 => self.gpu.set_scy(value),
                    0x43 => self.gpu.set_scx(value),
                    0x44 => self.gpu.set_ly(value),
                    0x45 => self.gpu.set_lyc(value),
                    0x46 => self.dma.start_dma(value),
                    0x47 => self.gpu.palettes().set_bgp(value),
                    0x48 => self.gpu.palettes().set_obp0(value),
                    0x49 => self.gpu.palettes().set_obp1(value),
                    0x4C => self.gpu.set_key0(value),
                    0x4D => self.speed_switch.set_key1(value),
                    0x51 => self.hdma.set_hdma1(value),
                    0x52 => self.hdma.set_hdma2(value),
                    0x53 => self.hdma.set_hdma3(value),
                    0x54 => self.hdma.set_hdma4(value),
                    0x55 => self.hdma.set_hdma5(value),
                    0x4A => self.gpu.set_wy(value),
                    0x4B => self.gpu.set_wx(value),
                    0x4F => self.gpu.set_cgb_vbk(value),
                    0x68 => self.gpu.palettes().set_cgb_bcps(value),
                    0x69 => self.gpu.palettes().set_cgb_bcpd(value),
                    0x6A => self.gpu.palettes().set_cgb_ocps(value),
                    0x6B => self.gpu.palettes().set_cgb_ocpd(value),
                    0x6C => self.gpu.set_cgb_opri(value),
                    0x70 => {
                        if self.cgb_mode {
                            self.svbk = value;
                        }
                    },
                    0x0F => self.interrupts.flags = value,
                    0x04 => self.timers.set_divider(value),
                    0x05 => self.timers.set_counter(value),
                    0x06 => self.timers.set_modulo(value),
                    0x07 => self.timers.set_control(value),
                    _ => ()
                }
            },
            _ => (),
        }
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

        self.timers.step(TimerParams {
            interrupt_registers: &mut self.interrupts
        });
        self.dma_step();
        self.gpu.step(GpuParams {
            interrupt_registers: &mut self.interrupts,
            hdma: &mut self.hdma,
            in_color_bios,
            renderer: self.renderer
        });
        self.apu.step(ApuParams {
            in_color_bios,
            divider: self.timers.divider(),
        });
        self.serial.step(SerialParams {
            interrupt_registers: &mut self.interrupts,
        });
    }

    pub fn speed_switch_readonly(&self) -> &SpeedSwitch {
        &self.speed_switch
    }

    pub fn speed_switch(&mut self) -> &mut SpeedSwitch {
        &mut self.speed_switch
    }

    pub fn processor_test_mode(&self) -> bool {
        self.processor_test_mode
    }

    pub fn set_processor_test_mode(&mut self, value: bool) {
        self.processor_test_mode = value;
    }

    pub fn interrupts_readonly(&self) -> &InterruptRegisters {
        &self.interrupts
    }

    pub fn interrupts(&mut self) -> &mut InterruptRegisters {
        &mut self.interrupts
    }

    pub fn hdma_readonly(&self) -> &HDMAState {
        &self.hdma
    }

    pub fn hdma(&mut self) -> &mut HDMAState {
        &mut self.hdma
    }

    pub fn in_bios(&self) -> bool {
        self.in_bios
    }

    pub fn cgb_mode(&self) -> bool {
        self.cgb_mode
    }

    pub fn set_cgb_mode(&mut self, value: bool) {
        self.cgb_mode = value;
        self.apu.set_cgb_mode(value);
        self.gpu.set_cgb_mode(value);
        self.serial.set_cgb_mode(value);
        self.hdma.set_cgb_mode(value);
        self.speed_switch.set_cgb_mode(value);
    }

    pub fn apu_readonly(&self) -> &Apu {
        &self.apu
    }

    pub fn apu(&mut self) -> &mut Apu {
        &mut self.apu
    }

    pub fn gpu_readonly(&self) -> &Gpu {
        &self.gpu
    }

    pub fn gpu(&mut self) -> &mut Gpu {
        &mut self.gpu
    }

    pub fn serial_readonly(&self) -> &Serial {
        &self.serial
    }

    pub fn serial(&mut self) -> &mut Serial {
        &mut self.serial
    }

    pub fn joypad_readonly(&self) -> &Joypad {
        &self.joypad
    }

    pub fn joypad(&mut self) -> &mut Joypad {
        &mut self.joypad
    }

    pub fn timers_readonly(&self) -> &TimerRegisters {
        &self.timers
    }

    pub fn timers(&mut self) -> &mut TimerRegisters {
        &mut self.timers
    }

    pub fn dma_readonly(&self) -> &DMAState {
        &self.dma
    }

    pub fn dma(&mut self) -> &mut DMAState {
        &mut self.dma
    }

    pub fn cartridge_mapper(&self) -> &dyn CartridgeMapper {
        self.cartridge_mapper.as_ref()
    }

    pub fn set_interrupts(&mut self, interrupts: InterruptRegisters) {
        self.interrupts = interrupts;
    }

    pub fn set_timers(&mut self, timers: TimerRegisters) {
        self.timers = timers;
    }

    pub fn set_gpu(&mut self, gpu: Gpu) {
        self.gpu = gpu;
    }

    pub fn set_dma(&mut self, dma: DMAState) {
        self.dma = dma;
    }

    pub fn set_hdma(&mut self, hdma: HDMAState) {
        self.hdma = hdma;
    }

    pub fn set_serial(&mut self, serial: Serial) {
        self.serial = serial;
    }

    pub fn set_speed_switch(&mut self, speed_switch: SpeedSwitch) {
        self.speed_switch = speed_switch;
    }

    pub fn set_apu(&mut self, apu: Apu) {
        self.apu = apu;
    }

    pub fn handle_key_press(&mut self, key: &Key) {
        self.joypad.handle_key_press(&mut self.interrupts, key);
    }

    pub fn handle_key_release(&mut self, key: &Key) {
        self.joypad.handle_key_release(key);
    }

    pub fn set_in_bios(&mut self, value: bool) {
        self.in_bios = value;
    }

    pub fn zero_page_ram_readonly(&self) -> &[u8; 0x80] {
        &self.zero_page_ram
    }

    pub fn zero_page_ram(&mut self) -> &mut [u8; 0x80] {
        &mut self.zero_page_ram
    }

    pub fn processor_test_ram_readonly(&self) -> &[u8; 0xFFFF] {
        &self.processor_test_ram
    }

    pub fn processor_test_ram(&mut self) -> &mut [u8; 0xFFFF] {
        &mut self.processor_test_ram
    }

    pub fn cheats(&mut self) -> &mut CheatState {
        &mut self.cheats
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
        AddressBus::new(|_| {})
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