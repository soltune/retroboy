use crate::cpu::{at_end_of_boot_rom, initialize_cpu, opcodes, CpuState};
use crate::cpu::interrupts::InterruptRegisters;
use crate::cpu::timers::TimerRegisters;
use crate::gpu::{self, initialize_gpu, GpuState};
use crate::mmu;
use crate::mmu::{Memory, initialize_memory};
use std::io;

pub struct Emulator {
    pub cpu: CpuState,
    pub interrupts: InterruptRegisters,
    pub timers: TimerRegisters,
    pub memory: Memory,
    pub gpu: GpuState
}

fn load_rom_by_filepath(emulator: Emulator, filepath: &str) -> io::Result<Emulator> {
    let loaded_memory = mmu::load_rom_by_filepath(emulator.memory, filepath)?;
    Ok(Emulator { memory: loaded_memory, ..emulator })
}

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

pub fn initialize_emulator_by_filepath(filepath: &str) -> io::Result<Emulator> {
    let emulator = initialize_emulator();
    load_rom_by_filepath(emulator, filepath)
}

pub fn transfer_to_game_rom(memory: &mut Memory) {
    memory.in_bios = true;
}

pub fn step(emulator: &mut Emulator) {
    if at_end_of_boot_rom(&mut emulator.cpu) {
        transfer_to_game_rom(&mut emulator.memory);
    }

    opcodes::step(emulator);
    gpu::step(emulator);
}