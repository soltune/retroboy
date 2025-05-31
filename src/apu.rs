use crate::apu::noise::NoiseChannel;
use crate::apu::wave::WaveChannel;
use crate::apu::pulse::PulseChannel;
use crate::apu::utils::{bounded_wrapping_add, as_dac_output};
use crate::utils::{get_bit, get_t_cycle_increment, is_bit_set};
use utils::{calculate_left_stereo_sample, calculate_right_stereo_sample};
use bincode::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode)]
pub struct Apu {
    enabled: bool,
    sound_panning: u8,
    master_volume: u8,
    channel1: PulseChannel,
    channel2: PulseChannel,
    channel3: WaveChannel,
    channel4: NoiseChannel,
    divider_apu: u8,
    last_divider_time: u8,
    audio_buffer_clock: u8,
    channel_clock: u8,
    left_sample_queue: Vec<f32>,
    right_sample_queue: Vec<f32>,
    summed_channel1_sample: f32,
    summed_channel2_sample: f32,
    summed_channel3_sample: f32,
    summed_channel4_sample: f32,
    enqueue_rate: u32,
    cgb_mode: bool,
    cgb_double_speed: bool
}

const CH3_DAC_ENABLED_INDEX: u8 = 7;
const APU_ENABLED_INDEX: u8 = 7;
const MAX_DIV_APU_STEPS: u8 = 7;

const CPU_RATE: u32 = 4194304;
const DEFAULT_SAMPLE_RATE: u32 = 44100;
const MAX_AUDIO_BUFFER_SIZE: usize = 512;

const CHANNEL_STEP_RATE: u8 = 4;

fn calculate_sample_weight(steps_per_enqueue: u8, steps_since_enqueue: u8) -> f32 {
    let step_index = steps_per_enqueue - steps_since_enqueue;
    ((step_index as f32).ln() + 1.0) / ((steps_per_enqueue as f32).ln() + 1.0)
}

fn generate_dac_output(summed_channel_sample: f32, steps_since_enqueue: u8) -> f32 {
    let avg_channel_sample = summed_channel_sample / steps_since_enqueue as f32;
    as_dac_output(avg_channel_sample)
}

fn in_length_period_first_half(current_divider_apu: u8) -> bool {
    let length_period_first_half_steps = vec![1,3,5,7];
    length_period_first_half_steps.contains(&current_divider_apu)
}

impl Apu {
    pub fn new() -> Self {
        Apu {
            enabled: false,
            sound_panning: 0,
            master_volume: 0,
            channel1: PulseChannel::new(),
            channel2: PulseChannel::new(),
            channel3: WaveChannel::new(),
            channel4: NoiseChannel::new(),
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
            enqueue_rate: CPU_RATE / DEFAULT_SAMPLE_RATE,
            cgb_mode: false,
            cgb_double_speed: false
        }
    }

    pub fn reset(&mut self) {
        self.enabled = false;
        self.sound_panning = 0;
        self.master_volume = 0;
        self.divider_apu = 0;
        self.last_divider_time = 0;
        self.channel1 = PulseChannel::reset(&self.channel1, self.cgb_mode);
        self.channel2 = PulseChannel::reset(&self.channel2, self.cgb_mode);
        self.channel3 = WaveChannel::reset(&self.channel3, self.cgb_mode);
        self.channel4 = NoiseChannel::reset(&self.channel4, self.cgb_mode);
    }

    fn should_step_div_apu(&self, divider: u8) -> bool {
        let bit_to_check = if self.cgb_double_speed { 5 } else { 4 };
        get_bit(self.last_divider_time, bit_to_check) == 1 &&
            get_bit(divider, bit_to_check) == 0
    }

    fn step_div_apu(&mut self, divider: u8) {
        if self.should_step_div_apu(divider) {
            let current_divider_apu = self.divider_apu;

            let envelope_step = 7;
            let length_steps = vec![0, 2, 4, 6];
            let sweep_steps = vec![2, 6];

            if current_divider_apu == envelope_step {
                self.channel1.step_envelope();
                self.channel2.step_envelope();
                self.channel4.step_envelope(); 
            }

            if length_steps.contains(&current_divider_apu) {
                self.channel1.step_length();
                self.channel2.step_length();
                self.channel3.step_length();
                self.channel4.step_length();
            }
            
            if sweep_steps.contains(&current_divider_apu) {
                self.channel1.step_sweep();
            }

            self.divider_apu = bounded_wrapping_add(self.divider_apu, MAX_DIV_APU_STEPS);
        }
    }

    pub fn audio_buffers_full(&self) -> bool {
        self.left_sample_queue.len() >= MAX_AUDIO_BUFFER_SIZE &&
            self.right_sample_queue.len() >= MAX_AUDIO_BUFFER_SIZE
    }

    pub fn clear_audio_buffers(&mut self) {
        self.left_sample_queue.clear();
        self.right_sample_queue.clear();
    }

    pub fn get_left_sample_queue(&self) -> &[f32] {
        &self.left_sample_queue.as_slice()
    }

    pub fn get_right_sample_queue(&self) -> &[f32] {
        &self.right_sample_queue.as_slice()
    }

    fn track_digital_outputs(&mut self, weight: f32) {
        let channel1_output = self.channel1.digital_output();
        let channel2_output = self.channel2.digital_output();
        let channel3_output = self.channel3.digital_output();
        let channel4_output = self.channel4.digital_output();

        self.summed_channel1_sample += channel1_output * weight;
        self.summed_channel2_sample += channel2_output * weight;
        self.summed_channel3_sample += channel3_output * weight;
        self.summed_channel4_sample += channel4_output * weight;
    }

    pub fn clear_summed_samples(&mut self) {
        self.summed_channel1_sample = 0.0;
        self.summed_channel2_sample = 0.0;
        self.summed_channel3_sample = 0.0;
        self.summed_channel4_sample = 0.0;
    }

    fn enqueue_left_sample(&mut self,
        channel1_dac_output: f32,
        channel2_dac_output: f32,
        channel3_dac_output: f32,
        channel4_dac_output: f32) {
        let left_master_volume = (self.master_volume & 0b01110000) >> 4;

        let left_sample = calculate_left_stereo_sample(self.sound_panning,
            left_master_volume,
            channel1_dac_output,
            channel2_dac_output,
            channel3_dac_output,
            channel4_dac_output);

        self.left_sample_queue.push(left_sample);
    }

    fn enqueue_right_sample(&mut self,
        channel1_dac_output: f32,
        channel2_dac_output: f32,
        channel3_dac_output: f32,
        channel4_dac_output: f32) {
        let right_master_volume = self.master_volume & 0b111;

        let right_sample = calculate_right_stereo_sample(self.sound_panning,
            right_master_volume,
            channel1_dac_output,
            channel2_dac_output,
            channel3_dac_output,
            channel4_dac_output);

        self.right_sample_queue.push(right_sample);
    }

    fn enqueue_audio_samples(&mut self, in_color_bios: bool) {
        /*
            This emulator uses audio syncing. It steps the emulator until the audio buffer is full, then 
            briefly pauses while it plays the audio in the buffer. Once the audio plays, the emulator
            resumes.
            
            The purpose of the BIOS check here is that I want my emulator to speed through the
            initial GBC BIOS so it appears as if it's skipping the BIOS altogether (even though it still
            runs it; it's just hidden).
        */
        if !in_color_bios {
            let t_cycle_increment = get_t_cycle_increment(self.cgb_double_speed);

            self.audio_buffer_clock += t_cycle_increment;
            let steps_since_enqueue = self.audio_buffer_clock / t_cycle_increment;
            let steps_per_enqueue = self.enqueue_rate as u8 + 1 / t_cycle_increment;

            let weight = calculate_sample_weight(steps_per_enqueue, steps_since_enqueue);
            self.track_digital_outputs(weight);

            if self.audio_buffer_clock as u32 >= self.enqueue_rate {
                self.audio_buffer_clock = 0;

                let channel1_dac_output = generate_dac_output(self.summed_channel1_sample, steps_since_enqueue);
                let channel2_dac_output = generate_dac_output(self.summed_channel2_sample, steps_since_enqueue);
                let channel3_dac_output = generate_dac_output(self.summed_channel3_sample, steps_since_enqueue);
                let channel4_dac_output = generate_dac_output(self.summed_channel4_sample, steps_since_enqueue);

                self.enqueue_left_sample(
                    channel1_dac_output,
                    channel2_dac_output,
                    channel3_dac_output,
                    channel4_dac_output);

                self.enqueue_right_sample(
                    channel1_dac_output,
                    channel2_dac_output,
                    channel3_dac_output,
                    channel4_dac_output);

                self.clear_summed_samples();
            }
        }
    }

    pub fn step(&mut self, in_color_bios: bool, divider: u8) {
        let t_cycle_increment = get_t_cycle_increment(self.cgb_double_speed);
        self.channel_clock += t_cycle_increment;

        if self.enabled {
            if self.channel_clock >= CHANNEL_STEP_RATE {
                let clock_cycles = self.channel_clock;
                self.channel_clock = 0;

                self.channel1.step(clock_cycles);
                self.channel2.step(clock_cycles);
                self.channel3.step(clock_cycles);
                self.channel4.step(clock_cycles);
            }

            self.step_div_apu(divider);
        }
        self.enqueue_audio_samples(in_color_bios);

        self.last_divider_time = divider;
    }

    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        self.enqueue_rate = CPU_RATE / sample_rate;
    }

    pub fn set_ch1_period_high(&mut self, new_period_high_value: u8) {
        if self.enabled {
            let original_period_high_value = self.channel1.period().high();
            self.channel1.period().set_high(new_period_high_value);

            let length_period_first_half = in_length_period_first_half(self.divider_apu);

            let clock_length_on_enable = self.channel1.should_clock_length_on_enable(original_period_high_value) &&
                length_period_first_half;

            if clock_length_on_enable {
                self.channel1.step_length();
            }

            if self.channel1.should_trigger() {
                self.channel1.trigger(true);

                if self.channel1.should_clock_length_on_trigger() && length_period_first_half {
                    self.channel1.step_length();
                }
            }
        }
    }

    pub fn set_ch2_period_high(&mut self, new_period_high_value: u8) {
        if self.enabled {
            let original_period_high_value = self.channel2.period().high();
            self.channel2.period().set_high(new_period_high_value);

            let length_period_first_half = in_length_period_first_half(self.divider_apu);

            let clock_length_on_enable = self.channel2.should_clock_length_on_enable(original_period_high_value) &&
                length_period_first_half;

            if clock_length_on_enable {
                self.channel2.step_length();
            }

            if self.channel2.should_trigger() {
                self.channel2.trigger(false);

                if self.channel2.should_clock_length_on_trigger() && length_period_first_half {
                    self.channel2.step_length();
                }
            }
        }
    }

    pub fn set_ch3_period_high(&mut self, new_period_high_value: u8) {
        if self.enabled {
            let original_period_high_value = self.channel3.period().high();
            self.channel3.period().set_high(new_period_high_value);

            let length_period_first_half = in_length_period_first_half(self.divider_apu);

            let clock_length_on_enable = self.channel3.should_clock_length_on_enable(original_period_high_value) &&
                length_period_first_half;

            if clock_length_on_enable {
                self.channel3.step_length();
            }

            if self.channel3.should_trigger() {
                self.channel3.trigger(self.cgb_mode);

                if self.channel3.should_clock_length_on_trigger() && length_period_first_half {
                    self.channel3.step_length();
                }
            }
        }
    }

    pub fn set_ch4_control(&mut self, new_control_value: u8) {
        if self.enabled {
            let original_control_value = self.channel4.control();
            self.channel4.set_control(new_control_value);

            let length_period_first_half = in_length_period_first_half(self.divider_apu);

            let clock_length_on_enable = self.channel4.should_clock_length_on_enable(original_control_value) &&
                length_period_first_half;

            if clock_length_on_enable {
                self.channel4.step_length();
            }

            if self.channel4.should_trigger() {
                self.channel4.trigger();

                if self.channel4.should_clock_length_on_trigger() && length_period_first_half {
                    self.channel4.step_length();
                }
            }
        }
    }

    pub fn set_ch1_envelope_settings(&mut self, new_envelope_settings: u8) {
        if self.enabled {
            self.channel1.envelope().set_initial_settings(new_envelope_settings);
            self.channel1.envelope().reset_settings();

            let should_disable = self.channel1.envelope().should_disable_dac();

            self.channel1.set_dac_enabled(!should_disable);

            if should_disable {
                self.channel1.set_enabled(false);
            }
        }
    }

    pub fn set_ch2_envelope_settings(&mut self, new_envelope_settings: u8) {
        if self.enabled {
            self.channel2.envelope().set_initial_settings(new_envelope_settings);
            self.channel2.envelope().reset_settings();

            let should_disable = self.channel2.envelope().should_disable_dac();
        
            self.channel2.set_dac_enabled(!should_disable);
        
            if should_disable {
                self.channel2.set_enabled(false);
            }
        }
    }

    pub fn set_ch3_dac_enabled(&mut self, new_dac_enabled_register_value: u8) {
        if self.enabled {
            let should_disable = !is_bit_set(new_dac_enabled_register_value, CH3_DAC_ENABLED_INDEX);

            self.channel3.set_dac_enabled(!should_disable);
            
            if should_disable {
                self.channel3.set_enabled(false);
            }
        }
    }

    pub fn set_ch4_envelope_settings(&mut self, new_envelope_settings: u8) {
        if self.enabled {
            self.channel4.envelope().set_initial_settings(new_envelope_settings);
            self.channel4.envelope().reset_settings();

            let should_disable = self.channel4.envelope().should_disable_dac();
        
            self.channel4.set_dac_enabled(!should_disable);
        
            if should_disable {
                self.channel4.set_enabled(false);
            }
        }
    }

    pub fn audio_master_control(&self) -> u8 {
        let apu_enabled = if self.enabled { 1 } else { 0 };
        let mask = 0b01110000;
        let ch4_enabled = if self.channel4.enabled() { 1 } else { 0 };
        let ch3_enabled = if self.channel3.enabled() { 1 } else { 0 };
        let ch2_enabled = if self.channel2.enabled() { 1 } else { 0 };
        let ch1_enabled = if self.channel1.enabled() { 1 } else { 0 };
        let result = (apu_enabled << 7)
            | mask
            | (ch4_enabled << 3)
            | (ch3_enabled << 2)
            | (ch2_enabled << 1)
            | ch1_enabled;
        result
    }

    pub fn get_wave_ram_byte(&self, localized_address: u8) -> u8 {
        let mut address = localized_address;

        if self.channel3.enabled() {
            address = self.channel3.wave_position() / 2;
            if self.channel3.period_readonly().reloaded() || self.cgb_mode {
                self.channel3.read_from_wave_ram(address)
            }
            else {
                0xFF
            }
        }
        else {
            self.channel3.read_from_wave_ram(address)
        }
    }

    pub fn set_wave_ram_byte(&mut self, localized_address: u8, new_value: u8) {
        let mut address = localized_address;

        if self.channel3.enabled() {
            address = self.channel3.wave_position() / 2;
            if self.channel3.period().reloaded() || self.cgb_mode {
                self.channel3.write_to_wave_ram(address, new_value);
            }
        }
        else {
            self.channel3.write_to_wave_ram(address, new_value);
        }
    }

    pub fn set_audio_master_control(&mut self, new_audio_master_control: u8) {
        self.enabled = is_bit_set(new_audio_master_control, APU_ENABLED_INDEX);

        if !self.enabled {
            self.reset();
        }
    }

    pub fn set_ch1_sweep_settings(&mut self, new_sweep_settings: u8) {
        if self.enabled {
            self.channel1.sweep().update_initial_settings(new_sweep_settings);
            if self.channel1.sweep().should_disable_channel() {
                self.channel1.set_enabled(false);
            }
        }
    }

    pub fn set_ch1_length_settings(&mut self, new_length_settings: u8) {
        if self.enabled || !self.cgb_mode {
            let new_initial_settings = if self.enabled {
                new_length_settings
            }
            else {
                new_length_settings & 0x3F
            };
            self.channel1.length().set_initial_settings(new_initial_settings);
            self.channel1.length().initialize_timer();
        }
    }

    pub fn set_ch1_period_low(&mut self, new_period_low: u8) {
        if self.enabled {
            self.channel1.period().set_low(new_period_low);
        }
    }

    pub fn set_ch2_length_settings(&mut self, new_length_settings: u8) {
        if self.enabled || !self.cgb_mode {
            let new_initial_settings = if self.enabled {
                new_length_settings
            }
            else {
                new_length_settings & 0x3F
            };
            self.channel2.length().set_initial_settings(new_initial_settings);
            self.channel2.length().initialize_timer();
        }
    }

    pub fn set_ch2_period_low(&mut self, new_period_low: u8) {
        if self.enabled {
            self.channel2.period().set_low(new_period_low);
        }
    }

    pub fn set_ch3_length_settings(&mut self, new_length_settings: u8) {
        if self.enabled || !self.cgb_mode {
            self.channel3.length().set_initial_settings(new_length_settings);
            self.channel3.length().initialize_timer();
        }
    }

    pub fn set_ch3_period_low(&mut self, new_period_low: u8) {
        if self.enabled {
            self.channel3.period().set_low(new_period_low);
        }
    }

    pub fn set_ch3_volume(&mut self, new_volume: u8) {
        if self.enabled {
            self.channel3.set_volume(new_volume);
        }
    }

    pub fn set_ch4_length_settings(&mut self, new_length_settings: u8) {
        if self.enabled || !self.cgb_mode {
            self.channel4.length().set_initial_settings(new_length_settings);
            self.channel4.length().initialize_timer();
        }
    }

    pub fn set_ch4_polynomial(&mut self, new_polynomial: u8) {
        if self.enabled {
            self.channel4.set_polynomial(new_polynomial);
        }
    }

    pub fn master_volume(&self) -> u8 {
        self.master_volume
    }

    pub fn set_master_volume(&mut self, new_master_volume: u8) {
        if self.enabled {
            self.master_volume = new_master_volume;
        }
    }

    pub fn sound_panning(&self) -> u8 {
        self.sound_panning
    }

    pub fn set_sound_panning(&mut self, new_sound_panning: u8) {
        if self.enabled {
            self.sound_panning = new_sound_panning;
        }
    }

    pub fn set_audio_buffer_clock(&mut self, new_audio_buffer_clock: u8) {
        self.audio_buffer_clock = new_audio_buffer_clock;
    }

    pub fn set_cgb_mode(&mut self, cgb_mode: bool) {
        self.cgb_mode = cgb_mode;
    }

    pub fn set_cgb_double_speed(&mut self, cgb_double_speed: bool) {
        self.cgb_double_speed = cgb_double_speed;
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn channel1(&mut self) -> &mut PulseChannel {
        &mut self.channel1
    }

    pub fn channel1_readonly(&self) -> &PulseChannel {
        &self.channel1
    }

    pub fn channel2(&mut self) -> &mut PulseChannel {
        &mut self.channel2
    }

    pub fn channel2_readonly(&self) -> &PulseChannel {
        &self.channel2
    }

    pub fn channel3(&mut self) -> &mut WaveChannel {
        &mut self.channel3
    }

    pub fn channel3_readonly(&self) -> &WaveChannel {
        &self.channel3
    }

    pub fn channel4(&mut self) -> &mut NoiseChannel {
        &mut self.channel4
    }

    pub fn channel4_readonly(&self) -> &NoiseChannel {
        &self.channel4
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
