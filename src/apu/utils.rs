use crate::utils::is_bit_set;

pub fn bounded_wrapping_add(original_value: u8, max_value: u8) -> u8 {
    let mut new_value = original_value + 1;
    if new_value > max_value {
        new_value = 0;
    }
    new_value
}

pub fn as_dac_output(dac_input: f32) -> f32 {
    (dac_input / 7.5) - 1.0
}

const LENGTH_ENABLED_INDEX: u8 = 6;

pub fn length_enabled(register_value_with_length: u8) -> bool {
    is_bit_set(register_value_with_length, LENGTH_ENABLED_INDEX) 
}

const CHANNEL4_LEFT_PANNING_INDEX: u8 = 7;
const CHANNEL3_LEFT_PANNING_INDEX: u8 = 6;
const CHANNEL2_LEFT_PANNING_INDEX: u8 = 5;
const CHANNEL1_LEFT_PANNING_INDEX: u8 = 4;
const CHANNEL4_RIGHT_PANNING_INDEX: u8 = 3;
const CHANNEL3_RIGHT_PANNING_INDEX: u8 = 2;
const CHANNEL2_RIGHT_PANNING_INDEX: u8 = 1;
const CHANNEL1_RIGHT_PANNING_INDEX: u8 = 0;

fn get_panned_output(sound_panning: u8, panning_bit_index: u8, output: f32) -> f32 {
    if is_bit_set(sound_panning, panning_bit_index) {
        output
    }
    else {
        0.0
    }
}

fn mix_samples(channel1_output: f32,
    channel2_output: f32,
    channel3_output: f32,
    channel4_output: f32) -> f32 {
    (channel1_output + channel2_output + channel3_output + channel4_output) / 4.0
}

fn apply_volume_reduction(sample: f32, master_volume: u8) -> f32 {
    let volume_reduction = (master_volume as f32 + 1.0) / 8.0;
    sample * volume_reduction
}

pub fn calculate_left_stereo_sample(sound_panning: u8,
    left_master_volume: u8,
    channel1_output: f32,
    channel2_output: f32,
    channel3_output: f32,
    channel4_output: f32) -> f32 {
    let channel1_panned_output = get_panned_output(sound_panning, CHANNEL1_LEFT_PANNING_INDEX, channel1_output);
    let channel2_panned_output = get_panned_output(sound_panning, CHANNEL2_LEFT_PANNING_INDEX, channel2_output);
    let channel3_panned_output = get_panned_output(sound_panning, CHANNEL3_LEFT_PANNING_INDEX, channel3_output);
    let channel4_panned_output = get_panned_output(sound_panning, CHANNEL4_LEFT_PANNING_INDEX, channel4_output);
 
    let left_sample = mix_samples(channel1_panned_output, 
        channel2_panned_output, 
        channel3_panned_output, 
        channel4_panned_output);

    apply_volume_reduction(left_sample, left_master_volume)
}

pub fn calculate_right_stereo_sample(sound_panning: u8,
    right_master_volume: u8,
    channel1_output: f32,
    channel2_output: f32,
    channel3_output: f32,
    channel4_output: f32) -> f32 {
    let channel1_panned_output = get_panned_output(sound_panning, CHANNEL1_RIGHT_PANNING_INDEX, channel1_output);
    let channel2_panned_output = get_panned_output(sound_panning, CHANNEL2_RIGHT_PANNING_INDEX, channel2_output);
    let channel3_panned_output = get_panned_output(sound_panning, CHANNEL3_RIGHT_PANNING_INDEX, channel3_output);
    let channel4_panned_output = get_panned_output(sound_panning, CHANNEL4_RIGHT_PANNING_INDEX, channel4_output);
 
    let right_sample = mix_samples(channel1_panned_output, 
        channel2_panned_output, 
        channel3_panned_output, 
        channel4_panned_output);

    apply_volume_reduction(right_sample, right_master_volume)
}

#[cfg(test)]
mod tests {
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
}