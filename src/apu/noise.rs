use crate::apu::envelope::Envelope;
use crate::apu::length::{DEFAULT_MAX_LENGTH, Length};
use crate::utils::is_bit_set;
use crate::apu::utils::length_enabled;
use bincode::{Encode, Decode};

#[derive(Clone, Debug, Encode, Decode)]
pub struct NoiseChannel {
    enabled: bool,
    dac_enabled: bool,
    length: Length,
    envelope: Envelope,
    polynomial: u8,
    lfsr: u16,
    control: u8,
    period_divider: u16,
    instruction_cycles: u16
}

const WIDTH_MODE_INDEX: u8 = 3;
const CONTROL_TRIGGER_INDEX: u8 = 7;

impl NoiseChannel {
    pub fn new() -> Self {
        NoiseChannel {
            enabled: false,
            dac_enabled: false,
            length: Length::new(DEFAULT_MAX_LENGTH),
            envelope: Envelope::new(),
            polynomial: 0,
            lfsr: 0,
            control: 0,
            period_divider: 0,
            instruction_cycles: 0
        }
    }

    pub fn reset(original_channel: &NoiseChannel, cgb_mode: bool) -> NoiseChannel {
        let mut new_channel = NoiseChannel::new();

        if !cgb_mode {
           new_channel.length = Length::reset_initial_settings(&original_channel.length); 
        }
        
        new_channel
    }

    fn calculate_period_divider(&self) -> u16 {
        let shift_amount = (self.polynomial & 0b11110000) >> 4;
        let divisor_code = self.polynomial & 0b111;
        let divisor = if divisor_code == 0 {
            8
        }
        else {
            (divisor_code as u16) << 4
        };
        divisor << shift_amount
    }

    fn calculate_next_lfsr(&self) -> u16 {
        let narrow_width = is_bit_set(self.polynomial, WIDTH_MODE_INDEX);

        let first_lfsr_bit = self.lfsr & 0b1;
        let second_lfsr_bit = (self.lfsr & 0b10) >> 1;
        let xor_result = (!(first_lfsr_bit ^ second_lfsr_bit)) & 0b1;

        let mut next_lfsr = self.lfsr | (xor_result << 15);

        if narrow_width {
            next_lfsr |= xor_result << 7;
        }

        next_lfsr >> 1
    }

    pub fn step(&mut self, last_instruction_clock_cycles: u8) {
        self.instruction_cycles += last_instruction_clock_cycles as u16;
        if self.instruction_cycles >= self.period_divider {
            self.instruction_cycles = 0;
            self.period_divider = self.calculate_period_divider();
            self.lfsr = self.calculate_next_lfsr();
        }
    }

    pub fn step_envelope(&mut self) {
        if self.enabled {
            self.envelope.step();
        }
    }

    pub fn should_clock_length_on_enable(&self, original_control_value: u8) -> bool {
        let new_control_value = self.control;
        !length_enabled(original_control_value) && length_enabled(new_control_value)
    }

    pub fn should_clock_length_on_trigger(&self) -> bool {
        self.length.at_max_length() && length_enabled(self.control)
    }

    pub fn step_length(&mut self) {
        if length_enabled(self.control) {
            self.length.step();
            if self.length.timer_expired() {
                self.set_enabled(false);
            }
        }
    }

    pub fn digital_output(&self) -> f32 {
        if self.enabled {
            let amplitude = (self.lfsr & 0x01) as u8;
            let current_volume = self.envelope.current_volume();
            (amplitude * current_volume) as f32
        }
        else {
            7.5
        }
    }

    pub fn trigger(&mut self) {
        if self.dac_enabled {
            self.enabled = true;
        }
        self.period_divider = self.calculate_period_divider();
        self.lfsr = 0;
        self.length.reload_timer();
        self.envelope.reset_settings();
    }

    pub fn should_trigger(&self) -> bool {
        is_bit_set(self.control, CONTROL_TRIGGER_INDEX)
    }

    pub fn dac_enabled(&self) -> bool {
        self.dac_enabled
    }

    pub fn set_dac_enabled(&mut self, value: bool) {
        self.dac_enabled = value;
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = value;
    }

    pub fn control(&self) -> u8 {
        self.control
    }

    pub fn set_control(&mut self, value: u8) {
        self.control = value;
    }

    pub fn polynomial(&self) -> u8 {
        self.polynomial
    }

    pub fn set_polynomial(&mut self, value: u8) {
        self.polynomial = value;
    }

    pub fn period_divider(&self) -> u16 {
        self.period_divider
    }

    pub fn set_period_divider(&mut self, value: u16) {
        self.period_divider = value;
    }

    pub fn lfsr(&self) -> u16 {
        self.lfsr
    }

    pub fn set_lfsr(&mut self, value: u16) {
        self.lfsr = value;
    }

    pub fn envelope(&mut self) -> &mut Envelope {
        &mut self.envelope
    }

    pub fn envelope_readonly(&self) -> &Envelope {
        &self.envelope
    }

    pub fn length(&mut self) -> &mut Length {
        &mut self.length
    }
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
        let mut channel = NoiseChannel::new();
        enable_noise_channel(&mut channel);

        channel.lfsr = 0xFFFE;
        channel.envelope.set_current_volume(0xA);

        assert_eq!(channel.digital_output(), 0.0);
    }

    #[test]
    fn should_calculate_dac_output_when_amplitude_is_one() {
        let mut channel = NoiseChannel::new();
        enable_noise_channel(&mut channel);

        channel.lfsr = 0xFFFF;
        channel.envelope.set_current_volume(0xA);

        assert_eq!(channel.digital_output(), 10.0);
    }

    #[test]
    fn should_produce_no_audio_output_if_channel_is_disabled() {
        let mut channel = NoiseChannel::new();

        channel.lfsr = 0;
        channel.envelope.set_current_volume(0xA);

        assert_eq!(channel.digital_output(), 7.5);
    }
}