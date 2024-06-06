#[derive(Debug)]
pub struct Length {
    pub initial_value_and_duty: u8,
    pub timer: u16
}   

pub fn initialize_length() -> Length {
    Length {
        initial_value_and_duty: 0,
        timer: 0
    }
}

pub fn step(length: &mut Length) {
    if length.timer > 0 {
        length.timer -= 1;
    }
}

pub fn trigger(length: &mut Length, channel_three: bool) {
    let initial_length = (length.initial_value_and_duty & 0b00111111) as u16;
    let max_length = if channel_three { 256 } else { 64 };
    let initial_timer_value = max_length - initial_length;
    length.timer = initial_timer_value;
}