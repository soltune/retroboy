use crate::apu::Apu;
use crate::cpu::CpuState;
use crate::cpu::interrupts::InterruptRegisters;
use crate::cpu::timers::TimerRegisters;
use crate::address_bus::hdma::HDMAState;
use crate::address_bus::dma::DMAState;
use crate::emulator::Emulator;
use crate::gpu::Gpu;
use crate::address_bus::MemorySnapshot;
use crate::serial::Serial;
use crate::address_bus::speed_switch::SpeedSwitch;
use bincode::{config, Encode, Decode};
use std::io::{Error, ErrorKind, Result};

pub struct SaveStateHeader {
    pub version: u8,
    pub title: String,
}

#[derive(Clone, Encode, Decode)]
pub struct SaveStateSnapshot {
    pub cpu: CpuState,
    pub interrupts: InterruptRegisters,
    pub timers: TimerRegisters,
    pub memory: MemorySnapshot,
    pub gpu: Gpu,
    pub apu: Apu,
    pub dma: DMAState,
    pub hdma: HDMAState,
    pub serial: Serial,
    pub speed_switch: SpeedSwitch
}

pub const MAJOR_VERSION: u8 = 4;
pub const HEADER_IDENTIFIER: &str = "HEADER";
pub const STATE_IDENTIFIER: &str = "STATE";
pub const FORMAT_ERROR: &str = "The provided save state file is in an invalid format.";

fn without_frame_buffer(gpu: &Gpu) -> Gpu {
    let mut gpu = gpu.clone();
    gpu.remove_frame_buffer();
    gpu
}

fn without_audio_buffers(apu: &Apu) -> Apu {
    let mut apu = apu.clone();
    apu.clear_audio_buffers();
    apu
}

fn as_state_snapshot(emulator: &Emulator) -> SaveStateSnapshot {
    SaveStateSnapshot {
        cpu: emulator.cpu.clone(),
        interrupts: emulator.address_bus.interrupts_readonly().clone(),
        timers: emulator.address_bus.timers_readonly().clone(),
        memory: emulator.address_bus.as_memory_snapshot(),
        gpu: without_frame_buffer(emulator.address_bus.gpu_readonly()),
        apu: without_audio_buffers(emulator.address_bus.apu_readonly()),
        dma: emulator.address_bus.dma_readonly().clone(),
        hdma: emulator.address_bus.hdma_readonly().clone(),
        serial: emulator.address_bus.serial_readonly().clone(),
        speed_switch: emulator.address_bus.speed_switch_readonly().clone()
    }
}

fn load_state_snapshot(emulator: &mut Emulator, save_state: SaveStateSnapshot) {
    emulator.cpu = save_state.cpu;
    emulator.address_bus.set_interrupts(save_state.interrupts);
    emulator.address_bus.set_timers(save_state.timers);
    emulator.address_bus.apply_memory_snapshot(save_state.memory);
    emulator.address_bus.set_gpu(save_state.gpu); 
    emulator.address_bus.set_dma(save_state.dma);
    emulator.address_bus.set_hdma(save_state.hdma);
    emulator.address_bus.set_serial(save_state.serial);
    emulator.address_bus.set_speed_switch(save_state.speed_switch);
    emulator.address_bus.set_apu(save_state.apu);

    emulator.address_bus.gpu().reset_frame_buffer();
    emulator.address_bus.apu().clear_summed_samples();
}

fn as_invalid_data_result(message: &str) -> Error {
    Error::new(ErrorKind::InvalidData, message)
}

fn as_format_error_result(message: &str) -> Error {
    Error::new(ErrorKind::InvalidData, format!("{} Error: {}", FORMAT_ERROR, message))
}

pub fn encode_save_state(emulator: &Emulator) -> Result<Vec<u8>> {
    let mut save_state_bytes = Vec::new();
    let state = as_state_snapshot(emulator);

    let header_identifier_bytes = HEADER_IDENTIFIER.as_bytes();
    save_state_bytes.extend_from_slice(header_identifier_bytes);

    save_state_bytes.push(MAJOR_VERSION);
    let title = emulator.address_bus.cartridge_mapper().title();
    save_state_bytes.push(title.len() as u8);
    save_state_bytes.extend_from_slice(title.as_bytes());

    let state_identifier_bytes = STATE_IDENTIFIER.as_bytes();
    save_state_bytes.extend_from_slice(state_identifier_bytes);

    match bincode::encode_to_vec(state, config::standard()) {
        Ok(data) => {
            save_state_bytes.extend_from_slice(&data);
            Ok(save_state_bytes)
        },
        Err(e) => {
            Err(as_invalid_data_result(e.to_string().as_str()))
        }
    }
}

fn decode_save_state(current_game_title: String, data: &[u8]) -> Result<SaveStateSnapshot> {
    let header_identifier_size = HEADER_IDENTIFIER.len();
    let header_identifier_bytes = &data[..header_identifier_size];
    if data.len() < header_identifier_size || header_identifier_bytes != HEADER_IDENTIFIER.as_bytes() {
        Err(as_format_error_result("Invalid save state header."))
    }
    else {
        let version = data[header_identifier_size];
        let title_length = data[header_identifier_size + 1] as usize;
        let title_start = header_identifier_size + 2;
        let state_identifier_start = title_start + title_length;
        let title_bytes = data[title_start..state_identifier_start].to_vec();
        let title = String::from_utf8(title_bytes)
            .map_err(|e| as_invalid_data_result(e.to_string().as_str()))?;
    
        let header = SaveStateHeader { version, title };
    
        let state_identifier_size = STATE_IDENTIFIER.len();
        let state_start = state_identifier_start + state_identifier_size;
        let state_identifier_bytes= &data[state_identifier_start..state_start];
    
        if state_start > data.len() || state_identifier_bytes != STATE_IDENTIFIER.as_bytes() {
            Err(as_format_error_result("Invalid save state identifier."))
        }
        else if version != MAJOR_VERSION {
            Err(as_format_error_result(format!("The save state is using an older incompatible version. Save state version: {}, Current version: {}.", header.version, MAJOR_VERSION).as_str()))
        }
        else if header.title != current_game_title {
            Err(as_format_error_result(format!("This save state is meant to be used for a different game. Save state game key: '{}', Current game key: '{}'.", header.title, current_game_title).as_str()))
        }
        else {
            let state_bytes = &data[state_start..];
            
            match bincode::decode_from_slice(state_bytes, config::standard()) {
                Ok((state_snapshot, _)) => Ok(state_snapshot),
                Err(e) => Err(as_format_error_result(e.to_string().as_str()))
            }
        }
    }
}

pub fn apply_save_state(emulator: &mut Emulator, data: &[u8]) -> Result<()> {
    let title = emulator.address_bus.cartridge_mapper().title();
    let snapshot = decode_save_state(title, data)?;
    load_state_snapshot(emulator, snapshot);
    Ok(())
}
