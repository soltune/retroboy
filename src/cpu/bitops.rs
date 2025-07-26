use crate::cpu::{Register, Cpu, REGISTER_HL};
use crate::cpu::microops;
use crate::utils::{is_bit_set, set_bit, reset_bit};

fn rotate_left(cpu_state: &mut Cpu, byte: u8) -> u8 {
    let most_significant_bit = byte >> 7;
    let rotated_value = byte << 1 | most_significant_bit;
    microops::set_flag_z(cpu_state, rotated_value == 0);
    microops::set_flag_n(cpu_state, false);
    microops::set_flag_h(cpu_state, false);
    microops::set_flag_c(cpu_state, most_significant_bit == 0x01);
    rotated_value
}

fn rotate_left_through_carry(cpu_state: &mut Cpu, byte: u8) -> u8 {
    let c_flag = if microops::is_c_flag_set(cpu_state) { 0x1 } else { 0x0 };
    let most_significant_bit = byte >> 7;
    let rotated_value = byte << 1 | c_flag;
    microops::set_flag_z(cpu_state, rotated_value == 0);
    microops::set_flag_n(cpu_state, false);
    microops::set_flag_h(cpu_state, false);
    microops::set_flag_c(cpu_state, most_significant_bit == 0x01);
    rotated_value
}

fn rotate_right(cpu_state: &mut Cpu, byte: u8) -> u8 {
    let least_significant_bit = byte & 0x1;
    let rotated_value: u8 = least_significant_bit << 7 | byte >> 1;
    microops::set_flag_z(cpu_state, rotated_value == 0);
    microops::set_flag_n(cpu_state, false);
    microops::set_flag_h(cpu_state, false);
    microops::set_flag_c(cpu_state, least_significant_bit == 0x01);
    rotated_value
}

fn rotate_right_through_carry(cpu_state: &mut Cpu, byte: u8) -> u8 {
    let c_flag = if microops::is_c_flag_set(cpu_state) { 0x1 } else { 0x0 };
    let least_significant_bit = byte & 0x1;
    let rotated_value = c_flag << 7 | byte >> 1;
    microops::set_flag_z(cpu_state, rotated_value == 0);
    microops::set_flag_n(cpu_state, false);
    microops::set_flag_h(cpu_state, false);
    microops::set_flag_c(cpu_state, least_significant_bit == 0x01);
    rotated_value
}

pub fn rotate_register_left(cpu_state: &mut Cpu, register: Register) {
    let byte = microops::read_from_register(cpu_state, &register);
    let rotated_value = rotate_left(cpu_state, byte);
    microops::store_in_register(cpu_state, register, rotated_value);
}

pub fn rotate_register_left_through_carry(cpu_state: &mut Cpu, register: Register) {
    let byte = microops::read_from_register(cpu_state, &register);
    let rotated_value = rotate_left_through_carry(cpu_state, byte);
    microops::store_in_register(cpu_state, register, rotated_value);
}

pub fn rotate_register_right(cpu_state: &mut Cpu, register: Register) {
    let byte = microops::read_from_register(cpu_state, &register);
    let rotated_value = rotate_right(cpu_state, byte);
    microops::store_in_register(cpu_state, register, rotated_value);
}

pub fn rotate_register_right_through_carry(cpu_state: &mut Cpu, register: Register) {
    let byte = microops::read_from_register(cpu_state, &register);
    let rotated_value = rotate_right_through_carry(cpu_state, byte);
    microops::store_in_register(cpu_state, register, rotated_value);
}

pub fn rotate_memory_byte_left(cpu_state: &mut Cpu) {
    let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
    let byte = microops::read_byte_from_memory(cpu_state, address);
    let rotated_value = rotate_left(cpu_state, byte);
    microops::store_byte_in_memory(cpu_state, address, rotated_value);
}

pub fn rotate_memory_byte_left_through_carry(cpu_state: &mut Cpu) {
    let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
    let byte = microops::read_byte_from_memory(cpu_state, address);
    let rotated_value = rotate_left_through_carry(cpu_state, byte);
    microops::store_byte_in_memory(cpu_state, address, rotated_value);
}

pub fn rotate_memory_byte_right(cpu_state: &mut Cpu) {
    let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
    let byte = microops::read_byte_from_memory(cpu_state, address);
    let rotated_value = rotate_right(cpu_state, byte);
    microops::store_byte_in_memory(cpu_state, address, rotated_value);
}

pub fn rotate_memory_byte_right_through_carry(cpu_state: &mut Cpu) {
    let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
    let byte = microops::read_byte_from_memory(cpu_state, address);
    let rotated_value = rotate_right_through_carry(cpu_state, byte);
    microops::store_byte_in_memory(cpu_state, address, rotated_value);
}

fn swap_nibbles(byte: u8) -> u8 {
    let first_nibble = (byte >> 4) & 0xF;
    let second_nibble = byte & 0xF;
    (second_nibble << 4) | first_nibble
}

pub fn swap_nibbles_in_register(cpu_state: &mut Cpu, register: Register) {
    let byte = microops::read_from_register(cpu_state, &register);
    let with_swapped_nibbles = swap_nibbles(byte);

    microops::store_in_register(cpu_state, register, with_swapped_nibbles);

    microops::set_flag_z(cpu_state, with_swapped_nibbles == 0);
    microops::set_flag_n(cpu_state, false);
    microops::set_flag_h(cpu_state, false);
    microops::set_flag_c(cpu_state, false); 
}

pub fn swap_nibbles_in_memory_byte(cpu_state: &mut Cpu, address: u16) {
    let byte = microops::read_byte_from_memory(cpu_state, address);
    let with_swapped_nibbles = swap_nibbles(byte);

    microops::store_byte_in_memory(cpu_state, address, with_swapped_nibbles);

    microops::set_flag_z(cpu_state, with_swapped_nibbles == 0);
    microops::set_flag_n(cpu_state, false);
    microops::set_flag_h(cpu_state, false);
    microops::set_flag_c(cpu_state, false);
}

fn shift_left(cpu_state: &mut Cpu, byte: u8) -> u8 {
    let most_significant_bit = byte >> 7;
    let shifted_value = byte << 1;
    microops::set_flag_z(cpu_state, shifted_value == 0);
    microops::set_flag_n(cpu_state, false);
    microops::set_flag_h(cpu_state, false);
    microops::set_flag_c(cpu_state, most_significant_bit == 0x01);
    shifted_value
}

pub fn shift_register_left(cpu_state: &mut Cpu, register: Register) {
    let byte = microops::read_from_register(cpu_state, &register);
    let shifted_value = shift_left(cpu_state, byte);
    microops::store_in_register(cpu_state, register, shifted_value);
}

pub fn shift_memory_byte_left(cpu_state: &mut Cpu) {
    let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
    let byte = microops::read_byte_from_memory(cpu_state, address);
    let shifted_value = shift_left(cpu_state, byte);
    microops::store_byte_in_memory(cpu_state, address, shifted_value);
}

fn shift_right(cpu_state: &mut Cpu, byte: u8, maintain_msb: bool) -> u8 {
    let least_significant_bit = byte & 0x1;
    let updated_most_significant_bit = if maintain_msb { byte & 0x80 } else { 0 };
    let shifted_value = byte >> 1 | updated_most_significant_bit;
    microops::set_flag_z(cpu_state, shifted_value == 0);
    microops::set_flag_n(cpu_state, false);
    microops::set_flag_h(cpu_state, false);
    microops::set_flag_c(cpu_state, least_significant_bit == 0x01);
    shifted_value
}

pub fn shift_register_right_maintaining_msb(cpu_state: &mut Cpu, register: Register) {
    let byte = microops::read_from_register(cpu_state, &register);
    let shifted_value = shift_right(cpu_state, byte, true);
    microops::store_in_register(cpu_state, register, shifted_value);
}

pub fn shift_memory_byte_right_maintaining_msb(cpu_state: &mut Cpu) {
    let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
    let byte = microops::read_byte_from_memory(cpu_state, address);
    let shifted_value = shift_right(cpu_state, byte, true);
    microops::store_byte_in_memory(cpu_state, address, shifted_value);
}

pub fn shift_register_right(cpu_state: &mut Cpu, register: Register) {
    let byte = microops::read_from_register(cpu_state, &register);
    let shifted_value = shift_right(cpu_state, byte, false);
    microops::store_in_register(cpu_state, register, shifted_value);
}

pub fn shift_memory_byte_right(cpu_state: &mut Cpu) {
    let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
    let byte = microops::read_byte_from_memory(cpu_state, address);
    let shifted_value = shift_right(cpu_state, byte, false);
    microops::store_byte_in_memory(cpu_state, address, shifted_value);
}

fn test_bit(cpu_state: &mut Cpu, byte: u8, bit_index: u8) {
    microops::set_flag_z(cpu_state, !is_bit_set(byte, bit_index));
    microops::set_flag_n(cpu_state, false);
    microops::set_flag_h(cpu_state, true);
}

pub fn test_register_bit(cpu_state: &mut Cpu, register: Register, bit_index: u8) {
    let byte = microops::read_from_register(cpu_state, &register);
    test_bit(cpu_state, byte, bit_index);
}

pub fn test_memory_bit(cpu_state: &mut Cpu, bit_index: u8) {
    let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
    let byte = microops::read_byte_from_memory(cpu_state, address);
    test_bit(cpu_state, byte, bit_index);
}

pub fn reset_register_bit(cpu_state: &mut Cpu, register: Register, bit_index: u8) {
    let byte = microops::read_from_register(cpu_state, &register);
    let updated_byte = reset_bit(byte, bit_index);
    microops::store_in_register(cpu_state, register, updated_byte);
}

pub fn reset_memory_bit(cpu_state: &mut Cpu, bit_index: u8) {
    let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
    let byte = microops::read_byte_from_memory(cpu_state, address);
    let updated_byte = reset_bit(byte, bit_index);
    microops::store_byte_in_memory(cpu_state, address, updated_byte);
}

pub fn set_register_bit(cpu_state: &mut Cpu, register: Register, bit_index: u8) {
    let byte = microops::read_from_register(cpu_state, &register);
    let updated_byte = set_bit(byte, bit_index);
    microops::store_in_register(cpu_state, register, updated_byte);
}

pub fn set_memory_bit(cpu_state: &mut Cpu, bit_index: u8) {
    let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
    let byte = microops::read_byte_from_memory(cpu_state, address);
    let updated_byte = set_bit(byte, bit_index);
    microops::store_byte_in_memory(cpu_state, address, updated_byte);
}