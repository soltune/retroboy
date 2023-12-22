use crate::cpu::{CpuState, read_next_instruction_byte, read_next_instruction_word};

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
