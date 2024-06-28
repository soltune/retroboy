use crate::apu::envelope;
use crate::apu::envelope::{initialize_envelope, Envelope};
use crate::apu::length;
use crate::apu::length::{initialize_length, Length};
use crate::apu::period;
use crate::apu::period::{initalize_period, Period};
use crate::apu::sweep;
use crate::apu::sweep::{initialize_sweep, Sweep};
use crate::apu::utils::{as_dac_output, bounded_wrapping_add, length_enabled};
use crate::utils::{get_bit, is_bit_set};
use std::collections::HashMap;

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

const MAX_WAVEFORM_STEPS: u8 = 7;
const PERIOD_HIGH_TRIGGER_INDEX: u8 = 7;

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

pub fn step_length(channel: &mut PulseChannel) {
    let length_timer_enabled = length_enabled(channel.period.high);
    if length_timer_enabled {
        length::step(&mut channel.length);
        if channel.length.timer == 0 {
            disable(channel);
        } 
    }
}

pub fn dac_output(channel: &PulseChannel) -> f32 {
    if channel.enabled {
        let waveforms: HashMap<u8, u8> = HashMap::from([
            (0b00, 0b00000001),
            (0b01, 0b00000011),
            (0b10, 0b00001111),
            (0b11, 0b11111100)
        ]);
    
        let wave_duty = (channel.length.initial_settings & 0b11000000) >> 6;
        let waveform = waveforms[&wave_duty];
        let amplitude = get_bit(waveform, channel.wave_duty_position);
        let current_volume = channel.envelope.current_volume;
        let dac_input = amplitude * current_volume;

        if current_volume > 0 {
            as_dac_output(dac_input)
        }
        else {
            0.0
        }
    }
    else {
        0.0
    }
}

pub fn step_sweep(channel: &mut PulseChannel) {
    if channel.enabled {
        sweep::step(channel);
    }
}

pub fn trigger(channel: &mut PulseChannel, with_sweep: bool) {
    channel.enabled = true;
    length::reload_timer_with_maximum(&mut channel.length, false);
    envelope::trigger(&mut channel.envelope);
    if with_sweep {
        sweep::trigger(channel);
    }
}

pub fn disable(channel: &mut PulseChannel) {
    channel.enabled = false;
}

pub fn should_trigger(channel: &PulseChannel) -> bool {
   channel.dac_enabled && is_bit_set(channel.period.high, PERIOD_HIGH_TRIGGER_INDEX)
}

#[cfg(test)]
mod tests;