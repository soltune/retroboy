use crate::cpu::{Register, RegisterPair, CpuState, REGISTER_AF, REGISTER_BC, REGISTER_DE, REGISTER_HL};
use crate::cpu::microops;
use crate::cpu::alu;
use crate::cpu::bitops;

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
        0x07 =>
            bitops::rotate_register_left(cpu_state, Register::A),
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
        0x0F =>
            bitops::rotate_register_right(cpu_state, Register::A),
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
        0x17 =>
            bitops::rotate_register_left_through_carry(cpu_state, Register::A),
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
        0x1F =>
            bitops::rotate_register_right_through_carry(cpu_state, Register::A),
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
        0xC3 => {
            cpu_state.registers.program_counter = read_next_instruction_word(cpu_state);
        },
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
        0x00 =>
            bitops::rotate_register_left(cpu_state, Register::B),
        0x01 =>
            bitops::rotate_register_left(cpu_state, Register::C),
        0x02 =>
            bitops::rotate_register_left(cpu_state, Register::D),
        0x03 =>
            bitops::rotate_register_left(cpu_state, Register::E),
        0x04 =>
            bitops::rotate_register_left(cpu_state, Register::H),
        0x05 =>
            bitops::rotate_register_left(cpu_state, Register::L),
        0x06 =>
            bitops::rotate_memory_byte_left(cpu_state),
        0x07 =>
            bitops::rotate_register_left(cpu_state, Register::A),
        0x08 =>
            bitops::rotate_register_right(cpu_state, Register::B),
        0x09 =>
            bitops::rotate_register_right(cpu_state, Register::C),
        0x0A =>
            bitops::rotate_register_right(cpu_state, Register::D),
        0x0B =>
            bitops::rotate_register_right(cpu_state, Register::E),
        0x0C =>
            bitops::rotate_register_right(cpu_state, Register::H),
        0x0D =>
            bitops::rotate_register_right(cpu_state, Register::L),
        0x0E =>
            bitops::rotate_memory_byte_right(cpu_state),
        0x0F =>
            bitops::rotate_register_right(cpu_state, Register::A),
        0x10 =>
            bitops::rotate_register_left_through_carry(cpu_state, Register::B),
        0x11 =>
            bitops::rotate_register_left_through_carry(cpu_state, Register::C),
        0x12 =>
            bitops::rotate_register_left_through_carry(cpu_state, Register::D),
        0x13 =>
            bitops::rotate_register_left_through_carry(cpu_state, Register::E),
        0x14 =>
            bitops::rotate_register_left_through_carry(cpu_state, Register::H),
        0x15 =>
            bitops::rotate_register_left_through_carry(cpu_state, Register::L),
        0x16 =>
            bitops::rotate_memory_byte_left_through_carry(cpu_state),
        0x17 =>
            bitops::rotate_register_left_through_carry(cpu_state, Register::A),
        0x18 =>
            bitops::rotate_register_right_through_carry(cpu_state, Register::B),
        0x19 =>
            bitops::rotate_register_right_through_carry(cpu_state, Register::C),
        0x1A =>
            bitops::rotate_register_right_through_carry(cpu_state, Register::D),
        0x1B =>
            bitops::rotate_register_right_through_carry(cpu_state, Register::E),
        0x1C =>
            bitops::rotate_register_right_through_carry(cpu_state, Register::H),
        0x1D =>
            bitops::rotate_register_right_through_carry(cpu_state, Register::L),    
        0x1E =>
            bitops::rotate_memory_byte_right_through_carry(cpu_state),
        0x1F =>
            bitops::rotate_register_right_through_carry(cpu_state, Register::A),
        0x20 =>
            bitops::shift_register_left(cpu_state, Register::B),
        0x21 =>
            bitops::shift_register_left(cpu_state, Register::C),
        0x22 =>
            bitops::shift_register_left(cpu_state, Register::D),
        0x23 =>
            bitops::shift_register_left(cpu_state, Register::E),
        0x24 =>
            bitops::shift_register_left(cpu_state, Register::H),
        0x25 =>
            bitops::shift_register_left(cpu_state, Register::L),
        0x26 =>
            bitops::shift_memory_byte_left(cpu_state),
        0x27 =>
            bitops::shift_register_left(cpu_state, Register::A),
        0x28 =>
            bitops::shift_register_right_maintaining_msb(cpu_state, Register::B),
        0x29 =>
            bitops::shift_register_right_maintaining_msb(cpu_state, Register::C),
        0x2A =>
            bitops::shift_register_right_maintaining_msb(cpu_state, Register::D),
        0x2B =>
            bitops::shift_register_right_maintaining_msb(cpu_state, Register::E),
        0x2C =>
            bitops::shift_register_right_maintaining_msb(cpu_state, Register::H),
        0x2D =>
            bitops::shift_register_right_maintaining_msb(cpu_state, Register::L),
        0x2E =>
            bitops::shift_memory_byte_right_maintaining_msb(cpu_state),
        0x2F =>
            bitops::shift_register_right_maintaining_msb(cpu_state, Register::A),
        0x30 =>
            bitops::swap_nibbles_in_register(cpu_state, Register::B),
        0x31 =>
            bitops::swap_nibbles_in_register(cpu_state, Register::C),
        0x32 =>
            bitops::swap_nibbles_in_register(cpu_state, Register::D),
        0x33 =>
            bitops::swap_nibbles_in_register(cpu_state, Register::E),
        0x34 =>
            bitops::swap_nibbles_in_register(cpu_state, Register::H),
        0x35 =>
            bitops::swap_nibbles_in_register(cpu_state, Register::L),
        0x36 => {
            let address = microops::read_from_register_pair(cpu_state, &REGISTER_HL);
            bitops::swap_nibbles_in_memory_byte(cpu_state, address);
        },
        0x37 =>
            bitops::swap_nibbles_in_register(cpu_state, Register::A),
        0x38 =>
            bitops::shift_register_right(cpu_state, Register::B),
        0x39 =>
            bitops::shift_register_right(cpu_state, Register::C),
        0x3A =>
            bitops::shift_register_right(cpu_state, Register::D),
        0x3B =>
            bitops::shift_register_right(cpu_state, Register::E),
        0x3C =>
            bitops::shift_register_right(cpu_state, Register::H),
        0x3D =>
            bitops::shift_register_right(cpu_state, Register::L),
        0x3E =>
            bitops::shift_memory_byte_right(cpu_state),
        0x3F =>
            bitops::shift_register_right(cpu_state, Register::A),
        0x40 =>
            bitops::test_register_bit(cpu_state, Register::B, 0),
        0x41 =>
            bitops::test_register_bit(cpu_state, Register::C, 0),
        0x42 =>
            bitops::test_register_bit(cpu_state, Register::D, 0),
        0x43 =>
            bitops::test_register_bit(cpu_state, Register::E, 0),   
        0x44 =>
            bitops::test_register_bit(cpu_state, Register::H, 0),
        0x45 =>
            bitops::test_register_bit(cpu_state, Register::L, 0),
        0x46 =>
            bitops::test_memory_bit(cpu_state, 0),
        0x47 =>
            bitops::test_register_bit(cpu_state, Register::A, 0),
        0x48 =>
            bitops::test_register_bit(cpu_state, Register::B, 1),
        0x49 =>
            bitops::test_register_bit(cpu_state, Register::C, 1),
        0x4A =>
            bitops::test_register_bit(cpu_state, Register::D, 1),
        0x4B =>
            bitops::test_register_bit(cpu_state, Register::E, 1),
        0x4C =>
            bitops::test_register_bit(cpu_state, Register::H, 1),
        0x4D =>
            bitops::test_register_bit(cpu_state, Register::L, 1),
        0x4E =>
            bitops::test_memory_bit(cpu_state, 1),
        0x4F =>
            bitops::test_register_bit(cpu_state, Register::A, 1),
        0x50 =>
            bitops::test_register_bit(cpu_state, Register::B, 2),
        0x51 =>
            bitops::test_register_bit(cpu_state, Register::C, 2),  
        0x52 =>
            bitops::test_register_bit(cpu_state, Register::D, 2),
        0x53 =>
            bitops::test_register_bit(cpu_state, Register::E, 2),
        0x54 =>
            bitops::test_register_bit(cpu_state, Register::H, 2),
        0x55 =>
            bitops::test_register_bit(cpu_state, Register::L, 2),
        0x56 =>
            bitops::test_memory_bit(cpu_state, 2),
        0x57 =>
            bitops::test_register_bit(cpu_state, Register::A, 2),
        0x58 =>
            bitops::test_register_bit(cpu_state, Register::B, 3),
        0x59 =>
            bitops::test_register_bit(cpu_state, Register::C, 3),
        0x5A =>
            bitops::test_register_bit(cpu_state, Register::D, 3),
        0x5B =>
            bitops::test_register_bit(cpu_state, Register::E, 3),
        0x5C =>
            bitops::test_register_bit(cpu_state, Register::H, 3),
        0x5D =>
            bitops::test_register_bit(cpu_state, Register::L, 3),
        0x5E =>
            bitops::test_memory_bit(cpu_state, 3),
        0x5F =>
            bitops::test_register_bit(cpu_state, Register::A, 3),
        0x60 =>
            bitops::test_register_bit(cpu_state, Register::B, 4),
        0x61 =>
            bitops::test_register_bit(cpu_state, Register::C, 4),
        0x62 =>
            bitops::test_register_bit(cpu_state, Register::D, 4),
        0x63 =>
            bitops::test_register_bit(cpu_state, Register::E, 4),
        0x64 =>
            bitops::test_register_bit(cpu_state, Register::H, 4),
        0x65 =>
            bitops::test_register_bit(cpu_state, Register::L, 4),
        0x66 =>
            bitops::test_memory_bit(cpu_state, 4),
        0x67 =>
            bitops::test_register_bit(cpu_state, Register::A, 4),
        0x68 =>
            bitops::test_register_bit(cpu_state, Register::B, 5),
        0x69 =>
            bitops::test_register_bit(cpu_state, Register::C, 5),
        0x6A =>
            bitops::test_register_bit(cpu_state, Register::D, 5),
        0x6B =>
            bitops::test_register_bit(cpu_state, Register::E, 5),
        0x6C =>
            bitops::test_register_bit(cpu_state, Register::H, 5),
        0x6D =>
            bitops::test_register_bit(cpu_state, Register::L, 5),
        0x6E =>
            bitops::test_memory_bit(cpu_state, 5),
        0x6F =>
            bitops::test_register_bit(cpu_state, Register::A, 5),
        0x70 =>
            bitops::test_register_bit(cpu_state, Register::B, 6),
        0x71 =>
            bitops::test_register_bit(cpu_state, Register::C, 6),
        0x72 =>
            bitops::test_register_bit(cpu_state, Register::D, 6),
        0x73 =>
            bitops::test_register_bit(cpu_state, Register::E, 6),
        0x74 =>
            bitops::test_register_bit(cpu_state, Register::H, 6),
        0x75 =>
            bitops::test_register_bit(cpu_state, Register::L, 6),
        0x76 =>
            bitops::test_memory_bit(cpu_state, 6),
        0x77 =>
            bitops::test_register_bit(cpu_state, Register::A, 6),
        0x78 =>
            bitops::test_register_bit(cpu_state, Register::B, 7),
        0x79 =>
            bitops::test_register_bit(cpu_state, Register::C, 7),
        0x7A =>
            bitops::test_register_bit(cpu_state, Register::D, 7),
        0x7B =>
            bitops::test_register_bit(cpu_state, Register::E, 7),
        0x7C =>
            bitops::test_register_bit(cpu_state, Register::H, 7),
        0x7D =>
            bitops::test_register_bit(cpu_state, Register::L, 7),
        0x7E =>
            bitops::test_memory_bit(cpu_state, 7),
        0x7F =>
            bitops::test_register_bit(cpu_state, Register::A, 7),
        0x80 =>
            bitops::reset_register_bit(cpu_state, Register::B, 0),
        0x81 =>
            bitops::reset_register_bit(cpu_state, Register::C, 0),
        0x82 =>
            bitops::reset_register_bit(cpu_state, Register::D, 0),
        0x83 =>
            bitops::reset_register_bit(cpu_state, Register::E, 0),
        0x84 =>
            bitops::reset_register_bit(cpu_state, Register::H, 0),
        0x85 =>
            bitops::reset_register_bit(cpu_state, Register::L, 0),
        0x86 =>
            bitops::reset_memory_bit(cpu_state, 0),
        0x87 =>
            bitops::reset_register_bit(cpu_state, Register::A, 0),
        0x88 =>
            bitops::reset_register_bit(cpu_state, Register::B, 1),
        0x89 =>
            bitops::reset_register_bit(cpu_state, Register::C, 1),
        0x8A =>
            bitops::reset_register_bit(cpu_state, Register::D, 1),
        0x8B =>
            bitops::reset_register_bit(cpu_state, Register::E, 1),
        0x8C =>
            bitops::reset_register_bit(cpu_state, Register::H, 1),
        0x8D =>
            bitops::reset_register_bit(cpu_state, Register::L, 1),
        0x8E =>
            bitops::reset_memory_bit(cpu_state, 1),
        0x8F =>
            bitops::reset_register_bit(cpu_state, Register::A, 1),
        0x90 =>
            bitops::reset_register_bit(cpu_state, Register::B, 2),
        0x91 =>
            bitops::reset_register_bit(cpu_state, Register::C, 2),
        0x92 =>
            bitops::reset_register_bit(cpu_state, Register::D, 2),
        0x93 =>
            bitops::reset_register_bit(cpu_state, Register::E, 2),
        0x94 =>
            bitops::reset_register_bit(cpu_state, Register::H, 2),
        0x95 =>
            bitops::reset_register_bit(cpu_state, Register::L, 2),
        0x96 =>
            bitops::reset_memory_bit(cpu_state, 2),
        0x97 =>
            bitops::reset_register_bit(cpu_state, Register::A, 2),
        0x98 =>
            bitops::reset_register_bit(cpu_state, Register::B, 3),
        0x99 =>
            bitops::reset_register_bit(cpu_state, Register::C, 3),
        0x9A =>
            bitops::reset_register_bit(cpu_state, Register::D, 3),
        0x9B =>
            bitops::reset_register_bit(cpu_state, Register::E, 3),
        0x9C =>
            bitops::reset_register_bit(cpu_state, Register::H, 3),
        0x9D =>
            bitops::reset_register_bit(cpu_state, Register::L, 3),
        0x9E =>
            bitops::reset_memory_bit(cpu_state, 3),
        0x9F =>
            bitops::reset_register_bit(cpu_state, Register::A, 3),
        0xA0 =>
            bitops::reset_register_bit(cpu_state, Register::B, 4),
        0xA1 =>
            bitops::reset_register_bit(cpu_state, Register::C, 4),
        0xA2 =>
            bitops::reset_register_bit(cpu_state, Register::D, 4),
        0xA3 =>
            bitops::reset_register_bit(cpu_state, Register::E, 4),
        0xA4 =>
            bitops::reset_register_bit(cpu_state, Register::H, 4),
        0xA5 =>
            bitops::reset_register_bit(cpu_state, Register::L, 4),
        0xA6 =>
            bitops::reset_memory_bit(cpu_state, 4),
        0xA7 =>
            bitops::reset_register_bit(cpu_state, Register::A, 4),
        0xA8 =>
            bitops::reset_register_bit(cpu_state, Register::B, 5),
        0xA9 =>
            bitops::reset_register_bit(cpu_state, Register::C, 5),
        0xAA =>
            bitops::reset_register_bit(cpu_state, Register::D, 5),
        0xAB =>
            bitops::reset_register_bit(cpu_state, Register::E, 5),
        0xAC =>
            bitops::reset_register_bit(cpu_state, Register::H, 5),
        0xAD =>
            bitops::reset_register_bit(cpu_state, Register::L, 5),
        0xAE =>
            bitops::reset_memory_bit(cpu_state, 5),
        0xAF =>
            bitops::reset_register_bit(cpu_state, Register::A, 5),
        0xB0 =>
            bitops::reset_register_bit(cpu_state, Register::B, 6),
        0xB1 =>
            bitops::reset_register_bit(cpu_state, Register::C, 6),
        0xB2 =>
            bitops::reset_register_bit(cpu_state, Register::D, 6),
        0xB3 =>
            bitops::reset_register_bit(cpu_state, Register::E, 6),
        0xB4 =>
            bitops::reset_register_bit(cpu_state, Register::H, 6),
        0xB5 =>
            bitops::reset_register_bit(cpu_state, Register::L, 6),
        0xB6 =>
            bitops::reset_memory_bit(cpu_state, 6),
        0xB7 =>
            bitops::reset_register_bit(cpu_state, Register::A, 6),
        0xB8 =>
            bitops::reset_register_bit(cpu_state, Register::B, 7),
        0xB9 =>
            bitops::reset_register_bit(cpu_state, Register::C, 7),
        0xBA =>
            bitops::reset_register_bit(cpu_state, Register::D, 7),
        0xBB =>
            bitops::reset_register_bit(cpu_state, Register::E, 7),
        0xBC =>
            bitops::reset_register_bit(cpu_state, Register::H, 7),
        0xBD =>
            bitops::reset_register_bit(cpu_state, Register::L, 7),
        0xBE =>
            bitops::reset_memory_bit(cpu_state, 7),
        0xBF =>
            bitops::reset_register_bit(cpu_state, Register::A, 7),
        0xC0 =>
            bitops::set_register_bit(cpu_state, Register::B, 0),
        0xC1 =>
            bitops::set_register_bit(cpu_state, Register::C, 0),
        0xC2 =>
            bitops::set_register_bit(cpu_state, Register::D, 0),
        0xC3 =>
            bitops::set_register_bit(cpu_state, Register::E, 0),
        0xC4 =>
            bitops::set_register_bit(cpu_state, Register::H, 0),
        0xC5 =>
            bitops::set_register_bit(cpu_state, Register::L, 0),
        0xC6 =>
            bitops::set_memory_bit(cpu_state, 0),
        0xC7 =>
            bitops::set_register_bit(cpu_state, Register::A, 0),
        0xC8 =>
            bitops::set_register_bit(cpu_state, Register::B, 1),
        0xC9 =>
            bitops::set_register_bit(cpu_state, Register::C, 1),
        0xCA =>
            bitops::set_register_bit(cpu_state, Register::D, 1),
        0xCB =>
            bitops::set_register_bit(cpu_state, Register::E, 1),
        0xCC =>
            bitops::set_register_bit(cpu_state, Register::H, 1),
        0xCD =>
            bitops::set_register_bit(cpu_state, Register::L, 1),
        0xCE =>
            bitops::set_memory_bit(cpu_state, 1),
        0xCF =>
            bitops::set_register_bit(cpu_state, Register::A, 1),
        0xD0 =>
            bitops::set_register_bit(cpu_state, Register::B, 2),
        0xD1 =>
            bitops::set_register_bit(cpu_state, Register::C, 2),
        0xD2 =>
            bitops::set_register_bit(cpu_state, Register::D, 2),
        0xD3 =>
            bitops::set_register_bit(cpu_state, Register::E, 2),
        0xD4 =>
            bitops::set_register_bit(cpu_state, Register::H, 2),
        0xD5 =>
            bitops::set_register_bit(cpu_state, Register::L, 2),
        0xD6 =>
            bitops::set_memory_bit(cpu_state, 2),
        0xD7 =>
            bitops::set_register_bit(cpu_state, Register::A, 2),
        0xD8 =>
            bitops::set_register_bit(cpu_state, Register::B, 3),
        0xD9 =>
            bitops::set_register_bit(cpu_state, Register::C, 3),
        0xDA =>
            bitops::set_register_bit(cpu_state, Register::D, 3),
        0xDB =>
            bitops::set_register_bit(cpu_state, Register::E, 3),
        0xDC =>
            bitops::set_register_bit(cpu_state, Register::H, 3),
        0xDD =>
            bitops::set_register_bit(cpu_state, Register::L, 3),
        0xDE =>
            bitops::set_memory_bit(cpu_state, 3),
        0xDF =>
            bitops::set_register_bit(cpu_state, Register::A, 3),
        0xE0 =>
            bitops::set_register_bit(cpu_state, Register::B, 4),
        0xE1 =>
            bitops::set_register_bit(cpu_state, Register::C, 4),
        0xE2 =>
            bitops::set_register_bit(cpu_state, Register::D, 4),
        0xE3 =>
            bitops::set_register_bit(cpu_state, Register::E, 4),
        0xE4 =>
            bitops::set_register_bit(cpu_state, Register::H, 4),
        0xE5 =>
            bitops::set_register_bit(cpu_state, Register::L, 4),
        0xE6 =>
            bitops::set_memory_bit(cpu_state, 4),
        0xE7 =>
            bitops::set_register_bit(cpu_state, Register::A, 4),
        0xE8 =>
            bitops::set_register_bit(cpu_state, Register::B, 5),
        0xE9 =>
            bitops::set_register_bit(cpu_state, Register::C, 5),
        0xEA =>
            bitops::set_register_bit(cpu_state, Register::D, 5),
        0xEB =>
            bitops::set_register_bit(cpu_state, Register::E, 5),
        0xEC =>
            bitops::set_register_bit(cpu_state, Register::H, 5),
        0xED =>
            bitops::set_register_bit(cpu_state, Register::L, 5),
        0xEE =>
            bitops::set_memory_bit(cpu_state, 5),
        0xEF =>
            bitops::set_register_bit(cpu_state, Register::A, 5),
        0xF0 =>
            bitops::set_register_bit(cpu_state, Register::B, 6),
        0xF1 =>
            bitops::set_register_bit(cpu_state, Register::C, 6),
        0xF2 =>
            bitops::set_register_bit(cpu_state, Register::D, 6),
        0xF3 =>
            bitops::set_register_bit(cpu_state, Register::E, 6),
        0xF4 =>
            bitops::set_register_bit(cpu_state, Register::H, 6),
        0xF5 =>
            bitops::set_register_bit(cpu_state, Register::L, 6),
        0xF6 =>
            bitops::set_memory_bit(cpu_state, 6),
        0xF7 =>
            bitops::set_register_bit(cpu_state, Register::A, 6),
        0xF8 =>
            bitops::set_register_bit(cpu_state, Register::B, 7),
        0xF9 =>
            bitops::set_register_bit(cpu_state, Register::C, 7),
        0xFA =>
            bitops::set_register_bit(cpu_state, Register::D, 7),
        0xFB =>
            bitops::set_register_bit(cpu_state, Register::E, 7),
        0xFC =>
            bitops::set_register_bit(cpu_state, Register::H, 7),
        0xFD =>
            bitops::set_register_bit(cpu_state, Register::L, 7),
        0xFE =>
            bitops::set_memory_bit(cpu_state, 7),
        0xFF =>
            bitops::set_register_bit(cpu_state, Register::A, 7),
    }
}

#[cfg(test)]
mod tests;
