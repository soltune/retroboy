use super::*;
use crate::cpu::{BusActivityEntry, BusActivityType};
use crate::emulator::{initialize_screenless_emulator, Mode};
use crate::mmu;
use crate::mmu::constants::*;
use crate::mmu::effects::empty_cartridge_effects;
use crate::mmu::test_utils::build_rom;

fn init_rom_with_test_instructions(test_instructions: Vec<u8>) -> Vec<u8> {
    let mut rom = build_rom(CART_TYPE_ROM_ONLY, ROM_SIZE_64KB, RAM_SIZE_2KB);
    for i in 0..test_instructions.len() {
        rom[i] = test_instructions[i];
    }
    rom
}

fn init_emulator_from_rom(rom: Vec<u8>) -> Emulator {
    let mut emulator = initialize_screenless_emulator();
    mmu::load_rom_buffer(&mut emulator.memory, rom, empty_cartridge_effects()).unwrap();
    emulator.memory.in_bios = false;

    // The Game Boy actually uses a decode/execute/prefetch loop, where fetching
    // the next instruction is the last step. Initially, ihe first instruction is always a NOP.
    // Source: https://gist.github.com/SonoSooS/c0055300670d678b5ae8433e20bea595#fetch-and-stuff
    // This is why we need to step twice to get to the first opcode under test.
    step(&mut emulator);

    emulator
}

fn init_emulator_with_test_instructions(test_instructions: Vec<u8>) -> Emulator {
    let rom = init_rom_with_test_instructions(test_instructions);
    init_emulator_from_rom(rom)
}

#[test]
fn loads_immediate_byte_into_register_b() {
    let mut emulator = init_emulator_with_test_instructions(vec![0x06, 0xA1]);
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.b, 0xA1);
    assert_eq!(emulator.cpu.registers.program_counter, 3);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn loads_register_b_into_register_a() {
    let mut emulator = init_emulator_with_test_instructions(vec![0x78]);
    emulator.cpu.registers.b = 0x2F;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0x2F);
    assert_eq!(emulator.cpu.registers.program_counter, 2);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 4);
}

#[test]
fn loads_byte_at_address_hl_into_register_a() {
    let mut rom = init_rom_with_test_instructions(vec![0x7e]);
    rom[0x5550] = 0xB1;
    let mut emulator = init_emulator_from_rom(rom);
    emulator.cpu.registers.h = 0x55;
    emulator.cpu.registers.l = 0x50;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0xB1);
    assert_eq!(emulator.cpu.registers.program_counter, 2);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn loads_register_b_into_address_hl() {
    let mut emulator = init_emulator_with_test_instructions(vec![0x70]);
    emulator.cpu.registers.b = 0x5A;
    emulator.cpu.registers.h = 0x81;
    emulator.cpu.registers.l = 0x9B;
    step(&mut emulator);
    assert_eq!(emulator.gpu.video_ram[0x019B], 0x5A);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn loads_immediate_byte_into_memory() {
    let mut emulator = init_emulator_with_test_instructions(vec![0x36, 0xE6]);
    emulator.cpu.registers.h = 0x82;
    emulator.cpu.registers.l = 0x44;
    step(&mut emulator);
    assert_eq!(emulator.gpu.video_ram[0x0244], 0xE6);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 12);
}

#[test]
fn loads_byte_at_address_nn_into_register_a() {
    let mut rom = init_rom_with_test_instructions(vec![0xFA, 0x1C, 0x4B]);
    rom[0x4B1C] = 0x22;
    let mut emulator = init_emulator_from_rom(rom);
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0x22);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 16);
}

#[test]
fn loads_byte_at_ff00_plus_register_c_into_register_a() {
    let mut emulator = init_emulator_with_test_instructions(vec![0xF2]);
    emulator.memory.zero_page_ram[0x1B] = 0x9A;
    emulator.cpu.registers.c = 0x9B;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0x9A);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn loads_register_a_into_ff00_plus_register_c() {
    let mut emulator = init_emulator_with_test_instructions(vec![0xE2]);
    emulator.cpu.registers.a = 0x9A;
    emulator.cpu.registers.c = 0x9B;
    step(&mut emulator);
    assert_eq!(emulator.memory.zero_page_ram[0x1B], 0x9A);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn loads_byte_at_address_hl_into_register_a_then_decrements_hl() {
    let mut rom = init_rom_with_test_instructions(vec![0x3a]);
    rom[0x2AB1] = 0xAA;
    let mut emulator = init_emulator_from_rom(rom);
    emulator.cpu.registers.h = 0x2A;
    emulator.cpu.registers.l = 0xB1;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0xAA);
    assert_eq!(emulator.cpu.registers.program_counter, 2);
    assert_eq!(emulator.cpu.registers.h, 0x2A);
    assert_eq!(emulator.cpu.registers.l, 0xB0);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn loads_register_a_into_address_hl_then_decrements_hl() {
    let mut emulator = init_emulator_with_test_instructions(vec![0x32]);
    emulator.cpu.registers.a = 0xBB;
    emulator.cpu.registers.h = 0x8A;
    emulator.cpu.registers.l = 0xB1;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.program_counter, 2);
    assert_eq!(emulator.gpu.video_ram[0x0AB1], 0xBB);
    assert_eq!(emulator.cpu.registers.h, 0x8A);
    assert_eq!(emulator.cpu.registers.l, 0xB0);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}


#[test]
fn loads_byte_at_address_hl_into_register_a_then_increments_hl() {
    let mut rom = init_rom_with_test_instructions(vec![0x2A]);
    rom[0x2AB1] = 0xAA;
    let mut emulator = init_emulator_from_rom(rom);
    emulator.cpu.registers.h = 0x2A;
    emulator.cpu.registers.l = 0xB1;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0xAA);
    assert_eq!(emulator.cpu.registers.program_counter, 2);
    assert_eq!(emulator.cpu.registers.h, 0x2A);
    assert_eq!(emulator.cpu.registers.l, 0xB2);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn loads_register_a_into_address_hl_then_increments_hl() {
    let mut emulator = init_emulator_with_test_instructions(vec![0x22]);
    emulator.cpu.registers.a = 0xBB;
    emulator.cpu.registers.h = 0x8A;
    emulator.cpu.registers.l = 0xB1;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.program_counter, 2);
    assert_eq!(emulator.gpu.video_ram[0x0AB1], 0xBB);
    assert_eq!(emulator.cpu.registers.h, 0x8A);
    assert_eq!(emulator.cpu.registers.l, 0xB2);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn loads_register_a_into_ff00_plus_immediate_byte() {
    let mut emulator = init_emulator_with_test_instructions(vec![0xE0, 0xB1]);
    emulator.cpu.registers.a = 0x9A;
    step(&mut emulator);
    assert_eq!(emulator.memory.zero_page_ram[0x31], 0x9A);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 12);
}

#[test]
fn loads_byte_at_address_ff00_plus_immediate_byte_into_register_a() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xF0, 0xB1]);
    emulator.memory.zero_page_ram[0x31] = 0x9A;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0x9A);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 12);
}

#[test]
fn loads_immediate_word_into_register_pair_bc() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x01, 0xA2, 0xA3]);
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.b, 0xA3);
    assert_eq!(emulator.cpu.registers.c, 0xA2);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 12);
}

#[test]
fn loads_immediate_word_into_stack_pointer() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x31, 0xA2, 0xA3]);
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.stack_pointer, 0xA3A2);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 12);
}

#[test]
fn loads_word_at_register_pair_hl_into_stack_pointer() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xF9]);
    emulator.cpu.registers.h = 0xAB;
    emulator.cpu.registers.l = 0x13;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.stack_pointer, 0xAB13);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn loads_stack_pointer_plus_immediate_byte_into_register_pair_hl_with_half_carry() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xF8, 0x19]);
    emulator.cpu.registers.stack_pointer = 0xB207;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.h, 0xB2);
    assert_eq!(emulator.cpu.registers.l, 0x20);
    assert_eq!(emulator.cpu.registers.stack_pointer, 0xB207);
    assert_eq!(emulator.cpu.registers.f, 0x20);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 12);
}

#[test]
fn loads_stack_pointer_plus_immediate_byte_into_register_pair_hl_with_carry() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xF8, 0x19]);
    emulator.cpu.registers.stack_pointer = 0xB2F7;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.h, 0xB3);
    assert_eq!(emulator.cpu.registers.l, 0x10);
    assert_eq!(emulator.cpu.registers.stack_pointer, 0xB2F7);
    assert_eq!(emulator.cpu.registers.f, 0x30);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 12);
}

#[test]
fn loads_stack_pointer_into_address_nn() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x08, 0x13, 0x82]);
    emulator.cpu.registers.stack_pointer = 0x9BB2;
    step(&mut emulator);
    assert_eq!(emulator.gpu.video_ram[0x0213], 0xB2);
    assert_eq!(emulator.gpu.video_ram[0x0214], 0x9B);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 20);
}

#[test]
fn pushes_register_pair_onto_stack() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xC5]);
    emulator.cpu.registers.b = 0xB1;
    emulator.cpu.registers.c = 0xDD;
    emulator.cpu.registers.stack_pointer = 0xFFFE;
    step(&mut emulator);
    assert_eq!(emulator.memory.zero_page_ram[0x7D], 0xB1);
    assert_eq!(emulator.memory.zero_page_ram[0x7C], 0xDD);
    assert_eq!(emulator.cpu.registers.stack_pointer, 0xFFFC);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 16);
}

#[test]
fn pops_word_into_register_pair_from_stack() {
    let mut rom = init_rom_with_test_instructions(vec![0xC1]);
    rom[0x2111] = 0xB1;
    rom[0x2110] = 0xDD;
    let mut emulator = init_emulator_from_rom(rom);
    emulator.cpu.registers.stack_pointer = 0x2110;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.b, 0xB1);
    assert_eq!(emulator.cpu.registers.c, 0xDD);
    assert_eq!(emulator.cpu.registers.stack_pointer, 0x2112);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 12);
}

#[test]
fn adds_register_and_register_a_with_half_carry() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x80]);
    emulator.cpu.registers.a = 0x2B;
    emulator.cpu.registers.b = 0xAF;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0xDA);
    assert_eq!(emulator.cpu.registers.b, 0xAF);
    assert_eq!(emulator.cpu.registers.f, 0x20);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 4);
}

#[test]
fn adds_register_and_register_a_with_carry() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x80]);
    emulator.cpu.registers.a = 0xC1;
    emulator.cpu.registers.b = 0x5A;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0x1B);
    assert_eq!(emulator.cpu.registers.b, 0x5A);
    assert_eq!(emulator.cpu.registers.f, 0x10);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 4);
}

#[test]
fn adds_register_and_register_a_and_carry_flag() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x88]);
    emulator.cpu.registers.a = 0x2B;
    emulator.cpu.registers.b = 0xBE;
    emulator.cpu.registers.f = 0x10;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0xEA);
    assert_eq!(emulator.cpu.registers.b, 0xBE);
    assert_eq!(emulator.cpu.registers.f, 0x20);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 4);
}

#[test]
fn subtracts_register_value_from_register_a_with_half_carry() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x90]);
    emulator.cpu.registers.a = 0xB1;
    emulator.cpu.registers.b = 0x7F;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0x32);
    assert_eq!(emulator.cpu.registers.b, 0x7F);
    assert_eq!(emulator.cpu.registers.f, 0x60);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 4);
}

#[test]
fn subtracts_register_value_from_register_a_with_carry() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x90]);
    emulator.cpu.registers.a = 0x02;
    emulator.cpu.registers.b = 0x04;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0xFE);
    assert_eq!(emulator.cpu.registers.b, 0x04);
    assert_eq!(emulator.cpu.registers.f, 0x70);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 4);
}

#[test]
fn subtracts_register_value_plus_carry_from_register_a_with_half_carry() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x98]);
    emulator.cpu.registers.a = 0xB1;
    emulator.cpu.registers.b = 0x74;
    emulator.cpu.registers.f = 0x10;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0x3C);
    assert_eq!(emulator.cpu.registers.b, 0x74);
    assert_eq!(emulator.cpu.registers.f, 0x60);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 4);
}

#[test]
fn subtracts_register_value_plus_carry_from_register_a_with_carry() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x98]);
    emulator.cpu.registers.a = 0x02;
    emulator.cpu.registers.b = 0x04;
    emulator.cpu.registers.f = 0x10;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0xFD);
    assert_eq!(emulator.cpu.registers.b, 0x04);
    assert_eq!(emulator.cpu.registers.f, 0x70);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 4);
}

#[test]
fn logical_ands_register_and_register_a() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xA0]);
    emulator.cpu.registers.a = 0x15;
    emulator.cpu.registers.b = 0x7E;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0x14);
    assert_eq!(emulator.cpu.registers.b, 0x7E);
    assert_eq!(emulator.cpu.registers.f, 0x20);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 4);
}

#[test]
fn logical_ors_register_and_register_a() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xB0]);
    emulator.cpu.registers.a = 0x15;
    emulator.cpu.registers.b = 0x7E;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0x7F);
    assert_eq!(emulator.cpu.registers.b, 0x7E);
    assert_eq!(emulator.cpu.registers.f, 0x0);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 4);
}

#[test]
fn logical_xors_register_and_register_a() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xA8]);
    emulator.cpu.registers.a = 0x15;
    emulator.cpu.registers.b = 0x7E;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0x6B);
    assert_eq!(emulator.cpu.registers.b, 0x7E);
    assert_eq!(emulator.cpu.registers.f, 0x0);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 4);
}

#[test]
#[should_panic(expected = "Encountered illegal opcode 0xFC")]
fn panics_on_illegal_opcode() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xFC]);
    step(&mut emulator);
}

#[test]
fn compares_register_value_with_register_a_resulting_in_half_carry() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xB8]);
    emulator.cpu.registers.a = 0xB1;
    emulator.cpu.registers.b = 0x7F;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0xB1);
    assert_eq!(emulator.cpu.registers.b, 0x7F);
    assert_eq!(emulator.cpu.registers.f, 0x60);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 4);
}

#[test]
fn compares_register_value_with_register_a_resulting_in_carry() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xB8]);
    emulator.cpu.registers.a = 0x02;
    emulator.cpu.registers.b = 0x04;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0x02);
    assert_eq!(emulator.cpu.registers.b, 0x04);
    assert_eq!(emulator.cpu.registers.f, 0x70);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 4);
}

#[test]
fn increments_register_with_half_carry() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x04]);
    emulator.cpu.registers.b = 0x0F;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.b, 0x10);
    assert_eq!(emulator.cpu.registers.f, 0x20);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 4);
}

#[test]
fn increments_register_without_carry() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x04]);
    emulator.cpu.registers.b = 0xA3;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.b, 0xA4);
    assert_eq!(emulator.cpu.registers.f, 0x00);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 4);
}

#[test]
fn decrements_register_with_half_carry() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x05]);
    emulator.cpu.registers.b = 0x10;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.b, 0x0F);
    assert_eq!(emulator.cpu.registers.f, 0x60);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 4);
}

#[test]
fn decrements_register_without_carry() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x05]);
    emulator.cpu.registers.b = 0xA3;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.b, 0xA2);
    assert_eq!(emulator.cpu.registers.f, 0x40);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 4);
}

#[test]
fn increments_memory_byte_with_half_carry() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x34]);
    emulator.gpu.video_ram[0x0C11] = 0x0F;
    emulator.cpu.registers.h = 0x8C;
    emulator.cpu.registers.l = 0x11;
    step(&mut emulator);
    assert_eq!(emulator.gpu.video_ram[0x0C11], 0x10);
    assert_eq!(emulator.cpu.registers.f, 0x20);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 12);
}

#[test]
fn decrements_memory_byte_with_half_carry() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x35]);
    emulator.gpu.video_ram[0x0C11] = 0x10;
    emulator.cpu.registers.h = 0x8C;
    emulator.cpu.registers.l = 0x11;
    step(&mut emulator);
    assert_eq!(emulator.gpu.video_ram[0x0C11], 0x0F);
    assert_eq!(emulator.cpu.registers.f, 0x60);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 12);
}

#[test]
fn increments_register_pair() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x03]);
    emulator.cpu.registers.b = 0x3C;
    emulator.cpu.registers.c = 0x4D;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.b, 0x3C);
    assert_eq!(emulator.cpu.registers.c, 0x4E);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn decrements_register_pair() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x0B]);
    emulator.cpu.registers.b = 0x3C;
    emulator.cpu.registers.c = 0x4D;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.b, 0x3C);
    assert_eq!(emulator.cpu.registers.c, 0x4C);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn increments_stack_pointer() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x33]);
    emulator.cpu.registers.stack_pointer = 0x1A33;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.stack_pointer, 0x1A34);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn decrements_stack_pointer() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x3B]);
    emulator.cpu.registers.stack_pointer = 0x1A33;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.stack_pointer, 0x1A32);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn adds_register_pair_and_register_pair_hl_with_carry() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x09]);
    emulator.cpu.registers.h = 0xFF;
    emulator.cpu.registers.l = 0xFE;
    emulator.cpu.registers.b = 0x00;
    emulator.cpu.registers.c = 0x04;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.h, 0x00);
    assert_eq!(emulator.cpu.registers.l, 0x02);
    assert_eq!(emulator.cpu.registers.b, 0x00);
    assert_eq!(emulator.cpu.registers.c, 0x04);
    assert_eq!(emulator.cpu.registers.f, 0x30);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn adds_register_pair_and_register_pair_hl_with_half_carry() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x09]);
    emulator.cpu.registers.h = 0xDF;
    emulator.cpu.registers.l = 0xFF;
    emulator.cpu.registers.b = 0x00;
    emulator.cpu.registers.c = 0x01;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.h, 0xE0);
    assert_eq!(emulator.cpu.registers.l, 0x00);
    assert_eq!(emulator.cpu.registers.b, 0x00);
    assert_eq!(emulator.cpu.registers.c, 0x01);
    assert_eq!(emulator.cpu.registers.f, 0x20);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn loads_stack_pointer_plus_immediate_byte_into_stack_pointer_with_half_carry() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xE8, 0x19]);
    emulator.cpu.registers.stack_pointer = 0xB207;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.stack_pointer, 0xB220);
    assert_eq!(emulator.cpu.registers.f, 0x20);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 16);
}

#[test]
fn loads_stack_pointer_plus_immediate_byte_into_stack_pointer_with_carry() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xE8, 0x19]);
    emulator.cpu.registers.stack_pointer = 0xB2F7;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.stack_pointer, 0xB310);
    assert_eq!(emulator.cpu.registers.f, 0x30);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 16);
}

#[test]
fn swaps_nibbles_in_register() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0x37]);
    emulator.cpu.registers.a = 0xA2;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0x2A);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn swaps_nibbles_in_memory_byte() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0x36]);
    emulator.gpu.video_ram[0x0AB1] = 0xBC;
    emulator.cpu.registers.h = 0x8A;
    emulator.cpu.registers.l = 0xB1;
    step(&mut emulator);
    assert_eq!(emulator.gpu.video_ram[0x0AB1], 0xCB);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 16);
}

#[test]
fn sets_carry_flag() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x37]);
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.f, 0x10);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 4);
}

#[test]
fn complement_a_register() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x2F]);
    emulator.cpu.registers.a = 0x4C;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0xB3);
    assert_eq!(emulator.cpu.registers.f, 0x60);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 4);
}

#[test]
fn complement_c_flag() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x3F]);
    emulator.cpu.registers.f = 0x30;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.f, 0x00);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 4);
}

#[test]
fn decimal_adjusts_register_a() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x27]);
    emulator.cpu.registers.a = 0xC0;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0x20);
    assert_eq!(emulator.cpu.registers.f, 0x10);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 4);
}

#[test]
fn rotates_register_a_left() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x07]);
    emulator.cpu.registers.a = 0xA7;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0x4F);
    assert_eq!(emulator.cpu.registers.f, 0x10);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 4);
}

#[test]
fn rotates_register_a_left_through_carry() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x17]);
    emulator.cpu.registers.a = 0xA7;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0x4E);
    assert_eq!(emulator.cpu.registers.f, 0x10);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 4);
}

#[test]
fn rotates_register_a_left_and_resets_z_flag_even_if_result_is_zero() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x17]);
    emulator.cpu.registers.a = 0x0;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0x0);
    assert_eq!(emulator.cpu.registers.f, 0x0);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 4);
}

#[test]
fn rotates_register_a_right() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x0F]);
    emulator.cpu.registers.a = 0xA7;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0xD3);
    assert_eq!(emulator.cpu.registers.f, 0x10);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 4);
}

#[test]
fn rotates_register_a_right_through_carry() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x1F]);
    emulator.cpu.registers.a = 0xA7;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0x53);
    assert_eq!(emulator.cpu.registers.f, 0x10);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 4);
}

#[test]
fn rotates_memory_location_hl_left() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0x06]);
    emulator.cpu.registers.h = 0x93;
    emulator.cpu.registers.l = 0xDA;
    emulator.gpu.video_ram[0x13DA] = 0xA7;
    step(&mut emulator);
    assert_eq!(emulator.gpu.video_ram[0x13DA], 0x4F);
    assert_eq!(emulator.cpu.registers.f, 0x10);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 16);
}

#[test]
fn rotates_memory_location_hl_left_through_carry() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0x16]);
    emulator.cpu.registers.h = 0x9A;
    emulator.cpu.registers.l = 0x51;
    emulator.gpu.video_ram[0x1A51] = 0xA7;
    step(&mut emulator);
    assert_eq!(emulator.gpu.video_ram[0x1A51], 0x4E);
    assert_eq!(emulator.cpu.registers.f, 0x10);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 16);
}

#[test]
fn rotates_memory_location_hl_right() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0x0E]);
    emulator.cpu.registers.h = 0x9A;
    emulator.cpu.registers.l = 0xAC;
    emulator.gpu.video_ram[0x1AAC] = 0xA7;
    step(&mut emulator);
    assert_eq!(emulator.gpu.video_ram[0x1AAC], 0xD3);
    assert_eq!(emulator.cpu.registers.f, 0x10);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 16);
}

#[test]
fn rotates_memory_location_hl_right_through_carry() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0x1E]);
    emulator.cpu.registers.h = 0x9A;
    emulator.cpu.registers.l = 0x51;
    emulator.gpu.video_ram[0x1A51] = 0xA7;
    step(&mut emulator);
    assert_eq!(emulator.gpu.video_ram[0x1A51], 0x53);
    assert_eq!(emulator.cpu.registers.f, 0x10);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 16);
}

#[test]
fn shifts_register_a_left() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0x27]);
    emulator.cpu.registers.a = 0xA7;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0x4E);
    assert_eq!(emulator.cpu.registers.f, 0x10);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn shifts_memory_location_hl_left() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0x26]);
    emulator.cpu.registers.h = 0x9A;
    emulator.cpu.registers.l = 0x51;
    emulator.gpu.video_ram[0x1A51] = 0xA7;
    step(&mut emulator);
    assert_eq!(emulator.gpu.video_ram[0x1A51], 0x4E);
    assert_eq!(emulator.cpu.registers.f, 0x10);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 16);
}

#[test]
fn shifts_register_a_right_maintaining_msb() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0x2F]);
    emulator.cpu.registers.a = 0xA7;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0xD3);
    assert_eq!(emulator.cpu.registers.f, 0x10);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn shifts_memory_location_hl_right_maintaining_msb() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0x2E]);
    emulator.cpu.registers.h = 0x9A;
    emulator.cpu.registers.l = 0x51;
    emulator.gpu.video_ram[0x1A51] = 0xA7;
    step(&mut emulator);
    assert_eq!(emulator.gpu.video_ram[0x1A51], 0xD3);
    assert_eq!(emulator.cpu.registers.f, 0x10);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 16);
}

#[test]
fn shifts_register_a_right() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0x3F]);
    emulator.cpu.registers.a = 0xA7;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0x53);
    assert_eq!(emulator.cpu.registers.f, 0x10);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn shifts_memory_location_hl_right() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0x3E]);
    emulator.cpu.registers.h = 0x9A;
    emulator.cpu.registers.l = 0x51;
    emulator.gpu.video_ram[0x1A51] = 0xA7;
    step(&mut emulator);
    assert_eq!(emulator.gpu.video_ram[0x1A51], 0x53);
    assert_eq!(emulator.cpu.registers.f, 0x10);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 16);
}

#[test]
fn test_bit_0_of_register_a() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0x47]);
    emulator.cpu.registers.a = 0xB5;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.f, 0x20);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn test_bit_1_of_register_a() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0x4F]);
    emulator.cpu.registers.a = 0xB5;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.f, 0xA0);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn test_bit_2_of_register_a() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0x57]);
    emulator.cpu.registers.a = 0xB5;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.f, 0x20);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn test_bit_3_of_register_a() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0x5F]);
    emulator.cpu.registers.a = 0xB5;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.f, 0xA0);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn test_bit_4_of_register_a() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0x67]);
    emulator.cpu.registers.a = 0xB5;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.f, 0x20);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn test_bit_5_of_register_a() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0x6F]);
    emulator.cpu.registers.a = 0xB5;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.f, 0x20);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn test_bit_6_of_register_a() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0x77]);
    emulator.cpu.registers.a = 0xB5;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.f, 0xA0);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn test_bit_7_of_register_a() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0x7F]);
    emulator.cpu.registers.a = 0xB5;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.f, 0x20);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn reset_bit_0_of_register_a() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0x87]);
    emulator.cpu.registers.a = 0xB5;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0xB4);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn set_bit_0_of_register_a() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0xC7]);
    emulator.cpu.registers.a = 0xB5;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0xB5);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn reset_bit_1_of_register_a() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0x8F]);
    emulator.cpu.registers.a = 0xB5;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0xB5);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn set_bit_1_of_register_a() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0xCF]);
    emulator.cpu.registers.a = 0xB5;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0xB7);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn reset_bit_2_of_register_a() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0x97]);
    emulator.cpu.registers.a = 0xB5;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0xB1);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn set_bit_2_of_register_a() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0xD7]);
    emulator.cpu.registers.a = 0xB5;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0xB5);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn reset_bit_3_of_register_a() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0x9F]);
    emulator.cpu.registers.a = 0xB5;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0xB5);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn set_bit_3_of_register_a() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0xDF]);
    emulator.cpu.registers.a = 0xB5;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0xBD);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn reset_bit_4_of_register_a() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0xA7]);
    emulator.cpu.registers.a = 0xB5;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0xA5);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn set_bit_4_of_register_a() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0xE7]);
    emulator.cpu.registers.a = 0xB5;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0xB5);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn reset_bit_5_of_register_a() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0xAF]);
    emulator.cpu.registers.a = 0xB5;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0x95);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn set_bit_5_of_register_a() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0xEF]);
    emulator.cpu.registers.a = 0xB5;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0xB5);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn reset_bit_6_of_register_a() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0xB7]);
    emulator.cpu.registers.a = 0xB5;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0xB5);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn set_bit_6_of_register_a() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0xF7]);
    emulator.cpu.registers.a = 0xB5;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0xF5);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn reset_bit_7_of_register_a() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0xBF]);
    emulator.cpu.registers.a = 0xB5;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0x35);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn set_bit_7_of_register_a() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCB, 0xFF]);
    emulator.cpu.registers.a = 0xB5;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.a, 0xB5);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn jumps_to_address_nn() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xC3, 0xAA, 0x54]);
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.program_counter, 0x54AB);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 16);
}

#[test]
fn avoids_jumping_to_address_nn_if_z_flag_is_reset() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xC2, 0xAA, 0x54]);
    emulator.cpu.registers.f = 0x80;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.program_counter, 0x04);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 12);
}

#[test]
fn jumps_to_address_nn_if_z_flag_is_set() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCA, 0xAA, 0x54]);
    emulator.cpu.registers.f = 0x80;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.program_counter, 0x54AB);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 16);
}

#[test]
fn jumps_to_address_nn_if_c_flag_is_reset() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xD2, 0xAA, 0x54]);
    emulator.cpu.registers.f = 0x80;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.program_counter, 0x54AB);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 16);
}

#[test]
fn jumps_to_address_nn_if_c_flag_is_set() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xDA, 0xAA, 0x54]);
    emulator.cpu.registers.f = 0x80;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.program_counter, 0x04);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 12);
}

#[test]
fn jumps_to_address_hl() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xE9]);
    emulator.cpu.registers.h = 0x4B;
    emulator.cpu.registers.l = 0x51;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.program_counter, 0x4B52);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 4);
}

#[test]
fn jumps_to_current_address_plus_n() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x18, 0x05]);
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.program_counter, 0x08);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 12);
}

#[test]
fn ignores_jumping_to_curent_address_plus_n_if_z_flag_is_set() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x20, 0x05]);
    emulator.cpu.registers.f = 0x80;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.program_counter, 0x03);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn jumps_to_current_address_plus_n_if_z_flag_is_reset() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x20, 0x02]);
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.program_counter, 0x05);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 12);
}

#[test]
fn jumps_to_current_address_minus_n_if_z_flag_is_reset() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x20, 0xFE]);
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.program_counter, 0x01);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 12);
}

#[test]
fn jumps_to_current_address_plus_n_if_z_flag_is_set() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x28, 0x05]);
    emulator.cpu.registers.f = 0x80;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.program_counter, 0x08);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 12);
}

#[test]
fn jumps_to_current_address_plus_n_if_c_flag_is_reset() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x30, 0x05]);
    emulator.cpu.registers.f = 0x80;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.program_counter, 0x08);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 12);
}

#[test]
fn jumps_to_curent_address_plus_n_if_c_flag_is_set() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x38, 0x05]);
    emulator.cpu.registers.f = 0x80;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.program_counter, 0x03);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn calls_address_nn() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCD, 0x4A, 0x51]);
    emulator.cpu.registers.stack_pointer = 0xFFFE;
    step(&mut emulator);
    assert_eq!(emulator.memory.zero_page_ram[0x7D], 0x00);
    assert_eq!(emulator.memory.zero_page_ram[0x7C], 0x03);
    assert_eq!(emulator.cpu.registers.stack_pointer, 0xFFFC);
    assert_eq!(emulator.cpu.registers.program_counter, 0x514B);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 24);
}

#[test]
fn calls_address_nn_if_z_flag_is_reset() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xC4, 0x4A, 0x51]);
    emulator.cpu.registers.stack_pointer = 0xFFFE;
    emulator.cpu.registers.f = 0x80;
    step(&mut emulator);
    assert_eq!(emulator.memory.zero_page_ram[0x7D], 0x00);
    assert_eq!(emulator.memory.zero_page_ram[0x7C], 0x00);
    assert_eq!(emulator.cpu.registers.stack_pointer, 0xFFFE);
    assert_eq!(emulator.cpu.registers.program_counter, 0x04);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 12);
}

#[test]
fn calls_address_nn_if_z_flag_is_set() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xCC, 0x4A, 0x51]);
    emulator.cpu.registers.stack_pointer = 0xFFFE;
    emulator.cpu.registers.f = 0x80;
    step(&mut emulator);
    assert_eq!(emulator.memory.zero_page_ram[0x7D], 0x00);
    assert_eq!(emulator.memory.zero_page_ram[0x7C], 0x03);
    assert_eq!(emulator.cpu.registers.stack_pointer, 0xFFFC);
    assert_eq!(emulator.cpu.registers.program_counter, 0x514B);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 24);
}

#[test]
fn calls_address_nn_if_c_flag_is_reset() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xD4, 0x4A, 0x51]);
    emulator.cpu.registers.stack_pointer = 0xFFFE;
    emulator.cpu.registers.f = 0x80;
    step(&mut emulator);
    assert_eq!(emulator.memory.zero_page_ram[0x7D], 0x00);
    assert_eq!(emulator.memory.zero_page_ram[0x7C], 0x03);
    assert_eq!(emulator.cpu.registers.stack_pointer, 0xFFFC);
    assert_eq!(emulator.cpu.registers.program_counter, 0x514B);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 24);
}

#[test]
fn calls_address_nn_if_c_flag_is_set() {
    let mut emulator = init_emulator_with_test_instructions(vec![0xDC, 0x4A, 0x51]);
    emulator.cpu.registers.stack_pointer = 0x2112;
    emulator.cpu.registers.f = 0x80;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.stack_pointer, 0x2112);
    assert_eq!(emulator.cpu.registers.program_counter, 0x04);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 12);
}

#[test]
fn restarts_address_0() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x00, 0x00, 0xC7]);
    emulator.cpu.registers.stack_pointer = 0xFFFE;
    step(&mut emulator);
    step(&mut emulator);
    step(&mut emulator);
    assert_eq!(emulator.memory.zero_page_ram[0x7D], 0x00);
    assert_eq!(emulator.memory.zero_page_ram[0x7C], 0x03);
    assert_eq!(emulator.cpu.registers.stack_pointer, 0xFFFC);
    assert_eq!(emulator.cpu.registers.program_counter, 0x01);
}

#[test]
fn returns_from_call() {
    let mut rom = init_rom_with_test_instructions(vec![0xC9]);
    rom[0x2111] = 0xB1;
    rom[0x2110] = 0xDD;
    let mut emulator = init_emulator_from_rom(rom);
    emulator.cpu.registers.stack_pointer = 0x2110;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.program_counter, 0xB1DE);
    assert_eq!(emulator.cpu.registers.stack_pointer, 0x2112);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 16);
}

#[test]
fn returns_from_call_if_z_flag_is_reset() {
    let mut rom = init_rom_with_test_instructions(vec![0xC0]);
    rom[0x2111] = 0xB1;
    rom[0x2110] = 0xDD;
    let mut emulator = init_emulator_from_rom(rom);
    emulator.cpu.registers.stack_pointer = 0x2110;
    emulator.cpu.registers.f = 0x80;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.program_counter, 0x02);
    assert_eq!(emulator.cpu.registers.stack_pointer, 0x2110);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn returns_from_call_if_z_flag_is_set() {
    let mut rom = init_rom_with_test_instructions(vec![0xC8]);
    rom[0x2111] = 0xB1;
    rom[0x2110] = 0xDD;
    let mut emulator = init_emulator_from_rom(rom);
    emulator.cpu.registers.stack_pointer = 0x2110;
    emulator.cpu.registers.f = 0x80;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.program_counter, 0xB1DE);
    assert_eq!(emulator.cpu.registers.stack_pointer, 0x2112);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 20);
}

#[test]
fn returns_from_call_if_c_flag_is_reset() {
    let mut rom = init_rom_with_test_instructions(vec![0xD0]);
    rom[0x2111] = 0xB1;
    rom[0x2110] = 0xDD;
    let mut emulator = init_emulator_from_rom(rom);
    emulator.cpu.registers.stack_pointer = 0x2110;
    emulator.cpu.registers.f = 0x80;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.program_counter, 0xB1DE);
    assert_eq!(emulator.cpu.registers.stack_pointer, 0x2112);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 20);
}


#[test]
fn returns_from_call_if_c_flag_is_set() {
    let mut rom = init_rom_with_test_instructions(vec![0xD8]);
    rom[0x2111] = 0xB1;
    rom[0x2110] = 0xDD;
    let mut emulator = init_emulator_from_rom(rom);
    emulator.cpu.registers.stack_pointer = 0x2110;
    emulator.cpu.registers.f = 0x80;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.program_counter, 0x02);
    assert_eq!(emulator.cpu.registers.stack_pointer, 0x2110);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 8);
}

#[test]
fn halts_the_cpu_until_interrupt() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x76, 0x15]);
    step(&mut emulator);
    assert_eq!(emulator.cpu.halted, true);
    assert_eq!(emulator.cpu.registers.program_counter, 0x1);
    step(&mut emulator);
    assert_eq!(emulator.cpu.halted, true);
    assert_eq!(emulator.cpu.registers.program_counter, 0x1);
    step(&mut emulator);
    assert_eq!(emulator.cpu.halted, true);
    assert_eq!(emulator.cpu.registers.program_counter, 0x1);

    emulator.cpu.interrupts.enabled = true;
    emulator.cpu.registers.stack_pointer = 0x2112;
    emulator.interrupts.enabled = 0x1F;
    emulator.interrupts.flags = 0x01;

    step(&mut emulator);
    assert_eq!(emulator.cpu.halted, false);
    assert_eq!(emulator.cpu.registers.program_counter, 0x41);
}

#[test]
fn enables_interrupts() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xFB, 0x00, 0x00, 0x00]);
    step(&mut emulator);
    assert_eq!(emulator.cpu.interrupts.enable_delay, 2);
    assert_eq!(emulator.cpu.interrupts.enabled, false);
    step(&mut emulator);
    assert_eq!(emulator.cpu.interrupts.enable_delay, 1);
    assert_eq!(emulator.cpu.interrupts.enabled, false);
    step(&mut emulator);
    assert_eq!(emulator.cpu.interrupts.enable_delay, 0);
    assert_eq!(emulator.cpu.interrupts.enabled, true);
}

#[test]
fn disables_interrupts() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0xF3, 0x00, 0x00, 0x00]);
    emulator.cpu.interrupts.enabled = true;
    step(&mut emulator);
    assert_eq!(emulator.cpu.interrupts.disable_delay, 2);
    assert_eq!(emulator.cpu.interrupts.enabled, true);
    step(&mut emulator);
    assert_eq!(emulator.cpu.interrupts.disable_delay, 1);
    assert_eq!(emulator.cpu.interrupts.enabled, true);
    step(&mut emulator);
    assert_eq!(emulator.cpu.interrupts.disable_delay, 0);
    assert_eq!(emulator.cpu.interrupts.enabled, false);
}

#[test]
fn returns_from_call_then_enables_interrupts() {
    let mut rom = init_rom_with_test_instructions(vec![0xD9]);
    rom[0x2111] = 0xB1;
    rom[0x2110] = 0xDD;
    let mut emulator: Emulator = init_emulator_from_rom(rom);
    emulator.cpu.registers.stack_pointer = 0x2110;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.program_counter, 0xB1DE);
    assert_eq!(emulator.cpu.registers.stack_pointer, 0x2112);
    assert_eq!(emulator.cpu.interrupts.enabled, true);
    assert_eq!(emulator.cpu.instruction_clock_cycles, 16);
}

#[test]
fn runs_vertical_blank_isr() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x00]);
    emulator.cpu.registers.stack_pointer = 0x2112;
    emulator.cpu.interrupts.enabled = true;
    emulator.interrupts.enabled = 0x1F;
    emulator.interrupts.flags = 0x01;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.stack_pointer, 0x2110);
    assert_eq!(emulator.cpu.interrupts.enabled, false);
    assert_eq!(emulator.cpu.registers.program_counter, 0x41);
    assert_eq!(emulator.interrupts.enabled, 0x1F);
    assert_eq!(emulator.interrupts.flags, 0x00);
}

#[test]
fn runs_lcd_status_isr() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x00]);
    emulator.cpu.registers.stack_pointer = 0x2112;
    emulator.cpu.interrupts.enabled = true;
    emulator.interrupts.enabled = 0x1F;
    emulator.interrupts.flags = 0x02;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.stack_pointer, 0x2110);
    assert_eq!(emulator.cpu.interrupts.enabled, false);
    assert_eq!(emulator.cpu.registers.program_counter, 0x49);
    assert_eq!(emulator.interrupts.enabled, 0x1F);
    assert_eq!(emulator.interrupts.flags, 0x00);
}

#[test]
fn runs_timer_overflow_isr() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x00]);
    emulator.cpu.registers.stack_pointer = 0x2112;
    emulator.cpu.interrupts.enabled = true;
    emulator.interrupts.enabled = 0x1F;
    emulator.interrupts.flags = 0x04;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.stack_pointer, 0x2110);
    assert_eq!(emulator.cpu.interrupts.enabled, false);
    assert_eq!(emulator.cpu.registers.program_counter, 0x51);
    assert_eq!(emulator.interrupts.enabled, 0x1F);
    assert_eq!(emulator.interrupts.flags, 0x00);
}

#[test]
fn runs_serial_link_isr() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x00]);
    emulator.cpu.registers.stack_pointer = 0x2112;
    emulator.cpu.interrupts.enabled = true;
    emulator.interrupts.enabled = 0x1F;
    emulator.interrupts.flags = 0x08;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.stack_pointer, 0x2110);
    assert_eq!(emulator.cpu.interrupts.enabled, false);
    assert_eq!(emulator.cpu.registers.program_counter, 0x59);
    assert_eq!(emulator.interrupts.enabled, 0x1F);
    assert_eq!(emulator.interrupts.flags, 0x00);
}

#[test]
fn runs_joypad_press_isr() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x00]);
    emulator.cpu.registers.stack_pointer = 0x2112;
    emulator.cpu.interrupts.enabled = true;
    emulator.interrupts.enabled = 0x1F;
    emulator.interrupts.flags = 0x10;
    step(&mut emulator);
    assert_eq!(emulator.cpu.registers.stack_pointer, 0x2110);
    assert_eq!(emulator.cpu.interrupts.enabled, false);
    assert_eq!(emulator.cpu.registers.program_counter, 0x61);
    assert_eq!(emulator.interrupts.enabled, 0x1F);
    assert_eq!(emulator.interrupts.flags, 0x00);
}

#[test]
fn toggles_cgb_double_speed_mode() {
    let mut emulator: Emulator = init_emulator_with_test_instructions(vec![0x10]);
    emulator.mode = Mode::CGB;
    emulator.speed_switch.armed = true;
    step(&mut emulator);
    assert_eq!(emulator.speed_switch.armed, false);
    assert_eq!(emulator.speed_switch.cgb_double_speed, true);
}

#[test]
fn records_bus_activity_for_loading_immediate_byte_into_register_b() {
    let mut emulator = initialize_screenless_emulator();

    emulator.processor_test_mode = true;
    emulator.memory.processor_test_ram[0x00] = 0x06;
    emulator.memory.processor_test_ram[0x01] = 0xA1;

    // Step once to make sure the first opcode is loaded
    step(&mut emulator);

    // Step again to execute opcode 0x06
    step(&mut emulator);

    let bus_activity = emulator.cpu.opcode_bus_activity;
    assert_eq!(bus_activity.len(), 2);

    let immediate_byte_read = BusActivityEntry { address: 0x01, value: 0xA1, activity_type: BusActivityType::Read };
    assert_eq!(bus_activity[0], Some(immediate_byte_read));

    let opcode_read = BusActivityEntry { address: 0x02, value: 0x00, activity_type: BusActivityType::Read }; 
    assert_eq!(bus_activity[1], Some(opcode_read));
}

#[test]
fn records_bus_activity_for_jump_to_address_nn() {
    let mut emulator: Emulator = initialize_screenless_emulator();

    emulator.processor_test_mode = true;
    emulator.memory.processor_test_ram[0x00] = 0xC3;
    emulator.memory.processor_test_ram[0x01] = 0xAA;
    emulator.memory.processor_test_ram[0x02] = 0x54;

    // Step once to make sure the first opcode is loaded
    step(&mut emulator);

    // Step again to execute opcode 0xC3
    step(&mut emulator);

    let bus_activity = emulator.cpu.opcode_bus_activity;
    assert_eq!(bus_activity.len(), 4);

    let first_byte_read = BusActivityEntry { address: 0x01, value: 0xAA, activity_type: BusActivityType::Read };
    assert_eq!(bus_activity[0], Some(first_byte_read));

    let second_byte_read = BusActivityEntry { address: 0x02, value: 0x54, activity_type: BusActivityType::Read };
    assert_eq!(bus_activity[1], Some(second_byte_read));

    // Machine cycle with no bus activity
    assert_eq!(bus_activity[2], None);

    let opcode_read = BusActivityEntry { address: 0x54AA, value: 0x0, activity_type: BusActivityType::Read };
    assert_eq!(bus_activity[3], Some(opcode_read));
}