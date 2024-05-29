use crate::emulator::Emulator;

#[derive(Debug)]
pub struct ApuState {
    pub audio_master_control: u8, // NR52
    pub sound_panning: u8, // NR51
    pub master_volume: u8, // NR50
    pub ch1_sweep: u8, // NR10
    pub ch1_length_and_duty: u8, // NR11
    pub ch1_wave_duty_position: u8,
    pub ch1_volume: u8, // NR12
    pub ch1_period_low: u8, // NR13
    pub ch1_period_high: u8, // NR14
    pub ch1_period_divider: u16,
    pub ch2_length_and_duty: u8, // NR21
    pub ch2_wave_duty_position: u8,
    pub ch2_volume: u8, // NR22
    pub ch2_period_low: u8, // NR23
    pub ch2_period_high: u8, // NR24
    pub ch2_period_divider: u16,
    pub ch3_dac_enable: u8, // NR30
    pub ch3_length: u8, // NR31
    pub ch3_volume: u8, // NR32
    pub ch3_period_low: u8, // NR33
    pub ch3_period_high: u8, // NR34
    pub ch4_length: u8, // NR41
    pub ch4_volume: u8, // NR42
    pub ch4_randomness: u8, // NR43
    pub ch4_control: u8, // NR44
    pub divider_apu: u8
}

pub fn initialize_apu() -> ApuState {
    ApuState {
        audio_master_control: 0,
        sound_panning: 0,
        master_volume: 0,
        ch1_sweep: 0,
        ch1_length_and_duty: 0,
        ch1_wave_duty_position: 0,
        ch1_volume: 0,
        ch1_period_low: 0,
        ch1_period_high: 0,
        ch1_period_divider: 0,
        ch2_length_and_duty: 0,
        ch2_wave_duty_position: 0,
        ch2_volume: 0,
        ch2_period_low: 0,
        ch2_period_high: 0,
        ch2_period_divider: 0,
        ch3_dac_enable: 0,
        ch3_length: 0,
        ch3_volume: 0,
        ch3_period_low: 0,
        ch3_period_high: 0,
        ch4_length: 0,
        ch4_volume: 0,
        ch4_randomness: 0,
        ch4_control: 0,
        divider_apu: 0
    }
}

// Work In Progress

pub fn step_channel_1(emulator: &mut Emulator) {
    let mut period_divider_increment = (emulator.cpu.clock.instruction_clock_cycles / 4) as u16;
    while period_divider_increment > 0 {
        emulator.apu.ch1_period_divider -= 1;
        if emulator.apu.ch1_period_divider == 0 {
            let period_high = (emulator.apu.ch1_period_high & 0b111) as u16;
            let period_low = (emulator.apu.ch1_period_low) as u16;
            let new_period = (period_high << 8) | period_low;
            emulator.apu.ch1_period_divider = 2048 - new_period;
            emulator.apu.ch1_wave_duty_position += 1;
            if emulator.apu.ch1_wave_duty_position > 7 {
                emulator.apu.ch1_wave_duty_position = 0;
            }
        }
        period_divider_increment -= 1;
    }
}

pub fn step(emulator: &mut Emulator) {
    step_channel_1(emulator);
}

#[cfg(test)]
mod tests;