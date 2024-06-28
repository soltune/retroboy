use crate::cpu::interrupts::InterruptRegisters;
use crate::emulator::Emulator;
use crate::utils::T_CYCLE_INCREMENT;

const BASE_SPEED_RATE: u8 = 4;
const DIVIDER_RATE: u8 = 16;

#[derive(Debug)]
pub struct TimerRegisters {
    pub m_cycles_clock: u8,
    pub divider_clock: u8,
    pub base_clock: u8,
    pub divider: u8,
    pub counter: u8,
    pub modulo: u8,
    pub control: u8
}

fn get_counter_rate(timer_registers: &TimerRegisters) -> Option<u8> {
    let control = timer_registers.control;
    match control & 0x07 {
        0x04 => Some(64),
        0x05 => Some(1),
        0x06 => Some(4),
        0x7 => Some(16),
        _ => None
    }
}

fn increment_div_register(timer_registers: &mut TimerRegisters) {
    timer_registers.divider_clock += 1;

    if timer_registers.divider_clock >= DIVIDER_RATE {
        timer_registers.divider = timer_registers.divider.wrapping_add(1);
        timer_registers.divider_clock = 0;
    }
}

fn increment_counter_register(timer_registers: &mut TimerRegisters,
                              interrupt_registers: &mut InterruptRegisters,
                              counter_rate: u8) {
    timer_registers.base_clock += 1;

    if timer_registers.base_clock >= counter_rate {
        timer_registers.base_clock = 0;

        if timer_registers.counter == 0xFF {
            timer_registers.counter = timer_registers.modulo;
            interrupt_registers.flags |= 0x04;
        }
        else {
            timer_registers.counter += 1
        }
    }
}

pub fn skip_bios(emulator: &mut Emulator) {
    emulator.timers.control = 0xF8;
    emulator.timers.divider = 0xAB;
}

pub fn step(emulator: &mut Emulator) {
    let timer_registers = &mut emulator.timers;
    let instruction_cycles = T_CYCLE_INCREMENT;
    let machine_cycles = instruction_cycles / 4;
    
    timer_registers.m_cycles_clock += machine_cycles;
    if timer_registers.m_cycles_clock >= BASE_SPEED_RATE {
        timer_registers.m_cycles_clock -= BASE_SPEED_RATE;

        increment_div_register(timer_registers);

        match get_counter_rate(timer_registers) {
            Some(counter_rate) => {
                let interrupt_registers = &mut emulator.interrupts;
                increment_counter_register(timer_registers, interrupt_registers, counter_rate);
            }
            _ => ()
        }
    }
}

#[cfg(test)]
mod tests;
