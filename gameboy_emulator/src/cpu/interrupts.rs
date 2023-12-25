use crate::cpu::{CpuState, jumps};

pub enum InterruptType {
    VBlank,
    LCDStatus,
    TimerOverflow,
    SerialLink,
    JoypadPress
}

fn get_fired_interrupt_bits(cpu_state: &CpuState) -> u8 {
    cpu_state.memory.interrupt_registers.enabled & cpu_state.memory.interrupt_registers.flags & 0x1F
}

fn get_fired_interrupt(cpu_state: &CpuState) -> Option<InterruptType> {
    let fired_interrupt_bits = get_fired_interrupt_bits(cpu_state);
    if (fired_interrupt_bits & 0x01) != 0 {
        Some(InterruptType::VBlank)
    }
    else if (fired_interrupt_bits & 0x02) != 0 {
        Some(InterruptType::LCDStatus)
    }
    else if (fired_interrupt_bits & 0x04) != 0 {
        Some(InterruptType::TimerOverflow)
    }
    else if (fired_interrupt_bits & 0x08) != 0 {
        Some(InterruptType::SerialLink)
    }
    else if (fired_interrupt_bits & 0x10) != 0 {
        Some(InterruptType::JoypadPress)
    }
    else {
        None
    }
}

fn get_interrupt_isr(interrupt_type: &InterruptType) -> u8 {
    match interrupt_type {
        InterruptType::VBlank => 0x40,
        InterruptType::LCDStatus => 0x48,
        InterruptType::TimerOverflow => 0x50,
        InterruptType::SerialLink => 0x58,
        InterruptType::JoypadPress => 0x60
    }
}

fn turn_off_interrupt_flag(cpu_state: &mut CpuState, interrupt_type: &InterruptType) {
    let interrupt_registers = &mut cpu_state.memory.interrupt_registers;
    match interrupt_type {
        InterruptType::VBlank =>
            interrupt_registers.flags = interrupt_registers.flags & !0x01,
        InterruptType::LCDStatus =>
            interrupt_registers.flags = interrupt_registers.flags & !0x02,
        InterruptType::TimerOverflow =>
            interrupt_registers.flags = interrupt_registers.flags & !0x04,
        InterruptType::SerialLink =>
            interrupt_registers.flags = interrupt_registers.flags & !0x08,
        InterruptType::JoypadPress =>
            interrupt_registers.flags = interrupt_registers.flags & !0x10
    }
}

pub fn interrupts_fired(cpu_state: &CpuState) -> bool {
    let fired_interrupt_bits = get_fired_interrupt_bits(cpu_state);
    fired_interrupt_bits != 0
}

pub fn handle_fired_interrupt(cpu_state: &mut CpuState) {
    if cpu_state.interrupts.enabled && interrupts_fired(cpu_state) {
        let maybe_fired_interrupt = get_fired_interrupt(cpu_state);
        match maybe_fired_interrupt {
            Some(interrupt_type) => {
                cpu_state.interrupts.enabled = false;
                turn_off_interrupt_flag(cpu_state, &interrupt_type);
                let isr_address = get_interrupt_isr(&interrupt_type);
                jumps::restart(cpu_state, isr_address as u16);
            },
            None => ()
        }
    }
}
