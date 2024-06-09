#[derive(Debug)]
pub struct Length {
    pub initial_settings: u8,
    pub timer: u16
}   

pub fn initialize_length() -> Length {
    Length {
        initial_settings: 0,
        timer: 0
    }
}

pub fn step(length: &mut Length) {
    if length.timer > 0 {
        length.timer -= 1;
    }
}

pub fn trigger(length: &mut Length, channel_three: bool) {
    let initial_timer_value = if channel_three {
        256 - length.initial_settings as u16
    }
    else {
        let initial_length = (length.initial_settings & 0b00111111) as u16;
        64 - initial_length
    };
    length.timer = initial_timer_value;
}