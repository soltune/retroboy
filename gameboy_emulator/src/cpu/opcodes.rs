use crate::{cpu, mmu};

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

const REGISTER_HL: RegisterPair = RegisterPair { first: Register::H, second: Register::L };
const REGISTER_BC: RegisterPair = RegisterPair { first: Register::B, second: Register::C };
const REGISTER_DE: RegisterPair = RegisterPair { first: Register::D, second: Register::E };

fn read_from_register(cpu_state: &mut cpu::CpuState, register: Register) -> u8 {
    match register {
        Register::A => cpu_state.registers.a,
        Register::B => cpu_state.registers.b,
        Register::C => cpu_state.registers.c,
        Register::D => cpu_state.registers.d,
        Register::E => cpu_state.registers.e,
        Register::F => cpu_state.registers.f,
        Register::H => cpu_state.registers.h,
        Register::L => cpu_state.registers.l
    } 
}

fn store_in_register(cpu_state: &mut cpu::CpuState, register: Register, value: u8) {
    match register {
        Register::A => cpu_state.registers.a = value,
        Register::B => cpu_state.registers.b = value,
        Register::C => cpu_state.registers.c = value,
        Register::D => cpu_state.registers.d = value,
        Register::E => cpu_state.registers.e = value,
        Register::F => cpu_state.registers.f = value,
        Register::H => cpu_state.registers.h = value,
        Register::L => cpu_state.registers.l = value
    } 
}

fn read_from_register_pair(cpu_state: &mut cpu::CpuState, register_pair: RegisterPair) -> u16 {
    let first_byte = read_from_register(cpu_state, register_pair.first);
    let second_byte = read_from_register(cpu_state, register_pair.second);
    ((first_byte as u16) << 8) | (second_byte as u16 & 0xFF)
}

fn store_in_register_pair(cpu_state: &mut cpu::CpuState, register_pair: RegisterPair, value: u16) {
    store_in_register(cpu_state, register_pair.first, ((value >> 8) & 0xFF) as u8);
    store_in_register(cpu_state, register_pair.second, (value & 0xFF) as u8);
}   

fn load_immediate_value(cpu_state: &mut cpu::CpuState, register: Register) {
    let immediate_byte = mmu::read_byte(&mut cpu_state.memory, cpu_state.registers.program_counter);

    store_in_register(cpu_state, register, immediate_byte);

    cpu_state.registers.program_counter += 1;
    
    cpu_state.clock.last_instr_clock_cycles = 8;
    cpu_state.clock.last_instr_machine_cycles = 2;
}

fn load_source_register_in_destination_register(cpu_state: &mut cpu::CpuState, source: Register, destination: Register) {
    let source_value = read_from_register(cpu_state, source);
    store_in_register(cpu_state, destination, source_value);

    cpu_state.clock.last_instr_clock_cycles = 4;
    cpu_state.clock.last_instr_machine_cycles = 1;
}

fn load_memory_byte_in_destination_register(cpu_state: &mut cpu::CpuState, register_pair: RegisterPair, destination: Register) {
    let address = read_from_register_pair(cpu_state, register_pair);
    let byte = mmu::read_byte(&mut cpu_state.memory, address);
    store_in_register(cpu_state, destination, byte);

    cpu_state.clock.last_instr_clock_cycles = 8;
    cpu_state.clock.last_instr_machine_cycles = 2;
}

fn load_source_register_in_memory(cpu_state: &mut cpu::CpuState, register_pair: RegisterPair, source: Register) {
    let address = read_from_register_pair(cpu_state, register_pair);
    let byte = read_from_register(cpu_state, source);
    mmu::write_byte(&mut cpu_state.memory, address, byte);

    cpu_state.clock.last_instr_clock_cycles = 8;
    cpu_state.clock.last_instr_machine_cycles = 2;
}

fn load_immediate_value_in_memory(cpu_state: &mut cpu::CpuState, register_pair: RegisterPair) {
    let address = read_from_register_pair(cpu_state, register_pair);
    let immediate_byte = mmu::read_byte(&mut cpu_state.memory, cpu_state.registers.program_counter);
    mmu::write_byte(&mut cpu_state.memory, address, immediate_byte);

    cpu_state.registers.program_counter += 1;
    
    cpu_state.clock.last_instr_clock_cycles = 12;
    cpu_state.clock.last_instr_machine_cycles = 3;
}

pub fn execute_opcode(cpu_state: &mut cpu::CpuState) {
    let opcode = mmu::read_byte(&mut cpu_state.memory, cpu_state.registers.program_counter);

    cpu_state.registers.program_counter += 1;
    
    match opcode {
        0x06 => load_immediate_value(cpu_state, Register::B),
        0x0a => load_memory_byte_in_destination_register(cpu_state, REGISTER_BC, Register::A),
        0x0e => load_immediate_value(cpu_state, Register::C),
        0x16 => load_immediate_value(cpu_state, Register::D),
        0x1a => load_memory_byte_in_destination_register(cpu_state, REGISTER_DE, Register::A),
        0x1e => load_immediate_value(cpu_state, Register::E),
        0x26 => load_immediate_value(cpu_state, Register::H),
        0x2e => load_immediate_value(cpu_state, Register::L),
        0x36 => load_immediate_value_in_memory(cpu_state, REGISTER_HL),
        0x40 => load_source_register_in_destination_register(cpu_state, Register::B, Register::B),
        0x41 => load_source_register_in_destination_register(cpu_state, Register::C, Register::B),
        0x42 => load_source_register_in_destination_register(cpu_state, Register::D, Register::B),
        0x43 => load_source_register_in_destination_register(cpu_state, Register::E, Register::B),
        0x44 => load_source_register_in_destination_register(cpu_state, Register::H, Register::B),
        0x45 => load_source_register_in_destination_register(cpu_state, Register::L, Register::B),
        0x46 => load_memory_byte_in_destination_register(cpu_state, REGISTER_HL, Register::B),
        0x48 => load_source_register_in_destination_register(cpu_state, Register::B, Register::C),
        0x49 => load_source_register_in_destination_register(cpu_state, Register::C, Register::C),
        0x4a => load_source_register_in_destination_register(cpu_state, Register::D, Register::C),
        0x4b => load_source_register_in_destination_register(cpu_state, Register::E, Register::C),
        0x4c => load_source_register_in_destination_register(cpu_state, Register::H, Register::C),
        0x4d => load_source_register_in_destination_register(cpu_state, Register::L, Register::C),
        0x4e => load_memory_byte_in_destination_register(cpu_state, REGISTER_HL, Register::C),
        0x50 => load_source_register_in_destination_register(cpu_state, Register::B, Register::D),
        0x51 => load_source_register_in_destination_register(cpu_state, Register::C, Register::D),
        0x52 => load_source_register_in_destination_register(cpu_state, Register::D, Register::D),
        0x53 => load_source_register_in_destination_register(cpu_state, Register::E, Register::D),
        0x54 => load_source_register_in_destination_register(cpu_state, Register::H, Register::D),
        0x55 => load_source_register_in_destination_register(cpu_state, Register::L, Register::D),
        0x56 => load_memory_byte_in_destination_register(cpu_state, REGISTER_HL, Register::D),
        0x58 => load_source_register_in_destination_register(cpu_state, Register::B, Register::E),
        0x59 => load_source_register_in_destination_register(cpu_state, Register::C, Register::E),
        0x5a => load_source_register_in_destination_register(cpu_state, Register::D, Register::E),
        0x5b => load_source_register_in_destination_register(cpu_state, Register::E, Register::E),
        0x5c => load_source_register_in_destination_register(cpu_state, Register::H, Register::E),
        0x5d => load_source_register_in_destination_register(cpu_state, Register::L, Register::E),
        0x5e => load_memory_byte_in_destination_register(cpu_state, REGISTER_HL, Register::E),
        0x60 => load_source_register_in_destination_register(cpu_state, Register::B, Register::H),
        0x61 => load_source_register_in_destination_register(cpu_state, Register::C, Register::H),
        0x62 => load_source_register_in_destination_register(cpu_state, Register::D, Register::H),
        0x63 => load_source_register_in_destination_register(cpu_state, Register::E, Register::H),
        0x64 => load_source_register_in_destination_register(cpu_state, Register::H, Register::H),
        0x65 => load_source_register_in_destination_register(cpu_state, Register::L, Register::H),
        0x66 => load_memory_byte_in_destination_register(cpu_state, REGISTER_HL, Register::H),
        0x68 => load_source_register_in_destination_register(cpu_state, Register::B, Register::L),
        0x69 => load_source_register_in_destination_register(cpu_state, Register::C, Register::L),
        0x6a => load_source_register_in_destination_register(cpu_state, Register::D, Register::L),
        0x6b => load_source_register_in_destination_register(cpu_state, Register::E, Register::L),
        0x6c => load_source_register_in_destination_register(cpu_state, Register::H, Register::L),
        0x6d => load_source_register_in_destination_register(cpu_state, Register::L, Register::L),
        0x6e => load_memory_byte_in_destination_register(cpu_state, REGISTER_HL, Register::L),
        0x70 => load_source_register_in_memory(cpu_state, REGISTER_HL, Register::B),
        0x71 => load_source_register_in_memory(cpu_state, REGISTER_HL, Register::C),
        0x72 => load_source_register_in_memory(cpu_state, REGISTER_HL, Register::D),
        0x73 => load_source_register_in_memory(cpu_state, REGISTER_HL, Register::E),
        0x74 => load_source_register_in_memory(cpu_state, REGISTER_HL, Register::H),
        0x75 => load_source_register_in_memory(cpu_state, REGISTER_HL, Register::L),
        0x78 => load_source_register_in_destination_register(cpu_state, Register::B, Register::A),
        0x79 => load_source_register_in_destination_register(cpu_state, Register::C, Register::A),
        0x7a => load_source_register_in_destination_register(cpu_state, Register::D, Register::A),
        0x7b => load_source_register_in_destination_register(cpu_state, Register::E, Register::A),
        0x7c => load_source_register_in_destination_register(cpu_state, Register::H, Register::A),
        0x7d => load_source_register_in_destination_register(cpu_state, Register::L, Register::A),
        0x7e => load_memory_byte_in_destination_register(cpu_state, REGISTER_HL, Register::A),
        0x7f => load_source_register_in_destination_register(cpu_state, Register::A, Register::A),
        _ => ()
    }
    
    cpu_state.clock.clock_cycles += cpu_state.clock.last_instr_clock_cycles as u32;
    cpu_state.clock.machine_cycles += cpu_state.clock.last_instr_machine_cycles as u32;
}

#[cfg(test)]
mod tests;
