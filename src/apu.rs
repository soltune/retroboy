use crate::apu::envelope::should_disable_dac;
use crate::apu::noise::{initialize_noise_channel, NoiseChannel};
use crate::apu::wave::{initialize_wave_channel, WaveChannel};
use crate::apu::pulse::{initialize_pulse_channel, PulseChannel};
use crate::apu::utils::bounded_wrapping_add;
use crate::emulator::Emulator;
use crate::utils::{get_bit, is_bit_set, reset_bit, set_bit};

#[derive(Debug)]
pub struct ApuState {
    pub audio_master_control: u8,
    pub sound_panning: u8,
    pub master_volume: u8,
    pub channel1: PulseChannel,
    pub channel2: PulseChannel,
    pub channel3: WaveChannel,
    pub channel4: NoiseChannel,
    pub divider_apu: u8,
    pub last_divider_time: u8
}

pub fn initialize_apu() -> ApuState {
    ApuState {
        audio_master_control: 0,
        sound_panning: 0,
        master_volume: 0,
        channel1: initialize_pulse_channel(),
        channel2: initialize_pulse_channel(),
        channel3: initialize_wave_channel(),
        channel4: initialize_noise_channel(),
        divider_apu: 0,
        last_divider_time: 0
    }
}

// Work In Progress
const CH1_ENABLED_INDEX: u8 = 0;
const CH2_ENABLED_INDEX: u8 = 1;
const CH3_ENABLED_INDEX: u8 = 2;
const CH3_DAC_ENABLED_INDEX: u8 = 7;
const APU_ENABLED_INDEX: u8 = 7;
const MAX_DIV_APU_STEPS: u8 = 7;

fn should_step_div_apu(emulator: &mut Emulator) -> bool {
    emulator.apu.last_divider_time > 0
        && emulator.timers.divider > 0
        && get_bit(emulator.apu.last_divider_time, 4) == 1
        && get_bit(emulator.timers.divider, 4) == 0
}

fn step_div_apu(emulator: &mut Emulator) {
    if should_step_div_apu(emulator) {
        let current_divider_apu = emulator.apu.divider_apu;

        let envelop_step = 7;
        let length_steps = vec![0, 2, 4, 6];
        let sweep_steps = vec![2, 6];

        if current_divider_apu == envelop_step {
            pulse::step_envelope(&mut emulator.apu.channel1);
            pulse::step_envelope(&mut emulator.apu.channel2); 
        }

        if length_steps.contains(&current_divider_apu) {
            pulse::step_length(&mut emulator.apu.channel1);
            pulse::step_length(&mut emulator.apu.channel2);
            wave::step_length(&mut emulator.apu.channel3); 
        }
        
        if sweep_steps.contains(&current_divider_apu) {
            pulse::step_sweep(&mut emulator.apu.channel1);
        }

        emulator.apu.last_divider_time = emulator.timers.divider;
        emulator.apu.divider_apu = bounded_wrapping_add(emulator.apu.divider_apu, MAX_DIV_APU_STEPS)
    }
}

fn apu_enabled(audio_master_control: u8) -> bool {
    is_bit_set(audio_master_control, APU_ENABLED_INDEX)
}

pub fn step(emulator: &mut Emulator) {    
    if apu_enabled(emulator.apu.audio_master_control) {
        let instruction_clock_cycles = emulator.cpu.clock.instruction_clock_cycles;
        pulse::step(&mut emulator.apu.channel1, instruction_clock_cycles);
        pulse::step(&mut emulator.apu.channel2, instruction_clock_cycles);
        wave::step(&mut emulator.apu.channel3, instruction_clock_cycles);
        step_div_apu(emulator);
    }    
}

pub fn set_ch1_period_high(emulator: &mut Emulator, new_period_high_value: u8) {
    emulator.apu.channel1.period.high = new_period_high_value;
    
    if pulse::should_trigger(&emulator.apu.channel1) { 
        pulse::trigger(&mut emulator.apu.channel1, true); 
        emulator.apu.audio_master_control = set_bit(emulator.apu.audio_master_control, CH1_ENABLED_INDEX);
    }
}

pub fn set_ch2_period_high(emulator: &mut Emulator, new_period_high_value: u8) {
    emulator.apu.channel2.period.high = new_period_high_value;
    
    if pulse::should_trigger(&emulator.apu.channel2) { 
        pulse::trigger(&mut emulator.apu.channel2, false);
        emulator.apu.audio_master_control = set_bit(emulator.apu.audio_master_control, CH2_ENABLED_INDEX);
    }
}

pub fn set_ch3_period_high(emulator: &mut Emulator, new_period_high_value: u8) {
    emulator.apu.channel3.period.high = new_period_high_value;

    if wave::should_trigger(&emulator.apu.channel3) {
        wave::trigger(&mut emulator.apu.channel3);
        emulator.apu.audio_master_control = set_bit(emulator.apu.audio_master_control, CH3_ENABLED_INDEX);
    }
}

pub fn set_ch1_envelope_settings(emulator: &mut Emulator, new_envelope_settings: u8) {
    emulator.apu.channel1.envelope.initial_settings = new_envelope_settings;

    if should_disable_dac(&emulator.apu.channel1.envelope) {
        pulse::disable(&mut emulator.apu.channel1); 
        emulator.apu.audio_master_control = reset_bit(emulator.apu.audio_master_control, CH1_ENABLED_INDEX);
    }
}

pub fn set_ch2_envelope_settings(emulator: &mut Emulator, new_envelope_settings: u8) {
    emulator.apu.channel2.envelope.initial_settings = new_envelope_settings;

    if should_disable_dac(&emulator.apu.channel2.envelope) {
        pulse::disable(&mut emulator.apu.channel2); 
        emulator.apu.audio_master_control = reset_bit(emulator.apu.audio_master_control, CH2_ENABLED_INDEX);
    }
}

pub fn set_ch3_dac_enabled(emulator: &mut Emulator, new_dac_enabled_register_value: u8) {
    let enabled = is_bit_set(new_dac_enabled_register_value, CH3_DAC_ENABLED_INDEX);

    emulator.apu.channel3.dac_enabled = enabled;
    
    if !enabled {
        wave::disable(&mut emulator.apu.channel3);
        emulator.apu.audio_master_control = reset_bit(emulator.apu.audio_master_control, CH3_ENABLED_INDEX);
    }
}

#[cfg(test)]
mod tests;

pub mod pulse;
pub mod wave;
pub mod noise;
pub mod length;
pub mod sweep;
mod envelope;
mod period;
mod utils;
