use crate::cpu::interrupts::InterruptRegisters;
use bincode::{Encode, Decode};

const BASE_SPEED_RATE: u8 = 4;
const DIVIDER_RATE: u8 = 16;

#[derive(Clone, Encode, Decode, Debug)]
pub struct TimerRegisters {
    m_cycles_clock: u8,
    divider_clock: u8,
    base_clock: u8,
    divider: u8,
    counter: u8,
    modulo: u8,
    control: u8
}

pub struct TimerParams<'a> {
    pub interrupt_registers: &'a mut InterruptRegisters,
}

impl TimerRegisters {
    pub fn new() -> Self {
        TimerRegisters {
            m_cycles_clock: 0,
            divider_clock: 0,
            base_clock: 0,
            divider: 0,
            counter: 0,
            modulo: 0,
            control: 0
        }
    }

    fn get_counter_rate(&self) -> Option<u8> {
        match self.control & 0x07 {
            0x04 => Some(64),
            0x05 => Some(1),
            0x06 => Some(4),
            0x7 => Some(16),
            _ => None
        }
    }

    fn increment_div_register(&mut self) {
        self.divider_clock += 1;

        if self.divider_clock >= DIVIDER_RATE {
            self.divider = self.divider.wrapping_add(1);
            self.divider_clock = 0;
        }
    }

    fn increment_counter_register(&mut self, interrupt_registers: &mut InterruptRegisters, counter_rate: u8) {
        self.base_clock += 1;

        if self.base_clock >= counter_rate {
            self.base_clock = 0;

            if self.counter == 0xFF {
                self.counter = self.modulo;
                interrupt_registers.flags |= 0x04;
            }
            else {
                self.counter += 1
            }
        }
    }

    pub fn step(&mut self, params: TimerParams) {
        self.m_cycles_clock += 1;

        if self.m_cycles_clock >= BASE_SPEED_RATE {
            self.m_cycles_clock -= BASE_SPEED_RATE;

            self.increment_div_register();

            match self.get_counter_rate() {
                Some(counter_rate) => {
                    self.increment_counter_register(params.interrupt_registers, counter_rate);
                }
                _ => ()
            }
        }
    }

    pub fn divider(&self) -> u8 {
        self.divider
    }

    pub fn counter(&self) -> u8 {
        self.counter
    }

    pub fn modulo(&self) -> u8 {
        self.modulo
    }

    pub fn control(&self) -> u8 {
        self.control
    }

    pub fn set_divider(&mut self, value: u8) {
        self.divider = value;
        self.divider_clock = 0;
        self.m_cycles_clock = 0;
    }

    pub fn set_counter(&mut self, value: u8) {
        self.counter = value;
    }

    pub fn set_modulo(&mut self, value: u8) {
        self.modulo = value;
    }

    pub fn set_control(&mut self, value: u8) {
        self.control = value;
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::interrupts::initialize_interrupt_registers;
    use super::*;

    fn step_timers(timers: &mut TimerRegisters) {
        let mut interrupts = initialize_interrupt_registers();
        let params = TimerParams { interrupt_registers: &mut interrupts };
        timers.step(params);
    }

    #[test]
    fn increments_base_speed_by_number_of_m_cycles() {
        let mut timers = TimerRegisters::new();
        step_timers(&mut timers);
        assert_eq!(timers.m_cycles_clock, 1);
    }

    #[test]
    fn resets_base_speed_after_four_m_cycles() {
        let mut timers = TimerRegisters::new();
        timers.m_cycles_clock = 3;
        step_timers(&mut timers);
        assert_eq!(timers.m_cycles_clock, 0);
    }

    #[test]
    fn increments_divider_clock_after_four_m_cycles() {
        let mut timers = TimerRegisters::new();
        timers.m_cycles_clock = 3;
        step_timers(&mut timers);
        assert_eq!(timers.divider_clock, 1);
    }

    #[test]
    fn increments_divider_register_after_sixteen_divider_clock_increments() {
        let mut timers = TimerRegisters::new();
        timers.m_cycles_clock = 3;
        timers.divider_clock = 15;
        step_timers(&mut timers);
        assert_eq!(timers.divider, 1);
        assert_eq!(timers.divider_clock, 0);
    }

    #[test]
    fn wraps_when_divider_register_overflows() {
        let mut timers = TimerRegisters::new();
        timers.m_cycles_clock = 3;
        timers.divider_clock = 15;
        timers.divider = 0xFF;
        step_timers(&mut timers);
        assert_eq!(timers.divider, 0);
        assert_eq!(timers.divider_clock, 0);
    }

    #[test]
    fn increments_counter_register_at_a_fourth_the_rate_of_base_speed_when_configured() {
        let mut timers = TimerRegisters::new();
        timers.m_cycles_clock = 3;
        timers.base_clock = 3;
        timers.control = 0x06;
        step_timers(&mut timers);
        assert_eq!(timers.counter, 1);
        assert_eq!(timers.base_clock, 0);
    }

    #[test]
    fn increments_counter_register_at_a_sixteenth_of_the_rate_of_base_speed_when_configured() {
        let mut timers = TimerRegisters::new();
        timers.m_cycles_clock = 3;
        timers.base_clock = 15;
        timers.control = 0x07;
        step_timers(&mut timers);
        assert_eq!(timers.counter, 1);
        assert_eq!(timers.base_clock, 0);
    }

    #[test]
    fn increments_counter_register_at_a_sixty_fourth_of_the_rate_of_base_speed_when_configured() {
        let mut timers = TimerRegisters::new();
        timers.m_cycles_clock = 3;
        timers.base_clock = 63;
        timers.control = 0x04;
        step_timers(&mut timers);
        assert_eq!(timers.counter, 1);
        assert_eq!(timers.base_clock, 0);
    }

    #[test]
    fn increments_counter_register_at_same_rate_of_base_speed_when_configured() {
        let mut timers = TimerRegisters::new();
        timers.m_cycles_clock = 3;
        timers.base_clock = 0;
        timers.control = 0x05;
        step_timers(&mut timers);
        assert_eq!(timers.counter, 1);
        assert_eq!(timers.base_clock, 0);
    }

    #[test]
    fn should_not_increment_counter_register_at_wrong_time() {
        let mut timers = TimerRegisters::new();
        timers.m_cycles_clock = 3;
        timers.base_clock = 14;
        timers.control = 0x07;
        step_timers(&mut timers);
        assert_eq!(timers.counter, 0);
        assert_eq!(timers.base_clock, 15);
    }

    #[test]
    fn should_not_increment_counter_register_if_timer_is_off() {
        let mut timers = TimerRegisters::new();
        timers.m_cycles_clock = 3;
        timers.base_clock = 15;
        timers.control = 0;
        step_timers(&mut timers);
        assert_eq!(timers.counter, 0);
        assert_eq!(timers.base_clock, 15);
    }

    #[test]
    fn should_fire_interrupt_on_counter_register_overflow() {
        let mut timers = TimerRegisters::new();
        let mut interrupts = initialize_interrupt_registers();
        timers.m_cycles_clock = 3;
        timers.base_clock = 0x15;
        timers.control = 0x07;
        timers.counter = 0xFF;
        let params = TimerParams { interrupt_registers: &mut interrupts };
        timers.step(params);
        assert_eq!(timers.counter, 0);
        assert_eq!(timers.base_clock, 0);
        assert_eq!(interrupts.flags, 0x04);
    }

    #[test]
    fn should_reset_counter_register_to_modulo_on_overflow() {
        let mut timers = TimerRegisters::new();
        let mut interrupts = initialize_interrupt_registers();
        timers.m_cycles_clock = 3;
        timers.base_clock = 0x15;
        timers.control = 0x07;
        timers.counter = 0xFF;
        timers.modulo = 0x04;
        let params = TimerParams { interrupt_registers: &mut interrupts };
        timers.step(params);
        assert_eq!(timers.counter, 0x04);
        assert_eq!(timers.base_clock, 0);
        assert_eq!(interrupts.flags, 0x04);
    }
}
