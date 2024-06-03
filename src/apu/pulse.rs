use crate::apu::envelope::{initialize_envelope, Envelope};
use crate::apu::period::{initalize_period, Period};
use crate::apu::utils::bounded_wrapping_add;

#[derive(Debug)]
pub struct PulseChannel {
    pub enabled: bool,
    pub dac_enabled: bool,
    pub sweep: u8,
    pub length_and_duty: u8,
    pub wave_duty_position: u8,
    pub envelope: Envelope,
    pub period: Period,
    pub channel_id: u8
}

pub fn initialize_pulse_channel(channel_id: u8) -> PulseChannel {
    PulseChannel {
        enabled: false,
        dac_enabled: false,
        sweep: 0,
        length_and_duty: 0,
        wave_duty_position: 0,
        envelope: initialize_envelope(),
        period: initalize_period(),
        channel_id
    } 
}

const MAX_WAVEFORM_STEPS: u8 = 7;

fn calculate_period_divider(ch_period_high: u8, ch_period_low: u8) -> u16 {
    let period_high = (ch_period_high & 0b111) as u16;
    let new_period = (period_high << 8) | ch_period_low as u16;
    2048 - new_period
}

pub fn step(channel: &mut PulseChannel, last_instruction_clock_cycles: u8) {
    if channel.enabled {
        let mut period_divider_increment = (last_instruction_clock_cycles / 4) as u16;
        while period_divider_increment > 0 {
            channel.period.divider -= 1;
            if channel.period.divider == 0 {
                channel.period.divider = calculate_period_divider(channel.period.high, channel.period.low);
                channel.wave_duty_position = bounded_wrapping_add(channel.wave_duty_position, MAX_WAVEFORM_STEPS)
            }
            period_divider_increment -= 1;
        } 
    }
}
