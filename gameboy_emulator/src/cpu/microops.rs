use crate::mmu;
use crate::cpu::{Register, RegisterPair, CpuState};

pub fn read_byte_from_memory(cpu_state: &mut CpuState, address: u16) -> u8 {
    let byte = mmu::read_byte(&mut cpu_state.memory, address);
    cpu_state.clock.total_clock_cycles += 4;
    byte
}

pub fn read_word_from_memory(cpu_state: &mut CpuState, address: u16) -> u16 {
    let word = mmu::read_word(&mut cpu_state.memory, address);
    cpu_state.clock.total_clock_cycles += 8;
    word
}

pub fn store_byte_in_memory(cpu_state: &mut CpuState, address: u16, byte: u8) {
    mmu::write_byte(&mut cpu_state.memory, address, byte);
    cpu_state.clock.total_clock_cycles += 4;
}

pub fn store_word_in_memory(cpu_state: &mut CpuState, address: u16, word: u16) {
    mmu::write_word(&mut cpu_state.memory, address, word);
    cpu_state.clock.total_clock_cycles += 8;
}

pub fn read_from_register(cpu_state: &mut CpuState, register: &Register) -> u8 {
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

pub fn store_in_register(cpu_state: &mut CpuState, register: Register, value: u8) {
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

pub fn read_from_register_pair(cpu_state: &mut CpuState, register_pair: RegisterPair) -> u16 {
    let first_byte = read_from_register(cpu_state, &register_pair.first);
    let second_byte = read_from_register(cpu_state, &register_pair.second);
    ((first_byte as u16) << 8) | (second_byte as u16 & 0xFF)
}

pub fn store_in_register_pair(cpu_state: &mut CpuState, register_pair: RegisterPair, value: u16) {
    store_in_register(cpu_state, register_pair.first, ((value >> 8) & 0xFF) as u8);
    store_in_register(cpu_state, register_pair.second, (value & 0xFF) as u8);
}

pub fn set_flag_z(cpu_state: &mut CpuState, flag: bool) {
    if flag {
        cpu_state.registers.f = cpu_state.registers.f | 0x80;
    } else {
        cpu_state.registers.f = cpu_state.registers.f & 0x7F;
    }
}

pub fn set_flag_n(cpu_state: &mut CpuState, flag: bool) {
    if flag {
        cpu_state.registers.f = cpu_state.registers.f | 0x40;
    } else {
        cpu_state.registers.f = cpu_state.registers.f & 0xBF;
    }
}

pub fn set_flag_h(cpu_state: &mut CpuState, flag: bool) {
    if flag {
        cpu_state.registers.f = cpu_state.registers.f | 0x20;
    } else {
        cpu_state.registers.f = cpu_state.registers.f & 0xDF;
    }
}

pub fn set_flag_c(cpu_state: &mut CpuState, flag: bool) {
    if flag {
        cpu_state.registers.f = cpu_state.registers.f | 0x10;
    } else {
        cpu_state.registers.f = cpu_state.registers.f & 0xEF;
    }
}

pub fn is_c_flag_set(cpu_state: &mut CpuState) -> bool {
    let value = read_from_register(cpu_state, &Register::F);
    (value & 0x10) == 0x10
}

pub fn run_extra_machine_cycle(cpu_state: &mut CpuState) {
    cpu_state.clock.total_clock_cycles += 4;
}
