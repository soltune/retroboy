use super::*;
use crate::cpu;
use crate::mmu;

fn run_test_instructions(test_instructions: Vec<u8>) -> cpu::CpuState {
    let mut cpu_state = cpu::initialize_cpu_state();
    cpu_state.memory.in_bios = false;
    mmu::load_rom_buffer(&mut cpu_state.memory, test_instructions);
    execute_opcode(&mut cpu_state);
    cpu_state
}

#[test]
fn loads_immediate_byte_into_register_a() {
    let cpu_state = run_test_instructions(vec![0x06, 0xA1]);
    assert_eq!(cpu_state.registers.a, 0xA1);
    assert_eq!(cpu_state.registers.program_counter, 2);
    assert_eq!(cpu_state.clock.machine_cycles, 2);
    assert_eq!(cpu_state.clock.clock_cycles, 8);
}
