use crate::address_bus::AddressBus;
use crate::cpu::jumps;
use crate::cpu::microops;
use crate::emulator::Emulator;
use crate::serializable::Serializable;
use crate::utils::is_bit_set;
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

pub fn interrupt_flags(address_bus: &AddressBus) -> u8 {
    let mut flags = 0;
    if address_bus.gpu().vblank_interrupt() {
        flags |= 0x1;
    }
    if address_bus.gpu().stat_interrupt() {
        flags |= 0x2;
    }
    if address_bus.timers().interrupt() {
        flags |= 0x4;
    }
    if address_bus.serial().interrupt() {
        flags |= 0x8;
    }
    if address_bus.joypad().interrupt() {
        flags |= 0x10;
    }
    flags
}

pub fn set_interrupt_flags(address_bus: &mut AddressBus, flags: u8) {
    address_bus.gpu_mut().set_vblank_interrupt(is_bit_set(flags, 0));
    address_bus.gpu_mut().set_stat_interrupt(is_bit_set(flags, 1));
    address_bus.timers_mut().set_interrupt(is_bit_set(flags, 2));
    address_bus.serial_mut().set_interrupt(is_bit_set(flags, 3));
    address_bus.joypad_mut().set_interrupt(is_bit_set(flags, 4));
}

fn get_fired_interrupt_bits(emulator: &Emulator) -> u8 {
    let interrupts = emulator.address_bus.interrupts();
    interrupts.enabled & interrupt_flags(&emulator.address_bus) & 0x1F
}

fn get_fired_interrupt(emulator: &Emulator) -> Option<InterruptType> {
    let fired_interrupt_bits = get_fired_interrupt_bits(emulator);
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

fn turn_off_interrupt_flag(emulator: &mut Emulator, interrupt_type: &InterruptType) {
    match interrupt_type {
        InterruptType::VBlank =>
            { emulator.address_bus.gpu_mut().set_vblank_interrupt(false); },
        InterruptType::LCDStatus =>
            { emulator.address_bus.gpu_mut().set_stat_interrupt(false); },
        InterruptType::TimerOverflow =>
            { emulator.address_bus.timers_mut().set_interrupt(false); },
        InterruptType::SerialLink =>
            { emulator.address_bus.serial_mut().set_interrupt(false); },
        InterruptType::JoypadPress =>
            { emulator.address_bus.joypad_mut().set_interrupt(false); }
    }
}

pub fn interrupts_fired(emulator: &Emulator) -> bool {
    let fired_interrupt_bits = get_fired_interrupt_bits(emulator);
    fired_interrupt_bits != 0
}

pub fn step(emulator: &mut Emulator) -> bool {
    if emulator.cpu.interrupts.enabled && interrupts_fired(emulator) {
        let maybe_fired_interrupt = get_fired_interrupt(emulator);
        match maybe_fired_interrupt {
            Some(interrupt_type) => {
                emulator.cpu.interrupts.enabled = false;
                turn_off_interrupt_flag(emulator, &interrupt_type);
                let isr_address = get_interrupt_isr(&interrupt_type);
                microops::step_machine_cycles(emulator, 2);
                jumps::restart(emulator, isr_address as u16);
                true
            },
            None => false
        }
    }
    else {
        false
    }
}
