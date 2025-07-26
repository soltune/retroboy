use crate::emulator::Emulator;
use crate::address_bus::AddressBus;
use crate::serializable::Serializable;
use serializable_derive::Serializable;
use std::io::{Read, Write};

#[derive(Debug, Serializable)]
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

#[derive(Debug, Serializable)]
pub struct Interrupts {
    enable_delay: u8,
    disable_delay: u8,
    enabled: bool
}

#[derive(Debug, PartialEq)]
pub enum BusActivityType {
    Read,
    Write
}

#[derive(Debug, PartialEq)]
pub struct BusActivityEntry {
    pub address: u16,
    pub value: u8,
    pub activity_type: BusActivityType
}

pub struct CpuState {
    pub registers: Registers,
    pub halted: bool,
    pub halt_bug: bool,
    pub interrupts: Interrupts,
    pub instruction_clock_cycles: u8,
    pub opcode_bus_activity: Vec<Option<BusActivityEntry>>,
    pub address_bus: AddressBus
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

pub fn initialize_cpu(address_bus: AddressBus) -> CpuState {
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
        opcode_bus_activity: Vec::new(),
        address_bus
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

impl Serializable for CpuState {
    fn serialize(&self, writer: &mut dyn Write)-> std::io::Result<()> {
        self.registers.serialize(writer)?;
        self.halted.serialize(writer)?;
        self.halt_bug.serialize(writer)?;
        self.interrupts.serialize(writer)?;
        self.instruction_clock_cycles.serialize(writer)?;
        self.address_bus.serialize(writer)?;
        Ok(())
    }

    fn deserialize(&mut self, reader: &mut dyn Read)-> std::io::Result<()> {
        self.registers.deserialize(reader)?;
        self.halted.deserialize(reader)?;
        self.halt_bug.deserialize(reader)?;
        self.interrupts.deserialize(reader)?;
        self.instruction_clock_cycles.deserialize(reader)?;
        self.address_bus.deserialize(reader)?;
        Ok(())
    }
}

mod microops;
mod alu;
mod bitops;
mod loads;
mod jumps;
pub mod interrupts;
pub mod timers;
pub mod opcodes;
