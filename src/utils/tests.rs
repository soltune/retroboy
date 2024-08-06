use super::*;

#[test]
fn should_get_bit() {
   let byte = 0b10000101 as u8;
   assert_eq!(get_bit(byte, 2), 0x1);
}

#[test]
fn should_get_bit_scenario_two() {
    let byte = 0b10000001 as u8;
    assert_eq!(get_bit(byte, 2), 0x0);
}

#[test]
fn should_combine_bytes_to_word() {
    let low_byte = 0xA1 as u8;
    let high_byte = 0x2B as u8;
    assert_eq!(as_word(low_byte, high_byte), 0x2BA1);
}

#[test]
fn should_split_word_into_bytes() {
    let word = 0x2BA1 as u16;
    assert_eq!(as_bytes(word), (0xA1, 0x2B));
}