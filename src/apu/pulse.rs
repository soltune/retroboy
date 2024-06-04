use crate::apu::envelope;
use crate::apu::envelope::{initialize_envelope, Envelope};
use crate::apu::period::{calculate_period_divider, initalize_period, Period};
use crate::apu::utils::bounded_wrapping_add;
use crate::utils::is_bit_set;

#[derive(Debug)]
pub struct PulseChannel {
    pub enabled: bool,
    pub dac_enabled: bool,
    pub sweep: u8,
    pub length_and_duty: u8,
    pub wave_duty_position: u8,
    pub envelope: Envelope,
    pub period: Period,
}

pub fn initialize_pulse_channel() -> PulseChannel {
    PulseChannel {
        enabled: false,
        dac_enabled: false,
        sweep: 0,
        length_and_duty: 0,
        wave_duty_position: 0,
        envelope: initialize_envelope(),
        period: initalize_period(),
    } 
}

const MAX_WAVEFORM_STEPS: u8 = 7;
const PERIOD_HIGH_TRIGGER_INDEX: u8 = 7;

pub fn step(channel: &mut PulseChannel, last_instruction_clock_cycles: u8) {
    if channel.enabled {
        let mut period_divider_increment = (last_instruction_clock_cycles / 4) as u16;
        while period_divider_increment > 0 {
            channel.period.divider -= 1;
            if channel.period.divider == 0 {
                channel.period.divider = calculate_period_divider(&channel.period);
                channel.wave_duty_position = bounded_wrapping_add(channel.wave_duty_position, MAX_WAVEFORM_STEPS)
            }
            period_divider_increment -= 1;
        } 
    }
}

pub fn trigger(channel: &mut PulseChannel) {
    channel.enabled = true;
    envelope::trigger(&mut channel.envelope);
}

pub fn should_trigger(channel: &PulseChannel) -> bool {
   channel.dac_enabled && is_bit_set(channel.period.high, PERIOD_HIGH_TRIGGER_INDEX)
}