use super::*;

fn enable_noise_channel(channel: &mut NoiseChannel) {
    channel.enabled = true;
    channel.dac_enabled = true;
}

#[test]
fn should_calculate_dac_output_when_amplitude_is_zero() {
    let mut channel = initialize_noise_channel();
    enable_noise_channel(&mut channel);

    channel.lfsr = 0xFFFF;
    channel.envelope.current_volume = 0xA;

    assert_eq!(dac_output(&channel), -1.0);
}

#[test]
fn should_calculate_dac_output_when_amplitude_is_one() {
    let mut channel = initialize_noise_channel();
    enable_noise_channel(&mut channel);

    channel.lfsr = 0;
    channel.envelope.current_volume = 0xA;

    assert_eq!(dac_output(&channel), 0.33333337);
}