pub fn bounded_wrapping_add(original_value: u8, max_value: u8) -> u8 {
    let mut new_value = original_value + 1;
    if new_value > max_value {
        new_value = 0;
    }
    new_value
}
