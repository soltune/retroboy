use crate::apu::envelope;
use crate::apu::envelope::{initialize_envelope, Envelope};
use crate::apu::length;
use crate::apu::length::{initialize_length, Length};
use crate::apu::period;
use crate::apu::period::{initalize_period, Period};
use crate::apu::sweep;
use crate::apu::sweep::{initialize_sweep, Sweep};
use crate::apu::utils::{bounded_wrapping_add, length_enabled};
use crate::utils::{get_bit, is_bit_set};

#[derive(Debug)]
pub struct PulseChannel {
    pub enabled: bool,
    pub dac_enabled: bool,
    pub wave_duty_position: u8,
    pub sweep: Sweep,
    pub length: Length,
    pub envelope: Envelope,
    pub period: Period,
}

pub fn initialize_pulse_channel() -> PulseChannel {
    PulseChannel {
        enabled: false,
        dac_enabled: false,
        wave_duty_position: 0,
        sweep: initialize_sweep(),
        length: initialize_length(),
        envelope: initialize_envelope(),
        period: initalize_period(),
    } 
}

pub fn reset_pulse_channel(original_pulse_channel: &PulseChannel) -> PulseChannel {
    let mut new_pulse_channel = initialize_pulse_channel();
    // On reset (when APU is powered down), maintain length timers, as this is expected behavior for DMG
    new_pulse_channel.length = length::reset_initial_settings(&original_pulse_channel.length);
    new_pulse_channel
}

const MAX_WAVEFORM_STEPS: u8 = 7;
const PERIOD_HIGH_TRIGGER_INDEX: u8 = 7;

const WAVEFORMS: [u8; 4] = [
    0b00000001,
    0b00000011,
    0b00001111,
    0b11111100
];

pub fn step(channel: &mut PulseChannel, last_instruction_clock_cycles: u8) {
    if channel.enabled {
        period::step(&mut channel.period, last_instruction_clock_cycles / 4, || {
            channel.wave_duty_position = bounded_wrapping_add(channel.wave_duty_position, MAX_WAVEFORM_STEPS);
        });
    }
}

pub fn step_envelope(channel: &mut PulseChannel) {
    if channel.enabled {
        envelope::step(&mut channel.envelope);
    }
}

pub fn should_clock_length_on_enable(channel: &PulseChannel, original_period_high_value: u8) -> bool {
    let new_period_high_value = channel.period.high;
    !length_enabled(original_period_high_value) && length_enabled(new_period_high_value)
}

pub fn should_clock_length_on_trigger(channel: &PulseChannel) -> bool {
    length::at_max_length(&channel.length) && length_enabled(channel.period.high)
}

pub fn step_length(channel: &mut PulseChannel) {
    let length_timer_enabled = length_enabled(channel.period.high);
    if length_timer_enabled {
        length::step(&mut channel.length);
        if channel.length.timer == 0 {
            disable(channel);
        } 
    }
}

pub fn digital_output(channel: &PulseChannel) -> f32 {
    if channel.enabled {    
        let wave_duty = (channel.length.initial_settings & 0b11000000) >> 6;
        let waveform = WAVEFORMS[wave_duty as usize];
        let amplitude = get_bit(waveform, channel.wave_duty_position);
        let current_volume = channel.envelope.current_volume;
        (amplitude * current_volume) as f32
    }
    else {
        7.5
    }
}

pub fn step_sweep(channel: &mut PulseChannel) {
    if channel.enabled {
        sweep::step(channel);
    }
}

pub fn trigger(channel: &mut PulseChannel, with_sweep: bool) {
    if channel.dac_enabled {
        channel.enabled = true;
    }
    period::trigger(&mut channel.period);
    length::reload_timer_with_maximum(&mut channel.length);
    envelope::trigger(&mut channel.envelope);
    if with_sweep {
        sweep::trigger(channel);
    }
}

pub fn disable(channel: &mut PulseChannel) {
    channel.enabled = false;
}

pub fn should_trigger(channel: &PulseChannel) -> bool {
   is_bit_set(channel.period.high, PERIOD_HIGH_TRIGGER_INDEX)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn enable_pulse_channel(channel: &mut PulseChannel) {
        channel.enabled = true;
        channel.dac_enabled = true;
    }

    fn initialize_amplitude_variables(channel: &mut PulseChannel,
        wave_duty: u8,
        wave_duty_position: u8,
        current_volume: u8) {
        channel.length.initial_settings = wave_duty << 6;
        channel.wave_duty_position = wave_duty_position;
        channel.envelope.current_volume = current_volume;
    }

    #[test]
    fn should_calculate_dac_output_when_amplitude_is_zero() {
        let mut channel = initialize_pulse_channel();
        enable_pulse_channel(&mut channel);

        let wave_duty = 1;
        let wave_duty_position = 2;
        let current_volume = 5;
        initialize_amplitude_variables(&mut channel, wave_duty, wave_duty_position, current_volume);

        assert_eq!(digital_output(&channel), 0.0);
    }

    #[test]
    fn should_calculate_dac_output_when_amplitude_is_one() {
        let mut channel = initialize_pulse_channel();
        enable_pulse_channel(&mut channel);

        let wave_duty = 1;
        let wave_duty_position = 1;
        let current_volume = 5;
        initialize_amplitude_variables(&mut channel, wave_duty, wave_duty_position, current_volume);

        assert_eq!(digital_output(&channel), 5.0);
    }

    #[test]
    fn should_calculate_dac_output_when_volume_is_at_ten() {
        let mut channel = initialize_pulse_channel();
        enable_pulse_channel(&mut channel);

        let wave_duty = 2;
        let wave_duty_position = 2;
        let current_volume = 10;
        initialize_amplitude_variables(&mut channel, wave_duty, wave_duty_position, current_volume);

        assert_eq!(digital_output(&channel), 10.0);
    }

    #[test]
    fn should_produce_no_audio_output_if_channel_is_disabled() {
        let mut channel = initialize_pulse_channel();

        let wave_duty = 2;
        let wave_duty_position = 2;
        let current_volume = 10;
        initialize_amplitude_variables(&mut channel, wave_duty, wave_duty_position, current_volume);

        assert_eq!(digital_output(&channel), 7.5);
    }
}