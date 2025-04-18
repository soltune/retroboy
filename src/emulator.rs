use crate::apu;
use crate::apu::{initialize_apu, ApuState};
use crate::cheats::{initialize_cheats, CheatState};
use crate::cpu::{self, initialize_cpu, timers, CpuState};
use crate::cpu::interrupts::{initialize_innterrupt_registers, InterruptRegisters};
use crate::cpu::timers::{initialize_timer_registers, TimerRegisters};
use crate::cpu::hdma::{HDMAState, initialize_hdma};
use crate::dma;
use crate::dma::{initialize_dma, DMAState};
use crate::gpu::{self, initialize_gpu, GpuState};
use crate::keys::{initialize_keys, KeyState};
use crate::mmu;
use crate::mmu::{Memory, initialize_memory};
use crate::serial::{self, initialize_serial, SerialState};
use crate::speed_switch::{initialize_speed_switch, SpeedSwitch};
use std::cell::{Ref, RefMut};
use std::io;

pub use crate::mmu::effects::CartridgeEffects;
pub use crate::mmu::{CartridgeHeader, RTCState};

#[derive(PartialEq, Eq)]
pub enum Mode {
    DMG,
    CGB
}

pub struct Emulator {
    pub cpu: CpuState,
    pub interrupts: InterruptRegisters,
    pub timers: TimerRegisters,
    pub memory: Memory,
    pub gpu: GpuState,
    pub keys: KeyState,
    pub apu: ApuState,
    pub dma: DMAState,
    pub hdma: HDMAState,
    pub serial: SerialState,
    pub cheats: CheatState,
    pub render: fn(&[u8]),
    pub mode: Mode,
    pub speed_switch: SpeedSwitch,
    pub processor_test_mode: bool
}

pub fn initialize_emulator(render: fn(&[u8])) -> Emulator {
    Emulator {
        cpu: initialize_cpu(),
        interrupts: initialize_innterrupt_registers(),
        timers: initialize_timer_registers(),
        memory: initialize_memory(),
        gpu: initialize_gpu(),
        keys: initialize_keys(),
        apu: initialize_apu(),
        dma: initialize_dma(),
        hdma: initialize_hdma(),
        serial: initialize_serial(),
        cheats: initialize_cheats(),
        render,
        mode: Mode::DMG,
        speed_switch: initialize_speed_switch(),
        processor_test_mode: false
    }
}

pub fn initialize_screenless_emulator() -> Emulator {
    initialize_emulator(|_| {})
}

pub fn is_cgb(emulator: &Emulator) -> bool {
    emulator.mode == Mode::CGB
}

pub fn in_color_bios(emulator: &Emulator) -> bool {
    emulator.memory.in_bios && is_cgb(emulator)
}

pub fn load_rom(emulator: &mut RefMut<Emulator>, rom: &[u8], cartridge_effects: Box<dyn CartridgeEffects>) -> io::Result<CartridgeHeader> {
    let buffer = rom.to_vec();
    mmu::load_rom_buffer(&mut emulator.memory, buffer, cartridge_effects)
}

pub fn set_cartridge_ram(emulator: &mut RefMut<Emulator>, ram: &[u8]) {
    mmu::set_cartridge_ram(&mut emulator.memory, ram.to_vec());
}

pub fn get_cartridge_ram(emulator: &Ref<Emulator>) -> Vec<u8> {
    mmu::get_cartridge_ram(&emulator.memory)
}

pub fn sync(emulator: &mut Emulator) {
    timers::step(emulator);
    dma::step(emulator);
    gpu::step(emulator);
    apu::step(emulator);
    serial::step(emulator);
}

pub fn set_mode(emulator: &mut Emulator, mode: Mode) {
    emulator.mode = mode;
    mmu::load_bios(emulator);
}

pub fn set_sample_rate(emulator: &mut Emulator, sample_rate: u32) {
    apu::set_sample_rate(emulator, sample_rate);
}

pub fn step(emulator: &mut Emulator) {
    cpu::opcodes::step(emulator);
}

pub fn step_until_next_audio_buffer(emulator: &mut Emulator) -> (&[f32], &[f32]) {
    apu::clear_audio_buffers(emulator);

    while !apu::audio_buffers_full(emulator) {
        step(emulator);
    }

    let left_samples_slice = apu::get_left_sample_queue(emulator);
    let right_samples_slice = apu::get_right_sample_queue(emulator);

    (left_samples_slice, right_samples_slice)
}
