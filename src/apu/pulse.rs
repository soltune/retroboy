use crate::apu::envelope;
use crate::apu::envelope::{initialize_envelope, Envelope};
use crate::apu::length;
use crate::apu::length::{initialize_length, Length};
use crate::apu::period;
use crate::apu::period::{initalize_period, Period};
use crate::apu::sweep;
use crate::apu::sweep::{initialize_sweep, Sweep};
use crate::apu::utils::bounded_wrapping_add;
use crate::utils::is_bit_set;

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
const PERIOD_HIGH_LENGTH_ENABLED_INDEX: u8 = 6;

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

pub fn step_length(channel: &mut PulseChannel) {
    if channel.enabled {
        let length_timer_enabled = is_bit_set(channel.period.high, PERIOD_HIGH_LENGTH_ENABLED_INDEX);
        if length_timer_enabled {
            length::step(&mut channel.length);
            if channel.length.timer == 0 {
                disable(channel);
            } 
        }
    }
}

pub fn step_sweep(channel: &mut PulseChannel) {
    if channel.enabled {
        sweep::step(channel);
    }
}

pub fn trigger(channel: &mut PulseChannel, with_sweep: bool) {
    channel.enabled = true;
    envelope::trigger(&mut channel.envelope);
    length::trigger(&mut channel.length, false);
    if with_sweep {
        sweep::trigger(channel);
    }
}

pub fn disable(channel: &mut PulseChannel) {
    channel.dac_enabled = false;
    channel.enabled = false;
}

pub fn should_trigger(channel: &PulseChannel) -> bool {
   channel.dac_enabled && is_bit_set(channel.period.high, PERIOD_HIGH_TRIGGER_INDEX)
}