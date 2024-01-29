use crate::cpu::{CpuState, initialize_cpu_state};
use crate::cpu::interrupts::InterruptRegisters;
use crate::cpu::timers::TimerRegisters;
use crate::gpu::{initialize_gpu, GpuState};
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
        cpu: initialize_cpu_state(),
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