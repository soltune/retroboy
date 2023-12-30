use crate::mmu::{Memory, TimerRegisters, InterruptRegisters};

static BASE_SPEED_RATE: u8 = 4;
static DIVIDER_RATE: u8 = 16;

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

pub fn increment_timer(memory: &mut Memory, opcode_machine_cycles: u8) {
    let timer_registers = &mut memory.timer_registers;

    timer_registers.m_cycles_clock += opcode_machine_cycles;
    if timer_registers.m_cycles_clock >= BASE_SPEED_RATE {
        timer_registers.m_cycles_clock -= BASE_SPEED_RATE;

        increment_div_register(timer_registers);

        match get_counter_rate(timer_registers) {
            Some(counter_rate) => {
                let interrupt_registers = &mut memory.interrupt_registers;
                increment_counter_register(timer_registers, interrupt_registers, counter_rate);
            }
            _ => ()
        }
    }
}

#[cfg(test)]
mod tests;
