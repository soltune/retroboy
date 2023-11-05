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

pub fn read_from_register(cpu_state: cpu::CpuState, register: Register) -> Result<u8, &'static str> {
    match register {
        Register::A => Ok(cpu_state.registers.a),
        Register::B => Ok(cpu_state.registers.b),
        Register::C => Ok(cpu_state.registers.c),
        Register::D => Ok(cpu_state.registers.d),
        Register::E => Ok(cpu_state.registers.e),
        Register::F => Ok(cpu_state.registers.f),
        Register::H => Ok(cpu_state.registers.h),
        Register::L => Ok(cpu_state.registers.l),
        _ => Err("Register not foumnd.")
    } 
}

pub fn store_in_register(cpu_state: &mut cpu::CpuState, register: Register, value: u8) {
    match register {
        Register::A => cpu_state.registers.a = value,
        Register::B => cpu_state.registers.b = value,
        Register::C => cpu_state.registers.c = value,
        Register::D => cpu_state.registers.d = value,
        Register::E => cpu_state.registers.e = value,
        Register::F => cpu_state.registers.f = value,
        Register::H => cpu_state.registers.h = value,
        Register::L => cpu_state.registers.l = value,
        _ => ()
    } 
}

pub fn load_immediate_value(cpu_state: &mut cpu::CpuState, register: Register) -> &mut cpu::CpuState {
    let immediate_byte = mmu::read_byte(&mut cpu_state.memory, cpu_state.registers.program_counter);
    store_in_register(cpu_state, register, immediate_byte);
    cpu_state.registers.program_counter += 1;
    cpu_state.clock.last_instr_clock_cycles = 8;
    cpu_state.clock.last_instr_machine_cycles = 2;
    cpu_state
}

pub fn execute_opcode(cpu_state: &mut cpu::CpuState) -> &mut cpu::CpuState {
    let opcode = mmu::read_byte(&mut cpu_state.memory, cpu_state.registers.program_counter);
    cpu_state.registers.program_counter += 1;
    let cpu_state = match opcode {
        0x06 => load_immediate_value(cpu_state, Register::A),
        _ => cpu_state
    };
    cpu_state.clock.clock_cycles += cpu_state.clock.last_instr_clock_cycles as u32;
    cpu_state.clock.machine_cycles += cpu_state.clock.last_instr_machine_cycles as u32;
    cpu_state
}

#[cfg(test)]
mod tests;
