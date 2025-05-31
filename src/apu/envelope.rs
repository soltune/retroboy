use crate::utils::is_bit_set;
use bincode::{Encode, Decode};

#[derive(Clone, Debug, Encode, Decode)]
pub struct Envelope {
    initial_settings: u8,
    current_volume: u8,
    timer: u8
}

const ENVELOPE_DIRECTION_INDEX: u8 = 3;

impl Envelope {
    pub fn new() -> Self {
        Envelope {
            initial_settings: 0,
            current_volume: 0,
            timer: 0
        }
    }

    pub fn step(&mut self) {
        let initial_timer = self.initial_settings & 0b00000111;
        let is_upwards = is_bit_set(self.initial_settings, ENVELOPE_DIRECTION_INDEX);

        if initial_timer != 0 {
            if self.timer > 0 {
                self.timer -= 1
            }
        
            if self.timer == 0 {
                self.timer = initial_timer;
        
                if (self.current_volume < 0xF && is_upwards) || (self.current_volume > 0x0 && !is_upwards) {
                    if is_upwards {
                        self.current_volume += 1;
                    }
                    else {
                        self.current_volume -= 1;
                    }
                }
            }
        }
    }

    pub fn reset_settings(&mut self) {
        let initial_timer = self.initial_settings & 0b00000111;
        let initial_volume = (self.initial_settings & 0b11110000) >> 4;
        self.timer = initial_timer;
        self.current_volume = initial_volume;
    }

    pub fn should_disable_dac(&self) -> bool {
        self.initial_settings & 0xF8 == 0
    }

    pub fn current_volume(&self) -> u8 {
        self.current_volume
    }

    pub fn set_current_volume(&mut self, volume: u8) {
        self.current_volume = volume;
    }

    pub fn initial_settings(&self) -> u8 {
        self.initial_settings
    }

    pub fn set_initial_settings(&mut self, value: u8) {
        self.initial_settings = value;
    }

    pub fn timer(&self) -> u8 {
        self.timer
    }

    pub fn set_timer(&mut self, timer: u8) {
        self.timer = timer;
    }
}
