use crate::cpu::{CpuState, Register, REGISTER_AF, REGISTER_BC, REGISTER_DE, REGISTER_HL, read_next_instruction_byte, read_next_instruction_word, handle_illegal_opcode};
use crate::cpu::microops;
use crate::cpu::alu;
use crate::cpu::bitops;
use crate::cpu::interrupts;
use crate::cpu::jumps;
use crate::cpu::loads;
use crate::emulator::Emulator;

fn emulate_halt_bug(cpu: &mut CpuState) {
    // Mimics halt bug behavior, which runs the instruction after HALT twice.
    if !cpu.halted && cpu.halt_bug {
        cpu.registers.program_counter -= 1;
        cpu.halt_bug = false;
    }
}

fn update_interrupt_flag_after_delay(cpu: &mut CpuState) {
    if cpu.interrupts.enable_delay > 0 {
        if cpu.interrupts.enable_delay == 1 {
            cpu.interrupts.enabled = true;
        }
        cpu.interrupts.enable_delay -= 1;
    }
    else if cpu.interrupts.disable_delay > 0 {
        if cpu.interrupts.disable_delay == 1 {
            cpu.interrupts.enabled = false;
        }
        cpu.interrupts.disable_delay -= 1;
    }
}

fn reset_instruction_clock_cycles(cpu: &mut CpuState) {
    cpu.instruction_clock_cycles = 0;
}

fn reset_last_opcode_bus_activity(emulator: &mut Emulator) {
    if emulator.cpu.address_bus.processor_test_mode() {
        emulator.cpu.opcode_bus_activity.clear();
    }
}

fn prefetch_next_opcode(emulator: &mut Emulator) {
    let next_opcode= read_next_instruction_byte(emulator);
    emulator.cpu.registers.opcode = next_opcode;
}

pub fn step(emulator: &mut Emulator) {
    if emulator.cpu.address_bus.hdma().in_progress() {
        emulator.cpu.address_bus.hdma_step();
    }

    execute_opcode(emulator);

    interrupts::step(emulator);

    prefetch_next_opcode(emulator);
}

fn execute_opcode(mut emulator: &mut Emulator) {
    reset_instruction_clock_cycles(&mut emulator.cpu);
    reset_last_opcode_bus_activity(&mut emulator);

    let opcode = emulator.cpu.registers.opcode;

    emulate_halt_bug(&mut emulator.cpu);
    update_interrupt_flag_after_delay(&mut emulator.cpu);

    match opcode {
        0x00 =>
            (),
        0x01 => {
            let word = read_next_instruction_word(emulator);
            microops::store_in_register_pair(&mut emulator.cpu, REGISTER_BC, word);
        },
        0x02 => {
            let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_BC);
            loads::load_source_register_in_memory(emulator, Register::A, address);
        },
        0x03 =>
            alu::increment_register_pair(emulator, REGISTER_BC),
        0x04 =>
            alu::increment_register(&mut emulator.cpu, Register::B),
        0x05 =>
            alu::decrement_register(&mut emulator.cpu, Register::B),
        0x06 =>
            loads::load_immediate_value(emulator, Register::B),
        0x07 => {
            bitops::rotate_register_left(&mut emulator.cpu, Register::A);
            microops::set_flag_z(&mut emulator.cpu, false);
        },
        0x08 => {
            let address = read_next_instruction_word(emulator);
            microops::store_word_in_memory(emulator, address, emulator.cpu.registers.stack_pointer);
        },
        0x09 => {
            let word = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_BC);
            alu::add_value_to_register_pair(emulator, REGISTER_HL, word);
        },
        0x0A => {
            let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_BC);
            loads::load_memory_byte_in_destination_register(emulator, address, Register::A);
        },
        0x0B =>
            alu::decrement_register_pair(emulator, REGISTER_BC),
        0x0C =>
            alu::increment_register(&mut emulator.cpu, Register::C),
        0x0D =>
            alu::decrement_register(&mut emulator.cpu, Register::C),
        0x0E =>
            loads::load_immediate_value(emulator, Register::C),
        0x0F => {
            bitops::rotate_register_right(&mut emulator.cpu, Register::A);
            microops::set_flag_z(&mut emulator.cpu, false);
        },
        0x10 =>
            emulator.cpu.address_bus.toggle_speed_switch(),
        0x11 => {
            let word = read_next_instruction_word(emulator);
            microops::store_in_register_pair(&mut emulator.cpu, REGISTER_DE, word);
        },
        0x12 => {
            let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_DE);
            loads::load_source_register_in_memory(emulator, Register::A, address);
        },
        0x13 =>
            alu::increment_register_pair(emulator, REGISTER_DE),
        0x14 =>
            alu::increment_register(&mut emulator.cpu, Register::D),
        0x15 =>
            alu::decrement_register(&mut emulator.cpu, Register::D),
        0x16 =>
            loads::load_immediate_value(emulator, Register::D),
        0x17 => {
            bitops::rotate_register_left_through_carry(&mut emulator.cpu, Register::A);
            microops::set_flag_z(&mut emulator.cpu, false);
        },
        0x18 => {
            let byte = read_next_instruction_byte(emulator) as i8;
            let original_program_counter = emulator.cpu.registers.program_counter;
            emulator.cpu.registers.program_counter = original_program_counter.wrapping_add_signed(byte.into());
            microops::step_one_machine_cycle(emulator);
        },
        0x19 => {
            let word = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_DE);
            alu::add_value_to_register_pair(emulator, REGISTER_HL, word);
        },
        0x1A => {
            let address = microops::read_from_register_pair(&mut emulator.cpu,&REGISTER_DE);
            loads::load_memory_byte_in_destination_register(emulator, address, Register::A)
        },
        0x1B =>
            alu::decrement_register_pair(emulator, REGISTER_DE),
        0x1C =>
            alu::increment_register(&mut emulator.cpu, Register::E),
        0x1D =>
            alu::decrement_register(&mut emulator.cpu, Register::E),
        0x1E =>
            loads::load_immediate_value(emulator, Register::E),
        0x1F => {
            bitops::rotate_register_right_through_carry(&mut emulator.cpu, Register::A);
            microops::set_flag_z(&mut emulator.cpu, false);
        },
        0x20 =>
            jumps::conditional_relative_jump(emulator, !microops::is_z_flag_set(&emulator.cpu)),
        0x21 => {
            let word = read_next_instruction_word(emulator);
            microops::store_in_register_pair(&mut emulator.cpu, REGISTER_HL, word);
        },
        0x22 => {
            let mut address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
            loads::load_source_register_in_memory(emulator, Register::A, address);
            address = address.wrapping_add(1);
            microops::store_in_register_pair(&mut emulator.cpu, REGISTER_HL, address);    
        },
        0x23 =>
            alu::increment_register_pair(emulator, REGISTER_HL),
        0x24 =>
            alu::increment_register(&mut emulator.cpu, Register::H),
        0x25 =>
            alu::decrement_register(&mut emulator.cpu, Register::H),
        0x26 =>
            loads::load_immediate_value(emulator, Register::H),
        0x27 =>
            alu::bcd_adjust(&mut emulator.cpu),
        0x28 =>
            jumps::conditional_relative_jump(emulator, microops::is_z_flag_set(&emulator.cpu)),
        0x29 => {
            let word = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
            alu::add_value_to_register_pair(emulator, REGISTER_HL, word);
        },
        0x2A => {
            let mut address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
            loads::load_memory_byte_in_destination_register(emulator, address, Register::A);
            address += 1;
            microops::store_in_register_pair(&mut emulator.cpu, REGISTER_HL, address);  
        },
        0x2B =>
            alu::decrement_register_pair(emulator, REGISTER_HL),
        0x2C =>
            alu::increment_register(&mut emulator.cpu, Register::L),
        0x2D =>
            alu::decrement_register(&mut emulator.cpu, Register::L),
        0x2E =>
            loads::load_immediate_value(emulator, Register::L),
        0x2F => {
            emulator.cpu.registers.a = emulator.cpu.registers.a ^ 0xFF;
            microops::set_flag_n(&mut emulator.cpu, true);
            microops::set_flag_h(&mut emulator.cpu, true);
        },
        0x30 =>
            jumps::conditional_relative_jump(emulator, !microops::is_c_flag_set(&emulator.cpu)),
        0x31 => {
            let word = read_next_instruction_word(emulator);
            emulator.cpu.registers.stack_pointer = word;            
        },
        0x32 => {
            let mut address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
            loads::load_source_register_in_memory(emulator, Register::A, address);
            address -= 1;
            microops::store_in_register_pair(&mut emulator.cpu, REGISTER_HL, address);           
        },
        0x33 => {
            emulator.cpu.registers.stack_pointer = emulator.cpu.registers.stack_pointer.wrapping_add(1);
            microops::step_one_machine_cycle(emulator);
        },
        0x34 =>
            alu::increment_memory_byte(emulator),
        0x35 =>
            alu::decrement_memory_byte(emulator),
        0x36 =>
            loads::load_immediate_value_in_memory(emulator, REGISTER_HL),
        0x37 => {
            microops::set_flag_c(&mut emulator.cpu, true);
            microops::set_flag_h(&mut emulator.cpu, false);
            microops::set_flag_n(&mut emulator.cpu, false);
        },
        0x38 =>
            jumps::conditional_relative_jump(emulator, microops::is_c_flag_set(&emulator.cpu)),
        0x39 => {
            let stack_pointer = emulator.cpu.registers.stack_pointer;
            alu::add_value_to_register_pair(emulator, REGISTER_HL, stack_pointer)
        }
        0x3A => {
            let mut address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
            loads::load_memory_byte_in_destination_register(emulator, address, Register::A);
            address -= 1;
            microops::store_in_register_pair(&mut emulator.cpu, REGISTER_HL, address);
        },
        0x3B => {
            emulator.cpu.registers.stack_pointer = emulator.cpu.registers.stack_pointer.wrapping_sub(1);
            microops::step_one_machine_cycle(emulator);
        },
        0x3C =>
            alu::increment_register(&mut emulator.cpu, Register::A),
        0x3D =>
            alu::decrement_register(&mut emulator.cpu, Register::A),
        0x3E =>
            loads::load_immediate_value(emulator, Register::A),
        0x3F => {
            let c_flag_set = microops::is_c_flag_set(&mut emulator.cpu);
            microops::set_flag_c(&mut emulator.cpu, !c_flag_set);
            microops::set_flag_n(&mut emulator.cpu, false);
            microops::set_flag_h(&mut emulator.cpu, false);
        },
        0x40 =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::B, Register::B),
        0x41 =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::C, Register::B),
        0x42 =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::D, Register::B),
        0x43 =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::E, Register::B),
        0x44 =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::H, Register::B),
        0x45 =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::L, Register::B),
        0x46 => {
            let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
            loads::load_memory_byte_in_destination_register(emulator, address, Register::B)
        },
        0x47 =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::A, Register::B),
        0x48 =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::B, Register::C),
        0x49 =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::C, Register::C),
        0x4A =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::D, Register::C),
        0x4B =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::E, Register::C),
        0x4C =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::H, Register::C),
        0x4D =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::L, Register::C),
        0x4E => {
            let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
            loads::load_memory_byte_in_destination_register(emulator, address, Register::C)
        },
        0x4F =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::A, Register::C),
        0x50 => 
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::B, Register::D),
        0x51 =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::C, Register::D),
        0x52 =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::D, Register::D),
        0x53 =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::E, Register::D),
        0x54 =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::H, Register::D),
        0x55 =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::L, Register::D),
        0x56 => {
            let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
            loads::load_memory_byte_in_destination_register(emulator, address, Register::D)
        },
        0x57 =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::A, Register::D),
        0x58 =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::B, Register::E),
        0x59 =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::C, Register::E),
        0x5A =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::D, Register::E),
        0x5B =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::E, Register::E),
        0x5C =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::H, Register::E),
        0x5D =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::L, Register::E),
        0x5E => {
            let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
            loads::load_memory_byte_in_destination_register(emulator, address, Register::E)
        },
        0x5F =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::A, Register::E),
        0x60 =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::B, Register::H),
        0x61 =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::C, Register::H),
        0x62 =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::D, Register::H),
        0x63 =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::E, Register::H),
        0x64 =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::H, Register::H),
        0x65 =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::L, Register::H),
        0x66 => {
            let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
            loads::load_memory_byte_in_destination_register(emulator, address, Register::H)
        },
        0x67 =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::A, Register::H),
        0x68 =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::B, Register::L),
        0x69 =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::C, Register::L),
        0x6A =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::D, Register::L),
        0x6B =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::E, Register::L),
        0x6C =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::H, Register::L),
        0x6D =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::L, Register::L),
        0x6E => {
            let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
            loads::load_memory_byte_in_destination_register(emulator, address, Register::L)
        },
        0x6F =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::A, Register::L),
        0x70 => {
            let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
            loads::load_source_register_in_memory(emulator, Register::B, address);
        },
        0x71 => {
            let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
            loads::load_source_register_in_memory(emulator, Register::C, address);
        },
        0x72 => {
            let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
            loads::load_source_register_in_memory(emulator, Register::D, address);
        },
        0x73 => {
            let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
            loads::load_source_register_in_memory(emulator, Register::E, address);
        },
        0x74 => {
            let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
            loads::load_source_register_in_memory(emulator, Register::H, address);
        },
        0x75 => {
            let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
            loads::load_source_register_in_memory(emulator, Register::L, address);
        },
        0x76 => {
            if interrupts::interrupts_fired(&emulator.cpu) {
                emulator.cpu.halted = false;

                if !emulator.cpu.interrupts.enabled {
                    emulator.cpu.halt_bug = true;
                }
            }
            else {
                emulator.cpu.halted = true;
                emulator.cpu.registers.program_counter -= 1;
            }
        },
        0x77 => {
            let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
            loads::load_source_register_in_memory(emulator, Register::A, address);
        },
        0x78 =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::B, Register::A),
        0x79 =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::C, Register::A),
        0x7A =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::D, Register::A),
        0x7B =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::E, Register::A),
        0x7C =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::H, Register::A),
        0x7D =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::L, Register::A),
        0x7E => {
            let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
            loads::load_memory_byte_in_destination_register(emulator, address, Register::A)
        },
        0x7F =>
            loads::load_source_register_in_destination_register(&mut emulator.cpu, Register::A, Register::A),
        0x80 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::B);
            alu::add_value_to_register(&mut emulator.cpu, Register::A, value);
        },
        0x81 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::C);
            alu::add_value_to_register(&mut emulator.cpu, Register::A, value);
        },
        0x82 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::D);
            alu::add_value_to_register(&mut emulator.cpu, Register::A, value);
        },
        0x83 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::E);
            alu::add_value_to_register(&mut emulator.cpu, Register::A, value);
        },
        0x84 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::H);
            alu::add_value_to_register(&mut emulator.cpu, Register::A, value);
        },
        0x85 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::L);
            alu::add_value_to_register(&mut emulator.cpu, Register::A, value);
        },
        0x86 => {
            let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
            let value = microops::read_byte_from_memory(emulator, address);
            alu::add_value_to_register(&mut emulator.cpu, Register::A, value);
        },
        0x87 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::A);
            alu::add_value_to_register(&mut emulator.cpu, Register::A, value);
        },
        0x88 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::B);
            alu::add_value_and_carry_to_register(&mut emulator.cpu, Register::A, value);
        },
        0x89 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::C);
            alu::add_value_and_carry_to_register(&mut emulator.cpu, Register::A, value);
        },
        0x8A => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::D);
            alu::add_value_and_carry_to_register(&mut emulator.cpu, Register::A, value);
        },
        0x8B => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::E);
            alu::add_value_and_carry_to_register(&mut emulator.cpu, Register::A, value);
        },
        0x8C => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::H);
            alu::add_value_and_carry_to_register(&mut emulator.cpu, Register::A, value);
        },
        0x8D => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::L);
            alu::add_value_and_carry_to_register(&mut emulator.cpu, Register::A, value);
        },
        0x8E => {
            let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
            let value = microops::read_byte_from_memory(emulator, address);
            alu::add_value_and_carry_to_register(&mut emulator.cpu, Register::A, value);
        },
        0x8F => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::A);
            alu::add_value_and_carry_to_register(&mut emulator.cpu, Register::A, value);
        },
        0x90 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::B);
            alu::subtract_value_from_register(&mut emulator.cpu, Register::A, value);
        },
        0x91 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::C);
            alu::subtract_value_from_register(&mut emulator.cpu, Register::A, value);
        },
        0x92 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::D);
            alu::subtract_value_from_register(&mut emulator.cpu, Register::A, value);
        },
        0x93 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::E);
            alu::subtract_value_from_register(&mut emulator.cpu, Register::A, value);
        },
        0x94 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::H);
            alu::subtract_value_from_register(&mut emulator.cpu, Register::A, value);
        },
        0x95 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::L);
            alu::subtract_value_from_register(&mut emulator.cpu, Register::A, value);
        },
        0x96 => {
            let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
            let value = microops::read_byte_from_memory(emulator, address);
            alu::subtract_value_from_register(&mut emulator.cpu, Register::A, value);
        },
        0x97 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::A);
            alu::subtract_value_from_register(&mut emulator.cpu, Register::A, value);
        },
        0x98 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::B);
            alu::subtract_value_and_carry_from_register(&mut emulator.cpu, Register::A, value);
        },
        0x99 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::C);
            alu::subtract_value_and_carry_from_register(&mut emulator.cpu, Register::A, value);
        },
        0x9A => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::D);
            alu::subtract_value_and_carry_from_register(&mut emulator.cpu, Register::A, value);
        },
        0x9B => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::E);
            alu::subtract_value_and_carry_from_register(&mut emulator.cpu, Register::A, value);
        },
        0x9C => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::H);
            alu::subtract_value_and_carry_from_register(&mut emulator.cpu, Register::A, value);
        },
        0x9D => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::L);
            alu::subtract_value_and_carry_from_register(&mut emulator.cpu, Register::A, value);
        },
        0x9E => {
            let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
            let value = microops::read_byte_from_memory(emulator, address);
            alu::subtract_value_and_carry_from_register(&mut emulator.cpu, Register::A, value);
        },
        0x9F => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::A);
            alu::subtract_value_and_carry_from_register(&mut emulator.cpu, Register::A, value);
        },
        0xA0 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::B);
            alu::logical_and_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xA1 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::C);
            alu::logical_and_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xA2 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::D);
            alu::logical_and_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xA3 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::E);
            alu::logical_and_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xA4 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::H);
            alu::logical_and_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xA5 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::L);
            alu::logical_and_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xA6 => {
            let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
            let value = microops::read_byte_from_memory(emulator, address);
            alu::logical_and_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xA7 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::A);
            alu::logical_and_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xA8 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::B);
            alu::logical_xor_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xA9 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::C);
            alu::logical_xor_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xAA => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::D);
            alu::logical_xor_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xAB => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::E);
            alu::logical_xor_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xAC => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::H);
            alu::logical_xor_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xAD => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::L);
            alu::logical_xor_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xAE => {
            let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
            let value = microops::read_byte_from_memory(emulator, address);
            alu::logical_xor_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xAF => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::A);
            alu::logical_xor_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xB0 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::B);
            alu::logical_or_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xB1 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::C);
            alu::logical_or_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xB2 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::D);
            alu::logical_or_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xB3 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::E);
            alu::logical_or_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xB4 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::H);
            alu::logical_or_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xB5 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::L);
            alu::logical_or_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xB6 => {
            let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
            let value = microops::read_byte_from_memory(emulator, address);
            alu::logical_or_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xB7 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::A);
            alu::logical_or_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xB8 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::B);
            alu::compare_value_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xB9 => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::C);
            alu::compare_value_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xBA => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::D);
            alu::compare_value_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xBB => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::E);
            alu::compare_value_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xBC => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::H);
            alu::compare_value_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xBD => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::L);
            alu::compare_value_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xBE => {
            let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
            let value = microops::read_byte_from_memory(emulator, address);
            alu::compare_value_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xBF => {
            let value = microops::read_from_register(&mut emulator.cpu, &Register::A);
            alu::compare_value_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xC0 =>
            jumps::conditional_stack_return(emulator, !microops::is_z_flag_set(&emulator.cpu)),
        0xC1 =>
            loads::pop_word_into_register_pair_from_stack(emulator, REGISTER_BC),
        0xC2 =>
            jumps::conditional_jump_using_immediate_word(emulator, !microops::is_z_flag_set(&emulator.cpu)),
        0xC3 => {
            emulator.cpu.registers.program_counter = read_next_instruction_word(emulator);
            microops::step_one_machine_cycle(emulator);
        },
        0xC4 =>
            jumps::conditional_call_using_immediate_word(emulator, !microops::is_z_flag_set(&emulator.cpu)),
        0xC5 =>
            loads::push_register_pair_to_stack(emulator, REGISTER_BC),
        0xC6 => {
            let value = read_next_instruction_byte(emulator);
            alu::add_value_to_register(&mut emulator.cpu, Register::A, value);
        },
        0xC7 =>
            jumps::restart(emulator, 0x0),
        0xC8 =>
            jumps::conditional_stack_return(emulator, microops::is_z_flag_set(&emulator.cpu)),
        0xC9 =>
            jumps::stack_return(emulator),
        0xCA =>
            jumps::conditional_jump_using_immediate_word(emulator, microops::is_z_flag_set(&emulator.cpu)),
        0xCB =>
            execute_cb_opcode(emulator),
        0xCC =>
            jumps::conditional_call_using_immediate_word(emulator, microops::is_z_flag_set(&emulator.cpu)),
        0xCD =>
            jumps::call(emulator),
        0xCE => {
            let value = read_next_instruction_byte(emulator);
            alu::add_value_and_carry_to_register(&mut emulator.cpu, Register::A, value);
        },
        0xCF =>
            jumps::restart(emulator, 0x8),
        0xD0 =>
            jumps::conditional_stack_return(emulator, !microops::is_c_flag_set(&emulator.cpu)),
        0xD1 =>
            loads::pop_word_into_register_pair_from_stack(emulator, REGISTER_DE),
        0xD2 =>
            jumps::conditional_jump_using_immediate_word(emulator, !microops::is_c_flag_set(&emulator.cpu)),
        0xD3 =>
            handle_illegal_opcode(opcode),
        0xD4 =>
            jumps::conditional_call_using_immediate_word(emulator, !microops::is_c_flag_set(&emulator.cpu)),
        0xD5 =>
            loads::push_register_pair_to_stack(emulator, REGISTER_DE),
        0xD6 => {
            let value = read_next_instruction_byte(emulator);
            alu::subtract_value_from_register(&mut emulator.cpu, Register::A, value);
        },
        0xD7 =>
            jumps::restart(emulator, 0x10),
        0xD8 =>
            jumps::conditional_stack_return(emulator, microops::is_c_flag_set(&emulator.cpu)),
        0xD9 => {
            jumps::stack_return(emulator);
            emulator.cpu.interrupts.enabled = true;
        },
        0xDA =>
            jumps::conditional_jump_using_immediate_word(emulator, microops::is_c_flag_set(&emulator.cpu)),
        0xDB =>
            handle_illegal_opcode(opcode),
        0xDC =>
            jumps::conditional_call_using_immediate_word(emulator, microops::is_c_flag_set(&emulator.cpu)),
        0xDD =>
            handle_illegal_opcode(opcode),
        0xDE => {
            let value = read_next_instruction_byte(emulator);
            alu::subtract_value_and_carry_from_register(&mut emulator.cpu, Register::A, value);
        },
        0xDF =>
            jumps::restart(emulator, 0x18),
        0xE0 => {
            let address = 0xFF00 + read_next_instruction_byte(emulator) as u16;
            loads::load_source_register_in_memory(emulator, Register::A, address);
        },
        0xE1 =>
            loads::pop_word_into_register_pair_from_stack(emulator, REGISTER_HL),
        0xE2 => {
            let address = 0xFF00 + microops::read_from_register(&mut emulator.cpu, &Register::C) as u16;
            loads::load_source_register_in_memory(emulator, Register::A, address);
        },
        0xE3 =>
            handle_illegal_opcode(opcode),
        0xE4 =>
            handle_illegal_opcode(opcode),
        0xE5 =>
            loads::push_register_pair_to_stack(emulator, REGISTER_HL),
        0xE6 => {
            let value = read_next_instruction_byte(emulator);
            alu::logical_and_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xE7 =>
            jumps::restart(emulator, 0x20),
        0xE8 => {
            let signed_byte = read_next_instruction_byte(emulator) as i8;
            let sum = emulator.cpu.registers.stack_pointer.wrapping_add_signed(signed_byte.into());
            let stack_pointer = emulator.cpu.registers.stack_pointer;
            
            microops::set_flag_z(&mut emulator.cpu, false);
            microops::set_flag_n(&mut emulator.cpu, false);
            microops::set_flag_h(&mut emulator.cpu, (sum & 0xF) < (stack_pointer & 0xF));
            microops::set_flag_c(&mut emulator.cpu, (sum & 0xFF) < (stack_pointer & 0xFF));

            emulator.cpu.registers.stack_pointer = sum;

            microops::step_one_machine_cycle(emulator);
            microops::step_one_machine_cycle(emulator);
        },
        0xE9 => {
            let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
            emulator.cpu.registers.program_counter = address;
        },
        0xEA => {
            let address = read_next_instruction_word(emulator);
            loads::load_source_register_in_memory(emulator, Register::A, address);
        },
        0xEB =>
            handle_illegal_opcode(opcode),
        0xEC =>
            handle_illegal_opcode(opcode),
        0xED =>
            handle_illegal_opcode(opcode),
        0xEE => {
            let value = read_next_instruction_byte(emulator);
            alu::logical_xor_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xEF =>
            jumps::restart(emulator, 0x28),
        0xF0 => {
            let address = 0xFF00 + read_next_instruction_byte(emulator) as u16;
            loads::load_memory_byte_in_destination_register(emulator, address, Register::A);
        },
        0xF1 => {
            let word = loads::pop_word_from_stack(emulator);
            microops::store_in_register_pair(&mut emulator.cpu, REGISTER_AF, word & 0xFFF0);
        }
        0xF2 => {
            let address = 0xFF00 + microops::read_from_register(&mut emulator.cpu, &Register::C) as u16;
            loads::load_memory_byte_in_destination_register(emulator, address, Register::A);
        },
        0xF3 => {
            emulator.cpu.interrupts.disable_delay = 2;
        },
        0xF4 =>
            handle_illegal_opcode(opcode),
        0xF5 =>
            loads::push_register_pair_to_stack(emulator, REGISTER_AF),
        0xF6 => {
            let value = read_next_instruction_byte(emulator);
            alu::logical_or_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xF7 =>
            jumps::restart(emulator, 0x30),
        0xF8 => {
            let signed_byte = read_next_instruction_byte(emulator) as i8;
            let stack_pointer = emulator.cpu.registers.stack_pointer;
            let sum = stack_pointer.wrapping_add_signed(signed_byte.into());

            microops::store_in_register_pair(&mut emulator.cpu, REGISTER_HL, sum);
            
            microops::set_flag_z(&mut emulator.cpu, false);
            microops::set_flag_n(&mut emulator.cpu, false);
            microops::set_flag_h(&mut emulator.cpu, (sum & 0xF) < (stack_pointer & 0xF));
            microops::set_flag_c(&mut emulator.cpu, (sum & 0xFF) < (stack_pointer & 0xFF));

            microops::step_one_machine_cycle(emulator);
        },
        0xF9 => {
            let word = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
            emulator.cpu.registers.stack_pointer = word;
            microops::step_one_machine_cycle(emulator);
        },
        0xFA => {
            let address = read_next_instruction_word(emulator);
            loads::load_memory_byte_in_destination_register(emulator, address, Register::A);
        },
        0xFB => {
            emulator.cpu.interrupts.enable_delay = 2;
        },
        0xFC =>
            handle_illegal_opcode(opcode),
        0xFD =>
            handle_illegal_opcode(opcode),
        0xFE => {
            let value = read_next_instruction_byte(emulator);
            alu::compare_value_with_register(&mut emulator.cpu, Register::A, value);
        },
        0xFF =>
            jumps::restart(emulator, 0x38),
    }
}

fn execute_cb_opcode(emulator: &mut Emulator) {
    let opcode = read_next_instruction_byte(emulator);
    match opcode {
        0x00 =>
            bitops::rotate_register_left(&mut emulator.cpu, Register::B),
        0x01 =>
            bitops::rotate_register_left(&mut emulator.cpu, Register::C),
        0x02 =>
            bitops::rotate_register_left(&mut emulator.cpu, Register::D),
        0x03 =>
            bitops::rotate_register_left(&mut emulator.cpu, Register::E),
        0x04 =>
            bitops::rotate_register_left(&mut emulator.cpu, Register::H),
        0x05 =>
            bitops::rotate_register_left(&mut emulator.cpu, Register::L),
        0x06 =>
            bitops::rotate_memory_byte_left(emulator),
        0x07 =>
            bitops::rotate_register_left(&mut emulator.cpu, Register::A),
        0x08 =>
            bitops::rotate_register_right(&mut emulator.cpu, Register::B),
        0x09 =>
            bitops::rotate_register_right(&mut emulator.cpu, Register::C),
        0x0A =>
            bitops::rotate_register_right(&mut emulator.cpu, Register::D),
        0x0B =>
            bitops::rotate_register_right(&mut emulator.cpu, Register::E),
        0x0C =>
            bitops::rotate_register_right(&mut emulator.cpu, Register::H),
        0x0D =>
            bitops::rotate_register_right(&mut emulator.cpu, Register::L),
        0x0E =>
            bitops::rotate_memory_byte_right(emulator),
        0x0F =>
            bitops::rotate_register_right(&mut emulator.cpu, Register::A),
        0x10 =>
            bitops::rotate_register_left_through_carry(&mut emulator.cpu, Register::B),
        0x11 =>
            bitops::rotate_register_left_through_carry(&mut emulator.cpu, Register::C),
        0x12 =>
            bitops::rotate_register_left_through_carry(&mut emulator.cpu, Register::D),
        0x13 =>
            bitops::rotate_register_left_through_carry(&mut emulator.cpu, Register::E),
        0x14 =>
            bitops::rotate_register_left_through_carry(&mut emulator.cpu, Register::H),
        0x15 =>
            bitops::rotate_register_left_through_carry(&mut emulator.cpu, Register::L),
        0x16 =>
            bitops::rotate_memory_byte_left_through_carry(emulator),
        0x17 =>
            bitops::rotate_register_left_through_carry(&mut emulator.cpu, Register::A),
        0x18 =>
            bitops::rotate_register_right_through_carry(&mut emulator.cpu, Register::B),
        0x19 =>
            bitops::rotate_register_right_through_carry(&mut emulator.cpu, Register::C),
        0x1A =>
            bitops::rotate_register_right_through_carry(&mut emulator.cpu, Register::D),
        0x1B =>
            bitops::rotate_register_right_through_carry(&mut emulator.cpu, Register::E),
        0x1C =>
            bitops::rotate_register_right_through_carry(&mut emulator.cpu, Register::H),
        0x1D =>
            bitops::rotate_register_right_through_carry(&mut emulator.cpu, Register::L),    
        0x1E =>
            bitops::rotate_memory_byte_right_through_carry(emulator),
        0x1F =>
            bitops::rotate_register_right_through_carry(&mut emulator.cpu, Register::A),
        0x20 =>
            bitops::shift_register_left(&mut emulator.cpu, Register::B),
        0x21 =>
            bitops::shift_register_left(&mut emulator.cpu, Register::C),
        0x22 =>
            bitops::shift_register_left(&mut emulator.cpu, Register::D),
        0x23 =>
            bitops::shift_register_left(&mut emulator.cpu, Register::E),
        0x24 =>
            bitops::shift_register_left(&mut emulator.cpu, Register::H),
        0x25 =>
            bitops::shift_register_left(&mut emulator.cpu, Register::L),
        0x26 =>
            bitops::shift_memory_byte_left(emulator),
        0x27 =>
            bitops::shift_register_left(&mut emulator.cpu, Register::A),
        0x28 =>
            bitops::shift_register_right_maintaining_msb(&mut emulator.cpu, Register::B),
        0x29 =>
            bitops::shift_register_right_maintaining_msb(&mut emulator.cpu, Register::C),
        0x2A =>
            bitops::shift_register_right_maintaining_msb(&mut emulator.cpu, Register::D),
        0x2B =>
            bitops::shift_register_right_maintaining_msb(&mut emulator.cpu, Register::E),
        0x2C =>
            bitops::shift_register_right_maintaining_msb(&mut emulator.cpu, Register::H),
        0x2D =>
            bitops::shift_register_right_maintaining_msb(&mut emulator.cpu, Register::L),
        0x2E =>
            bitops::shift_memory_byte_right_maintaining_msb(emulator),
        0x2F =>
            bitops::shift_register_right_maintaining_msb(&mut emulator.cpu, Register::A),
        0x30 =>
            bitops::swap_nibbles_in_register(&mut emulator.cpu, Register::B),
        0x31 =>
            bitops::swap_nibbles_in_register(&mut emulator.cpu, Register::C),
        0x32 =>
            bitops::swap_nibbles_in_register(&mut emulator.cpu, Register::D),
        0x33 =>
            bitops::swap_nibbles_in_register(&mut emulator.cpu, Register::E),
        0x34 =>
            bitops::swap_nibbles_in_register(&mut emulator.cpu, Register::H),
        0x35 =>
            bitops::swap_nibbles_in_register(&mut emulator.cpu, Register::L),
        0x36 => {
            let address = microops::read_from_register_pair(&mut emulator.cpu, &REGISTER_HL);
            bitops::swap_nibbles_in_memory_byte(emulator, address);
        },
        0x37 =>
            bitops::swap_nibbles_in_register(&mut emulator.cpu, Register::A),
        0x38 =>
            bitops::shift_register_right(&mut emulator.cpu, Register::B),
        0x39 =>
            bitops::shift_register_right(&mut emulator.cpu, Register::C),
        0x3A =>
            bitops::shift_register_right(&mut emulator.cpu, Register::D),
        0x3B =>
            bitops::shift_register_right(&mut emulator.cpu, Register::E),
        0x3C =>
            bitops::shift_register_right(&mut emulator.cpu, Register::H),
        0x3D =>
            bitops::shift_register_right(&mut emulator.cpu, Register::L),
        0x3E =>
            bitops::shift_memory_byte_right(emulator),
        0x3F =>
            bitops::shift_register_right(&mut emulator.cpu, Register::A),
        0x40 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::B, 0),
        0x41 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::C, 0),
        0x42 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::D, 0),
        0x43 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::E, 0),   
        0x44 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::H, 0),
        0x45 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::L, 0),
        0x46 =>
            bitops::test_memory_bit(emulator, 0),
        0x47 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::A, 0),
        0x48 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::B, 1),
        0x49 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::C, 1),
        0x4A =>
            bitops::test_register_bit(&mut emulator.cpu, Register::D, 1),
        0x4B =>
            bitops::test_register_bit(&mut emulator.cpu, Register::E, 1),
        0x4C =>
            bitops::test_register_bit(&mut emulator.cpu, Register::H, 1),
        0x4D =>
            bitops::test_register_bit(&mut emulator.cpu, Register::L, 1),
        0x4E =>
            bitops::test_memory_bit(emulator, 1),
        0x4F =>
            bitops::test_register_bit(&mut emulator.cpu, Register::A, 1),
        0x50 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::B, 2),
        0x51 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::C, 2),  
        0x52 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::D, 2),
        0x53 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::E, 2),
        0x54 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::H, 2),
        0x55 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::L, 2),
        0x56 =>
            bitops::test_memory_bit(emulator, 2),
        0x57 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::A, 2),
        0x58 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::B, 3),
        0x59 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::C, 3),
        0x5A =>
            bitops::test_register_bit(&mut emulator.cpu, Register::D, 3),
        0x5B =>
            bitops::test_register_bit(&mut emulator.cpu, Register::E, 3),
        0x5C =>
            bitops::test_register_bit(&mut emulator.cpu, Register::H, 3),
        0x5D =>
            bitops::test_register_bit(&mut emulator.cpu, Register::L, 3),
        0x5E =>
            bitops::test_memory_bit(emulator, 3),
        0x5F =>
            bitops::test_register_bit(&mut emulator.cpu, Register::A, 3),
        0x60 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::B, 4),
        0x61 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::C, 4),
        0x62 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::D, 4),
        0x63 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::E, 4),
        0x64 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::H, 4),
        0x65 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::L, 4),
        0x66 =>
            bitops::test_memory_bit(emulator, 4),
        0x67 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::A, 4),
        0x68 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::B, 5),
        0x69 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::C, 5),
        0x6A =>
            bitops::test_register_bit(&mut emulator.cpu, Register::D, 5),
        0x6B =>
            bitops::test_register_bit(&mut emulator.cpu, Register::E, 5),
        0x6C =>
            bitops::test_register_bit(&mut emulator.cpu, Register::H, 5),
        0x6D =>
            bitops::test_register_bit(&mut emulator.cpu, Register::L, 5),
        0x6E =>
            bitops::test_memory_bit(emulator, 5),
        0x6F =>
            bitops::test_register_bit(&mut emulator.cpu, Register::A, 5),
        0x70 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::B, 6),
        0x71 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::C, 6),
        0x72 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::D, 6),
        0x73 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::E, 6),
        0x74 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::H, 6),
        0x75 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::L, 6),
        0x76 =>
            bitops::test_memory_bit(emulator, 6),
        0x77 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::A, 6),
        0x78 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::B, 7),
        0x79 =>
            bitops::test_register_bit(&mut emulator.cpu, Register::C, 7),
        0x7A =>
            bitops::test_register_bit(&mut emulator.cpu, Register::D, 7),
        0x7B =>
            bitops::test_register_bit(&mut emulator.cpu, Register::E, 7),
        0x7C =>
            bitops::test_register_bit(&mut emulator.cpu, Register::H, 7),
        0x7D =>
            bitops::test_register_bit(&mut emulator.cpu, Register::L, 7),
        0x7E =>
            bitops::test_memory_bit(emulator, 7),
        0x7F =>
            bitops::test_register_bit(&mut emulator.cpu, Register::A, 7),
        0x80 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::B, 0),
        0x81 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::C, 0),
        0x82 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::D, 0),
        0x83 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::E, 0),
        0x84 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::H, 0),
        0x85 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::L, 0),
        0x86 =>
            bitops::reset_memory_bit(emulator, 0),
        0x87 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::A, 0),
        0x88 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::B, 1),
        0x89 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::C, 1),
        0x8A =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::D, 1),
        0x8B =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::E, 1),
        0x8C =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::H, 1),
        0x8D =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::L, 1),
        0x8E =>
            bitops::reset_memory_bit(emulator, 1),
        0x8F =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::A, 1),
        0x90 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::B, 2),
        0x91 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::C, 2),
        0x92 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::D, 2),
        0x93 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::E, 2),
        0x94 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::H, 2),
        0x95 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::L, 2),
        0x96 =>
            bitops::reset_memory_bit(emulator, 2),
        0x97 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::A, 2),
        0x98 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::B, 3),
        0x99 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::C, 3),
        0x9A =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::D, 3),
        0x9B =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::E, 3),
        0x9C =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::H, 3),
        0x9D =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::L, 3),
        0x9E =>
            bitops::reset_memory_bit(emulator, 3),
        0x9F =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::A, 3),
        0xA0 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::B, 4),
        0xA1 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::C, 4),
        0xA2 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::D, 4),
        0xA3 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::E, 4),
        0xA4 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::H, 4),
        0xA5 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::L, 4),
        0xA6 =>
            bitops::reset_memory_bit(emulator, 4),
        0xA7 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::A, 4),
        0xA8 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::B, 5),
        0xA9 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::C, 5),
        0xAA =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::D, 5),
        0xAB =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::E, 5),
        0xAC =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::H, 5),
        0xAD =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::L, 5),
        0xAE =>
            bitops::reset_memory_bit(emulator, 5),
        0xAF =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::A, 5),
        0xB0 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::B, 6),
        0xB1 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::C, 6),
        0xB2 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::D, 6),
        0xB3 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::E, 6),
        0xB4 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::H, 6),
        0xB5 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::L, 6),
        0xB6 =>
            bitops::reset_memory_bit(emulator, 6),
        0xB7 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::A, 6),
        0xB8 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::B, 7),
        0xB9 =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::C, 7),
        0xBA =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::D, 7),
        0xBB =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::E, 7),
        0xBC =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::H, 7),
        0xBD =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::L, 7),
        0xBE =>
            bitops::reset_memory_bit(emulator, 7),
        0xBF =>
            bitops::reset_register_bit(&mut emulator.cpu, Register::A, 7),
        0xC0 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::B, 0),
        0xC1 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::C, 0),
        0xC2 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::D, 0),
        0xC3 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::E, 0),
        0xC4 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::H, 0),
        0xC5 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::L, 0),
        0xC6 =>
            bitops::set_memory_bit(emulator, 0),
        0xC7 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::A, 0),
        0xC8 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::B, 1),
        0xC9 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::C, 1),
        0xCA =>
            bitops::set_register_bit(&mut emulator.cpu, Register::D, 1),
        0xCB =>
            bitops::set_register_bit(&mut emulator.cpu, Register::E, 1),
        0xCC =>
            bitops::set_register_bit(&mut emulator.cpu, Register::H, 1),
        0xCD =>
            bitops::set_register_bit(&mut emulator.cpu, Register::L, 1),
        0xCE =>
            bitops::set_memory_bit(emulator, 1),
        0xCF =>
            bitops::set_register_bit(&mut emulator.cpu, Register::A, 1),
        0xD0 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::B, 2),
        0xD1 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::C, 2),
        0xD2 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::D, 2),
        0xD3 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::E, 2),
        0xD4 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::H, 2),
        0xD5 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::L, 2),
        0xD6 =>
            bitops::set_memory_bit(emulator, 2),
        0xD7 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::A, 2),
        0xD8 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::B, 3),
        0xD9 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::C, 3),
        0xDA =>
            bitops::set_register_bit(&mut emulator.cpu, Register::D, 3),
        0xDB =>
            bitops::set_register_bit(&mut emulator.cpu, Register::E, 3),
        0xDC =>
            bitops::set_register_bit(&mut emulator.cpu, Register::H, 3),
        0xDD =>
            bitops::set_register_bit(&mut emulator.cpu, Register::L, 3),
        0xDE =>
            bitops::set_memory_bit(emulator, 3),
        0xDF =>
            bitops::set_register_bit(&mut emulator.cpu, Register::A, 3),
        0xE0 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::B, 4),
        0xE1 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::C, 4),
        0xE2 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::D, 4),
        0xE3 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::E, 4),
        0xE4 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::H, 4),
        0xE5 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::L, 4),
        0xE6 =>
            bitops::set_memory_bit(emulator, 4),
        0xE7 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::A, 4),
        0xE8 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::B, 5),
        0xE9 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::C, 5),
        0xEA =>
            bitops::set_register_bit(&mut emulator.cpu, Register::D, 5),
        0xEB =>
            bitops::set_register_bit(&mut emulator.cpu, Register::E, 5),
        0xEC =>
            bitops::set_register_bit(&mut emulator.cpu, Register::H, 5),
        0xED =>
            bitops::set_register_bit(&mut emulator.cpu, Register::L, 5),
        0xEE =>
            bitops::set_memory_bit(emulator, 5),
        0xEF =>
            bitops::set_register_bit(&mut emulator.cpu, Register::A, 5),
        0xF0 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::B, 6),
        0xF1 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::C, 6),
        0xF2 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::D, 6),
        0xF3 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::E, 6),
        0xF4 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::H, 6),
        0xF5 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::L, 6),
        0xF6 =>
            bitops::set_memory_bit(emulator, 6),
        0xF7 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::A, 6),
        0xF8 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::B, 7),
        0xF9 =>
            bitops::set_register_bit(&mut emulator.cpu, Register::C, 7),
        0xFA =>
            bitops::set_register_bit(&mut emulator.cpu, Register::D, 7),
        0xFB =>
            bitops::set_register_bit(&mut emulator.cpu, Register::E, 7),
        0xFC =>
            bitops::set_register_bit(&mut emulator.cpu, Register::H, 7),
        0xFD =>
            bitops::set_register_bit(&mut emulator.cpu, Register::L, 7),
        0xFE =>
            bitops::set_memory_bit(emulator, 7),
        0xFF =>
            bitops::set_register_bit(&mut emulator.cpu, Register::A, 7),
    }
}

#[cfg(test)]
mod tests;
