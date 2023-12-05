use crate::cpu::{Register, RegisterPair, CpuState, REGISTER_AF, REGISTER_BC, REGISTER_DE, REGISTER_HL};
use crate::cpu::microops;
use crate::cpu::alu;

use super::alu::{increment_memory_byte, decrement_memory_byte, increment_register_pair, decrement_register_pair};

fn read_next_instruction_byte(cpu_state: &mut CpuState) -> u8 {
    let byte = microops::read_byte_from_memory(cpu_state, cpu_state.registers.program_counter);
    cpu_state.registers.program_counter += 1;
    byte
}

fn read_next_instruction_word(cpu_state: &mut CpuState) -> u16 {
    let word = microops::read_word_from_memory(cpu_state, cpu_state.registers.program_counter);
    cpu_state.registers.program_counter += 2;
    word
}

fn load_immediate_value(cpu_state: &mut CpuState, register: Register) {
    let immediate_byte = read_next_instruction_byte(cpu_state);
    microops::store_in_register(cpu_state, register, immediate_byte);
}

fn load_source_register_in_destination_register(cpu_state: &mut CpuState, source: Register, destination: Register) {
    let source_value = microops::read_from_register(cpu_state, &source);
    microops::store_in_register(cpu_state, destination, source_value);
}

fn load_memory_byte_in_destination_register(cpu_state: &mut CpuState, address: u16, destination: Register) {
    let byte = microops::read_byte_from_memory(cpu_state, address);
    microops::store_in_register(cpu_state, destination, byte);
}

fn load_source_register_in_memory(cpu_state: &mut CpuState, source: Register, address: u16) {
    let byte = microops::read_from_register(cpu_state, &source);
    microops::store_byte_in_memory(cpu_state, address, byte);
}

fn load_immediate_value_in_memory(cpu_state: &mut CpuState, register_pair: RegisterPair) {
    let address = microops::read_from_register_pair(cpu_state, &register_pair);
    let immediate_byte = read_next_instruction_byte(cpu_state);
    microops::store_byte_in_memory(cpu_state, address, immediate_byte);
}

fn push_register_pair_to_stack(cpu_state: &mut CpuState, register_pair: RegisterPair) {
    let word = microops::read_from_register_pair(cpu_state, &register_pair);
    cpu_state.registers.stack_pointer = cpu_state.registers.stack_pointer - 1;
    microops::store_byte_in_memory(cpu_state, cpu_state.registers.stack_pointer, (word >> 8) as u8);
    cpu_state.registers.stack_pointer = cpu_state.registers.stack_pointer - 1;
    microops::store_byte_in_memory(cpu_state, cpu_state.registers.stack_pointer, (word & 0xFF) as u8);
    microops::run_extra_machine_cycle(cpu_state);
}

fn pop_word_into_register_pair_from_stack(cpu_state: &mut CpuState, register_pair: RegisterPair) {
    let first_byte = microops::read_byte_from_memory(cpu_state, cpu_state.registers.stack_pointer) as u16;
    cpu_state.registers.stack_pointer = cpu_state.registers.stack_pointer + 1;
    let second_byte = microops::read_byte_from_memory(cpu_state, cpu_state.registers.stack_pointer) as u16;
    cpu_state.registers.stack_pointer = cpu_state.registers.stack_pointer + 1;
    let word = (second_byte << 8) + first_byte;
    microops::store_in_register_pair(cpu_state, register_pair, word);
}

fn handle_illegal_opcode(opcode: u8) {
    panic!("Encountered illegal opcode {:#04X}", opcode);
}

fn swap_nibbles(byte: u8) -> u8 {
    let first_nibble = (byte >> 4) & 0xF;
    let second_nibble = byte & 0xF;
    (second_nibble << 4) | first_nibble
}

fn swap_nibbles_in_register(cpu_state: &mut CpuState, register: Register) {
    let byte = microops::read_from_register(cpu_state, &register);
    microops::store_in_register(cpu_state, register, swap_nibbles(byte));
}

fn swap_nibbles_in_memory_byte(cpu_state: &mut CpuState, address: u16) {
    let byte = microops::read_byte_from_memory(cpu_state, address);
    microops::store_byte_in_memory(cpu_state, address, swap_nibbles(byte));
}

pub fn execute_opcode(cpu_state: &mut CpuState) {
    let opcode = read_next_instruction_byte(cpu_state);
    match opcode {
        0x00 =>
            (),
        0x01 => {
            let word = read_next_instruction_word(cpu_state);
            microops::store_in_register_pair(cpu_state, REGISTER_BC, word);
        },
        0x02 => {
            let address = microops::read_from_register_pair(cpu_state, &REGISTER_BC);
            load_source_register_in_memory(cpu_state, Register::A, address);
        },
        0x03 =>
            increment_register_pair(cpu_state, REGISTER_BC),
        0x04 =>
            alu::increment_register(cpu_state, Register::B),
        0x05 =>
            alu::decrement_register(cpu_state, Register::B),
        0x06 =>
            load_immediate_value(cpu_state, Register::B),
        0x08 => {
            let address = read_next_instruction_word(cpu_state);
            microops::store_word_in_memory(cpu_state, address, cpu_state.registers.stack_pointer);
        },
        0x09 => {
            let word = microops::read_from_register_pair(cpu_state, &REGISTER_BC);
            alu::add_value_to_register_pair(cpu_state, REGISTER_HL, word);
        },
        0x0A => {
            let address = microops::read_from_register_pair(cpu_state, &REGISTER_BC);
            load_memory_byte_in_destination_register(cpu_state, address, Register::A);
        },
        0x0B =>
            decrement_register_pair(cpu_state, REGISTER_BC),
        0x0C =>
            alu::increment_register(cpu_state, Register::C),
        0x0D =>
            alu::decrement_register(cpu_state, Register::C),
        0x0E =>
            load_immediate_value(cpu_state, Register::C),
        0x11 => {
            let word = read_next_instruction_word(cpu_state);
            microops::store_in_register_pair(cpu_state, REGISTER_DE, word);
        },
        0x12 => {
            let address = microops::read_from_register_pair(cpu_state, &REGISTER_DE);
            load_source_register_in_memory(cpu_state, Register::A, address);
        },
        0x13 =>
            increment_register_pair(cpu_state, REGISTER_DE),
        0x14 =>
            alu::increment_register(cpu_state, Register::D),
        0x15 =>
            alu::decrement_register(cpu_state, Register::D),
        0x16 =>
            load_immediate_value(cpu_state, Register::D),
        0x1A => {
            let address = microops::read_from_register_pair(cpu_state,&REGISTER_DE);
            load_memory_byte_in_destination_register(cpu_state, address, Register::A)
        },
        0x1B =>
            decrement_register_pair(cpu_state, REGISTER_DE),
        0x19 => {
            let word = microops::read_from_register_pair(cpu_state, &REGISTER_DE);
            alu::add_value_to_register_pair(cpu_state, REGISTER_HL, word);
        },
        0x1C =>
            alu::increment_register(cpu_state, Register::E),
        0x1D =>
            alu::decrement_register(cpu_state, Register::E),
        0x1E =>
            load_immediate_value(cpu_state, Register::E),
        0x21 => {
            let word = read_next_instruction_word(cpu_state);
            microops::store_in_register_pair(cpu_state, REGISTER_HL, word);
        },
        0x22 => {
            let mut address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
            load_source_register_in_memory(cpu_state, Register::A, address);
            address += 1;
            microops::store_in_register_pair(cpu_state, REGISTER_HL, address);    
        },
        0x23 =>
            increment_register_pair(cpu_state, REGISTER_HL),
        0x24 =>
            alu::increment_register(cpu_state, Register::H),
        0x25 =>
            alu::decrement_register(cpu_state, Register::H),
        0x26 =>
            load_immediate_value(cpu_state, Register::H),
        0x27 => {
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
        },
        0x29 => {
            let word = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
            alu::add_value_to_register_pair(cpu_state, REGISTER_HL, word);
        },
        0x2A => {
            let mut address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
            load_memory_byte_in_destination_register(cpu_state, address, Register::A);
            address += 1;
            microops::store_in_register_pair(cpu_state, REGISTER_HL, address);  
        },
        0x2B =>
            decrement_register_pair(cpu_state, REGISTER_HL),
        0x2C =>
            alu::increment_register(cpu_state, Register::L),
        0x2D =>
            alu::decrement_register(cpu_state, Register::L),
        0x2E =>
            load_immediate_value(cpu_state, Register::L),
        0x2F => {
            cpu_state.registers.a = cpu_state.registers.a ^ 0xFF;
            microops::set_flag_n(cpu_state, true);
            microops::set_flag_h(cpu_state, true);
        },
        0x31 => {
            let word = read_next_instruction_word(cpu_state);
            cpu_state.registers.stack_pointer = word;            
        },
        0x32 => {
            let mut address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
            load_source_register_in_memory(cpu_state, Register::A, address);
            address -= 1;
            microops::store_in_register_pair(cpu_state, REGISTER_HL, address);           
        },
        0x33 => {
            cpu_state.registers.stack_pointer = cpu_state.registers.stack_pointer.wrapping_add(1);
            microops::run_extra_machine_cycle(cpu_state);
        },
        0x34 =>
            increment_memory_byte(cpu_state),
        0x35 =>
            decrement_memory_byte(cpu_state),
        0x36 =>
            load_immediate_value_in_memory(cpu_state, REGISTER_HL),
        0x37 => {
            microops::set_flag_c(cpu_state, true);
            microops::set_flag_h(cpu_state, false);
            microops::set_flag_n(cpu_state, false);
        },
        0x39 =>
            alu::add_value_to_register_pair(cpu_state, REGISTER_HL, cpu_state.registers.stack_pointer),
        0x3A => {
            let mut address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
            load_memory_byte_in_destination_register(cpu_state, address, Register::A);
            address -= 1;
            microops::store_in_register_pair(cpu_state, REGISTER_HL, address);
        },
        0x3B => {
            cpu_state.registers.stack_pointer = cpu_state.registers.stack_pointer.wrapping_sub(1);
            microops::run_extra_machine_cycle(cpu_state);
        },
        0x3C =>
            alu::increment_register(cpu_state, Register::A),
        0x3D =>
            alu::decrement_register(cpu_state, Register::A),
        0x3E =>
            load_immediate_value(cpu_state, Register::A),
        0x3F => {
            let c_flag_set = microops::is_c_flag_set(cpu_state);
            microops::set_flag_c(cpu_state, !c_flag_set);
            microops::set_flag_n(cpu_state, false);
            microops::set_flag_h(cpu_state, false);
        },
        0x40 =>
            load_source_register_in_destination_register(cpu_state, Register::B, Register::B),
        0x41 =>
            load_source_register_in_destination_register(cpu_state, Register::C, Register::B),
        0x42 =>
            load_source_register_in_destination_register(cpu_state, Register::D, Register::B),
        0x43 =>
            load_source_register_in_destination_register(cpu_state, Register::E, Register::B),
        0x44 =>
            load_source_register_in_destination_register(cpu_state, Register::H, Register::B),
        0x45 =>
            load_source_register_in_destination_register(cpu_state, Register::L, Register::B),
        0x46 => {
            let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
            load_memory_byte_in_destination_register(cpu_state, address, Register::B)
        },
        0x47 =>
            load_source_register_in_destination_register(cpu_state, Register::A, Register::B),
        0x48 =>
            load_source_register_in_destination_register(cpu_state, Register::B, Register::C),
        0x49 =>
            load_source_register_in_destination_register(cpu_state, Register::C, Register::C),
        0x4A =>
            load_source_register_in_destination_register(cpu_state, Register::D, Register::C),
        0x4B =>
            load_source_register_in_destination_register(cpu_state, Register::E, Register::C),
        0x4C =>
            load_source_register_in_destination_register(cpu_state, Register::H, Register::C),
        0x4D =>
            load_source_register_in_destination_register(cpu_state, Register::L, Register::C),
        0x4E => {
            let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
            load_memory_byte_in_destination_register(cpu_state, address, Register::C)
        },
        0x4F =>
            load_source_register_in_destination_register(cpu_state, Register::A, Register::C),
        0x50 => 
            load_source_register_in_destination_register(cpu_state, Register::B, Register::D),
        0x51 =>
            load_source_register_in_destination_register(cpu_state, Register::C, Register::D),
        0x52 =>
            load_source_register_in_destination_register(cpu_state, Register::D, Register::D),
        0x53 =>
            load_source_register_in_destination_register(cpu_state, Register::E, Register::D),
        0x54 =>
            load_source_register_in_destination_register(cpu_state, Register::H, Register::D),
        0x55 =>
            load_source_register_in_destination_register(cpu_state, Register::L, Register::D),
        0x56 => {
            let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
            load_memory_byte_in_destination_register(cpu_state, address, Register::D)
        },
        0x57 =>
            load_source_register_in_destination_register(cpu_state, Register::A, Register::D),
        0x58 =>
            load_source_register_in_destination_register(cpu_state, Register::B, Register::E),
        0x59 =>
            load_source_register_in_destination_register(cpu_state, Register::C, Register::E),
        0x5A =>
            load_source_register_in_destination_register(cpu_state, Register::D, Register::E),
        0x5B =>
            load_source_register_in_destination_register(cpu_state, Register::E, Register::E),
        0x5C =>
            load_source_register_in_destination_register(cpu_state, Register::H, Register::E),
        0x5D =>
            load_source_register_in_destination_register(cpu_state, Register::L, Register::E),
        0x5E => {
            let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
            load_memory_byte_in_destination_register(cpu_state, address, Register::E)
        },
        0x5F =>
            load_source_register_in_destination_register(cpu_state, Register::A, Register::E),
        0x60 =>
            load_source_register_in_destination_register(cpu_state, Register::B, Register::H),
        0x61 =>
            load_source_register_in_destination_register(cpu_state, Register::C, Register::H),
        0x62 =>
            load_source_register_in_destination_register(cpu_state, Register::D, Register::H),
        0x63 =>
            load_source_register_in_destination_register(cpu_state, Register::E, Register::H),
        0x64 =>
            load_source_register_in_destination_register(cpu_state, Register::H, Register::H),
        0x65 =>
            load_source_register_in_destination_register(cpu_state, Register::L, Register::H),
        0x66 => {
            let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
            load_memory_byte_in_destination_register(cpu_state, address, Register::H)
        },
        0x67 =>
            load_source_register_in_destination_register(cpu_state, Register::A, Register::H),
        0x68 =>
            load_source_register_in_destination_register(cpu_state, Register::B, Register::L),
        0x69 =>
            load_source_register_in_destination_register(cpu_state, Register::C, Register::L),
        0x6A =>
            load_source_register_in_destination_register(cpu_state, Register::D, Register::L),
        0x6B =>
            load_source_register_in_destination_register(cpu_state, Register::E, Register::L),
        0x6C =>
            load_source_register_in_destination_register(cpu_state, Register::H, Register::L),
        0x6D =>
            load_source_register_in_destination_register(cpu_state, Register::L, Register::L),
        0x6E => {
            let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
            load_memory_byte_in_destination_register(cpu_state, address, Register::L)
        },
        0x6F =>
            load_source_register_in_destination_register(cpu_state, Register::A, Register::L),
        0x70 => {
            let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
            load_source_register_in_memory(cpu_state, Register::B, address);
        },
        0x71 => {
            let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
            load_source_register_in_memory(cpu_state, Register::C, address);
        },
        0x72 => {
            let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
            load_source_register_in_memory(cpu_state, Register::D, address);
        },
        0x73 => {
            let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
            load_source_register_in_memory(cpu_state, Register::E, address);
        },
        0x74 => {
            let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
            load_source_register_in_memory(cpu_state, Register::H, address);
        },
        0x75 => {
            let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
            load_source_register_in_memory(cpu_state, Register::L, address);
        },
        0x77 => {
            let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
            load_source_register_in_memory(cpu_state, Register::A, address);
        },
        0x78 =>
            load_source_register_in_destination_register(cpu_state, Register::B, Register::A),
        0x79 =>
            load_source_register_in_destination_register(cpu_state, Register::C, Register::A),
        0x7A =>
            load_source_register_in_destination_register(cpu_state, Register::D, Register::A),
        0x7B =>
            load_source_register_in_destination_register(cpu_state, Register::E, Register::A),
        0x7C =>
            load_source_register_in_destination_register(cpu_state, Register::H, Register::A),
        0x7D =>
            load_source_register_in_destination_register(cpu_state, Register::L, Register::A),
        0x7E => {
            let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
            load_memory_byte_in_destination_register(cpu_state, address, Register::A)
        },
        0x7F =>
            load_source_register_in_destination_register(cpu_state, Register::A, Register::A),
        0x80 => {
            let value = microops::read_from_register(cpu_state, &Register::B);
            alu::add_value_to_register(cpu_state, Register::A, value);
        },
        0x81 => {
            let value = microops::read_from_register(cpu_state, &Register::C);
            alu::add_value_to_register(cpu_state, Register::A, value);
        },
        0x82 => {
            let value = microops::read_from_register(cpu_state, &Register::D);
            alu::add_value_to_register(cpu_state, Register::A, value);
        },
        0x83 => {
            let value = microops::read_from_register(cpu_state, &Register::E);
            alu::add_value_to_register(cpu_state, Register::A, value);
        },
        0x84 => {
            let value = microops::read_from_register(cpu_state, &Register::H);
            alu::add_value_to_register(cpu_state, Register::A, value);
        },
        0x85 => {
            let value = microops::read_from_register(cpu_state, &Register::L);
            alu::add_value_to_register(cpu_state, Register::A, value);
        },
        0x86 => {
            let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
            let value = microops::read_byte_from_memory(cpu_state, address);
            alu::add_value_to_register(cpu_state, Register::A, value);
        },
        0x87 => {
            let value = microops::read_from_register(cpu_state, &Register::A);
            alu::add_value_to_register(cpu_state, Register::A, value);
        },
        0x88 => {
            let value = microops::read_from_register(cpu_state, &Register::B);
            alu::add_value_and_carry_to_register(cpu_state, Register::A, value);
        },
        0x89 => {
            let value = microops::read_from_register(cpu_state, &Register::C);
            alu::add_value_and_carry_to_register(cpu_state, Register::A, value);
        },
        0x8A => {
            let value = microops::read_from_register(cpu_state, &Register::D);
            alu::add_value_and_carry_to_register(cpu_state, Register::A, value);
        },
        0x8B => {
            let value = microops::read_from_register(cpu_state, &Register::E);
            alu::add_value_and_carry_to_register(cpu_state, Register::A, value);
        },
        0x8C => {
            let value = microops::read_from_register(cpu_state, &Register::H);
            alu::add_value_and_carry_to_register(cpu_state, Register::A, value);
        },
        0x8D => {
            let value = microops::read_from_register(cpu_state, &Register::L);
            alu::add_value_and_carry_to_register(cpu_state, Register::A, value);
        },
        0x8E => {
            let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
            let value = microops::read_byte_from_memory(cpu_state, address);
            alu::add_value_and_carry_to_register(cpu_state, Register::A, value);
        },
        0x8F => {
            let value = microops::read_from_register(cpu_state, &Register::A);
            alu::add_value_and_carry_to_register(cpu_state, Register::A, value);
        },
        0x90 => {
            let value = microops::read_from_register(cpu_state, &Register::B);
            alu::subtract_value_from_register(cpu_state, Register::A, value);
        },
        0x91 => {
            let value = microops::read_from_register(cpu_state, &Register::C);
            alu::subtract_value_from_register(cpu_state, Register::A, value);
        },
        0x92 => {
            let value = microops::read_from_register(cpu_state, &Register::D);
            alu::subtract_value_from_register(cpu_state, Register::A, value);
        },
        0x93 => {
            let value = microops::read_from_register(cpu_state, &Register::E);
            alu::subtract_value_from_register(cpu_state, Register::A, value);
        },
        0x94 => {
            let value = microops::read_from_register(cpu_state, &Register::H);
            alu::subtract_value_from_register(cpu_state, Register::A, value);
        },
        0x95 => {
            let value = microops::read_from_register(cpu_state, &Register::L);
            alu::subtract_value_from_register(cpu_state, Register::A, value);
        },
        0x96 => {
            let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
            let value = microops::read_byte_from_memory(cpu_state, address);
            alu::subtract_value_from_register(cpu_state, Register::A, value);
        },
        0x97 => {
            let value = microops::read_from_register(cpu_state, &Register::A);
            alu::subtract_value_from_register(cpu_state, Register::A, value);
        },
        0x98 => {
            let value = microops::read_from_register(cpu_state, &Register::B);
            alu::subtract_value_and_carry_from_register(cpu_state, Register::A, value);
        },
        0x99 => {
            let value = microops::read_from_register(cpu_state, &Register::C);
            alu::subtract_value_and_carry_from_register(cpu_state, Register::A, value);
        },
        0x9A => {
            let value = microops::read_from_register(cpu_state, &Register::D);
            alu::subtract_value_and_carry_from_register(cpu_state, Register::A, value);
        },
        0x9B => {
            let value = microops::read_from_register(cpu_state, &Register::E);
            alu::subtract_value_and_carry_from_register(cpu_state, Register::A, value);
        },
        0x9C => {
            let value = microops::read_from_register(cpu_state, &Register::H);
            alu::subtract_value_and_carry_from_register(cpu_state, Register::A, value);
        },
        0x9D => {
            let value = microops::read_from_register(cpu_state, &Register::L);
            alu::subtract_value_and_carry_from_register(cpu_state, Register::A, value);
        },
        0x9E => {
            let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
            let value = microops::read_byte_from_memory(cpu_state, address);
            alu::subtract_value_and_carry_from_register(cpu_state, Register::A, value);
        },
        0x9F => {
            let value = microops::read_from_register(cpu_state, &Register::A);
            alu::subtract_value_and_carry_from_register(cpu_state, Register::A, value);
        },
        0xA0 => {
            let value = microops::read_from_register(cpu_state, &Register::B);
            alu::logical_and_with_register(cpu_state, Register::A, value);
        },
        0xA1 => {
            let value = microops::read_from_register(cpu_state, &Register::C);
            alu::logical_and_with_register(cpu_state, Register::A, value);
        },
        0xA2 => {
            let value = microops::read_from_register(cpu_state, &Register::D);
            alu::logical_and_with_register(cpu_state, Register::A, value);
        },
        0xA3 => {
            let value = microops::read_from_register(cpu_state, &Register::E);
            alu::logical_and_with_register(cpu_state, Register::A, value);
        },
        0xA4 => {
            let value = microops::read_from_register(cpu_state, &Register::H);
            alu::logical_and_with_register(cpu_state, Register::A, value);
        },
        0xA5 => {
            let value = microops::read_from_register(cpu_state, &Register::L);
            alu::logical_and_with_register(cpu_state, Register::A, value);
        },
        0xA6 => {
            let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
            let value = microops::read_byte_from_memory(cpu_state, address);
            alu::logical_and_with_register(cpu_state, Register::A, value);
        },
        0xA7 => {
            let value = microops::read_from_register(cpu_state, &Register::A);
            alu::logical_and_with_register(cpu_state, Register::A, value);
        },
        0xA8 => {
            let value = microops::read_from_register(cpu_state, &Register::B);
            alu::logical_xor_with_register(cpu_state, Register::A, value);
        },
        0xA9 => {
            let value = microops::read_from_register(cpu_state, &Register::C);
            alu::logical_xor_with_register(cpu_state, Register::A, value);
        },
        0xAA => {
            let value = microops::read_from_register(cpu_state, &Register::D);
            alu::logical_xor_with_register(cpu_state, Register::A, value);
        },
        0xAB => {
            let value = microops::read_from_register(cpu_state, &Register::E);
            alu::logical_xor_with_register(cpu_state, Register::A, value);
        },
        0xAC => {
            let value = microops::read_from_register(cpu_state, &Register::H);
            alu::logical_xor_with_register(cpu_state, Register::A, value);
        },
        0xAD => {
            let value = microops::read_from_register(cpu_state, &Register::L);
            alu::logical_xor_with_register(cpu_state, Register::A, value);
        },
        0xAE => {
            let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
            let value = microops::read_byte_from_memory(cpu_state, address);
            alu::logical_xor_with_register(cpu_state, Register::A, value);
        },
        0xAF => {
            let value = microops::read_from_register(cpu_state, &Register::A);
            alu::logical_xor_with_register(cpu_state, Register::A, value);
        },
        0xB0 => {
            let value = microops::read_from_register(cpu_state, &Register::B);
            alu::logical_or_with_register(cpu_state, Register::A, value);
        },
        0xB1 => {
            let value = microops::read_from_register(cpu_state, &Register::C);
            alu::logical_or_with_register(cpu_state, Register::A, value);
        },
        0xB2 => {
            let value = microops::read_from_register(cpu_state, &Register::D);
            alu::logical_or_with_register(cpu_state, Register::A, value);
        },
        0xB3 => {
            let value = microops::read_from_register(cpu_state, &Register::E);
            alu::logical_or_with_register(cpu_state, Register::A, value);
        },
        0xB4 => {
            let value = microops::read_from_register(cpu_state, &Register::H);
            alu::logical_or_with_register(cpu_state, Register::A, value);
        },
        0xB5 => {
            let value = microops::read_from_register(cpu_state, &Register::L);
            alu::logical_or_with_register(cpu_state, Register::A, value);
        },
        0xB6 => {
            let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
            let value = microops::read_byte_from_memory(cpu_state, address);
            alu::logical_or_with_register(cpu_state, Register::A, value);
        },
        0xB7 => {
            let value = microops::read_from_register(cpu_state, &Register::A);
            alu::logical_or_with_register(cpu_state, Register::A, value);
        },
        0xB8 => {
            let value = microops::read_from_register(cpu_state, &Register::B);
            alu::compare_value_with_register(cpu_state, Register::A, value);
        },
        0xB9 => {
            let value = microops::read_from_register(cpu_state, &Register::C);
            alu::compare_value_with_register(cpu_state, Register::A, value);
        },
        0xBA => {
            let value = microops::read_from_register(cpu_state, &Register::D);
            alu::compare_value_with_register(cpu_state, Register::A, value);
        },
        0xBB => {
            let value = microops::read_from_register(cpu_state, &Register::E);
            alu::compare_value_with_register(cpu_state, Register::A, value);
        },
        0xBC => {
            let value = microops::read_from_register(cpu_state, &Register::H);
            alu::compare_value_with_register(cpu_state, Register::A, value);
        },
        0xBD => {
            let value = microops::read_from_register(cpu_state, &Register::L);
            alu::compare_value_with_register(cpu_state, Register::A, value);
        },
        0xBE => {
            let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
            let value = microops::read_byte_from_memory(cpu_state, address);
            alu::compare_value_with_register(cpu_state, Register::A, value);
        },
        0xBF => {
            let value = microops::read_from_register(cpu_state, &Register::A);
            alu::compare_value_with_register(cpu_state, Register::A, value);
        },
        0xC1 =>
            pop_word_into_register_pair_from_stack(cpu_state, REGISTER_BC),
        0xC5 =>
            push_register_pair_to_stack(cpu_state, REGISTER_BC),
        0xC6 => {
            let value = read_next_instruction_byte(cpu_state);
            alu::add_value_to_register(cpu_state, Register::A, value);
        },
        0xCB =>
            execute_cb_opcode(cpu_state),
        0xCE => {
            let value = read_next_instruction_byte(cpu_state);
            alu::add_value_and_carry_to_register(cpu_state, Register::A, value);
        },
        0xD1 =>
            pop_word_into_register_pair_from_stack(cpu_state, REGISTER_DE),
        0xD3 =>
            handle_illegal_opcode(opcode),
        0xD5 =>
            push_register_pair_to_stack(cpu_state, REGISTER_DE),
        0xD6 => {
            let value = read_next_instruction_byte(cpu_state);
            alu::subtract_value_from_register(cpu_state, Register::A, value);
        },
        0xDB =>
            handle_illegal_opcode(opcode),
        0xE0 => {
            let address = 0xFF00 + read_next_instruction_byte(cpu_state) as u16;
            load_source_register_in_memory(cpu_state, Register::A, address);
        },
        0xE1 =>
            pop_word_into_register_pair_from_stack(cpu_state, REGISTER_HL),
        0xE2 => {
            let address = 0xFF00 + microops::read_from_register(cpu_state, &Register::C) as u16;
            load_source_register_in_memory(cpu_state, Register::A, address);
        },
        0xE3 =>
            handle_illegal_opcode(opcode),
        0xE4 =>
            handle_illegal_opcode(opcode),
        0xE5 =>
            push_register_pair_to_stack(cpu_state, REGISTER_HL),
        0xE6 => {
            let value = read_next_instruction_byte(cpu_state);
            alu::logical_and_with_register(cpu_state, Register::A, value);
        },
        0xE8 => {
            let signed_byte = read_next_instruction_byte(cpu_state) as i8;
            let sum = cpu_state.registers.stack_pointer.wrapping_add_signed(signed_byte.into());
            
            microops::set_flag_z(cpu_state, false);
            microops::set_flag_n(cpu_state, false);
            microops::set_flag_h(cpu_state, (sum & 0xF) < (cpu_state.registers.stack_pointer & 0xF));
            microops::set_flag_c(cpu_state, (sum & 0xFF) < (cpu_state.registers.stack_pointer & 0xFF));

            cpu_state.registers.stack_pointer = sum;

            microops::run_extra_machine_cycle(cpu_state);
        },
        0xEA => {
            let address = read_next_instruction_word(cpu_state);
            load_source_register_in_memory(cpu_state, Register::A, address);
        },
        0xEB =>
            handle_illegal_opcode(opcode),
        0xEC =>
            handle_illegal_opcode(opcode),
        0xED =>
            handle_illegal_opcode(opcode),
        0xEE => {
            let value = read_next_instruction_byte(cpu_state);
            alu::logical_xor_with_register(cpu_state, Register::A, value);
        },
        0xF0 => {
            let address = 0xFF00 + read_next_instruction_byte(cpu_state) as u16;
            load_memory_byte_in_destination_register(cpu_state, address, Register::A);
        },
        0xF1 =>
            pop_word_into_register_pair_from_stack(cpu_state, REGISTER_AF),
        0xF2 => {
            let address = 0xFF00 + microops::read_from_register(cpu_state, &Register::C) as u16;
            load_memory_byte_in_destination_register(cpu_state, address, Register::A);
        },
        0xF4 =>
            handle_illegal_opcode(opcode),
        0xF5 =>
            push_register_pair_to_stack(cpu_state, REGISTER_AF),
        0xF6 => {
            let value = read_next_instruction_byte(cpu_state);
            alu::logical_or_with_register(cpu_state, Register::A, value);
        },
        0xF8 => {
            let signed_byte = read_next_instruction_byte(cpu_state) as i8;
            let sum = cpu_state.registers.stack_pointer.wrapping_add_signed(signed_byte.into());
            microops::store_in_register_pair(cpu_state, REGISTER_HL, sum);
            
            microops::set_flag_z(cpu_state, false);
            microops::set_flag_n(cpu_state, false);
            microops::set_flag_h(cpu_state, (sum & 0xF) < (cpu_state.registers.stack_pointer & 0xF));
            microops::set_flag_c(cpu_state, (sum & 0xFF) < (cpu_state.registers.stack_pointer & 0xFF));

            microops::run_extra_machine_cycle(cpu_state);
        },
        0xF9 => {
            let word = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
            cpu_state.registers.stack_pointer = word;
            microops::run_extra_machine_cycle(cpu_state);
        },
        0xFA => {
            let address = read_next_instruction_word(cpu_state);
            load_memory_byte_in_destination_register(cpu_state, address, Register::A);
        },
        0xFC =>
            handle_illegal_opcode(opcode),
        0xFD =>
            handle_illegal_opcode(opcode),
        0xFE => {
            let value = read_next_instruction_byte(cpu_state);
            alu::compare_value_with_register(cpu_state, Register::A, value);
        },
        _ =>
            ()
    }
}

fn execute_cb_opcode(cpu_state: &mut CpuState) {
    let opcode = read_next_instruction_byte(cpu_state);
    match opcode {
        0x30 =>
            swap_nibbles_in_register(cpu_state, Register::B),
        0x31 =>
            swap_nibbles_in_register(cpu_state, Register::C),
        0x32 =>
            swap_nibbles_in_register(cpu_state, Register::D),
        0x33 =>
            swap_nibbles_in_register(cpu_state, Register::E),
        0x34 =>
            swap_nibbles_in_register(cpu_state, Register::H),
        0x35 =>
            swap_nibbles_in_register(cpu_state, Register::L),
        0x36 => {
            let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
            swap_nibbles_in_memory_byte(cpu_state, address);
        },
        0x37 =>
            swap_nibbles_in_register(cpu_state, Register::A),
        _ =>
            ()
    }
}

#[cfg(test)]
mod tests;
