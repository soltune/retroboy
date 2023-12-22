use crate::cpu::{CpuState, read_next_instruction_byte, read_next_instruction_word};
use crate::cpu::loads;
use crate::cpu::microops;

fn conditional_jump(cpu_state: &mut CpuState, new_address: u16, condition: bool) {
    if condition {
        cpu_state.registers.program_counter = new_address;
    }   
}

pub fn conditional_relative_jump(cpu_state: &mut CpuState, condition: bool) {
    let offset_byte = read_next_instruction_byte(cpu_state) as u16;
    conditional_jump(cpu_state, cpu_state.registers.program_counter + offset_byte, condition);
}

pub fn conditional_jump_using_immediate_word(cpu_state: &mut CpuState, condition: bool) {
    let address = read_next_instruction_word(cpu_state);
    conditional_jump(cpu_state, address, condition);
}

pub fn call(cpu_state: &mut CpuState) {
    let word = read_next_instruction_word(cpu_state);
    loads::push_word_to_stack(cpu_state, cpu_state.registers.program_counter);
    cpu_state.registers.program_counter = word;
}

pub fn conditional_call_using_immediate_word(cpu_state: &mut CpuState, condition: bool) {
    let word = read_next_instruction_word(cpu_state);
    if condition {
        loads::push_word_to_stack(cpu_state, cpu_state.registers.program_counter);
        cpu_state.registers.program_counter = word;
    }
}

pub fn stack_return(cpu_state: &mut CpuState) {
    let word = loads::pop_word_from_stack(cpu_state);
    cpu_state.registers.program_counter = word;
    microops::run_extra_machine_cycle(cpu_state);
}

pub fn conditional_stack_return(cpu_state: &mut CpuState, condition: bool) {
    if condition {
        stack_return(cpu_state);
    }
    microops::run_extra_machine_cycle(cpu_state);
}

pub fn restart(cpu_state: &mut CpuState, new_address: u16) {
    loads::push_word_to_stack(cpu_state, cpu_state.registers.program_counter);
    cpu_state.registers.program_counter = new_address;
}
