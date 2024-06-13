pub fn bounded_wrapping_add(original_value: u8, max_value: u8) -> u8 {
    let mut new_value = original_value + 1;
    if new_value > max_value {
        new_value = 0;
    }
    new_value
}

pub fn as_dac_output(dac_input: u8) -> f32 {
    (dac_input as f32 / 7.5) - 1.0
}