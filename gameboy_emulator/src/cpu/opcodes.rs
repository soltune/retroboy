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

fn load_memory_byte_in_destination_register(cpu_state: &mut cpu::CpuState, address_source_register_pair: RegisterPair, destination: Register) {
    let address = read_from_register_pair(cpu_state, address_source_register_pair);
    let byte = mmu::read_byte(&mut cpu_state.memory, address);
    store_in_register(cpu_state, destination, byte);

    cpu_state.clock.last_instr_clock_cycles = 8;
    cpu_state.clock.last_instr_machine_cycles = 2;
}

pub fn execute_opcode(cpu_state: &mut cpu::CpuState) {
    let opcode = mmu::read_byte(&mut cpu_state.memory, cpu_state.registers.program_counter);

    cpu_state.registers.program_counter += 1;
    
    match opcode {
        0x06 => load_immediate_value(cpu_state, Register::B),
        0x0e => load_immediate_value(cpu_state, Register::C),
        0x16 => load_immediate_value(cpu_state, Register::D),
        0x1e => load_immediate_value(cpu_state, Register::E),
        0x26 => load_immediate_value(cpu_state, Register::H),
        0x2e => load_immediate_value(cpu_state, Register::L),
        0x78 => load_source_register_in_destination_register(cpu_state, Register::B, Register::A),
        0x79 => load_source_register_in_destination_register(cpu_state, Register::C, Register::A),
        0x7f => load_source_register_in_destination_register(cpu_state, Register::A, Register::A),
        _ => ()
    }
    
    cpu_state.clock.clock_cycles += cpu_state.clock.last_instr_clock_cycles as u32;
    cpu_state.clock.machine_cycles += cpu_state.clock.last_instr_machine_cycles as u32;
}

#[cfg(test)]
mod tests;
