use crate::emulator::initialize_emulator;

use super::*;

fn enable_wave_channel(channel: &mut WaveChannel) {
    channel.enabled = true;
    channel.dac_enabled = true;
}

#[test]
fn should_calculate_dac_output_when_amplitude_is_zero() {
    let mut emulator = initialize_emulator();
    enable_wave_channel(&mut emulator.apu.channel3);

    emulator.memory.wave_pattern_ram[0] = 0xAC;
    emulator.memory.wave_pattern_ram[1] = 0xC0;
    emulator.memory.wave_pattern_ram[2] = 0x04;
    emulator.memory.wave_pattern_ram[3] = 0xDC;

    emulator.apu.channel3.wave_position = 3;
    emulator.apu.channel3.volume = 0b00100000;

    assert_eq!(dac_output(&emulator), -1.0);
}

#[test]
fn should_calculate_dac_output_when_amplitude_is_non_zero() {
    let mut emulator = initialize_emulator();
    enable_wave_channel(&mut emulator.apu.channel3);

    emulator.memory.wave_pattern_ram[0] = 0xAC;
    emulator.memory.wave_pattern_ram[1] = 0xC0;
    emulator.memory.wave_pattern_ram[2] = 0x04;
    emulator.memory.wave_pattern_ram[3] = 0xDC;

    emulator.apu.channel3.wave_position = 5;
    emulator.apu.channel3.volume = 0b00100000;

    assert_eq!(dac_output(&emulator), -0.46666664); 
}

#[test]
fn should_generate_no_sound_if_channel_is_muted() {
    let mut emulator = initialize_emulator();
    enable_wave_channel(&mut emulator.apu.channel3);

    emulator.memory.wave_pattern_ram[0] = 0xAC;
    emulator.memory.wave_pattern_ram[1] = 0xC0;
    emulator.memory.wave_pattern_ram[2] = 0x04;
    emulator.memory.wave_pattern_ram[3] = 0xDC;

    emulator.apu.channel3.wave_position = 5;
    emulator.apu.channel3.volume = 0;

    assert_eq!(dac_output(&emulator), 0.0); 
}

#[test]
fn should_shift_sample_right_once_if_channel_is_set_to_half_of_volume() {
    let mut emulator = initialize_emulator();
    enable_wave_channel(&mut emulator.apu.channel3);

    emulator.memory.wave_pattern_ram[0] = 0xAC;
    emulator.memory.wave_pattern_ram[1] = 0xC0;
    emulator.memory.wave_pattern_ram[2] = 0x04;
    emulator.memory.wave_pattern_ram[3] = 0xDC;

    emulator.apu.channel3.wave_position = 5;
    emulator.apu.channel3.volume = 0b01000000;

    assert_eq!(dac_output(&emulator), -0.73333335); 
}

#[test]
fn should_produce_no_audio_output_if_channel_is_disabled() {
    let mut emulator = initialize_emulator();

    emulator.memory.wave_pattern_ram[0] = 0xAC;
    emulator.memory.wave_pattern_ram[1] = 0xC0;
    emulator.memory.wave_pattern_ram[2] = 0x04;
    emulator.memory.wave_pattern_ram[3] = 0xDC;

    emulator.apu.channel3.wave_position = 5;
    emulator.apu.channel3.volume = 0b01000000;

    assert_eq!(dac_output(&emulator), 0.0); 
}