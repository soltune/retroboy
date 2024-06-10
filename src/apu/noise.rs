use crate::apu::envelope::{initialize_envelope, Envelope};
use crate::apu::length::{initialize_length, Length};

#[derive(Debug)]
pub struct NoiseChannel {
    pub enabled: bool,
    pub dac_enabled: bool,
    pub length: Length,
    pub envelope: Envelope,
    pub polynomial: u8,
    pub lfsr: u16,
    pub control: u8,
    pub period_divider: u16,
    pub instruction_cycles: u8
}

pub fn initialize_noise_channel() -> NoiseChannel {
    NoiseChannel {
        enabled: false,
        dac_enabled: false,
        length: initialize_length(),
        envelope: initialize_envelope(),
        polynomial: 0,
        lfsr: 0,
        control: 0,
        period_divider: 0,
        instruction_cycles: 0
    }
}

// Divider for noise channel clocked at 266,144 Hz. Four times slower
// than pulse channel. Therefore, we should only decrement the period
// divider every 16 T-cycles.
const PERIOD_DIVIDER_RATE_IN_INSTRUCTION_CYCLES: u8 = 16;

pub fn step(channel: &mut NoiseChannel, last_instruction_clock_cycles: u8) {
    channel.instruction_cycles += last_instruction_clock_cycles;
    if channel.instruction_cycles >= PERIOD_DIVIDER_RATE_IN_INSTRUCTION_CYCLES {
        channel.instruction_cycles = 0;
        channel.period_divider -= 1;
    }
}