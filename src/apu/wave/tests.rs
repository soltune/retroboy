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

    assert_eq!(dac_output(&emulator), -0.46666664); 
}