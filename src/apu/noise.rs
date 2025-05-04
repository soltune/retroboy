use crate::apu::envelope;
use crate::apu::envelope::{initialize_envelope, Envelope};
use crate::apu::length;
use crate::apu::length::{initialize_length, Length};
use crate::utils::is_bit_set;
use crate::apu::utils::length_enabled;
use bincode::{Encode, Decode};

#[derive(Clone, Debug, Encode, Decode)]
pub struct NoiseChannel {
    pub enabled: bool,
    pub dac_enabled: bool,
    pub length: Length,
    pub envelope: Envelope,
    pub polynomial: u8,
    pub lfsr: u16,
    pub control: u8,
    pub period_divider: u16,
    pub instruction_cycles: u16
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

pub fn reset_noise_channel(original_noise_channel: &NoiseChannel, is_cgb: bool) -> NoiseChannel {
    let mut new_noise_channel = initialize_noise_channel();
    if !is_cgb {
        // On reset (when APU is powered down), maintain length timers, as this is expected behavior for DMG
        new_noise_channel.length = length::reset_initial_settings(&original_noise_channel.length);
    }
    new_noise_channel
}

const WIDTH_MODE_INDEX: u8 = 3;
const CONTROL_TRIGGER_INDEX: u8 = 7;

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
    let narrow_width = is_bit_set(channel.polynomial, WIDTH_MODE_INDEX);

    let first_lfsr_bit = channel.lfsr & 0b1;
    let second_lfsr_bit = (channel.lfsr & 0b10) >> 1;
    let xor_result = (!(first_lfsr_bit ^ second_lfsr_bit)) & 0b1;

    let mut next_lfsr = channel.lfsr | (xor_result << 15);

    if narrow_width {
        next_lfsr |= xor_result << 7;
    }

    next_lfsr >> 1
}

pub fn step(channel: &mut NoiseChannel, last_instruction_clock_cycles: u8) {
    channel.instruction_cycles += last_instruction_clock_cycles as u16;
    if channel.instruction_cycles >= channel.period_divider {
        channel.instruction_cycles = 0;
        channel.period_divider = calculate_period_divider(channel);
        channel.lfsr = calculate_next_lfsr(channel);
    }
}

pub fn step_envelope(channel: &mut NoiseChannel) {
    if channel.enabled {
        envelope::step(&mut channel.envelope);
    }
}

pub fn should_clock_length_on_enable(channel: &NoiseChannel, original_control_value: u8) -> bool {
    let new_control_value = channel.control;
    !length_enabled(original_control_value) && length_enabled(new_control_value)
}

pub fn should_clock_length_on_trigger(channel: &NoiseChannel) -> bool {
    length::at_max_length(&channel.length) && length_enabled(channel.control)
}

pub fn step_length(channel: &mut NoiseChannel) {
    let length_timer_enabled = length_enabled(channel.control);
    if length_timer_enabled {
        length::step(&mut channel.length);
        if channel.length.timer == 0 {
            disable(channel);
        } 
    }
}

pub fn digital_output(channel: &NoiseChannel) -> f32 {
    if channel.enabled {
        let amplitude = (channel.lfsr & 0x01) as u8;
        let current_volume = channel.envelope.current_volume;
        (amplitude * current_volume) as f32
    }
    else {
        7.5
    }
}

pub fn trigger(channel: &mut NoiseChannel) {
    if channel.dac_enabled {
        channel.enabled = true;
    }
    channel.period_divider = calculate_period_divider(channel);
    channel.lfsr = 0;
    length::reload_timer_with_maximum(&mut channel.length);
    envelope::reset_settings(&mut channel.envelope);
}

pub fn disable(channel: &mut NoiseChannel) {
    channel.enabled = false;
}

pub fn should_trigger(channel: &NoiseChannel) -> bool {
    is_bit_set(channel.control, CONTROL_TRIGGER_INDEX)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn enable_noise_channel(channel: &mut NoiseChannel) {
        channel.enabled = true;
        channel.dac_enabled = true;
    }

    #[test]
    fn should_calculate_dac_output_when_amplitude_is_zero() {
        let mut channel = initialize_noise_channel();
        enable_noise_channel(&mut channel);

        channel.lfsr = 0xFFFE;
        channel.envelope.current_volume = 0xA;

        assert_eq!(digital_output(&channel), 0.0);
    }

    #[test]
    fn should_calculate_dac_output_when_amplitude_is_one() {
        let mut channel = initialize_noise_channel();
        enable_noise_channel(&mut channel);

        channel.lfsr = 0xFFFF;
        channel.envelope.current_volume = 0xA;

        assert_eq!(digital_output(&channel), 10.0);
    }

    #[test]
    fn should_produce_no_audio_output_if_channel_is_disabled() {
        let mut channel = initialize_noise_channel();

        channel.lfsr = 0;
        channel.envelope.current_volume = 0xA;

        assert_eq!(digital_output(&channel), 7.5);
    }
}