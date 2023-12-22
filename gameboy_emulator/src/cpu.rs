use std::io;
use crate::mmu;

#[derive(Debug)]
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    f: u8,
    program_counter: u16,
    stack_pointer: u16,
}

#[derive(Debug)]
pub struct Clock {
    total_clock_cycles: u32
}

#[derive(Debug)]
pub struct CpuState {
    pub registers: Registers,
    pub clock: Clock,
    pub memory: mmu::Memory
}

pub enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L
}

pub struct RegisterPair {
    pub first: Register,
    pub second: Register
}

pub const REGISTER_AF: RegisterPair = RegisterPair { first: Register::A, second: Register::F };
pub const REGISTER_HL: RegisterPair = RegisterPair { first: Register::H, second: Register::L };
pub const REGISTER_BC: RegisterPair = RegisterPair { first: Register::B, second: Register::C };
pub const REGISTER_DE: RegisterPair = RegisterPair { first: Register::D, second: Register::E }; 

pub fn initialize_cpu_state() -> CpuState {
    CpuState {
        registers: Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            f: 0,
            program_counter: 0,
            stack_pointer: 0
        },
        clock: Clock {
            total_clock_cycles: 0,
        },
        memory: mmu::initialize_memory()
    }
}

pub fn load_rom_by_filepath(cpu_state: CpuState, filepath: &str) -> io::Result<CpuState> {
    let loaded_memory = mmu::load_rom_by_filepath(cpu_state.memory, filepath)?;
    Ok(CpuState { memory: loaded_memory, ..cpu_state })
}

pub fn read_next_instruction_byte(cpu_state: &mut CpuState) -> u8 {
    let byte = microops::read_byte_from_memory(cpu_state, cpu_state.registers.program_counter);
    cpu_state.registers.program_counter += 1;
    byte
}

pub fn read_next_instruction_word(cpu_state: &mut CpuState) -> u16 {
    let word = microops::read_word_from_memory(cpu_state, cpu_state.registers.program_counter);
    cpu_state.registers.program_counter += 2;
    word
}

pub fn handle_illegal_opcode(opcode: u8) {
    panic!("Encountered illegal opcode {:#04X}", opcode);
}

mod microops;
mod alu;
mod bitops;
mod loads;
mod jumps;
pub mod opcodes;