use crate::apu::period;
use crate::apu::period::{initalize_period, Period};
use crate::apu::length;
use crate::apu::length::{initialize_length, Length};
use crate::apu::utils::{as_dac_output, bounded_wrapping_add, length_enabled};
use crate::emulator::Emulator;
use crate::utils::is_bit_set;

#[derive(Debug)]
pub struct WaveChannel {
    pub enabled: bool,
    pub dac_enabled: bool,
    pub length: Length,
    pub volume: u8,
    pub period: Period,
    pub wave_position: u8,
    pub wave_pattern_ram: [u8; 0x10],
}

pub fn initialize_wave_channel() -> WaveChannel {
    WaveChannel {
        enabled: false,
        dac_enabled: false,
        length: initialize_length(),
        volume: 0,
        period: initalize_period(),
        wave_position: 1,
        wave_pattern_ram: [0; 0x10],
    }
}

pub fn reset_wave_channel(original_wave_channel: &WaveChannel) -> WaveChannel {
    let mut new_wave_channel = initialize_wave_channel();
    // On reset (when APU is powered down), maintain length timers, as this is expected behavior for DMG
    new_wave_channel.length = length::reset_initial_settings(&original_wave_channel.length);
    
    // APU powering down should also not affect wave RAM.
    new_wave_channel.wave_pattern_ram = original_wave_channel.wave_pattern_ram;
    new_wave_channel.wave_position = original_wave_channel.wave_position;

    new_wave_channel
}

const MAX_WAVE_SAMPLE_STEPS: u8 = 31;
const PERIOD_HIGH_TRIGGER_INDEX: u8 = 7;

pub fn step(channel: &mut WaveChannel, last_instruction_clock_cycles: u8) {
    if channel.enabled {
        period::step(&mut channel.period, last_instruction_clock_cycles / 2, || {
            channel.wave_position = bounded_wrapping_add(channel.wave_position, MAX_WAVE_SAMPLE_STEPS);
        });
    }
}

pub fn should_clock_length_on_enable(channel: &WaveChannel, original_period_high_value: u8) -> bool {
    let new_period_high_value = channel.period.high;
    !length_enabled(original_period_high_value) && length_enabled(new_period_high_value)
}

pub fn should_clock_length_on_trigger(channel: &WaveChannel) -> bool {
    length::at_max_wave_channel_length(&channel.length) && length_enabled(channel.period.high)
}

pub fn step_length(channel: &mut WaveChannel) {
    let length_timer_enabled = length_enabled(channel.period.high);
    if length_timer_enabled {
        length::step(&mut channel.length);
        if channel.length.timer == 0 {
            disable(channel);
        } 
    }
}

pub fn read_from_wave_ram(channel: &WaveChannel, localized_address: u8) -> u8 {
    channel.wave_pattern_ram[localized_address as usize]
}

pub fn write_to_wave_ram(channel: &mut WaveChannel, localized_address: u8, new_value: u8) {
    channel.wave_pattern_ram[localized_address as usize] = new_value;
}

pub fn dac_output(emulator: &Emulator) -> f32 {
    if emulator.apu.channel3.enabled {
        let localized_address = emulator.apu.channel3.wave_position / 2;
        let byte_offset = emulator.apu.channel3.wave_position % 2;
    
        let byte = read_from_wave_ram(&emulator.apu.channel3, localized_address);
        let sample = if byte_offset == 0 { (byte & 0xF0) >> 4 } else { byte & 0xF };
    
        let output_level = (emulator.apu.channel3.volume & 0b01100000) >> 5;
        match output_level {
            0b01 => as_dac_output(sample),
            0b10 => as_dac_output(sample >> 1),
            0b11 => as_dac_output(sample >> 2),
            _ => 0.0
        }
    }
    else {
        0.0
    }
}

pub fn wave_form_just_read(channel: &WaveChannel) -> bool {
    let max_period_divider = period::calculate_period_divider(&channel.period);
    channel.period.divider == max_period_divider
}

fn corrupt_wave_ram_bug(channel: &mut WaveChannel) {
    // DMG has a bug that will corrupt wave RAM if the channel is re-triggered
    // right before it reads from wave RAM.
    let offset = (((channel.wave_position + 1) >> 1) & 0xF) as usize;
    if offset < 4 {
        channel.wave_pattern_ram[0] = channel.wave_pattern_ram[offset];
    }
    else {
        let copy_base_position = offset & !3;
        for copy_offset in 0..=3 {
            let copy_position = copy_base_position + copy_offset;
            channel.wave_pattern_ram[copy_offset] = channel.wave_pattern_ram[copy_position];
        }
    } 
}

pub fn trigger(channel: &mut WaveChannel) {
    if channel.enabled && wave_form_just_read(channel) {
        corrupt_wave_ram_bug(channel);
    }

    channel.wave_position = 1;

    if channel.dac_enabled {
        channel.enabled = true;
    }

    period::trigger(&mut channel.period);
    period::apply_wave_channel_trigger_delay(&mut channel.period);
    length::reload_wave_channel_timer_with_maximum(&mut channel.length);
}

pub fn disable(channel: &mut WaveChannel) {
    channel.enabled = false;
}

pub fn should_trigger(channel: &WaveChannel) -> bool {
   is_bit_set(channel.period.high, PERIOD_HIGH_TRIGGER_INDEX)
}

#[cfg(test)]
mod tests;