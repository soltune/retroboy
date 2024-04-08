use crate::cpu::{self, at_end_of_boot_rom, initialize_cpu, CpuState};
use crate::cpu::interrupts::InterruptRegisters;
use crate::cpu::timers::TimerRegisters;
use crate::gpu::{self, initialize_gpu, GpuState};
use crate::mmu;
use crate::mmu::{Memory, initialize_memory};
use std::io;

#[derive(Debug)]
pub struct Emulator {
    pub cpu: CpuState,
    pub interrupts: InterruptRegisters,
    pub timers: TimerRegisters,
    pub memory: Memory,
    pub gpu: GpuState
}

const SUPPORTED_CARTRIDGE_TYPES: [u8; 1] = [0x00]; 

pub fn initialize_emulator() -> Emulator {
    Emulator {
        cpu: initialize_cpu(),
        interrupts: InterruptRegisters {
            enabled: 0,
            flags: 0
        },
        timers: TimerRegisters {
            m_cycles_clock: 0,
            base_clock: 0,
            divider_clock: 0,
            divider: 0,
            counter: 0,
            modulo: 0,
            control: 0
        },
        memory: initialize_memory(),
        gpu: initialize_gpu()
    }
}

fn load_rom_by_filepath(emulator: Emulator, rom_filepath: &str, bios_filepath: &str) -> io::Result<Emulator> {
    let with_loaded_rom = mmu::load_rom_by_filepath(emulator.memory, rom_filepath)?;
    let loaded_memory = mmu::load_bios_by_filepath(with_loaded_rom, bios_filepath)?; 
    let cartridge_type = loaded_memory.cartridge_header.type_code;
    if SUPPORTED_CARTRIDGE_TYPES.contains(&cartridge_type) {
        Ok(Emulator { memory: loaded_memory, ..emulator })
    }
    else {
        let error_message  = format!("Unsupported cartridge type {cartridge_type}."); 
        Err(io::Error::new(io::ErrorKind::Other, error_message)) 
    }
}

pub fn initialize_emulator_by_filepath(rom_filepath: &str, bios_filepath: &str) -> io::Result<Emulator> {
    let emulator = initialize_emulator();
    load_rom_by_filepath(emulator, rom_filepath, bios_filepath)
}

fn transfer_to_game_rom(memory: &mut Memory) {
    memory.in_bios = true;
}

pub fn step(emulator: &mut Emulator, render: impl FnMut(&Vec<u32>)) {
    if at_end_of_boot_rom(&mut emulator.cpu) {
        transfer_to_game_rom(&mut emulator.memory);
    }

    cpu::opcodes::step(emulator);
    gpu::step(emulator, render);
}