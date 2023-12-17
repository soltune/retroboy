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
    assert_eq!(cpu_state.memory.rom[0x2111], 0xB1);
    assert_eq!(cpu_state.memory.rom[0x2110], 0xDD);
    assert_eq!(cpu_state.registers.stack_pointer, 0x2110);
    assert_eq!(cpu_state.clock.total_clock_cycles, 16);
}

#[test]
fn pops_word_into_register_pair_from_stack() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xC1]);
    cpu_state.registers.stack_pointer = 0x2110;
    cpu_state.memory.rom[0x2111] = 0xB1;
    cpu_state.memory.rom[0x2110] = 0xDD;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.b, 0xB1);
    assert_eq!(cpu_state.registers.c, 0xDD);
    assert_eq!(cpu_state.registers.stack_pointer, 0x2112);
    assert_eq!(cpu_state.clock.total_clock_cycles, 12);
}

#[test]
fn adds_register_and_register_a_with_half_carry() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0x80]);
    cpu_state.registers.a = 0x2B;
    cpu_state.registers.b = 0xAF;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.a, 0xDA);
    assert_eq!(cpu_state.registers.b, 0xAF);
    assert_eq!(cpu_state.registers.f, 0x20);
    assert_eq!(cpu_state.clock.total_clock_cycles, 4);
}

#[test]
fn adds_register_and_register_a_with_carry() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0x80]);
    cpu_state.registers.a = 0xC1;
    cpu_state.registers.b = 0x5A;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.a, 0x1B);
    assert_eq!(cpu_state.registers.b, 0x5A);
    assert_eq!(cpu_state.registers.f, 0x10);
    assert_eq!(cpu_state.clock.total_clock_cycles, 4);
}

#[test]
fn adds_register_and_register_a_and_carry_flag() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0x88]);
    cpu_state.registers.a = 0x2B;
    cpu_state.registers.b = 0xBE;
    cpu_state.registers.f = 0x10;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.a, 0xEA);
    assert_eq!(cpu_state.registers.b, 0xBE);
    assert_eq!(cpu_state.registers.f, 0x20);
    assert_eq!(cpu_state.clock.total_clock_cycles, 4);
}

#[test]
fn subtracts_register_value_from_register_a_with_half_carry() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0x90]);
    cpu_state.registers.a = 0xB1;
    cpu_state.registers.b = 0x7F;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.a, 0x32);
    assert_eq!(cpu_state.registers.b, 0x7F);
    assert_eq!(cpu_state.registers.f, 0x60);
    assert_eq!(cpu_state.clock.total_clock_cycles, 4);
}

#[test]
fn subtracts_register_value_from_register_a_with_carry() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0x90]);
    cpu_state.registers.a = 0x02;
    cpu_state.registers.b = 0x04;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.a, 0xFE);
    assert_eq!(cpu_state.registers.b, 0x04);
    assert_eq!(cpu_state.registers.f, 0x70);
    assert_eq!(cpu_state.clock.total_clock_cycles, 4);
}

#[test]
fn subtracts_register_value_plus_carry_from_register_a_with_half_carry() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0x98]);
    cpu_state.registers.a = 0xB1;
    cpu_state.registers.b = 0x74;
    cpu_state.registers.f = 0x10;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.a, 0x3C);
    assert_eq!(cpu_state.registers.b, 0x74);
    assert_eq!(cpu_state.registers.f, 0x60);
    assert_eq!(cpu_state.clock.total_clock_cycles, 4);
}

#[test]
fn subtracts_register_value_plus_carry_from_register_a_with_carry() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0x98]);
    cpu_state.registers.a = 0x02;
    cpu_state.registers.b = 0x04;
    cpu_state.registers.f = 0x10;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.a, 0xFD);
    assert_eq!(cpu_state.registers.b, 0x04);
    assert_eq!(cpu_state.registers.f, 0x70);
    assert_eq!(cpu_state.clock.total_clock_cycles, 4);
}

#[test]
fn logical_ands_register_and_register_a() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xA0]);
    cpu_state.registers.a = 0x15;
    cpu_state.registers.b = 0x7E;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.a, 0x14);
    assert_eq!(cpu_state.registers.b, 0x7E);
    assert_eq!(cpu_state.registers.f, 0x20);
    assert_eq!(cpu_state.clock.total_clock_cycles, 4);
}

#[test]
fn logical_ors_register_and_register_a() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xB0]);
    cpu_state.registers.a = 0x15;
    cpu_state.registers.b = 0x7E;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.a, 0x7F);
    assert_eq!(cpu_state.registers.b, 0x7E);
    assert_eq!(cpu_state.registers.f, 0x0);
    assert_eq!(cpu_state.clock.total_clock_cycles, 4);
}

#[test]
fn logical_xors_register_and_register_a() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xA8]);
    cpu_state.registers.a = 0x15;
    cpu_state.registers.b = 0x7E;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.a, 0x6B);
    assert_eq!(cpu_state.registers.b, 0x7E);
    assert_eq!(cpu_state.registers.f, 0x0);
    assert_eq!(cpu_state.clock.total_clock_cycles, 4);
}

#[test]
#[should_panic(expected = "Encountered illegal opcode 0xFC")]
fn panics_on_illegal_opcode() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xFC]);
    execute_opcode(&mut cpu_state);
}

#[test]
fn compares_register_value_with_register_a_resulting_in_half_carry() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xB8]);
    cpu_state.registers.a = 0xB1;
    cpu_state.registers.b = 0x7F;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.a, 0xB1);
    assert_eq!(cpu_state.registers.b, 0x7F);
    assert_eq!(cpu_state.registers.f, 0x60);
    assert_eq!(cpu_state.clock.total_clock_cycles, 4);
}

#[test]
fn compares_register_value_with_register_a_resulting_in_carry() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xB8]);
    cpu_state.registers.a = 0x02;
    cpu_state.registers.b = 0x04;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.a, 0x02);
    assert_eq!(cpu_state.registers.b, 0x04);
    assert_eq!(cpu_state.registers.f, 0x70);
    assert_eq!(cpu_state.clock.total_clock_cycles, 4);
}

#[test]
fn increments_register_with_half_carry() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0x04]);
    cpu_state.registers.b = 0x0F;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.b, 0x10);
    assert_eq!(cpu_state.registers.f, 0x20);
    assert_eq!(cpu_state.clock.total_clock_cycles, 4);
}

#[test]
fn increments_register_without_carry() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0x04]);
    cpu_state.registers.b = 0xA3;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.b, 0xA4);
    assert_eq!(cpu_state.registers.f, 0x00);
    assert_eq!(cpu_state.clock.total_clock_cycles, 4);
}

#[test]
fn decrements_register_with_half_carry() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0x05]);
    cpu_state.registers.b = 0x10;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.b, 0x0F);
    assert_eq!(cpu_state.registers.f, 0x60);
    assert_eq!(cpu_state.clock.total_clock_cycles, 4);
}

#[test]
fn decrements_register_without_carry() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0x05]);
    cpu_state.registers.b = 0xA3;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.b, 0xA2);
    assert_eq!(cpu_state.registers.f, 0x40);
    assert_eq!(cpu_state.clock.total_clock_cycles, 4);
}

#[test]
fn increments_memory_byte_with_half_carry() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0x34]);
    cpu_state.memory.rom[0x2C11] = 0x0F;
    cpu_state.registers.h = 0x2C;
    cpu_state.registers.l = 0x11;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.memory.rom[0x2C11], 0x10);
    assert_eq!(cpu_state.registers.f, 0x20);
    assert_eq!(cpu_state.clock.total_clock_cycles, 12);
}

#[test]
fn decrements_memory_byte_with_half_carry() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0x35]);
    cpu_state.memory.rom[0x2C11] = 0x10;
    cpu_state.registers.h = 0x2C;
    cpu_state.registers.l = 0x11;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.memory.rom[0x2C11], 0x0F);
    assert_eq!(cpu_state.registers.f, 0x60);
    assert_eq!(cpu_state.clock.total_clock_cycles, 12);
}

#[test]
fn increments_register_pair() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0x03]);
    cpu_state.registers.b = 0x3C;
    cpu_state.registers.c = 0x4D;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.b, 0x3C);
    assert_eq!(cpu_state.registers.c, 0x4E);
    assert_eq!(cpu_state.clock.total_clock_cycles, 8);
}

#[test]
fn decrements_register_pair() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0x0B]);
    cpu_state.registers.b = 0x3C;
    cpu_state.registers.c = 0x4D;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.b, 0x3C);
    assert_eq!(cpu_state.registers.c, 0x4C);
    assert_eq!(cpu_state.clock.total_clock_cycles, 8);
}

#[test]
fn increments_stack_pointer() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0x33]);
    cpu_state.registers.stack_pointer = 0x1A33;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.stack_pointer, 0x1A34);
    assert_eq!(cpu_state.clock.total_clock_cycles, 8);
}

#[test]
fn decrements_stack_pointer() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0x3B]);
    cpu_state.registers.stack_pointer = 0x1A33;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.stack_pointer, 0x1A32);
    assert_eq!(cpu_state.clock.total_clock_cycles, 8);
}

#[test]
fn adds_register_pair_and_register_pair_hl_with_carry() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0x09]);
    cpu_state.registers.h = 0xFF;
    cpu_state.registers.l = 0xFE;
    cpu_state.registers.b = 0x00;
    cpu_state.registers.c = 0x04;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.h, 0x00);
    assert_eq!(cpu_state.registers.l, 0x02);
    assert_eq!(cpu_state.registers.b, 0x00);
    assert_eq!(cpu_state.registers.c, 0x04);
    assert_eq!(cpu_state.registers.f, 0x30);
    assert_eq!(cpu_state.clock.total_clock_cycles, 8);
}

#[test]
fn adds_register_pair_and_register_pair_hl_with_half_carry() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0x09]);
    cpu_state.registers.h = 0xDF;
    cpu_state.registers.l = 0xFF;
    cpu_state.registers.b = 0x00;
    cpu_state.registers.c = 0x01;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.h, 0xE0);
    assert_eq!(cpu_state.registers.l, 0x00);
    assert_eq!(cpu_state.registers.b, 0x00);
    assert_eq!(cpu_state.registers.c, 0x01);
    assert_eq!(cpu_state.registers.f, 0x20);
    assert_eq!(cpu_state.clock.total_clock_cycles, 8);
}

#[test]
fn loads_stack_pointer_plus_immediate_byte_into_stack_pointer_with_half_carry() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xE8, 0x19]);
    cpu_state.registers.stack_pointer = 0xB207;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.stack_pointer, 0xB220);
    assert_eq!(cpu_state.registers.f, 0x20);
    assert_eq!(cpu_state.clock.total_clock_cycles, 12);
}

#[test]
fn loads_stack_pointer_plus_immediate_byte_into_stack_pointer_with_carry() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xE8, 0x19]);
    cpu_state.registers.stack_pointer = 0xB2F7;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.stack_pointer, 0xB310);
    assert_eq!(cpu_state.registers.f, 0x30);
    assert_eq!(cpu_state.clock.total_clock_cycles, 12);
}

#[test]
fn swaps_nibbles_in_register() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xCB, 0x37]);
    cpu_state.registers.a = 0xA2;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.a, 0x2A);
    assert_eq!(cpu_state.clock.total_clock_cycles, 8);
}

#[test]
fn swaps_nibbles_in_memory_byte() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xCB, 0x36]);
    cpu_state.memory.rom[0x4AB1] = 0xBC;
    cpu_state.registers.h = 0x4A;
    cpu_state.registers.l = 0xB1;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.memory.rom[0x4AB1], 0xCB);
    assert_eq!(cpu_state.clock.total_clock_cycles, 16);
}

#[test]
fn sets_carry_flag() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0x37]);
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.f, 0x10);
    assert_eq!(cpu_state.clock.total_clock_cycles, 4);
}

#[test]
fn complement_a_register() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0x2F]);
    cpu_state.registers.a = 0x4C;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.a, 0xB3);
    assert_eq!(cpu_state.registers.f, 0x60);
    assert_eq!(cpu_state.clock.total_clock_cycles, 4);
}

#[test]
fn complement_c_flag() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0x3F]);
    cpu_state.registers.f = 0x30;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.f, 0x00);
    assert_eq!(cpu_state.clock.total_clock_cycles, 4);
}

#[test]
fn decimal_adjusts_register_a() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0x27]);
    cpu_state.registers.a = 0xC0;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.a, 0x20);
    assert_eq!(cpu_state.registers.f, 0x10);
    assert_eq!(cpu_state.clock.total_clock_cycles, 4);
}

#[test]
fn rotates_register_a_left() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0x07]);
    cpu_state.registers.a = 0xA7;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.a, 0x4F);
    assert_eq!(cpu_state.registers.f, 0x10);
    assert_eq!(cpu_state.clock.total_clock_cycles, 4);
}

#[test]
fn rotates_register_a_left_through_carry() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0x17]);
    cpu_state.registers.a = 0xA7;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.a, 0x4E);
    assert_eq!(cpu_state.registers.f, 0x10);
    assert_eq!(cpu_state.clock.total_clock_cycles, 4);
}

#[test]
fn rotates_register_a_right() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0x0F]);
    cpu_state.registers.a = 0xA7;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.a, 0xD3);
    assert_eq!(cpu_state.registers.f, 0x10);
    assert_eq!(cpu_state.clock.total_clock_cycles, 4);
}

#[test]
fn rotates_register_a_right_through_carry() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0x1F]);
    cpu_state.registers.a = 0xA7;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.a, 0x53);
    assert_eq!(cpu_state.registers.f, 0x10);
    assert_eq!(cpu_state.clock.total_clock_cycles, 4);
}

#[test]
fn rotates_memory_location_hl_left() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xCB, 0x06]);
    cpu_state.registers.h = 0x1A;
    cpu_state.registers.l = 0x51;
    cpu_state.memory.rom[0x1A51] = 0xA7;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.memory.rom[0x1A51], 0x4F);
    assert_eq!(cpu_state.registers.f, 0x10);
    assert_eq!(cpu_state.clock.total_clock_cycles, 16);
}

#[test]
fn rotates_memory_location_hl_left_through_carry() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xCB, 0x16]);
    cpu_state.registers.h = 0x1A;
    cpu_state.registers.l = 0x51;
    cpu_state.memory.rom[0x1A51] = 0xA7;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.memory.rom[0x1A51], 0x4E);
    assert_eq!(cpu_state.registers.f, 0x10);
    assert_eq!(cpu_state.clock.total_clock_cycles, 16);
}

#[test]
fn rotates_memory_location_hl_right() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xCB, 0x0E]);
    cpu_state.registers.h = 0x1A;
    cpu_state.registers.l = 0x51;
    cpu_state.memory.rom[0x1A51] = 0xA7;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.memory.rom[0x1A51], 0xD3);
    assert_eq!(cpu_state.registers.f, 0x10);
    assert_eq!(cpu_state.clock.total_clock_cycles, 16);
}

#[test]
fn rotates_memory_location_hl_right_through_carry() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xCB, 0x1E]);
    cpu_state.registers.h = 0x1A;
    cpu_state.registers.l = 0x51;
    cpu_state.memory.rom[0x1A51] = 0xA7;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.memory.rom[0x1A51], 0x53);
    assert_eq!(cpu_state.registers.f, 0x10);
    assert_eq!(cpu_state.clock.total_clock_cycles, 16);
}

#[test]
fn shifts_register_a_left() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xCB, 0x27]);
    cpu_state.registers.a = 0xA7;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.a, 0x4E);
    assert_eq!(cpu_state.registers.f, 0x10);
    assert_eq!(cpu_state.clock.total_clock_cycles, 8);
}

#[test]
fn shifts_memory_location_hl_left() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xCB, 0x26]);
    cpu_state.registers.h = 0x1A;
    cpu_state.registers.l = 0x51;
    cpu_state.memory.rom[0x1A51] = 0xA7;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.memory.rom[0x1A51], 0x4E);
    assert_eq!(cpu_state.registers.f, 0x10);
    assert_eq!(cpu_state.clock.total_clock_cycles, 16);
}

#[test]
fn shifts_register_a_right_maintaining_msb() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xCB, 0x2F]);
    cpu_state.registers.a = 0xA7;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.a, 0xD3);
    assert_eq!(cpu_state.registers.f, 0x10);
    assert_eq!(cpu_state.clock.total_clock_cycles, 8);
}

#[test]
fn shifts_memory_location_hl_right_maintaining_msb() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xCB, 0x2E]);
    cpu_state.registers.h = 0x1A;
    cpu_state.registers.l = 0x51;
    cpu_state.memory.rom[0x1A51] = 0xA7;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.memory.rom[0x1A51], 0xD3);
    assert_eq!(cpu_state.registers.f, 0x10);
    assert_eq!(cpu_state.clock.total_clock_cycles, 16);
}

#[test]
fn shifts_register_a_right() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xCB, 0x3F]);
    cpu_state.registers.a = 0xA7;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.a, 0x53);
    assert_eq!(cpu_state.registers.f, 0x10);
    assert_eq!(cpu_state.clock.total_clock_cycles, 8);
}

#[test]
fn shifts_memory_location_hl_right() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xCB, 0x3E]);
    cpu_state.registers.h = 0x1A;
    cpu_state.registers.l = 0x51;
    cpu_state.memory.rom[0x1A51] = 0xA7;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.memory.rom[0x1A51], 0x53);
    assert_eq!(cpu_state.registers.f, 0x10);
    assert_eq!(cpu_state.clock.total_clock_cycles, 16);
}

#[test]
fn test_bit_0_of_register_a() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xCB, 0x47]);
    cpu_state.registers.a = 0xB5;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.f, 0x20);
    assert_eq!(cpu_state.clock.total_clock_cycles, 8);
}

#[test]
fn test_bit_1_of_register_a() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xCB, 0x4F]);
    cpu_state.registers.a = 0xB5;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.f, 0xA0);
    assert_eq!(cpu_state.clock.total_clock_cycles, 8);
}

#[test]
fn test_bit_2_of_register_a() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xCB, 0x57]);
    cpu_state.registers.a = 0xB5;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.f, 0x20);
    assert_eq!(cpu_state.clock.total_clock_cycles, 8);
}

#[test]
fn test_bit_3_of_register_a() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xCB, 0x5F]);
    cpu_state.registers.a = 0xB5;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.f, 0xA0);
    assert_eq!(cpu_state.clock.total_clock_cycles, 8);
}

#[test]
fn test_bit_4_of_register_a() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xCB, 0x67]);
    cpu_state.registers.a = 0xB5;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.f, 0x20);
    assert_eq!(cpu_state.clock.total_clock_cycles, 8);
}

#[test]
fn test_bit_5_of_register_a() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xCB, 0x6F]);
    cpu_state.registers.a = 0xB5;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.f, 0x20);
    assert_eq!(cpu_state.clock.total_clock_cycles, 8);
}

#[test]
fn test_bit_6_of_register_a() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xCB, 0x77]);
    cpu_state.registers.a = 0xB5;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.f, 0xA0);
    assert_eq!(cpu_state.clock.total_clock_cycles, 8);
}

#[test]
fn test_bit_7_of_register_a() {
    let mut cpu_state: CpuState = init_cpu_with_test_instructions(vec![0xCB, 0x7F]);
    cpu_state.registers.a = 0xB5;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.f, 0x20);
    assert_eq!(cpu_state.clock.total_clock_cycles, 8);
}
