#[derive(Debug)]
pub struct Length {
    pub initial_settings: u8,
    pub timer: u16
}

const WAVE_MAX_LENGTH: u16 = 256;
const DEFAULT_MAX_LENGTH: u16 = 64;

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

pub fn initialize_timer(length: &mut Length) {
    let initial_length = (length.initial_settings & 0b00111111) as u16;
    let initial_timer_value = DEFAULT_MAX_LENGTH - initial_length;
    length.timer = initial_timer_value;
}

pub fn initialize_wave_channel_timer(length: &mut Length) {
    length.timer = WAVE_MAX_LENGTH - length.initial_settings as u16;
}

pub fn reload_timer_with_maximum(length: &mut Length) {
    if length.timer == 0 {
        length.timer = DEFAULT_MAX_LENGTH;
    }
}

pub fn reload_wave_channel_timer_with_maximum(length: &mut Length) {
    if length.timer == 0 {
        length.timer = WAVE_MAX_LENGTH;
    }
}

pub fn at_max_length(length: &Length) -> bool {
    length.timer == DEFAULT_MAX_LENGTH
}

pub fn at_max_wave_channel_length(length: &Length) -> bool {
    length.timer == WAVE_MAX_LENGTH
}