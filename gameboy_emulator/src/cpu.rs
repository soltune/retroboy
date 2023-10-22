use crate::mmu;

#[derive(Debug)]
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    f: u8,
    program_counter: u16,
    stack_pointer: u16,
}

#[derive(Debug)]
pub struct Clock {
    last_instr_machine_cycles: u8,
    last_instr_clock_cycles: u8,
    machine_cycles: u32,
    clock_cycles: u32
}

#[derive(Debug)]
pub struct CpuState {
    pub registers: Registers,
    pub clock: Clock,
    pub memory: mmu::Memory
}

pub fn initialize_cpu_state() -> CpuState {
    CpuState {
        registers: Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            f: 0,
            program_counter: 0,
            stack_pointer: 0
        },
        clock: Clock {
            last_instr_machine_cycles: 0,
            last_instr_clock_cycles: 0,
            machine_cycles: 0,
            clock_cycles: 0
        },
        memory: mmu::Memory {
            in_bios: false,
            bios: [0; 0x100],
            rom: [0; 0x8000],
            video_ram: [0; 0x2000],
            object_attribute_memory: [0; 0xa0],
            working_ram: [0; 0x3e00],
            external_ram: [0; 0x2000],
            zero_page_ram: [0; 0x80]
        }
    }
}
