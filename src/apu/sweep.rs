use crate::utils::is_bit_set;
use crate::apu::period::calculate_period_value;
use crate::apu::pulse::{disable, PulseChannel};

#[derive(Debug)]
pub struct Sweep {
    pub initial_settings: u8,
    pub enabled: bool,
    pub shadow_frequency: u16,
    pub timer: u8
}

pub fn initialize_sweep() -> Sweep {
    Sweep {
        initial_settings: 0,
        enabled: false,
        shadow_frequency: 0,
        timer: 0
    }
}

const SWEEP_DIRECTION_INDEX: u8 = 3;

fn initial_sweep_shift(sweep: &Sweep) -> u8 {
    sweep.initial_settings & 0b111
}

fn initial_sweep_period(sweep: &Sweep) -> u8 {
    (sweep.initial_settings & 0b01110000) >> 4 
}

pub fn calculate_frequency(channel: &mut PulseChannel) -> u16 {
    let sweep_shift = initial_sweep_shift(&channel.sweep);
    let mut new_frequency = channel.sweep.shadow_frequency >> sweep_shift;

    let is_decrementing = is_bit_set(channel.sweep.initial_settings, SWEEP_DIRECTION_INDEX);

    if is_decrementing {
        new_frequency = channel.sweep.shadow_frequency - new_frequency;
    } else {
        new_frequency = channel.sweep.shadow_frequency + new_frequency;
    }

    if new_frequency > 2047 {
        disable(channel);
    }

    new_frequency
}

pub fn load_sweep_timer(channel: &mut PulseChannel, sweep_period: u8) {
    if sweep_period > 0 {
        channel.sweep.timer = sweep_period;
    }
    else {
        channel.sweep.timer = 8;
    } 
}

pub fn step(channel: &mut PulseChannel) {
    if channel.sweep.timer > 0 {
        channel.sweep.timer -= 1;
    }

    if channel.sweep.timer == 0 {
        let sweep_period = initial_sweep_period(&channel.sweep);
        load_sweep_timer(channel, sweep_period);

        if channel.sweep.enabled && sweep_period > 0 {
            let new_frequency = calculate_frequency(channel);
            let sweep_shift = initial_sweep_shift(&channel.sweep);

            if new_frequency <= 2047 && sweep_shift > 0 {
                channel.sweep.shadow_frequency = new_frequency;

                let low_bits = (new_frequency & 0b11111111) as u8;
                let high_bits = ((new_frequency & 0b11100000000) >> 8) as u8;

                channel.period.low = low_bits;
                channel.period.high = (channel.period.high & 0b11111000) | high_bits;
                
                calculate_frequency(channel);
            }
        }
    }
}

pub fn trigger(channel: &mut PulseChannel) {
    channel.sweep.shadow_frequency = calculate_period_value(&channel.period);

    let sweep_period = initial_sweep_period(&channel.sweep);
    load_sweep_timer(channel, sweep_period);

    let sweep_shift = initial_sweep_shift(&channel.sweep);
    if sweep_period > 0 || sweep_shift > 0 {
        channel.sweep.enabled = true;
    }
    
    if sweep_shift > 0 {
        calculate_frequency(channel);
    }
}