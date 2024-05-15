use crate::emulator::Emulator;

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
    stack_pointer: u16
}

#[derive(Debug)]
pub struct Clock {
    pub instruction_clock_cycles: u8,
    total_clock_cycles: u32
}

#[derive(Debug)]
pub struct Interrupts {
    enable_delay: u8,
    disable_delay: u8,
    enabled: bool
}

#[derive(Debug)]
pub struct CpuState {
    pub registers: Registers,
    pub clock: Clock,
    pub halted: bool,
    pub interrupts: Interrupts
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
            program_counter: 0,
            stack_pointer: 0
        },
        clock: Clock {
            instruction_clock_cycles: 0,
            total_clock_cycles: 0,
        },
        halted: false,
        interrupts: Interrupts {
            enable_delay: 0,
            disable_delay: 0,
            enabled: false
        }
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

pub fn skip_bios(cpu_state: &mut CpuState) {
    // Initialize the CPU to a state that it would be after running the BIOS.
    // This code assumes the DMG boot ROM has run.
    cpu_state.registers.a = 0x01;
    cpu_state.registers.f = 0xB0;
    cpu_state.registers.b = 0x00;
    cpu_state.registers.c = 0x13;
    cpu_state.registers.d = 0x00;
    cpu_state.registers.e = 0xD8;
    cpu_state.registers.h = 0x01;
    cpu_state.registers.l = 0x4D;
    cpu_state.registers.program_counter = 0x100;
    cpu_state.registers.stack_pointer = 0xFFFE;
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
pub mod opcodes;