use crate::cpu::{self, initialize_cpu, CpuState};
use crate::address_bus::AddressBus;
use crate::serializable::Serializable;
use serializable_derive::Serializable;
use std::cell::{Ref, RefMut};
use std::io::Result;

pub use crate::address_bus::effects::CartridgeEffects;
pub use crate::address_bus::{CartridgeHeader, RTCState};

#[derive(PartialEq, Eq, Serializable)]
pub enum Mode {
    DMG,
    CGB
}

#[derive(Serializable)]
pub struct Emulator {
    pub cpu: CpuState,
    pub mode: Mode
}

pub fn initialize_emulator(renderer: fn(&[u8])) -> Emulator {
    Emulator {
        cpu: initialize_cpu(AddressBus::new(renderer)),
        mode: Mode::DMG
    }
}

pub fn initialize_screenless_emulator() -> Emulator {
    initialize_emulator(|_| {})
}

pub fn is_cgb(emulator: &Emulator) -> bool {
    emulator.mode == Mode::CGB
}

pub fn in_color_bios(emulator: &Emulator) -> bool {
    emulator.cpu.address_bus.in_bios() && is_cgb(emulator)
}

pub fn load_rom(emulator: &mut RefMut<Emulator>, rom: &[u8], cartridge_effects: Box<dyn CartridgeEffects>) -> Result<CartridgeHeader> {
    let buffer = rom.to_vec();
    emulator.cpu.address_bus.load_rom_buffer(buffer, cartridge_effects)
}

pub fn set_cartridge_ram(emulator: &mut RefMut<Emulator>, ram: &[u8]) {
    emulator.cpu.address_bus.set_cartridge_ram(ram.to_vec());
}

pub fn get_cartridge_ram(emulator: &Ref<Emulator>) -> Vec<u8> {
    emulator.cpu.address_bus.get_cartridge_ram()
}

pub fn set_mode(emulator: &mut Emulator, mode: Mode) {
    emulator.mode = mode;

    let is_cgb = is_cgb(emulator);
    emulator.cpu.address_bus.set_cgb_mode(is_cgb);
    emulator.cpu.address_bus.load_bios(is_cgb);
}

pub fn set_sample_rate(emulator: &mut Emulator, sample_rate: u32) {
    emulator.cpu.address_bus.apu_mut().set_sample_rate(sample_rate);
}

pub fn step(emulator: &mut Emulator) {
    cpu::opcodes::step(&mut emulator.cpu);
}

pub fn step_until_next_audio_buffer(emulator: &mut Emulator) -> (&[f32], &[f32]) {
    emulator.cpu.address_bus.apu_mut().clear_audio_buffers();

    while !emulator.cpu.address_bus.apu().audio_buffers_full() {
        step(emulator);
    }

    let apu = emulator.cpu.address_bus.apu();
    let left_samples_slice = apu.get_left_sample_queue();
    let right_samples_slice = apu.get_right_sample_queue();

    (left_samples_slice, right_samples_slice)
}