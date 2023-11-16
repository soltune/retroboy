use crate::cpu::{Register, RegisterPair, CpuState, REGISTER_BC, REGISTER_DE, REGISTER_HL};
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
    let source_value = microops::read_from_register(cpu_state, source);
    microops::store_in_register(cpu_state, destination, source_value);
}

fn load_memory_byte_in_destination_register(cpu_state: &mut CpuState, address: u16, destination: Register) {
    let byte = microops::read_byte_from_memory(cpu_state, address);
    microops::store_in_register(cpu_state, destination, byte);
}

fn load_source_register_in_memory(cpu_state: &mut CpuState, source: Register, address: u16) {
    let byte = microops::read_from_register(cpu_state, source);
    microops::store_byte_in_memory(cpu_state, address, byte);
}

fn load_immediate_value_in_memory(cpu_state: &mut CpuState, register_pair: RegisterPair) {
    let address = microops::read_from_register_pair(cpu_state, register_pair);
    let immediate_byte = read_next_instruction_byte(cpu_state);
    microops::store_byte_in_memory(cpu_state, address, immediate_byte);
}

pub fn execute_opcode(cpu_state: &mut CpuState) {
    let opcode = read_next_instruction_byte(cpu_state);
    match opcode {
        0x00 =>
            (),
        0x02 => {
            let address = microops::read_from_register_pair(cpu_state, REGISTER_BC);
            load_source_register_in_memory(cpu_state, Register::A, address);
        },
        0x06 =>
            load_immediate_value(cpu_state, Register::B),
        0x0a => {
            let address = microops::read_from_register_pair(cpu_state, REGISTER_BC);
            load_memory_byte_in_destination_register(cpu_state, address, Register::A)
        },
        0x0e =>
            load_immediate_value(cpu_state, Register::C),
        0x12 => {
            let address = microops::read_from_register_pair(cpu_state, REGISTER_DE);
            load_source_register_in_memory(cpu_state, Register::A, address);
        },
        0x16 =>
            load_immediate_value(cpu_state, Register::D),
        0x1a => {
            let address = microops::read_from_register_pair(cpu_state, REGISTER_DE);
            load_memory_byte_in_destination_register(cpu_state, address, Register::A)
        },
        0x1e =>
            load_immediate_value(cpu_state, Register::E),
        0x26 =>
            load_immediate_value(cpu_state, Register::H),
        0x2e =>
            load_immediate_value(cpu_state, Register::L),
        0x36 =>
            load_immediate_value_in_memory(cpu_state, REGISTER_HL),
        0x3e =>
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
        0x4a =>
            load_source_register_in_destination_register(cpu_state, Register::D, Register::C),
        0x4b =>
            load_source_register_in_destination_register(cpu_state, Register::E, Register::C),
        0x4c =>
            load_source_register_in_destination_register(cpu_state, Register::H, Register::C),
        0x4d =>
            load_source_register_in_destination_register(cpu_state, Register::L, Register::C),
        0x4e => {
            let address = microops::read_from_register_pair(cpu_state, REGISTER_HL);
            load_memory_byte_in_destination_register(cpu_state, address, Register::C)
        },
        0x4f =>
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
        0x5a =>
            load_source_register_in_destination_register(cpu_state, Register::D, Register::E),
        0x5b =>
            load_source_register_in_destination_register(cpu_state, Register::E, Register::E),
        0x5c =>
            load_source_register_in_destination_register(cpu_state, Register::H, Register::E),
        0x5d =>
            load_source_register_in_destination_register(cpu_state, Register::L, Register::E),
        0x5e => {
            let address = microops::read_from_register_pair(cpu_state, REGISTER_HL);
            load_memory_byte_in_destination_register(cpu_state, address, Register::E)
        },
        0x5f =>
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
        0x6a =>
            load_source_register_in_destination_register(cpu_state, Register::D, Register::L),
        0x6b =>
            load_source_register_in_destination_register(cpu_state, Register::E, Register::L),
        0x6c =>
            load_source_register_in_destination_register(cpu_state, Register::H, Register::L),
        0x6d =>
            load_source_register_in_destination_register(cpu_state, Register::L, Register::L),
        0x6e => {
            let address = microops::read_from_register_pair(cpu_state, REGISTER_HL);
            load_memory_byte_in_destination_register(cpu_state, address, Register::L)
        },
        0x6f =>
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
        0x7a =>
            load_source_register_in_destination_register(cpu_state, Register::D, Register::A),
        0x7b =>
            load_source_register_in_destination_register(cpu_state, Register::E, Register::A),
        0x7c =>
            load_source_register_in_destination_register(cpu_state, Register::H, Register::A),
        0x7d =>
            load_source_register_in_destination_register(cpu_state, Register::L, Register::A),
        0x7e => {
            let address = microops::read_from_register_pair(cpu_state, REGISTER_HL);
            load_memory_byte_in_destination_register(cpu_state, address, Register::A)
        },
        0x7f =>
            load_source_register_in_destination_register(cpu_state, Register::A, Register::A),
        0xea => {
            let address = read_next_instruction_word(cpu_state);
            load_source_register_in_memory(cpu_state, Register::A, address);
        },
        0xfa => {
            let address = read_next_instruction_word(cpu_state);
            load_memory_byte_in_destination_register(cpu_state, address, Register::A)
        },
        _ => ()
    }
}

#[cfg(test)]
mod tests;
