use crate::cpu::{Register, RegisterPair, CpuState, REGISTER_AF, REGISTER_BC, REGISTER_DE, REGISTER_HL};
use crate::cpu::microops;

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
    let address = microops::read_from_register_pair(cpu_state, register_pair);
    let immediate_byte = read_next_instruction_byte(cpu_state);
    microops::store_byte_in_memory(cpu_state, address, immediate_byte);
}

fn push_register_pair_to_stack(cpu_state: &mut CpuState, register_pair: RegisterPair) {
    let word = microops::read_from_register_pair(cpu_state, register_pair);
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

fn add_value_to_register(cpu_state: &mut CpuState, register: Register, value: u16) {
    let byte = microops::read_from_register(cpu_state, &register) as u16;
    let sum = ((value + byte) & 0xFF) as u8;

    microops::store_in_register(cpu_state, register, sum);

    microops::set_flag_z(cpu_state, sum == 0);
    microops::set_flag_n(cpu_state, false);
    microops::set_flag_h(cpu_state, (value & 0xF) + (byte & 0xF) > 0xF);
    microops::set_flag_c(cpu_state, (value + byte) > 0xFF);
}

fn add_value_and_carry_to_register(cpu_state: &mut CpuState, register: Register, value: u16) {
    let carry_value = if microops::is_c_flag_set(cpu_state) { 1 } else { 0 };
    add_value_to_register(cpu_state, register, value + carry_value);
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
            let address = microops::read_from_register_pair(cpu_state, REGISTER_BC);
            load_source_register_in_memory(cpu_state, Register::A, address);
        },
        0x06 =>
            load_immediate_value(cpu_state, Register::B),
        0x08 => {
            let address = read_next_instruction_word(cpu_state);
            microops::store_word_in_memory(cpu_state, address, cpu_state.registers.stack_pointer);
        },
        0x0A => {
            let address = microops::read_from_register_pair(cpu_state, REGISTER_BC);
            load_memory_byte_in_destination_register(cpu_state, address, Register::A);
        },
        0x0E =>
            load_immediate_value(cpu_state, Register::C),
        0x11 => {
            let word = read_next_instruction_word(cpu_state);
            microops::store_in_register_pair(cpu_state, REGISTER_DE, word);
        },
        0x12 => {
            let address = microops::read_from_register_pair(cpu_state, REGISTER_DE);
            load_source_register_in_memory(cpu_state, Register::A, address);
        },
        0x16 =>
            load_immediate_value(cpu_state, Register::D),
        0x1A => {
            let address = microops::read_from_register_pair(cpu_state, REGISTER_DE);
            load_memory_byte_in_destination_register(cpu_state, address, Register::A)
        },
        0x1E =>
            load_immediate_value(cpu_state, Register::E),
        0x21 => {
            let word = read_next_instruction_word(cpu_state);
            microops::store_in_register_pair(cpu_state, REGISTER_HL, word);
        },
        0x22 => {
            let mut address = microops::read_from_register_pair(cpu_state, REGISTER_HL);
            load_source_register_in_memory(cpu_state, Register::A, address);
            address += 1;
            microops::store_in_register_pair(cpu_state, REGISTER_HL, address);    
        },
        0x26 =>
            load_immediate_value(cpu_state, Register::H),
        0x2A => {
            let mut address = microops::read_from_register_pair(cpu_state, REGISTER_HL);
            load_memory_byte_in_destination_register(cpu_state, address, Register::A);
            address += 1;
            microops::store_in_register_pair(cpu_state, REGISTER_HL, address);  
        },
        0x2E =>
            load_immediate_value(cpu_state, Register::L),
        0x31 => {
            let word = read_next_instruction_word(cpu_state);
            cpu_state.registers.stack_pointer = word;            
        },
        0x32 => {
            let mut address = microops::read_from_register_pair(cpu_state, REGISTER_HL);
            load_source_register_in_memory(cpu_state, Register::A, address);
            address -= 1;
            microops::store_in_register_pair(cpu_state, REGISTER_HL, address);           
        },
        0x36 =>
            load_immediate_value_in_memory(cpu_state, REGISTER_HL),
        0x3A => {
            let mut address = microops::read_from_register_pair(cpu_state, REGISTER_HL);
            load_memory_byte_in_destination_register(cpu_state, address, Register::A);
            address -= 1;
            microops::store_in_register_pair(cpu_state, REGISTER_HL, address);
        },
        0x3E =>
            load_immediate_value(cpu_state, Register::A),
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
            let address = microops::read_from_register_pair(cpu_state, REGISTER_HL);
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
            let address = microops::read_from_register_pair(cpu_state, REGISTER_HL);
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
            let address = microops::read_from_register_pair(cpu_state, REGISTER_HL);
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
            let address = microops::read_from_register_pair(cpu_state, REGISTER_HL);
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
            let address = microops::read_from_register_pair(cpu_state, REGISTER_HL);
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
            let address = microops::read_from_register_pair(cpu_state, REGISTER_HL);
            load_memory_byte_in_destination_register(cpu_state, address, Register::L)
        },
        0x6F =>
            load_source_register_in_destination_register(cpu_state, Register::A, Register::L),
        0x70 => {
            let address = microops::read_from_register_pair(cpu_state, REGISTER_HL);
            load_source_register_in_memory(cpu_state, Register::B, address);
        },
        0x71 => {
            let address = microops::read_from_register_pair(cpu_state, REGISTER_HL);
            load_source_register_in_memory(cpu_state, Register::C, address);
        },
        0x72 => {
            let address = microops::read_from_register_pair(cpu_state, REGISTER_HL);
            load_source_register_in_memory(cpu_state, Register::D, address);
        },
        0x73 => {
            let address = microops::read_from_register_pair(cpu_state, REGISTER_HL);
            load_source_register_in_memory(cpu_state, Register::E, address);
        },
        0x74 => {
            let address = microops::read_from_register_pair(cpu_state, REGISTER_HL);
            load_source_register_in_memory(cpu_state, Register::H, address);
        },
        0x75 => {
            let address = microops::read_from_register_pair(cpu_state, REGISTER_HL);
            load_source_register_in_memory(cpu_state, Register::L, address);
        },
        0x77 => {
            let address = microops::read_from_register_pair(cpu_state, REGISTER_HL);
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
            let address = microops::read_from_register_pair(cpu_state, REGISTER_HL);
            load_memory_byte_in_destination_register(cpu_state, address, Register::A)
        },
        0x7F =>
            load_source_register_in_destination_register(cpu_state, Register::A, Register::A),
        0x80 => {
            let value = microops::read_from_register(cpu_state, &Register::B);
            add_value_to_register(cpu_state, Register::A, value as u16);
        },
        0x81 => {
            let value = microops::read_from_register(cpu_state, &Register::C);
            add_value_to_register(cpu_state, Register::A, value as u16);
        },
        0x82 => {
            let value = microops::read_from_register(cpu_state, &Register::D);
            add_value_to_register(cpu_state, Register::A, value as u16);
        },
        0x83 => {
            let value = microops::read_from_register(cpu_state, &Register::E);
            add_value_to_register(cpu_state, Register::A, value as u16);
        },
        0x84 => {
            let value = microops::read_from_register(cpu_state, &Register::H);
            add_value_to_register(cpu_state, Register::A, value as u16);
        },
        0x85 => {
            let value = microops::read_from_register(cpu_state, &Register::L);
            add_value_to_register(cpu_state, Register::A, value as u16);
        },
        0x86 => {
            let address = microops::read_from_register_pair(cpu_state, REGISTER_HL);
            let value = microops::read_byte_from_memory(cpu_state, address);
            add_value_to_register(cpu_state, Register::A, value as u16);
        },
        0x87 => {
            let value = microops::read_from_register(cpu_state, &Register::A);
            add_value_to_register(cpu_state, Register::A, value as u16);
        },
        0x88 => {
            let value = microops::read_from_register(cpu_state, &Register::B);
            add_value_and_carry_to_register(cpu_state, Register::A, value as u16);
        },
        0x89 => {
            let value = microops::read_from_register(cpu_state, &Register::C);
            add_value_and_carry_to_register(cpu_state, Register::A, value as u16);
        },
        0x8A => {
            let value = microops::read_from_register(cpu_state, &Register::D);
            add_value_and_carry_to_register(cpu_state, Register::A, value as u16);
        },
        0x8B => {
            let value = microops::read_from_register(cpu_state, &Register::E);
            add_value_and_carry_to_register(cpu_state, Register::A, value as u16);
        },
        0x8C => {
            let value = microops::read_from_register(cpu_state, &Register::H);
            add_value_and_carry_to_register(cpu_state, Register::A, value as u16);
        },
        0x8D => {
            let value = microops::read_from_register(cpu_state, &Register::L);
            add_value_and_carry_to_register(cpu_state, Register::A, value as u16);
        },
        0x8E => {
            let address = microops::read_from_register_pair(cpu_state, REGISTER_HL);
            let value = microops::read_byte_from_memory(cpu_state, address);
            add_value_and_carry_to_register(cpu_state, Register::A, value as u16);
        },
        0x8F => {
            let value = microops::read_from_register(cpu_state, &Register::A);
            add_value_and_carry_to_register(cpu_state, Register::A, value as u16);
        },
        0xC1 =>
            pop_word_into_register_pair_from_stack(cpu_state, REGISTER_BC),
        0xC5 =>
            push_register_pair_to_stack(cpu_state, REGISTER_BC),
        0xC6 => {
            let value = read_next_instruction_byte(cpu_state);
            add_value_to_register(cpu_state, Register::A, value as u16);
        },
        0xCE => {
            let value = read_next_instruction_byte(cpu_state);
            add_value_and_carry_to_register(cpu_state, Register::A, value as u16);
        },
        0xD1 =>
            pop_word_into_register_pair_from_stack(cpu_state, REGISTER_DE),
        0xD5 =>
            push_register_pair_to_stack(cpu_state, REGISTER_DE),
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
        0xE5 =>
            push_register_pair_to_stack(cpu_state, REGISTER_HL),
        0xEA => {
            let address = read_next_instruction_word(cpu_state);
            load_source_register_in_memory(cpu_state, Register::A, address);
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
        0xF5 =>
            push_register_pair_to_stack(cpu_state, REGISTER_AF),
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
            let word = microops::read_from_register_pair(cpu_state, REGISTER_HL);
            cpu_state.registers.stack_pointer = word;
            microops::run_extra_machine_cycle(cpu_state);
        },
        0xFA => {
            let address = read_next_instruction_word(cpu_state);
            load_memory_byte_in_destination_register(cpu_state, address, Register::A);
        },
        _ => ()
    }
}

#[cfg(test)]
mod tests;
