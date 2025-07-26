use crate::cpu::{read_next_instruction_byte, read_next_instruction_word, Cpu};
use crate::cpu::loads;
use crate::cpu::microops;

fn conditional_jump(cpu_state: &mut Cpu, new_address: u16, condition: bool) {
    if condition {
        cpu_state.registers.program_counter = new_address;
        microops::step_one_machine_cycle(cpu_state);
    }   
}

pub fn conditional_relative_jump(cpu_state: &mut Cpu, condition: bool) {
    let offset_byte = read_next_instruction_byte(cpu_state) as i8;
    let program_counter = cpu_state.registers.program_counter;
    let result_address = program_counter.wrapping_add_signed(offset_byte.into());
    conditional_jump(cpu_state, result_address, condition);
}

pub fn conditional_jump_using_immediate_word(cpu_state: &mut Cpu, condition: bool) {
    let address = read_next_instruction_word(cpu_state);
    conditional_jump(cpu_state, address, condition);
}

pub fn call(cpu_state: &mut Cpu) {
    let word = read_next_instruction_word(cpu_state);
    let program_counter = cpu_state.registers.program_counter;
    loads::push_word_to_stack(cpu_state, program_counter);
    cpu_state.registers.program_counter = word;
}

pub fn conditional_call_using_immediate_word(cpu_state: &mut Cpu, condition: bool) {
    let word = read_next_instruction_word(cpu_state);
    if condition {
        let program_counter = cpu_state.registers.program_counter;
        loads::push_word_to_stack(cpu_state, program_counter);
        cpu_state.registers.program_counter = word;
    }
}

pub fn stack_return(cpu_state: &mut Cpu) {
    let word = loads::pop_word_from_stack(cpu_state);
    cpu_state.registers.program_counter = word;
    microops::step_one_machine_cycle(cpu_state);
}

pub fn conditional_stack_return(cpu_state: &mut Cpu, condition: bool) {
    microops::step_one_machine_cycle(cpu_state);
    if condition {
        stack_return(cpu_state);
    }
}

pub fn restart(cpu_state: &mut Cpu, new_address: u16) {
    loads::push_word_to_stack(cpu_state, cpu_state.registers.program_counter);
    cpu_state.registers.program_counter = new_address;
}
