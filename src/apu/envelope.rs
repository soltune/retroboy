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
    
pub fn should_disable_dac(envelope: &Envelope) -> bool {
    envelope.initial_settings & 0xF8 == 0
}