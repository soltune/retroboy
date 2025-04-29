use crate::emulator::Emulator;
use bincode::{Encode, Decode};

#[derive(Clone, Debug, Encode, Decode)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub f: u8,
    pub opcode: u8,
    pub program_counter: u16,
    pub stack_pointer: u16
}

#[derive(Clone, Debug, Encode, Decode)]
pub struct Interrupts {
    enable_delay: u8,
    disable_delay: u8,
    enabled: bool
}

#[derive(Clone, Debug, Encode, Decode, PartialEq)]
pub enum BusActivityType {
    Read,
    Write
}

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
pub struct BusActivityEntry {
    pub address: u16,
    pub value: u8,
    pub activity_type: BusActivityType
}

#[derive(Clone, Debug, Encode, Decode)]
pub struct CpuState {
    pub registers: Registers,
    pub halted: bool,
    pub halt_bug: bool,
    pub interrupts: Interrupts,
    pub instruction_clock_cycles: u8,
    pub opcode_bus_activity: Vec<Option<BusActivityEntry>>
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

pub fn initialize_cpu() -> CpuState {
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
            opcode: 0,
            program_counter: 0,
            stack_pointer: 0
        },
        halted: false,
        halt_bug: false,
        interrupts: Interrupts {
            enable_delay: 0,
            disable_delay: 0,
            enabled: false
        },
        instruction_clock_cycles: 0,
        opcode_bus_activity: Vec::new()
    }
}

pub fn read_next_instruction_byte(emulator: &mut Emulator) -> u8 {
    let byte = microops::read_byte_from_memory(emulator, emulator.cpu.registers.program_counter);
    emulator.cpu.registers.program_counter += 1;
    byte
}

pub fn read_next_instruction_word(emulator: &mut Emulator) -> u16 {
    let word = microops::read_word_from_memory(emulator, emulator.cpu.registers.program_counter);
    emulator.cpu.registers.program_counter += 2;
    word
}

pub fn handle_illegal_opcode(opcode: u8) {
    panic!("Encountered illegal opcode {:#04X}", opcode);
}

pub fn at_end_of_boot_rom(cpu_state: &mut CpuState) -> bool {
    cpu_state.registers.program_counter == 0x100
}

mod microops;
mod alu;
mod bitops;
mod loads;
mod jumps;
pub mod interrupts;
pub mod timers;
pub mod hdma;
pub mod opcodes;