pub fn is_bit_set(byte: u8, bit_index: u8) -> bool {
    let mask = 1 << bit_index;
    (mask & byte) > 0
}

pub fn get_bit(byte: u8, bit_index: u8) -> u8 {
    let is_set = is_bit_set(byte, bit_index);
    if is_set { 1 } else { 0 }
}

#[cfg(test)]
mod tests;