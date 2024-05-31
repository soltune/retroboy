use crate::{emulator::Emulator, utils::{get_bit, set_bit, is_bit_set}};

#[derive(Debug)]
pub struct ApuState {
    pub audio_master_control: u8, // NR52
    pub sound_panning: u8, // NR51
    pub master_volume: u8, // NR50
    pub ch1_enabled: bool,
    pub ch1_dac_enabled: bool,
    pub ch1_sweep: u8, // NR10
    pub ch1_length_and_duty: u8, // NR11
    pub ch1_wave_duty_position: u8,
    pub ch1_volume: u8, // NR12
    pub ch1_period_low: u8, // NR13
    pub ch1_period_high: u8, // NR14
    pub ch1_period_divider: u16,
    pub ch2_enabled: bool,
    pub ch2_dac_enabled: bool,
    pub ch2_length_and_duty: u8, // NR21
    pub ch2_wave_duty_position: u8,
    pub ch2_volume: u8, // NR22
    pub ch2_period_low: u8, // NR23
    pub ch2_period_high: u8, // NR24
    pub ch2_period_divider: u16,
    pub ch3_enabled: bool,
    pub ch3_dac_enable: u8, // NR30
    pub ch3_length: u8, // NR31
    pub ch3_volume: u8, // NR32
    pub ch3_period_low: u8, // NR33
    pub ch3_period_high: u8, // NR34
    pub ch4_enabled: bool,
    pub ch4_dac_enabled: bool,
    pub ch4_length: u8, // NR41
    pub ch4_volume: u8, // NR42
    pub ch4_randomness: u8, // NR43
    pub ch4_control: u8, // NR44
    pub divider_apu: u8,
    pub last_divider_time: u8
}

pub fn initialize_apu() -> ApuState {
    ApuState {
        audio_master_control: 0,
        sound_panning: 0,
        master_volume: 0,
        ch1_enabled: false,
        ch1_dac_enabled: false,
        ch1_sweep: 0,
        ch1_length_and_duty: 0,
        ch1_wave_duty_position: 0,
        ch1_volume: 0,
        ch1_period_low: 0,
        ch1_period_high: 0,
        ch1_period_divider: 0,
        ch2_enabled: false,
        ch2_dac_enabled: false,
        ch2_length_and_duty: 0,
        ch2_wave_duty_position: 0,
        ch2_volume: 0,
        ch2_period_low: 0,
        ch2_period_high: 0,
        ch2_period_divider: 0,
        ch3_enabled: false,
        ch3_dac_enable: 0,
        ch3_length: 0,
        ch3_volume: 0,
        ch3_period_low: 0,
        ch3_period_high: 0,
        ch4_enabled: false,
        ch4_dac_enabled: false,
        ch4_length: 0,
        ch4_volume: 0,
        ch4_randomness: 0,
        ch4_control: 0,
        divider_apu: 0,
        last_divider_time: 0
    }
}

// Work In Progress

const APU_ENABLED_INDEX: u8 = 7;
const CH1_ENABLED_INDEX: u8 = 0;
const NR14_TIRGER_INDEX: u8 = 7;
const MAX_WAVEFORM_STEPS: u8 = 7;
const MAX_DIV_APU_STEPS: u8 = 7;

fn calculate_period_divider(ch_period_high: u8, ch_period_low: u8) -> u16 {
    let period_high = (ch_period_high & 0b111) as u16;
    let new_period = (period_high << 8) | ch_period_low as u16;
    2048 - new_period
}

fn bounded_wrapping_add(original_value: u8, max_value: u8) -> u8 {
    let mut new_value = original_value + 1;
    if new_value > max_value {
        new_value = 0;
    }
    new_value
}

fn step_channel_1(emulator: &mut Emulator) {
    if emulator.apu.ch1_enabled {
        let mut period_divider_increment = (emulator.cpu.clock.instruction_clock_cycles / 4) as u16;
        while period_divider_increment > 0 {
            emulator.apu.ch1_period_divider -= 1;
            if emulator.apu.ch1_period_divider == 0 {
                emulator.apu.ch1_period_divider = calculate_period_divider(emulator.apu.ch1_period_high, emulator.apu.ch1_period_low);
                emulator.apu.ch1_wave_duty_position = bounded_wrapping_add(emulator.apu.ch1_wave_duty_position, MAX_WAVEFORM_STEPS)
            }
            period_divider_increment -= 1;
        } 
    }
}

fn should_step_div_apu(emulator: &mut Emulator) -> bool {
    emulator.apu.last_divider_time > 0
        && emulator.timers.divider > 0
        && get_bit(emulator.apu.last_divider_time, 4) == 1
        && get_bit(emulator.timers.divider, 4) == 0
}

fn step_div_apu(emulator: &mut Emulator) {
    // TODO: Add logic to step length, envelope, and sweep

    if should_step_div_apu(emulator) {
        emulator.apu.last_divider_time = emulator.timers.divider;
        emulator.apu.divider_apu = bounded_wrapping_add(emulator.apu.divider_apu, MAX_DIV_APU_STEPS)
    }
}

fn apu_enabled(audio_master_control: u8) -> bool {
    is_bit_set(audio_master_control, APU_ENABLED_INDEX)
}

pub fn set_nr14(emulator: &mut Emulator, new_nr14_value: u8) {
    emulator.apu.ch1_period_high = new_nr14_value;
    
    let should_trigger_ch1 = emulator.apu.ch1_dac_enabled 
        && is_bit_set(emulator.apu.ch1_period_high, NR14_TIRGER_INDEX);
    
    if should_trigger_ch1 { 
        emulator.apu.ch1_enabled = true;
        emulator.apu.audio_master_control = set_bit(emulator.apu.audio_master_control, CH1_ENABLED_INDEX);
    }
}

pub fn step(emulator: &mut Emulator) {    
    if apu_enabled(emulator.apu.audio_master_control) {
        step_channel_1(emulator);
        step_div_apu(emulator);
    }    
}

#[cfg(test)]
mod tests;