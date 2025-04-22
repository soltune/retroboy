use crate::{cpu::CpuState, emulator::Emulator};
use crate::cpu::interrupts::InterruptRegisters;
use crate::cpu::timers::TimerRegisters;
use crate::cpu::hdma::HDMAState;
use crate::gpu::{self, GpuState};
use crate::apu::ApuState;
use crate::dma::DMAState;
use crate::mmu::{self, MemorySnapshot};
use crate::serial::SerialState;
use crate::speed_switch::SpeedSwitch;
use bincode::{config, Encode, Decode};
use std::io::{Error, ErrorKind, Result};

#[derive(Clone, Encode, Decode)]
pub struct EmulatorSaveState {
    pub cpu: CpuState,
    pub interrupts: InterruptRegisters,
    pub timers: TimerRegisters,
    pub memory: MemorySnapshot,
    pub gpu: GpuState,
    pub apu: ApuState,
    pub dma: DMAState,
    pub hdma: HDMAState,
    pub serial: SerialState,
    pub speed_switch: SpeedSwitch
}

fn without_frame_buffer(gpu: &GpuState) -> GpuState {
    GpuState {
        frame_buffer: Vec::new(),
        ..gpu.clone()
    }
}

fn without_audio_buffers(apu: &ApuState) -> ApuState {
    ApuState {
        left_sample_queue: Vec::new(),
        right_sample_queue: Vec::new(),
        ..apu.clone()
    }
}

fn as_save_state(emulator: &Emulator) -> EmulatorSaveState {
    EmulatorSaveState {
        cpu: emulator.cpu.clone(),
        interrupts: emulator.interrupts.clone(),
        timers: emulator.timers.clone(),
        memory: mmu::as_snapshot(&emulator.memory),
        gpu: without_frame_buffer(&emulator.gpu),
        apu: without_audio_buffers(&emulator.apu),
        dma: emulator.dma.clone(),
        hdma: emulator.hdma.clone(),
        serial: emulator.serial.clone(),
        speed_switch: emulator.speed_switch.clone()
    }
}

fn load_save_state(emulator: &mut Emulator, save_state: EmulatorSaveState) {
    emulator.cpu = save_state.cpu;
    emulator.interrupts = save_state.interrupts;
    emulator.timers = save_state.timers;
    mmu::apply_snapshot(emulator, save_state.memory);
    emulator.gpu = save_state.gpu; 
    emulator.dma = save_state.dma;
    emulator.hdma = save_state.hdma;
    emulator.serial = save_state.serial;
    emulator.speed_switch = save_state.speed_switch;

    gpu::reset_frame_buffer(emulator);
}

pub fn encode_save_state(emulator: &Emulator) -> Result<Vec<u8>> {
    let config = config::standard();
    let save_state = as_save_state(emulator);
    match bincode::encode_to_vec(save_state, config) {
        Ok(data) => Ok(data),
        Err(e) => Err(Error::new(ErrorKind::InvalidData, e.to_string())),
    }
}

pub fn decode_save_state(emulator: &mut Emulator, data: &[u8]) -> Option<String> {
    let config = config::standard();
    match bincode::decode_from_slice(data, config) {
        Ok((save_state, _)) => {
            load_save_state(emulator, save_state);
            None
        },
        Err(e) =>
            Some(e.to_string())
    }
}
