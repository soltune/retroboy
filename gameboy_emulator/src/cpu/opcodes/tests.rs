use super::*;
use crate::cpu;
use crate::mmu;

fn init_cpu_with_test_instructions(test_instructions: Vec<u8>) -> cpu::CpuState {
    let mut cpu_state = cpu::initialize_cpu_state();
    cpu_state.memory.in_bios = false;
    mmu::load_rom_buffer(&mut cpu_state.memory, test_instructions);
    cpu_state
}

fn assert_cycles(cpu_state: &cpu::CpuState, machine_cycles: u8) {
    assert_eq!(cpu_state.clock.machine_cycles, machine_cycles as u32);
    assert_eq!(cpu_state.clock.clock_cycles, (machine_cycles * 4) as u32);
}

#[test]
fn loads_immediate_byte_into_register_b() {
    let mut cpu_state = init_cpu_with_test_instructions(vec![0x06, 0xA1]);
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.b, 0xA1);
    assert_eq!(cpu_state.registers.program_counter, 2);
    assert_cycles(&cpu_state, 2)
}

#[test]
fn loads_register_b_into_register_a() {
    let mut cpu_state = init_cpu_with_test_instructions(vec![0x78]);
    cpu_state.registers.b = 0x2F;
    execute_opcode(&mut cpu_state);
    assert_eq!(cpu_state.registers.a, 0x2F);
    assert_eq!(cpu_state.registers.program_counter, 1);
    assert_cycles(&cpu_state, 1)
}
