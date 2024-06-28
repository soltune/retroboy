use utils::{calculate_left_stereo_sample, calculate_right_stereo_sample};
use crate::apu::envelope::should_disable_dac;
use crate::apu::noise::{initialize_noise_channel, NoiseChannel};
use crate::apu::wave::{initialize_wave_channel, WaveChannel};
use crate::apu::pulse::{initialize_pulse_channel, PulseChannel};
use crate::apu::utils::bounded_wrapping_add;
use crate::emulator::Emulator;
use crate::utils::{get_bit, is_bit_set, T_CYCLE_INCREMENT};

#[derive(Debug)]
pub struct ApuState {
    pub enabled: bool,
    pub sound_panning: u8,
    pub master_volume: u8,
    pub channel1: PulseChannel,
    pub channel2: PulseChannel,
    pub channel3: WaveChannel,
    pub channel4: NoiseChannel,
    pub divider_apu: u8,
    pub last_divider_time: u8,
    pub instruction_cycles: u8,
    pub left_sample_queue: Vec<f32>,
    pub right_sample_queue: Vec<f32>
}

pub fn initialize_apu() -> ApuState {
    ApuState {
        enabled: false,
        sound_panning: 0,
        master_volume: 0,
        channel1: initialize_pulse_channel(),
        channel2: initialize_pulse_channel(),
        channel3: initialize_wave_channel(),
        channel4: initialize_noise_channel(),
        divider_apu: 0,
        last_divider_time: 0,
        instruction_cycles: 0,
        left_sample_queue: Vec::new(),
        right_sample_queue: Vec::new()
    }
}

const CH3_DAC_ENABLED_INDEX: u8 = 7;
const APU_ENABLED_INDEX: u8 = 7;
const MAX_DIV_APU_STEPS: u8 = 7;

const CPU_RATE: u32 = 4194304;
const SAMPLE_RATE: u32 = 48000;
const ENQUEUE_RATE: u32 = CPU_RATE / SAMPLE_RATE;
const MAX_AUDIO_BUFFER_SIZE: usize = 512;

fn should_step_div_apu(emulator: &mut Emulator) -> bool {
    get_bit(emulator.apu.last_divider_time, 4) == 1
    && get_bit(emulator.timers.divider, 4) == 0
}

fn step_div_apu(emulator: &mut Emulator) {
    if should_step_div_apu(emulator) {
        let current_divider_apu = emulator.apu.divider_apu;

        let envelope_step = 7;
        let length_steps = vec![0, 2, 4, 6];
        let sweep_steps = vec![2, 6];

        if current_divider_apu == envelope_step {
            pulse::step_envelope(&mut emulator.apu.channel1);
            pulse::step_envelope(&mut emulator.apu.channel2);
            noise::step_envelope(&mut emulator.apu.channel4); 
        }

        if length_steps.contains(&current_divider_apu) {
            pulse::step_length(&mut emulator.apu.channel1);
            pulse::step_length(&mut emulator.apu.channel2);
            wave::step_length(&mut emulator.apu.channel3);
            noise::step_length(&mut emulator.apu.channel4);
        }
        
        if sweep_steps.contains(&current_divider_apu) {
            pulse::step_sweep(&mut emulator.apu.channel1);
        }

        emulator.apu.divider_apu = bounded_wrapping_add(emulator.apu.divider_apu, MAX_DIV_APU_STEPS)
    }
}

pub fn audio_buffers_full(emulator: &mut Emulator) -> bool {
    emulator.apu.left_sample_queue.len() >= MAX_AUDIO_BUFFER_SIZE
    && emulator.apu.right_sample_queue.len() >= MAX_AUDIO_BUFFER_SIZE
}

pub fn clear_audio_buffers(emulator: &mut Emulator) {
    emulator.apu.left_sample_queue.clear();
    emulator.apu.right_sample_queue.clear();
}

pub fn get_left_sample_queue(emulator: &Emulator) -> &[f32] {
    &emulator.apu.left_sample_queue.as_slice()
}

pub fn get_right_sample_queue(emulator: &Emulator) -> &[f32] {
    &emulator.apu.right_sample_queue.as_slice()
}

fn enqueue_audio_samples(emulator: &mut Emulator) {
    if emulator.apu.instruction_cycles as u32 >= ENQUEUE_RATE {
        emulator.apu.instruction_cycles = 0;

        let sound_panning = emulator.apu.sound_panning;

        let channel1_output = pulse::dac_output(&emulator.apu.channel1);
        let channel2_output = pulse::dac_output(&emulator.apu.channel2);
        let channel3_output = wave::dac_output(&emulator);
        let channel4_output = noise::dac_output(&emulator.apu.channel4);

        let left_master_volume = (emulator.apu.master_volume & 0b01110000) >> 4;

        let left_sample = calculate_left_stereo_sample(sound_panning,
            left_master_volume,
            channel1_output,
            channel2_output,
            channel3_output,
            channel4_output);

        emulator.apu.left_sample_queue.push(left_sample);

        let right_master_volume = emulator.apu.master_volume & 0b111;

        let right_sample = calculate_right_stereo_sample(sound_panning,
            right_master_volume,
            channel1_output,
            channel2_output,
            channel3_output,
            channel4_output);

        emulator.apu.right_sample_queue.push(right_sample);
    }
}

pub fn step(emulator: &mut Emulator) {
    let instruction_clock_cycles = T_CYCLE_INCREMENT;
    emulator.apu.instruction_cycles += instruction_clock_cycles;
    
    if emulator.apu.enabled {
        pulse::step(&mut emulator.apu.channel1, instruction_clock_cycles);
        pulse::step(&mut emulator.apu.channel2, instruction_clock_cycles);
        wave::step(&mut emulator.apu.channel3, instruction_clock_cycles);
        noise::step(&mut emulator.apu.channel4, instruction_clock_cycles);
        step_div_apu(emulator);
    }

    enqueue_audio_samples(emulator);
    emulator.apu.last_divider_time = emulator.timers.divider;
}

fn in_length_period_first_half(current_divider_apu: u8) -> bool {
    let length_period_first_half_steps = vec![1,3,5,7];
    length_period_first_half_steps.contains(&current_divider_apu)
}

pub fn set_ch1_period_high(emulator: &mut Emulator, new_period_high_value: u8) {
    if emulator.apu.enabled {
        let original_period_high_value = emulator.apu.channel1.period.high;

        emulator.apu.channel1.period.high = new_period_high_value;
    
        if pulse::should_trigger(&emulator.apu.channel1) { 
            pulse::trigger(&mut emulator.apu.channel1, true);
        }

        let clock_length_on_enable = pulse::should_clock_length_on_enable(&emulator.apu.channel1, original_period_high_value)
            && in_length_period_first_half(emulator.apu.divider_apu);

        if clock_length_on_enable {
            pulse::step_length(&mut emulator.apu.channel1);
        }
    }
}

pub fn set_ch2_period_high(emulator: &mut Emulator, new_period_high_value: u8) {
    if emulator.apu.enabled {
        let original_period_high_value = emulator.apu.channel2.period.high;

        emulator.apu.channel2.period.high = new_period_high_value;
    
        if pulse::should_trigger(&emulator.apu.channel2) { 
            pulse::trigger(&mut emulator.apu.channel2, false);
        }

        let clock_length_on_enable = pulse::should_clock_length_on_enable(&emulator.apu.channel2, original_period_high_value)
            && in_length_period_first_half(emulator.apu.divider_apu);

        if clock_length_on_enable {
            pulse::step_length(&mut emulator.apu.channel2);
        }
    }
}

pub fn set_ch3_period_high(emulator: &mut Emulator, new_period_high_value: u8) {
    if emulator.apu.enabled {
        let original_period_high_value = emulator.apu.channel3.period.high;

        emulator.apu.channel3.period.high = new_period_high_value;

        if wave::should_trigger(&emulator.apu.channel3) {
            wave::trigger(&mut emulator.apu.channel3);
        }

        let clock_length_on_enable = wave::should_clock_length_on_enable(&emulator.apu.channel3, original_period_high_value)
            && in_length_period_first_half(emulator.apu.divider_apu);

        if clock_length_on_enable {
            wave::step_length(&mut emulator.apu.channel3);
        }
   }
}

pub fn set_ch4_control(emulator: &mut Emulator, new_control_value: u8) {
    if emulator.apu.enabled {
        let original_control_value = emulator.apu.channel4.control;

        emulator.apu.channel4.control = new_control_value;

        if noise::should_trigger(&emulator.apu.channel4) {
            noise::trigger(&mut emulator.apu.channel4);
        }

        let clock_length_on_enable = noise::should_clock_length_on_enable(&emulator.apu.channel4, original_control_value)
            && in_length_period_first_half(emulator.apu.divider_apu);

        if clock_length_on_enable {
            noise::step_length(&mut emulator.apu.channel4);
        }
    }
}

pub fn set_ch1_envelope_settings(emulator: &mut Emulator, new_envelope_settings: u8) {
    if emulator.apu.enabled{
        emulator.apu.channel1.envelope.initial_settings = new_envelope_settings;

        let should_disable = should_disable_dac(&emulator.apu.channel1.envelope);
    
        emulator.apu.channel1.dac_enabled = !should_disable;
    
        if should_disable {
            pulse::disable(&mut emulator.apu.channel1); 
        }
    }
}

pub fn set_ch2_envelope_settings(emulator: &mut Emulator, new_envelope_settings: u8) {
    if emulator.apu.enabled {
        emulator.apu.channel2.envelope.initial_settings = new_envelope_settings;

        let should_disable = should_disable_dac(&emulator.apu.channel2.envelope);
    
        emulator.apu.channel2.dac_enabled = !should_disable;
    
        if should_disable {
            pulse::disable(&mut emulator.apu.channel2); 
        }
    }
}

pub fn set_ch3_dac_enabled(emulator: &mut Emulator, new_dac_enabled_register_value: u8) {
    if emulator.apu.enabled {
        let should_disable = !is_bit_set(new_dac_enabled_register_value, CH3_DAC_ENABLED_INDEX);

        emulator.apu.channel3.dac_enabled = !should_disable;
        
        if should_disable {
            wave::disable(&mut emulator.apu.channel3);
        }
    }
}

pub fn set_ch4_envelope_settings(emulator: &mut Emulator, new_envelope_settings: u8) {
    if emulator.apu.enabled {
        emulator.apu.channel4.envelope.initial_settings = new_envelope_settings;

        let should_disable = should_disable_dac(&emulator.apu.channel4.envelope);
    
        emulator.apu.channel4.dac_enabled = !should_disable;
    
        if should_disable {
            noise::disable(&mut emulator.apu.channel4);
        }
    }
}

pub fn get_audio_master_control(emulator: &Emulator) -> u8 {
    let apu_enabled = if emulator.apu.enabled { 1 } else { 0 };
    let mask = 0b01110000;
    let ch4_enabled = if emulator.apu.channel4.enabled { 1 } else { 0 };
    let ch3_enabled = if emulator.apu.channel3.enabled { 1 } else { 0 };
    let ch2_enabled = if emulator.apu.channel2.enabled { 1 } else { 0 };
    let ch1_enabled = if emulator.apu.channel1.enabled { 1 } else { 0 };
    (apu_enabled << 7)
        | mask
        | (ch4_enabled << 3)
        | (ch3_enabled << 2)
        | (ch2_enabled << 1)
        | ch1_enabled
}

pub fn set_audio_master_control(emulator: &mut Emulator, new_audio_master_control: u8) {
    emulator.apu.enabled = is_bit_set(new_audio_master_control, APU_ENABLED_INDEX);

    if !emulator.apu.enabled {
        emulator.apu = initialize_apu();
    }
}

pub fn set_ch1_sweep_settings(emulator: &mut Emulator, new_sweep_settings: u8) {
    if emulator.apu.enabled {
        emulator.apu.channel1.sweep.initial_settings = new_sweep_settings;
    }
}

pub fn set_ch1_length_settings(emulator: &mut Emulator, new_length_settings: u8) {
    if emulator.apu.enabled {
        emulator.apu.channel1.length.initial_settings = new_length_settings;
        length::initialize_timer(&mut emulator.apu.channel1.length, false);
    }
}

pub fn set_ch1_period_low(emulator: &mut Emulator, new_period_low: u8) {
    if emulator.apu.enabled {
        emulator.apu.channel1.period.low = new_period_low;
    }
}

pub fn set_ch2_length_settings(emulator: &mut Emulator, new_length_settings: u8) {
    if emulator.apu.enabled{
        emulator.apu.channel2.length.initial_settings = new_length_settings;
        length::initialize_timer(&mut emulator.apu.channel2.length, false);
    }
}

pub fn set_ch2_period_low(emulator: &mut Emulator, new_period_low: u8) {
    if emulator.apu.enabled {
        emulator.apu.channel2.period.low = new_period_low;
    }
}

pub fn set_ch3_length_settings(emulator: &mut Emulator, new_length_settings: u8) {
    if emulator.apu.enabled{
        emulator.apu.channel3.length.initial_settings = new_length_settings;
        length::initialize_timer(&mut emulator.apu.channel3.length, true);
    }
}

pub fn set_ch3_period_low(emulator: &mut Emulator, new_period_low: u8) {
    if emulator.apu.enabled {
        emulator.apu.channel3.period.low = new_period_low;
    }
}

pub fn set_ch3_volume(emulator: &mut Emulator, new_volume: u8) {
    if emulator.apu.enabled{
        emulator.apu.channel3.volume = new_volume;
    }
}

pub fn set_ch4_length_settings(emulator: &mut Emulator, new_length_settings: u8) {
    emulator.apu.channel4.length.initial_settings = new_length_settings;
    length::initialize_timer(&mut emulator.apu.channel4.length, false);
}

pub fn set_ch4_polynomial(emulator: &mut Emulator, new_polynomial: u8) {
    if emulator.apu.enabled {
        emulator.apu.channel4.polynomial = new_polynomial;
    }
}

pub fn set_master_volume(emulator: &mut Emulator, new_master_volume: u8) {
    if emulator.apu.enabled {
        emulator.apu.master_volume = new_master_volume;
    }
}

pub fn set_sound_panning(emulator: &mut Emulator, new_sound_panning: u8) {
    if emulator.apu.enabled {
        emulator.apu.sound_panning = new_sound_panning;
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
