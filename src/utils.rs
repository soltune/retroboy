pub const T_CYCLE_INCREMENT: u8 = 4;

pub fn is_bit_set(byte: u8, bit_index: u8) -> bool {
    let mask = 1 << bit_index;
    (mask & byte) > 0
}

pub fn get_bit(byte: u8, bit_index: u8) -> u8 {
    let is_set = is_bit_set(byte, bit_index);
    if is_set { 1 } else { 0 }
}

pub fn set_bit(byte: u8, bit_index: u8) -> u8 {
    let mask: u8 = 1 << bit_index;
    byte | mask
}

pub fn reset_bit(byte: u8, bit_index: u8) -> u8 {
    let mask: u8 = !(1 << bit_index);
    byte & mask
}

pub fn as_word(low_byte: u8, high_byte: u8) -> u16 {
    ((high_byte as u16) << 8) | low_byte as u16
}

pub fn as_bytes(word: u16) -> (u8, u8) {
    let low_byte = (word & 0xFF) as u8;
    let high_byte = (word >> 8) as u8;
    (low_byte, high_byte)
}

#[cfg(test)]
mod tests;