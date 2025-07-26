use crate::cpu::{Cpu, jumps};
use crate::cpu::microops;
use crate::serializable::Serializable;
use serializable_derive::Serializable;

pub enum InterruptType {
    VBlank,
    LCDStatus,
    TimerOverflow,
    SerialLink,
    JoypadPress
}

#[derive(Serializable, Debug)]
pub struct InterruptRegisters {
    pub enabled: u8,
}

pub fn initialize_interrupt_registers() -> InterruptRegisters {
    InterruptRegisters {
        enabled: 0,
    }
}

fn get_fired_interrupt_bits(cpu_state: &Cpu) -> u8 {
    let interrupts = cpu_state.address_bus.interrupts();
    interrupts.enabled & cpu_state.address_bus.interrupt_flags() & 0x1F
}

fn get_fired_interrupt(cpu_state: &Cpu) -> Option<InterruptType> {
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

fn turn_off_interrupt_flag(cpu_state: &mut Cpu, interrupt_type: &InterruptType) {
    match interrupt_type {
        InterruptType::VBlank =>
            { cpu_state.address_bus.gpu_mut().set_vblank_interrupt(false); },
        InterruptType::LCDStatus =>
            { cpu_state.address_bus.gpu_mut().set_stat_interrupt(false); },
        InterruptType::TimerOverflow =>
            { cpu_state.address_bus.timers_mut().set_interrupt(false); },
        InterruptType::SerialLink =>
            { cpu_state.address_bus.serial_mut().set_interrupt(false); },
        InterruptType::JoypadPress =>
            { cpu_state.address_bus.joypad_mut().set_interrupt(false); }
    }
}

pub fn interrupts_fired(cpu_state: &Cpu) -> bool {
    let fired_interrupt_bits = get_fired_interrupt_bits(cpu_state);
    fired_interrupt_bits != 0
}

pub fn step(cpu_state: &mut Cpu) -> bool {
    if cpu_state.interrupts.enabled && interrupts_fired(cpu_state) {
        let maybe_fired_interrupt = get_fired_interrupt(cpu_state);
        match maybe_fired_interrupt {
            Some(interrupt_type) => {
                cpu_state.interrupts.enabled = false;
                turn_off_interrupt_flag(cpu_state, &interrupt_type);
                let isr_address = get_interrupt_isr(&interrupt_type);
                microops::step_machine_cycles(cpu_state, 2);
                jumps::restart(cpu_state, isr_address as u16);
                true
            },
            None => false
        }
    }
    else {
        false
    }
}
