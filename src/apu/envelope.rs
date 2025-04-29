use crate::utils::is_bit_set;
use bincode::{Encode, Decode};


#[derive(Clone, Debug, Encode, Decode)]
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

const ENVELOPE_DIRECTION_INDEX: u8 = 3;

pub fn step(envelope: &mut Envelope) {
    let initial_timer = envelope.initial_settings & 0b00000111;
    let is_upwards = is_bit_set(envelope.initial_settings, ENVELOPE_DIRECTION_INDEX);

    if initial_timer != 0 {
        if envelope.timer > 0 {
            envelope.timer -= 1
        }
    
        if envelope.timer == 0 {
            envelope.timer = initial_timer;
    
            if (envelope.current_volume < 0xF && is_upwards) || (envelope.current_volume > 0x0 && !is_upwards) {
                if is_upwards {
                    envelope.current_volume += 1;
                }
                else {
                    envelope.current_volume -= 1;
                }
            }
        }
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