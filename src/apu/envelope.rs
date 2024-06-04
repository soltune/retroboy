#[derive(Debug)]
pub struct Envelope {
    pub initial_settings: u8,
    pub current_volume: u8,
    pub timer: u8
}

pub fn initialize_envelope() -> Envelope {
    Envelope {
        initial_settings: 0,
        current_volume: 0,
        timer: 0
    }
}

pub fn trigger(envelope: &mut Envelope) {
    let initial_timer = envelope.initial_settings & 0b00000111;
    let initial_volume = (envelope.initial_settings & 0b11110000) >> 4;
    envelope.timer = initial_timer;
    envelope.current_volume = initial_volume;
}
    
pub fn should_disable_dac(envelope: &Envelope) -> bool {
    envelope.initial_settings & 0xF8 == 0
}