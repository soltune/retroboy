use crate::apu::envelope;
use crate::apu::envelope::{initialize_envelope, Envelope};
use crate::apu::length;
use crate::apu::length::{initialize_length, Length};
use crate::utils::is_bit_set;

use super::utils::as_dac_output;

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

const WIDTH_MODE_INDEX: u8 = 3;
const CONTROL_TRIGGER_INDEX: u8 = 7;
const CONTROL_LENGTH_ENABLED_INDEX: u8 = 6;

fn calculate_period_divider(channel: &NoiseChannel) -> u16 {
    let shift_amount = (channel.polynomial & 0b11110000) >> 4;
    let divisor_code = channel.polynomial & 0b111;
    let divisor = if divisor_code == 0 {
        8
    }
    else {
        (divisor_code as u16) << 4
    };
    divisor << shift_amount
}

fn calculate_next_lfsr(channel: &NoiseChannel) -> u16 {
    let width_mode = is_bit_set(channel.polynomial, WIDTH_MODE_INDEX);

    let first_lfsr_bit = channel.lfsr & 0b1;
    let second_lfsr_bit = (channel.lfsr & 0b10) >> 1;
    let xor_result = first_lfsr_bit ^ second_lfsr_bit;

    let mut next_lfsr = (channel.lfsr >> 1) | (xor_result << 14);

    if width_mode {
        next_lfsr &= !(1 << 6);
        next_lfsr |= xor_result << 6;
    }

    next_lfsr
}

pub fn step(channel: &mut NoiseChannel, last_instruction_clock_cycles: u8) {
    channel.instruction_cycles += last_instruction_clock_cycles;
    if channel.instruction_cycles >= PERIOD_DIVIDER_RATE_IN_INSTRUCTION_CYCLES {
        channel.instruction_cycles = 0;
        channel.period_divider -= 1;
        if channel.period_divider == 0 {
            channel.period_divider = calculate_period_divider(channel);
            channel.lfsr = calculate_next_lfsr(channel);
        }
    }
}

pub fn step_envelope(channel: &mut NoiseChannel) {
    if channel.enabled {
        envelope::step(&mut channel.envelope);
    }
}

pub fn step_length(channel: &mut NoiseChannel) {
    if channel.enabled {
        let length_timer_enabled = is_bit_set(channel.control, CONTROL_LENGTH_ENABLED_INDEX);
        if length_timer_enabled {
            length::step(&mut channel.length);
            if channel.length.timer == 0 {
                disable(channel);
            } 
        }
    }
}

pub fn dac_output(channel: &NoiseChannel) -> f32 {
    let amplitude = (!channel.lfsr & 0x01) as u8;
    let current_volume = channel.envelope.current_volume;
    let dac_input = amplitude * current_volume;
    as_dac_output(dac_input)
}

pub fn trigger(channel: &mut NoiseChannel) {
    channel.enabled = true;
    channel.lfsr = 0xFFFF;
    envelope::trigger(&mut channel.envelope);
    length::trigger(&mut channel.length, false);
}

pub fn disable(channel: &mut NoiseChannel) {
    channel.dac_enabled = false;
    channel.enabled = false;
}

pub fn should_trigger(channel: &NoiseChannel) -> bool {
   channel.dac_enabled && is_bit_set(channel.control, CONTROL_TRIGGER_INDEX)
}

#[cfg(test)]
mod tests;