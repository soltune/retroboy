use crate::apu::envelope::{initialize_envelope, Envelope};

#[derive(Debug)]
pub struct NoiseChannel {
    pub enabled: bool,
    pub dac_enabled: bool,
    pub length: u8,
    pub envelope: Envelope,
    pub randomness: u8,
    pub control: u8
}

pub fn initialize_noise_channel() -> NoiseChannel {
    NoiseChannel {
        enabled: false,
        dac_enabled: false,
        length: 0,
        envelope: initialize_envelope(),
        randomness: 0,
        control: 0
    }
}