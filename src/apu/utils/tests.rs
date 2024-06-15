use super::*;

fn mix_left_samples(master_volume: u8) -> f32 {
    let sound_panning = 0b11010000;
    let channel1_output = 0.25 as f32;
    let channel2_output = 0.5 as f32;
    let channel3_output = -0.15 as f32;
    let channel4_output = 1.0 as f32;

    calculate_left_stereo_sample(sound_panning,
        master_volume,
        channel1_output,
        channel2_output,
        channel3_output,
        channel4_output)
}

fn mix_right_samples(master_volume: u8) -> f32 {
    let sound_panning = 0b00001110;
    let channel1_output = 0.25 as f32;
    let channel2_output = 0.5 as f32;
    let channel3_output = -0.15 as f32;
    let channel4_output = 1.0 as f32;

    calculate_right_stereo_sample(sound_panning,
        master_volume,
        channel1_output,
        channel2_output,
        channel3_output,
        channel4_output)
}

#[test]
fn should_mix_channels_and_generate_left_stereo_sample() {
    let left_master_volume = 0b111;
    let left_stereo_sample = mix_left_samples(left_master_volume);
    assert_eq!(left_stereo_sample, 0.275);
}

#[test]
fn should_reduce_volume_of_left_stereo_sample() {
    let left_master_volume = 0b011;
    let left_stereo_sample = mix_left_samples(left_master_volume);
    assert_eq!(left_stereo_sample, 0.1375);
}

#[test]
fn should_mix_channels_and_generate_right_stereo_sample() {
    let right_master_volume = 0b111;
    let right_stereo_sample = mix_right_samples(right_master_volume);
    assert_eq!(right_stereo_sample, 0.3375);
}

#[test]
fn should_reduce_volume_of_right_stereo_sample() {
    let right_master_volume = 0b011;
    let right_stereo_sample = mix_right_samples(right_master_volume);
    assert_eq!(right_stereo_sample, 0.16875);
}