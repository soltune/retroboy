use utils::{calculate_left_stereo_sample, calculate_right_stereo_sample};
use crate::apu::envelope::should_disable_dac;
use crate::apu::noise::{initialize_noise_channel, reset_noise_channel, NoiseChannel};
use crate::apu::wave::{initialize_wave_channel, reset_wave_channel, WaveChannel};
use crate::apu::pulse::{initialize_pulse_channel, reset_pulse_channel, PulseChannel};
use crate::apu::utils::{bounded_wrapping_add, as_dac_output};
use crate::emulator::{in_color_bios, is_cgb, Emulator};
use crate::utils::{get_bit, get_t_cycle_increment, is_bit_set};
use bincode::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode)]
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
    pub audio_buffer_clock: u8,
    pub channel_clock: u8,
    pub left_sample_queue: Vec<f32>,
    pub right_sample_queue: Vec<f32>,
    pub summed_channel1_sample: f32,
    pub summed_channel2_sample: f32,
    pub summed_channel3_sample: f32,
    pub summed_channel4_sample: f32,
    pub enqueue_rate: u32
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
        audio_buffer_clock: 0,
        channel_clock: 0,
        left_sample_queue: Vec::new(),
        right_sample_queue: Vec::new(),
        summed_channel1_sample: 0.0,
        summed_channel2_sample: 0.0,
        summed_channel3_sample: 0.0,
        summed_channel4_sample: 0.0,
        enqueue_rate: CPU_RATE / DEFAULT_SAMPLE_RATE
    }
}

pub fn reset_apu(emulator: &mut Emulator) {
    let is_cgb = is_cgb(emulator);
    emulator.apu.enabled = false;
    emulator.apu.sound_panning = 0;
    emulator.apu.master_volume = 0;
    emulator.apu.divider_apu = 0;
    emulator.apu.last_divider_time = 0;
    emulator.apu.channel1 = reset_pulse_channel(&emulator.apu.channel1, is_cgb);
    emulator.apu.channel2 = reset_pulse_channel(&emulator.apu.channel2, is_cgb);
    emulator.apu.channel3 = reset_wave_channel(&emulator.apu.channel3, is_cgb);
    emulator.apu.channel4 = reset_noise_channel(&emulator.apu.channel4, is_cgb);
}

const CH3_DAC_ENABLED_INDEX: u8 = 7;
const APU_ENABLED_INDEX: u8 = 7;
const MAX_DIV_APU_STEPS: u8 = 7;

const CPU_RATE: u32 = 4194304;
const DEFAULT_SAMPLE_RATE: u32 = 44100;
const MAX_AUDIO_BUFFER_SIZE: usize = 512;

const CHANNEL_STEP_RATE: u8 = 4;

fn should_step_div_apu(emulator: &mut Emulator) -> bool {
    let double_speed_mode = emulator.speed_switch.cgb_double_speed;
    let bit_to_check = if double_speed_mode { 5 } else { 4 };
    get_bit(emulator.apu.last_divider_time, bit_to_check) == 1
    && get_bit(emulator.timers.divider, bit_to_check) == 0
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

fn calculate_sample_weight(steps_per_enqueue: u8, steps_since_enqueue: u8) -> f32 {
    let step_index = steps_per_enqueue - steps_since_enqueue;
    ((step_index as f32).ln() + 1.0) / ((steps_per_enqueue as f32).ln() + 1.0)
}

fn generate_dac_output(summed_channel_sample: f32, steps_since_enqueue: u8) -> f32 {
    let avg_channel_sample = summed_channel_sample / steps_since_enqueue as f32;
    as_dac_output(avg_channel_sample)
}

fn track_digital_outputs(emulator: &mut Emulator, weight: f32) {
    let channel1_output = pulse::digital_output(&emulator.apu.channel1);
    let channel2_output = pulse::digital_output(&emulator.apu.channel2);
    let channel3_output = wave::digital_output(&emulator);
    let channel4_output = noise::digital_output(&emulator.apu.channel4);

    emulator.apu.summed_channel1_sample += channel1_output * weight;
    emulator.apu.summed_channel2_sample += channel2_output * weight;
    emulator.apu.summed_channel3_sample += channel3_output * weight;
    emulator.apu.summed_channel4_sample += channel4_output * weight;
}

fn clear_summed_samples(emulator: &mut Emulator) {
    emulator.apu.summed_channel1_sample = 0.0;
    emulator.apu.summed_channel2_sample = 0.0;
    emulator.apu.summed_channel3_sample = 0.0;
    emulator.apu.summed_channel4_sample = 0.0;
}

fn enqueue_left_sample(emulator: &mut Emulator,
    channel1_dac_output: f32,
    channel2_dac_output: f32,
    channel3_dac_output: f32,
    channel4_dac_output: f32) {
    let left_master_volume = (emulator.apu.master_volume & 0b01110000) >> 4;

    let left_sample = calculate_left_stereo_sample(emulator.apu.sound_panning,
        left_master_volume,
        channel1_dac_output,
        channel2_dac_output,
        channel3_dac_output,
        channel4_dac_output);

    emulator.apu.left_sample_queue.push(left_sample);
}

fn enqueue_right_sample(emulator: &mut Emulator,
    channel1_dac_output: f32,
    channel2_dac_output: f32,
    channel3_dac_output: f32,
    channel4_dac_output: f32) {
    let right_master_volume = emulator.apu.master_volume & 0b111;

    let right_sample = calculate_right_stereo_sample(emulator.apu.sound_panning,
        right_master_volume,
        channel1_dac_output,
        channel2_dac_output,
        channel3_dac_output,
        channel4_dac_output);

    emulator.apu.right_sample_queue.push(right_sample);
}

fn enqueue_audio_samples(emulator: &mut Emulator) {
    /*
        This emulator uses audio syncing. It steps the emulator until the audio buffer is full, then 
        briefly pauses while it plays the audio in the buffer. Once the audio plays, the emulator
        resumes.
        
        The purpose of the BIOS check here is that I want my emulator to speed through the
        initial GBC BIOS so it appears as if it's skipping the BIOS altogether (even though it still
        runs it; it's just hidden).
    */
    if !in_color_bios(emulator) {
        let cgb_double_speed = emulator.speed_switch.cgb_double_speed;
        let t_cycle_increment = get_t_cycle_increment(cgb_double_speed);

        emulator.apu.audio_buffer_clock += t_cycle_increment;
        let steps_since_enqueue = emulator.apu.audio_buffer_clock / t_cycle_increment;
        let steps_per_enqueue = emulator.apu.enqueue_rate as u8 + 1 / t_cycle_increment;

        let weight = calculate_sample_weight(steps_per_enqueue, steps_since_enqueue);
        track_digital_outputs(emulator, weight);

        if emulator.apu.audio_buffer_clock as u32 >= emulator.apu.enqueue_rate {
            emulator.apu.audio_buffer_clock = 0;
    
            let channel1_dac_output = generate_dac_output(emulator.apu.summed_channel1_sample, steps_since_enqueue);
            let channel2_dac_output = generate_dac_output(emulator.apu.summed_channel2_sample, steps_since_enqueue);
            let channel3_dac_output = generate_dac_output(emulator.apu.summed_channel3_sample, steps_since_enqueue);
            let channel4_dac_output = generate_dac_output(emulator.apu.summed_channel4_sample, steps_since_enqueue);

            enqueue_left_sample(emulator,
                channel1_dac_output,
                channel2_dac_output,
                channel3_dac_output,
                channel4_dac_output);

            enqueue_right_sample(emulator,
                channel1_dac_output,
                channel2_dac_output,
                channel3_dac_output,
                channel4_dac_output);

            clear_summed_samples(emulator);
        }
    }
}

pub fn step(emulator: &mut Emulator) {
    let double_speed_mode = emulator.speed_switch.cgb_double_speed;
    let t_cycle_increment = get_t_cycle_increment(double_speed_mode);
    emulator.apu.channel_clock += t_cycle_increment;
    
    if emulator.apu.enabled {
        if emulator.apu.channel_clock >= CHANNEL_STEP_RATE {
            let clock_cycles = emulator.apu.channel_clock;
            emulator.apu.channel_clock = 0;
    
            pulse::step(&mut emulator.apu.channel1, clock_cycles);
            pulse::step(&mut emulator.apu.channel2, clock_cycles);
            wave::step(&mut emulator.apu.channel3, clock_cycles);
            noise::step(&mut emulator.apu.channel4, clock_cycles);
        }

        step_div_apu(emulator);
    }

    enqueue_audio_samples(emulator);
    emulator.apu.last_divider_time = emulator.timers.divider;
}

pub fn set_sample_rate(emulator: &mut Emulator, sample_rate: u32) {
    emulator.apu.enqueue_rate = CPU_RATE / sample_rate;
}

fn in_length_period_first_half(current_divider_apu: u8) -> bool {
    let length_period_first_half_steps = vec![1,3,5,7];
    length_period_first_half_steps.contains(&current_divider_apu)
}

pub fn set_ch1_period_high(emulator: &mut Emulator, new_period_high_value: u8) {
    if emulator.apu.enabled {
        let original_period_high_value = emulator.apu.channel1.period.high;
        emulator.apu.channel1.period.high = new_period_high_value;

        let length_period_first_half = in_length_period_first_half(emulator.apu.divider_apu);

        let clock_length_on_enable = pulse::should_clock_length_on_enable(&emulator.apu.channel1, original_period_high_value)
            && length_period_first_half;

        if clock_length_on_enable {
            pulse::step_length(&mut emulator.apu.channel1);
        }

        if pulse::should_trigger(&emulator.apu.channel1) { 
            pulse::trigger(&mut emulator.apu.channel1, true);

            if pulse::should_clock_length_on_trigger(&emulator.apu.channel1) && length_period_first_half {
               pulse::step_length(&mut emulator.apu.channel1);
            }
        }
    }
}

pub fn set_ch2_period_high(emulator: &mut Emulator, new_period_high_value: u8) {
    if emulator.apu.enabled {
        let original_period_high_value = emulator.apu.channel2.period.high;
        emulator.apu.channel2.period.high = new_period_high_value;

        let length_period_first_half = in_length_period_first_half(emulator.apu.divider_apu);
    
        let clock_length_on_enable = pulse::should_clock_length_on_enable(&emulator.apu.channel2, original_period_high_value)
            && length_period_first_half;

        if clock_length_on_enable {
            pulse::step_length(&mut emulator.apu.channel2);
        }

        if pulse::should_trigger(&emulator.apu.channel2) { 
            pulse::trigger(&mut emulator.apu.channel2, false);

            if pulse::should_clock_length_on_trigger(&emulator.apu.channel2) && length_period_first_half {
               pulse::step_length(&mut emulator.apu.channel2);
            }
        }
    }
}

pub fn set_ch3_period_high(emulator: &mut Emulator, new_period_high_value: u8) {
    if emulator.apu.enabled {
        let original_period_high_value = emulator.apu.channel3.period.high;
        emulator.apu.channel3.period.high = new_period_high_value;

        let length_period_first_half = in_length_period_first_half(emulator.apu.divider_apu);

        let clock_length_on_enable = wave::should_clock_length_on_enable(&emulator.apu.channel3, original_period_high_value)
            && length_period_first_half;

        if clock_length_on_enable {
            wave::step_length(&mut emulator.apu.channel3);
        }

        if wave::should_trigger(&emulator.apu.channel3) {
            wave::trigger(emulator);

            if wave::should_clock_length_on_trigger(&emulator.apu.channel3) && length_period_first_half {
               wave::step_length(&mut emulator.apu.channel3);
            }
        }
   }
}

pub fn set_ch4_control(emulator: &mut Emulator, new_control_value: u8) {
    if emulator.apu.enabled {
        let original_control_value = emulator.apu.channel4.control;
        emulator.apu.channel4.control = new_control_value;

        let length_period_first_half = in_length_period_first_half(emulator.apu.divider_apu);

        let clock_length_on_enable = noise::should_clock_length_on_enable(&emulator.apu.channel4, original_control_value)
            && length_period_first_half;

        if clock_length_on_enable {
            noise::step_length(&mut emulator.apu.channel4);
        }

        if noise::should_trigger(&emulator.apu.channel4) {
            noise::trigger(&mut emulator.apu.channel4);

            if noise::should_clock_length_on_trigger(&emulator.apu.channel4) && length_period_first_half {
               noise::step_length(&mut emulator.apu.channel4);
            }
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

pub fn get_wave_ram_byte(emulator: &Emulator, localized_address: u8) -> u8 {
    let mut address = localized_address;

    if emulator.apu.channel3.enabled {
        address = emulator.apu.channel3.wave_position / 2;
        if emulator.apu.channel3.period.reloaded || is_cgb(emulator) {
            wave::read_from_wave_ram(&emulator.apu.channel3, address)
        }
        else {
            0xFF
        }
    }
    else {
        wave::read_from_wave_ram(&emulator.apu.channel3, address)
    }
}

pub fn set_wave_ram_byte(emulator: &mut Emulator, localized_address: u8, new_value: u8) {
    let mut address = localized_address;

    if emulator.apu.channel3.enabled {
        address = emulator.apu.channel3.wave_position / 2;
        if emulator.apu.channel3.period.reloaded || is_cgb(emulator) {
            wave::write_to_wave_ram(&mut emulator.apu.channel3, address, new_value);
        }
    }
    else {
        wave::write_to_wave_ram(&mut emulator.apu.channel3, address, new_value);
    }
}

pub fn set_audio_master_control(emulator: &mut Emulator, new_audio_master_control: u8) {
    emulator.apu.enabled = is_bit_set(new_audio_master_control, APU_ENABLED_INDEX);

    if !emulator.apu.enabled {
        reset_apu(emulator);
    }
}

pub fn set_ch1_sweep_settings(emulator: &mut Emulator, new_sweep_settings: u8) {
    if emulator.apu.enabled {
        sweep::update_initial_settings(&mut emulator.apu.channel1, new_sweep_settings);
    }
}

pub fn set_ch1_length_settings(emulator: &mut Emulator, new_length_settings: u8) {
    if emulator.apu.enabled || !is_cgb(emulator) {
        emulator.apu.channel1.length.initial_settings = if emulator.apu.enabled {
            new_length_settings
        }
        else {
            new_length_settings & 0x3F
        };
    
        length::initialize_timer(&mut emulator.apu.channel1.length); 
    }
}

pub fn set_ch1_period_low(emulator: &mut Emulator, new_period_low: u8) {
    if emulator.apu.enabled {
        emulator.apu.channel1.period.low = new_period_low;
    }
}

pub fn set_ch2_length_settings(emulator: &mut Emulator, new_length_settings: u8) {
    if emulator.apu.enabled || !is_cgb(emulator) {
        emulator.apu.channel2.length.initial_settings = if emulator.apu.enabled {
            new_length_settings
        }
        else {
            new_length_settings & 0x3F
        };

        length::initialize_timer(&mut emulator.apu.channel2.length); 
    }
}

pub fn set_ch2_period_low(emulator: &mut Emulator, new_period_low: u8) {
    if emulator.apu.enabled {
        emulator.apu.channel2.period.low = new_period_low;
    }
}

pub fn set_ch3_length_settings(emulator: &mut Emulator, new_length_settings: u8) {
    if emulator.apu.enabled || !is_cgb(emulator) {
        emulator.apu.channel3.length.initial_settings = new_length_settings;
        length::initialize_wave_channel_timer(&mut emulator.apu.channel3.length); 
    }
}

pub fn set_ch3_period_low(emulator: &mut Emulator, new_period_low: u8) {
    if emulator.apu.enabled {
        emulator.apu.channel3.period.low = new_period_low;
    }
}

pub fn set_ch3_volume(emulator: &mut Emulator, new_volume: u8) {
    if emulator.apu.enabled {
        emulator.apu.channel3.volume = new_volume;
    }
}

pub fn set_ch4_length_settings(emulator: &mut Emulator, new_length_settings: u8) {
    if emulator.apu.enabled || !is_cgb(emulator) {
        emulator.apu.channel4.length.initial_settings = new_length_settings;
        length::initialize_timer(&mut emulator.apu.channel4.length); 
    }
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
