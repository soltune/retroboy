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