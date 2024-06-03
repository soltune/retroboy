use crate::apu::period::{initalize_period, Period};

#[derive(Debug)]
pub struct WaveChannel {
    pub enabled: bool,
    pub dac_enabled: bool,
    pub length: u8,
    pub volume: u8,
    pub period: Period
}

pub fn initialize_wave_channel() -> WaveChannel {
    WaveChannel {
        enabled: false,
        dac_enabled: false,
        length: 0,
        volume: 0,
        period: initalize_period()
    }
}
