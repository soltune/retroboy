use super::*;
use crate::mmu;

#[test]
fn loads_immediate_byte() {
    let mut cpu_state = cpu::initialize_cpu_state();
    cpu_state.memory.in_bios = false;

    let mut test_instructions = vec![];
    test_instructions.push(0x06);
    test_instructions.push(0xA1);

    mmu::load_rom_buffer(&mut cpu_state.memory, test_instructions);

    let next_cpu_state = execute_opcode(&mut cpu_state);

    assert_eq!(next_cpu_state.registers.a, 0xA1);
    assert_eq!(next_cpu_state.registers.program_counter, 2);
    assert_eq!(next_cpu_state.clock.machine_cycles, 2);
    assert_eq!(next_cpu_state.clock.clock_cycles, 8);
}
