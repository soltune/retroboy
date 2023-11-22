use super::*;
use crate::cpu;
use crate::mmu;

fn init_cpu_with_test_instructions(test_instructions: Vec<u8>) -> cpu::CpuState {
    let mut cpu_state = cpu::initialize_cpu_state();
    cpu_state.memory.in_bios = false;
    mmu::load_rom_buffer(&mut cpu_state.memory, test_instructions);
    cpu_state
}

#[test]
fn loads_immediate_byte_into_register_b() {
    let mut cpu_state = init_cpu_with_test_instructions(vec![0x06, 0xA1]);
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.b, 0xA1);
    assert_eq!(cpu_state.registers.program_counter, 2);
    assert_eq!(cpu_state.clock.total_clock_cycles, 8);
}

#[test]
fn loads_register_b_into_register_a() {
    let mut cpu_state = init_cpu_with_test_instructions(vec![0x78]);
    cpu_state.registers.b = 0x2F;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.a, 0x2F);
    assert_eq!(cpu_state.registers.program_counter, 1);
    assert_eq!(cpu_state.clock.total_clock_cycles, 4);
}

#[test]
fn loads_byte_at_address_hl_into_register_a() {
    let mut cpu_state = init_cpu_with_test_instructions(vec![0x7e]);
    cpu_state.registers.h = 0x55;
    cpu_state.registers.l = 0x50;
    cpu_state.memory.rom[0x5550] = 0xB1;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.a, 0xB1);
    assert_eq!(cpu_state.registers.program_counter, 1);
    assert_eq!(cpu_state.clock.total_clock_cycles, 8);
}

#[test]
fn loads_register_b_into_address_hl() {
    let mut cpu_state = init_cpu_with_test_instructions(vec![0x70]);
    cpu_state.registers.b = 0x5A;
    cpu_state.registers.h = 0x41;
    cpu_state.registers.l = 0x9B;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.memory.rom[0x419B], 0x5A);
    assert_eq!(cpu_state.clock.total_clock_cycles, 8);
}

#[test]
fn loads_immediate_byte_into_memory() {
    let mut cpu_state = init_cpu_with_test_instructions(vec![0x36, 0xE6]);
    cpu_state.registers.h = 0x52;
    cpu_state.registers.l = 0x44;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.memory.rom[0x5244], 0xE6);
    assert_eq!(cpu_state.clock.total_clock_cycles, 12);
}

#[test]
fn loads_byte_at_address_nn_into_register_a() {
    let mut cpu_state = init_cpu_with_test_instructions(vec![0xFA, 0x1C, 0x4B]);
    cpu_state.memory.rom[0x4B1C] = 0x22;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.a, 0x22);
    assert_eq!(cpu_state.clock.total_clock_cycles, 16);
}

#[test]
fn loads_byte_at_ff00_plus_register_c_into_register_a() {
    let mut cpu_state = init_cpu_with_test_instructions(vec![0xF2]);
    cpu_state.memory.zero_page_ram[0x1B] = 0x9A;
    cpu_state.registers.c = 0x9B;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.a, 0x9A);
    assert_eq!(cpu_state.clock.total_clock_cycles, 8);
}

#[test]
fn loads_register_a_into_ff00_plus_register_c() {
    let mut cpu_state = init_cpu_with_test_instructions(vec![0xE2]);
    cpu_state.registers.a = 0x9A;
    cpu_state.registers.c = 0x9B;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.memory.zero_page_ram[0x1B], 0x9A);
    assert_eq!(cpu_state.clock.total_clock_cycles, 8);
}

#[test]
fn loads_byte_at_address_hl_into_register_a_then_decrements_hl() {
    let mut cpu_state = init_cpu_with_test_instructions(vec![0x3a]);
    cpu_state.registers.h = 0x2A;
    cpu_state.registers.l = 0xB1;
    cpu_state.memory.rom[0x2AB1] = 0xAA;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.a, 0xAA);
    assert_eq!(cpu_state.registers.program_counter, 1);
    assert_eq!(cpu_state.registers.h, 0x2A);
    assert_eq!(cpu_state.registers.l, 0xB0);
    assert_eq!(cpu_state.clock.total_clock_cycles, 8);
}

#[test]
fn loads_register_a_into_address_hl_then_decrements_hl() {
    let mut cpu_state = init_cpu_with_test_instructions(vec![0x32]);
    cpu_state.registers.a = 0xBB;
    cpu_state.registers.h = 0x2A;
    cpu_state.registers.l = 0xB1;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.program_counter, 1);
    assert_eq!(cpu_state.memory.rom[0x2AB1], 0xBB);
    assert_eq!(cpu_state.registers.h, 0x2A);
    assert_eq!(cpu_state.registers.l, 0xB0);
    assert_eq!(cpu_state.clock.total_clock_cycles, 8);
}


#[test]
fn loads_byte_at_address_hl_into_register_a_then_increments_hl() {
    let mut cpu_state = init_cpu_with_test_instructions(vec![0x2A]);
    cpu_state.registers.h = 0x2A;
    cpu_state.registers.l = 0xB1;
    cpu_state.memory.rom[0x2AB1] = 0xAA;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.a, 0xAA);
    assert_eq!(cpu_state.registers.program_counter, 1);
    assert_eq!(cpu_state.registers.h, 0x2A);
    assert_eq!(cpu_state.registers.l, 0xB2);
    assert_eq!(cpu_state.clock.total_clock_cycles, 8);
}

#[test]
fn loads_register_a_into_address_hl_then_increments_hl() {
    let mut cpu_state = init_cpu_with_test_instructions(vec![0x22]);
    cpu_state.registers.a = 0xBB;
    cpu_state.registers.h = 0x2A;
    cpu_state.registers.l = 0xB1;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.program_counter, 1);
    assert_eq!(cpu_state.memory.rom[0x2AB1], 0xBB);
    assert_eq!(cpu_state.registers.h, 0x2A);
    assert_eq!(cpu_state.registers.l, 0xB2);
    assert_eq!(cpu_state.clock.total_clock_cycles, 8);
}

#[test]
fn loads_register_a_into_ff00_plus_immediate_byte() {
    let mut cpu_state = init_cpu_with_test_instructions(vec![0xE0, 0xB1]);
    cpu_state.registers.a = 0x9A;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.memory.zero_page_ram[0x31], 0x9A);
    assert_eq!(cpu_state.clock.total_clock_cycles, 12);
}

#[test]
fn loads_byte_at_address_ff00_plus_immediate_byte_into_register_a() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xF0, 0xB1]);
    cpu_state.memory.zero_page_ram[0x31] = 0x9A;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.a, 0x9A);
    assert_eq!(cpu_state.clock.total_clock_cycles, 12);
}

#[test]
fn loads_immediate_word_into_register_pair_bc() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0x01, 0xA2, 0xA3]);
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.b, 0xA3);
    assert_eq!(cpu_state.registers.c, 0xA2);
    assert_eq!(cpu_state.clock.total_clock_cycles, 12);
}

#[test]
fn loads_immediate_word_into_stack_pointer() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0x31, 0xA2, 0xA3]);
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.stack_pointer, 0xA3A2);
    assert_eq!(cpu_state.clock.total_clock_cycles, 12);
}

#[test]
fn loads_word_at_register_pair_hl_into_stack_pointer() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xF9]);
    cpu_state.registers.h = 0xAB;
    cpu_state.registers.l = 0x13;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.stack_pointer, 0xAB13);
    assert_eq!(cpu_state.clock.total_clock_cycles, 8);
}

#[test]
fn loads_stack_pointer_plus_immediate_byte_into_register_pair_hl_with_half_carry() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xF8, 0x19]);
    cpu_state.registers.stack_pointer = 0xB207;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.h, 0xB2);
    assert_eq!(cpu_state.registers.l, 0x20);
    assert_eq!(cpu_state.registers.stack_pointer, 0xB207);
    assert_eq!(cpu_state.registers.f, 0x20);
    assert_eq!(cpu_state.clock.total_clock_cycles, 12);
}

#[test]
fn loads_stack_pointer_plus_immediate_byte_into_register_pair_hl_with_carry() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xF8, 0x19]);
    cpu_state.registers.stack_pointer = 0xB2F7;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.h, 0xB3);
    assert_eq!(cpu_state.registers.l, 0x10);
    assert_eq!(cpu_state.registers.stack_pointer, 0xB2F7);
    assert_eq!(cpu_state.registers.f, 0x30);
    assert_eq!(cpu_state.clock.total_clock_cycles, 12);
}

#[test]
fn loads_stack_pointer_into_address_nn() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0x08, 0x13, 0x32]);
    cpu_state.registers.stack_pointer = 0x9BB2;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.memory.rom[0x3213], 0xB2);
    assert_eq!(cpu_state.memory.rom[0x3214], 0x9B);
    assert_eq!(cpu_state.clock.total_clock_cycles, 20);
}

#[test]
fn pushes_register_pair_onto_stack() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xC5]);
    cpu_state.registers.b = 0xB1;
    cpu_state.registers.c = 0xDD;
    cpu_state.registers.stack_pointer = 0x2112;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.memory.rom[0x2112], 0xDD);
    assert_eq!(cpu_state.memory.rom[0x2113], 0xB1);
    assert_eq!(cpu_state.registers.stack_pointer, 0x2110);
    assert_eq!(cpu_state.clock.total_clock_cycles, 16);
}