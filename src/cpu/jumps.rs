use crate::cpu::{CpuState, read_next_instruction_byte, read_next_instruction_word};
use crate::cpu::loads;
use crate::cpu::microops;
use crate::emulator::Emulator;

fn conditional_jump(cpu_state: &mut CpuState, new_address: u16, condition: bool) {
    if condition {
        cpu_state.registers.program_counter = new_address;
        microops::run_extra_machine_cycle(cpu_state);
    }   
}

pub fn conditional_relative_jump(emulator: &mut Emulator, condition: bool) {
    let offset_byte = read_next_instruction_byte(emulator) as i8;
    let program_counter = emulator.cpu.registers.program_counter;
    let result_address = program_counter.wrapping_add_signed(offset_byte.into());
    conditional_jump(&mut emulator.cpu, result_address, condition);
}

pub fn conditional_jump_using_immediate_word(emulator: &mut Emulator, condition: bool) {
    let address = read_next_instruction_word(emulator);
    conditional_jump(&mut emulator.cpu, address, condition);
}

pub fn call(emulator: &mut Emulator) {
    let word = read_next_instruction_word(emulator);
    let program_counter = emulator.cpu.registers.program_counter;
    loads::push_word_to_stack(emulator, program_counter);
    emulator.cpu.registers.program_counter = word;
}

pub fn conditional_call_using_immediate_word(emulator: &mut Emulator, condition: bool) {
    let word = read_next_instruction_word(emulator);
    if condition {
        let program_counter = emulator.cpu.registers.program_counter;
        loads::push_word_to_stack(emulator, program_counter);
        emulator.cpu.registers.program_counter = word;
    }
}

pub fn stack_return(emulator: &mut Emulator) {
    let word = loads::pop_word_from_stack(emulator);
    emulator.cpu.registers.program_counter = word;
    microops::run_extra_machine_cycle(&mut emulator.cpu);
}

pub fn conditional_stack_return(emulator: &mut Emulator, condition: bool) {
    if condition {
        stack_return(emulator);
    }
    microops::run_extra_machine_cycle(&mut emulator.cpu);
}

pub fn restart(emulator: &mut Emulator, new_address: u16) {
    loads::push_word_to_stack(emulator, emulator.cpu.registers.program_counter);
    emulator.cpu.registers.program_counter = new_address;
}
