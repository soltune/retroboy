use crate::apu::period;
use crate::apu::period::{initalize_period, Period};
use crate::apu::length;
use crate::apu::length::{initialize_length, Length};
use crate::apu::utils::bounded_wrapping_add;
use crate::utils::is_bit_set;

#[derive(Debug)]
pub struct WaveChannel {
    pub enabled: bool,
    pub dac_enabled: bool,
    pub length: Length,
    pub volume: u8,
    pub period: Period,
    pub wave_position: u8
}

pub fn initialize_wave_channel() -> WaveChannel {
    WaveChannel {
        enabled: false,
        dac_enabled: false,
        length: initialize_length(),
        volume: 0,
        period: initalize_period(),
        wave_position: 0
    }
}

const MAX_WAVE_SAMPLE_STEPS: u8 = 32;
const PERIOD_HIGH_TRIGGER_INDEX: u8 = 7;

pub fn step(channel: &mut WaveChannel, last_instruction_clock_cycles: u8) {
    if channel.enabled {
        period::step(&mut channel.period, last_instruction_clock_cycles / 2, || {
            channel.wave_position = bounded_wrapping_add(channel.wave_position, MAX_WAVE_SAMPLE_STEPS);
        });
    }
}

pub fn trigger(channel: &mut WaveChannel) {
    channel.enabled = true;
    length::trigger(&mut channel.length, true);
}

pub fn should_trigger(channel: &WaveChannel) -> bool {
   channel.dac_enabled && is_bit_set(channel.period.high, PERIOD_HIGH_TRIGGER_INDEX)
}