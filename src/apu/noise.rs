use crate::apu::envelope::{initialize_envelope, Envelope};
use crate::apu::length::{initialize_length, Length};

#[derive(Debug)]
pub struct NoiseChannel {
    pub enabled: bool,
    pub dac_enabled: bool,
    pub length: Length,
    pub envelope: Envelope,
    pub polynomial: u8,
    pub lfsr: u16,
    pub control: u8
}

pub fn initialize_noise_channel() -> NoiseChannel {
    NoiseChannel {
        enabled: false,
        dac_enabled: false,
        length: initialize_length(),
        envelope: initialize_envelope(),
        polynomial: 0,
        lfsr: 0,
        control: 0
    }
}