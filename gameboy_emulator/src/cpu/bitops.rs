use crate::cpu::{Register, CpuState, REGISTER_HL};
use crate::cpu::microops;
use crate::emulator::Emulator;

fn rotate_left(cpu_state: &mut CpuState, byte: u8) -> u8 {
    let most_significant_bit = byte >> 7;
    let rotated_value = byte << 1 | most_significant_bit;
    microops::set_flag_z(cpu_state, rotated_value == 0);
    microops::set_flag_n(cpu_state, false);
    microops::set_flag_h(cpu_state, false);
    microops::set_flag_c(cpu_state, most_significant_bit == 0x01);
    rotated_value
}

fn rotate_left_through_carry(cpu_state: &mut CpuState, byte: u8) -> u8 {
    let c_flag = if microops::is_c_flag_set(cpu_state) { 0x1 } else { 0x0 };
    let most_significant_bit = byte >> 7;
    let rotated_value = byte << 1 | c_flag;
    microops::set_flag_z(cpu_state, rotated_value == 0);
    microops::set_flag_n(cpu_state, false);
    microops::set_flag_h(cpu_state, false);
    microops::set_flag_c(cpu_state, most_significant_bit == 0x01);
    rotated_value
}

fn rotate_right(cpu_state: &mut CpuState, byte: u8) -> u8 {
    let least_significant_bit = byte & 0x1;
    let rotated_value: u8 = least_significant_bit << 7 | byte >> 1;
    microops::set_flag_z(cpu_state, rotated_value == 0);
    microops::set_flag_n(cpu_state, false);
    microops::set_flag_h(cpu_state, false);
    microops::set_flag_c(cpu_state, least_significant_bit == 0x01);
    rotated_value
}

fn rotate_right_through_carry(cpu_state: &mut CpuState, byte: u8) -> u8 {
    let c_flag = if microops::is_c_flag_set(cpu_state) { 0x1 } else { 0x0 };
    let least_significant_bit = byte & 0x1;
    let rotated_value = c_flag << 7 | byte >> 1;
    microops::set_flag_z(cpu_state, rotated_value == 0);
    microops::set_flag_n(cpu_state, false);
    microops::set_flag_h(cpu_state, false);
    microops::set_flag_c(cpu_state, least_significant_bit == 0x01);
    rotated_value
}

pub fn rotate_register_left(cpu_state: &mut CpuState, register: Register) {
    let byte = microops::read_from_register(cpu_state, &register);
    let rotated_value = rotate_left(cpu_state, byte);
    microops::store_in_register(cpu_state, register, rotated_value);
}

pub fn rotate_register_left_through_carry(cpu_state: &mut CpuState, register: Register) {
    let byte = microops::read_from_register(cpu_state, &register);
    let rotated_value = rotate_left_through_carry(cpu_state, byte);
    microops::store_in_register(cpu_state, register, rotated_value);
}

pub fn rotate_register_right(cpu_state: &mut CpuState, register: Register) {
    let byte = microops::read_from_register(cpu_state, &register);
    let rotated_value = rotate_right(cpu_state, byte);
    microops::store_in_register(cpu_state, register, rotated_value);
}

pub fn rotate_register_right_through_carry(cpu_state: &mut CpuState, register: Register) {
    let byte = microops::read_from_register(cpu_state, &register);
    let rotated_value = rotate_right_through_carry(cpu_state, byte);
    microops::store_in_register(cpu_state, register, rotated_value);
}

pub fn rotate_memory_byte_left(emulator: &mut Emulator) {
    let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
    let byte = microops::read_byte_from_memory(emulator, address);
    let rotated_value = rotate_left(&mut emulator.cpu, byte);
    microops::store_byte_in_memory(emulator, address, rotated_value);
}

pub fn rotate_memory_byte_left_through_carry(emulator: &mut Emulator) {
    let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
    let byte = microops::read_byte_from_memory(emulator, address);
    let rotated_value = rotate_left_through_carry(&mut emulator.cpu, byte);
    microops::store_byte_in_memory(emulator, address, rotated_value);
}

pub fn rotate_memory_byte_right(emulator: &mut Emulator) {
    let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
    let byte = microops::read_byte_from_memory(emulator, address);
    let rotated_value = rotate_right(&mut emulator.cpu, byte);
    microops::store_byte_in_memory(emulator, address, rotated_value);
}

pub fn rotate_memory_byte_right_through_carry(emulator: &mut Emulator) {
    let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
    let byte = microops::read_byte_from_memory(emulator, address);
    let rotated_value = rotate_right_through_carry(&mut emulator.cpu, byte);
    microops::store_byte_in_memory(emulator, address, rotated_value);
}

fn swap_nibbles(byte: u8) -> u8 {
    let first_nibble = (byte >> 4) & 0xF;
    let second_nibble = byte & 0xF;
    (second_nibble << 4) | first_nibble
}

pub fn swap_nibbles_in_register(cpu_state: &mut CpuState, register: Register) {
    let byte = microops::read_from_register(cpu_state, &register);
    microops::store_in_register(cpu_state, register, swap_nibbles(byte));
}

pub fn swap_nibbles_in_memory_byte(emulator: &mut Emulator, address: u16) {
    let byte = microops::read_byte_from_memory(emulator, address);
    microops::store_byte_in_memory(emulator, address, swap_nibbles(byte));
}

fn shift_left(cpu_state: &mut CpuState, byte: u8) -> u8 {
    let most_significant_bit = byte >> 7;
    let shifted_value = byte << 1;
    microops::set_flag_z(cpu_state, shifted_value == 0);
    microops::set_flag_n(cpu_state, false);
    microops::set_flag_h(cpu_state, false);
    microops::set_flag_c(cpu_state, most_significant_bit == 0x01);
    shifted_value
}

pub fn shift_register_left(cpu_state: &mut CpuState, register: Register) {
    let byte = microops::read_from_register(cpu_state, &register);
    let shifted_value = shift_left(cpu_state, byte);
    microops::store_in_register(cpu_state, register, shifted_value);
}

pub fn shift_memory_byte_left(emulator: &mut Emulator) {
    let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
    let byte = microops::read_byte_from_memory(emulator, address);
    let shifted_value = shift_left(&mut emulator.cpu, byte);
    microops::store_byte_in_memory(emulator, address, shifted_value);
}

fn shift_right(cpu_state: &mut CpuState, byte: u8, maintain_msb: bool) -> u8 {
    let least_significant_bit = byte & 0x1;
    let updated_most_significant_bit = if maintain_msb { byte & 0x80 } else { 0 };
    let shifted_value = byte >> 1 | updated_most_significant_bit;
    microops::set_flag_z(cpu_state, shifted_value == 0);
    microops::set_flag_n(cpu_state, false);
    microops::set_flag_h(cpu_state, false);
    microops::set_flag_c(cpu_state, least_significant_bit == 0x01);
    shifted_value
}

pub fn shift_register_right_maintaining_msb(cpu_state: &mut CpuState, register: Register) {
    let byte = microops::read_from_register(cpu_state, &register);
    let shifted_value = shift_right(cpu_state, byte, true);
    microops::store_in_register(cpu_state, register, shifted_value);
}

pub fn shift_memory_byte_right_maintaining_msb(emulator: &mut Emulator) {
    let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
    let byte = microops::read_byte_from_memory(emulator, address);
    let shifted_value = shift_right(&mut emulator.cpu, byte, true);
    microops::store_byte_in_memory(emulator, address, shifted_value);
}

pub fn shift_register_right(cpu_state: &mut CpuState, register: Register) {
    let byte = microops::read_from_register(cpu_state, &register);
    let shifted_value = shift_right(cpu_state, byte, false);
    microops::store_in_register(cpu_state, register, shifted_value);
}

pub fn shift_memory_byte_right(emulator: &mut Emulator) {
    let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
    let byte = microops::read_byte_from_memory(emulator, address);
    let shifted_value = shift_right(&mut emulator.cpu, byte, false);
    microops::store_byte_in_memory(emulator, address, shifted_value);
}

fn test_bit(cpu_state: &mut CpuState, byte: u8, bit_index: u8) {
    let mask = 1 << bit_index;
    let bit_is_set = (mask & byte) > 0;
    microops::set_flag_z(cpu_state, !bit_is_set);
    microops::set_flag_n(cpu_state, false);
    microops::set_flag_h(cpu_state, true);
}

pub fn test_register_bit(cpu_state: &mut CpuState, register: Register, bit_index: u8) {
    let byte = microops::read_from_register(cpu_state, &register);
    test_bit(cpu_state, byte, bit_index);
}

pub fn test_memory_bit(emulator: &mut Emulator, bit_index: u8) {
    let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
    let byte = microops::read_byte_from_memory(emulator, address);
    test_bit(&mut emulator.cpu, byte, bit_index);
}

fn reset_bit(byte: u8, bit_index: u8) -> u8 {
    let mask: u8 = !(1 << bit_index);
    byte & mask
}

pub fn reset_register_bit(cpu_state: &mut CpuState, register: Register, bit_index: u8) {
    let byte = microops::read_from_register(cpu_state, &register);
    let updated_byte = reset_bit(byte, bit_index);
    microops::store_in_register(cpu_state, register, updated_byte);
}

pub fn reset_memory_bit(emulator: &mut Emulator, bit_index: u8) {
    let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
    let byte = microops::read_byte_from_memory(emulator, address);
    let updated_byte = reset_bit(byte, bit_index);
    microops::store_byte_in_memory(emulator, address, updated_byte);
}

fn set_bit(byte: u8, bit_index: u8) -> u8 {
    let mask: u8 = 1 << bit_index;
    byte | mask
}

pub fn set_register_bit(cpu_state: &mut CpuState, register: Register, bit_index: u8) {
    let byte = microops::read_from_register(cpu_state, &register);
    let updated_byte = set_bit(byte, bit_index);
    microops::store_in_register(cpu_state, register, updated_byte);
}

pub fn set_memory_bit(emulator: &mut Emulator, bit_index: u8) {
    let cpu_state = &mut emulator.cpu;
    let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
    let byte = microops::read_byte_from_memory(emulator, address);
    let updated_byte = set_bit(byte, bit_index);
    microops::store_byte_in_memory(emulator, address, updated_byte);
}