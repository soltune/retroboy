use super::*;

fn enable_pulse_channel(channel: &mut PulseChannel) {
    channel.enabled = true;
    channel.dac_enabled = true;
}

fn initialize_amplitude_variables(channel: &mut PulseChannel,
    wave_duty: u8,
    wave_duty_position: u8,
    current_volume: u8) {
    channel.length.initial_settings = wave_duty << 6;
    channel.wave_duty_position = wave_duty_position;
    channel.envelope.current_volume = current_volume;
}

#[test]
fn should_calculate_dac_output_when_amplitude_is_zero() {
    let mut channel = initialize_pulse_channel();
    enable_pulse_channel(&mut channel);

    let wave_duty = 1;
    let wave_duty_position = 2;
    let current_volume = 5;
    initialize_amplitude_variables(&mut channel, wave_duty, wave_duty_position, current_volume);

    assert_eq!(dac_output(&channel), -1.0);
}

#[test]
fn should_calculate_dac_output_when_amplitude_is_one() {
    let mut channel = initialize_pulse_channel();
    enable_pulse_channel(&mut channel);

    let wave_duty = 1;
    let wave_duty_position = 1;
    let current_volume = 5;
    initialize_amplitude_variables(&mut channel, wave_duty, wave_duty_position, current_volume);

    assert_eq!(dac_output(&channel), -0.3333333);
}

#[test]
fn should_calculate_dac_output_when_volume_is_at_ten() {
    let mut channel = initialize_pulse_channel();
    enable_pulse_channel(&mut channel);

    let wave_duty = 2;
    let wave_duty_position = 2;
    let current_volume = 10;
    initialize_amplitude_variables(&mut channel, wave_duty, wave_duty_position, current_volume);

    assert_eq!(dac_output(&channel), 0.33333337);
}