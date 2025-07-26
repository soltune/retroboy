use crate::cpu::{Register, Cpu};
use crate::cpu::microops;

use super::{REGISTER_HL, RegisterPair};

pub fn add_value_to_register(cpu_state: &mut Cpu, register: Register, value: u8) {
    let byte = microops::read_from_register(cpu_state, &register);
    let sum = byte.wrapping_add(value);

    microops::store_in_register(cpu_state, register, sum);

    microops::set_flag_z(cpu_state, sum == 0);
    microops::set_flag_n(cpu_state, false);
    microops::set_flag_h(cpu_state, (value & 0xF) + (byte & 0xF) > 0xF);
    microops::set_flag_c(cpu_state, (value as u16 + byte as u16) > 0xFF);
}

pub fn add_value_to_register_pair(cpu_state: &mut Cpu, register_pair: RegisterPair, value: u16) {
    let word = microops::read_from_register_pair(cpu_state, &register_pair);
    let sum = word.wrapping_add(value);

    microops::store_in_register_pair(cpu_state, register_pair, sum);

    microops::set_flag_n(cpu_state, false);
    microops::set_flag_h(cpu_state, (value & 0xFFF) + (word & 0xFFF) > 0xFFF);
    microops::set_flag_c(cpu_state, (value as u32 + word as u32) > 0xFFFF);

    microops::step_one_machine_cycle(cpu_state);
}

pub fn add_value_and_carry_to_register(cpu_state: &mut Cpu, register: Register, value: u8) {
    let carry_value = if microops::is_c_flag_set(cpu_state) { 1 as u8 } else { 0 as u8 };
    let byte = microops::read_from_register(cpu_state, &register);
    let sum = byte.wrapping_add(value).wrapping_add(carry_value);

    microops::store_in_register(cpu_state, register, sum);

    microops::set_flag_z(cpu_state, sum == 0);
    microops::set_flag_n(cpu_state, false);
    microops::set_flag_h(cpu_state, ((value & 0xF) + (byte & 0xF) + carry_value) > 0xF);
    microops::set_flag_c(cpu_state, (value as u16 + byte as u16 + carry_value as u16) > 0xFF);
}

pub fn subtract_value_from_register(cpu_state: &mut Cpu, register: Register, value: u8) {
    let byte = microops::read_from_register(cpu_state, &register);
    let difference = byte.wrapping_sub(value);

    microops::store_in_register(cpu_state, register, difference);

    microops::set_flag_z(cpu_state, difference == 0);
    microops::set_flag_n(cpu_state, true);
    microops::set_flag_h(cpu_state, (byte & 0xF) < (value & 0xF));
    microops::set_flag_c(cpu_state, byte < value);
}

pub fn subtract_value_and_carry_from_register(cpu_state: &mut Cpu, register: Register, value: u8) {
    let carry_value = if microops::is_c_flag_set(cpu_state) { 1 as u8 } else { 0 as u8 };
    let byte = microops::read_from_register(cpu_state, &register);
    let difference = byte.wrapping_sub(value).wrapping_sub(carry_value);

    microops::store_in_register(cpu_state, register, difference);

    microops::set_flag_z(cpu_state, difference == 0);
    microops::set_flag_n(cpu_state, true);
    microops::set_flag_h(cpu_state, (byte & 0xF) < ((value & 0xF) + carry_value));
    microops::set_flag_c(cpu_state, ((value as u16) + (carry_value as u16)) > (byte as u16));
}

pub fn logical_and_with_register(cpu_state: &mut Cpu, register: Register, value: u8) {
    let byte = microops::read_from_register(cpu_state, &register);
    let result = byte & value;

    microops::store_in_register(cpu_state, register, result);

    microops::set_flag_z(cpu_state, result == 0);
    microops::set_flag_n(cpu_state, false);
    microops::set_flag_h(cpu_state, true);
    microops::set_flag_c(cpu_state, false);
}

pub fn logical_or_with_register(cpu_state: &mut Cpu, register: Register, value: u8) {
    let byte = microops::read_from_register(cpu_state, &register);
    let result = byte | value;

    microops::store_in_register(cpu_state, register, result);

    microops::set_flag_z(cpu_state, result == 0);
    microops::set_flag_n(cpu_state, false);
    microops::set_flag_h(cpu_state, false);
    microops::set_flag_c(cpu_state, false);
}

pub fn logical_xor_with_register(cpu_state: &mut Cpu, register: Register, value: u8) {
    let byte = microops::read_from_register(cpu_state, &register);
    let result = byte ^ value;

    microops::store_in_register(cpu_state, register, result);

    microops::set_flag_z(cpu_state, result == 0);
    microops::set_flag_n(cpu_state, false);
    microops::set_flag_h(cpu_state, false);
    microops::set_flag_c(cpu_state, false);
}

pub fn compare_value_with_register(cpu_state: &mut Cpu, register: Register, value: u8) {
    let byte = microops::read_from_register(cpu_state, &register);
    let difference = byte.wrapping_sub(value);

    microops::set_flag_z(cpu_state, difference == 0);
    microops::set_flag_n(cpu_state, true);
    microops::set_flag_h(cpu_state, (byte & 0xF) < (value & 0xF));
    microops::set_flag_c(cpu_state, byte < value);
}

pub fn increment_register(cpu_state: &mut Cpu, register: Register) {
    let byte = microops::read_from_register(cpu_state, &register);
    let sum = byte.wrapping_add(1);

    microops::store_in_register(cpu_state, register, sum);

    microops::set_flag_z(cpu_state, sum == 0);
    microops::set_flag_n(cpu_state, false);
    microops::set_flag_h(cpu_state, (1 + (byte & 0xF)) > 0xF);
}

pub fn decrement_register(cpu_state: &mut Cpu, register: Register) {
    let byte = microops::read_from_register(cpu_state, &register);
    let difference = byte.wrapping_sub(1);

    microops::store_in_register(cpu_state, register, difference);

    microops::set_flag_z(cpu_state, difference == 0);
    microops::set_flag_n(cpu_state, true);
    microops::set_flag_h(cpu_state, (byte & 0xF) < 1);
}

pub fn increment_memory_byte(cpu_state: &mut Cpu) {
    let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
    let byte = microops::read_byte_from_memory(cpu_state, address);
    let sum = byte.wrapping_add(1);

    microops::store_byte_in_memory(cpu_state, address, sum);

    microops::set_flag_z(cpu_state, sum == 0);
    microops::set_flag_n(cpu_state, false);
    microops::set_flag_h(cpu_state, (1 + (byte & 0xF)) > 0xF);
}

pub fn decrement_memory_byte(cpu_state: &mut Cpu) {
    let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
    let byte = microops::read_byte_from_memory(cpu_state, address);
    let difference = byte.wrapping_sub(1);

    microops::store_byte_in_memory(cpu_state, address, difference);

    microops::set_flag_z(cpu_state, difference == 0);
    microops::set_flag_n(cpu_state, true);
    microops::set_flag_h(cpu_state, (byte & 0xF) < 1);
}

pub fn increment_register_pair(cpu_state: &mut Cpu, register_pair: RegisterPair) {
    let word = microops::read_from_register_pair(cpu_state, &register_pair);
    let sum = word.wrapping_add(1);
    microops::store_in_register_pair(cpu_state, register_pair, sum);
    microops::step_one_machine_cycle(cpu_state);
}

pub fn decrement_register_pair(cpu_state: &mut Cpu, register_pair: RegisterPair) {
    let word = microops::read_from_register_pair(cpu_state, &register_pair);
    let sum = word.wrapping_sub(1);
    microops::store_in_register_pair(cpu_state, register_pair, sum);
    microops::step_one_machine_cycle(cpu_state);
}

pub fn bcd_adjust(cpu_state: &mut Cpu) {
    let c_flag = microops::is_c_flag_set(cpu_state);
    let n_flag = microops::is_n_flag_set(cpu_state);
    let h_flag = microops::is_h_flag_set(cpu_state);

    let mut value = microops::read_from_register(cpu_state, &Register::A);

    if n_flag {
        if c_flag {
            value = value.wrapping_sub(0x60);
        }
        if h_flag {
            value = value.wrapping_sub(0x6);
        }
    }
    else {
        if c_flag || value > 0x99 {
            value = value.wrapping_add(0x60);
            microops::set_flag_c(cpu_state, true);
        }
        if h_flag || (value & 0xF) > 0x9 {
            value = value.wrapping_add(0x6);
        }
    }

    microops::store_in_register(cpu_state, Register::A, value);

    microops::set_flag_z(cpu_state, value == 0);
    microops::set_flag_h(cpu_state, false);
}
