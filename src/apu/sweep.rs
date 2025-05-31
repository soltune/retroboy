use crate::utils::is_bit_set;
use crate::apu::period::Period;
use bincode::{Encode, Decode};

#[derive(Clone, Debug, Encode, Decode)]
pub struct Sweep {
    initial_settings: u8,
    enabled: bool,
    shadow_frequency: u16,
    timer: u8,
    frequency_calculated: bool,
    should_disable_channel: bool
}

const SWEEP_DIRECTION_INDEX: u8 = 3;

impl Sweep {
    pub fn new() -> Self {
        Sweep {
            initial_settings: 0,
            enabled: false,
            shadow_frequency: 0,
            timer: 0,
            frequency_calculated: false,
            should_disable_channel: false
        }
    }

    pub fn initial_sweep_shift(&self) -> u8 {
        self.initial_settings & 0b111
    }

    pub fn initial_sweep_period(&self) -> u8 {
        (self.initial_settings & 0b01110000) >> 4
    }

    pub fn calculate_frequency(&mut self) -> u16 {
        let sweep_shift = self.initial_sweep_shift();
        let mut new_frequency = self.shadow_frequency >> sweep_shift;

        let is_decrementing = is_bit_set(self.initial_settings, SWEEP_DIRECTION_INDEX);

        if is_decrementing {
            new_frequency = self.shadow_frequency - new_frequency;
        } else {
            new_frequency = self.shadow_frequency + new_frequency;
        }

        if new_frequency > 2047 {
            self.should_disable_channel = true;
        }
        else {
            self.frequency_calculated = true;
        }

        new_frequency
    }

    pub fn load_sweep_timer(&mut self, sweep_period: u8) {
        if sweep_period > 0 {
            self.timer = sweep_period;
        }
        else {
            self.timer = 8;
        } 
    }

    pub fn update_initial_settings(&mut self, new_initial_settings: u8) {
        let original_sweep_settings = self.initial_settings;
        self.initial_settings = new_initial_settings;

        let original_is_decrementing = is_bit_set(original_sweep_settings, SWEEP_DIRECTION_INDEX);
        let new_is_decrementing = is_bit_set(self.initial_settings, SWEEP_DIRECTION_INDEX);
        let exiting_negate_mode = original_is_decrementing && !new_is_decrementing;

        if exiting_negate_mode && self.frequency_calculated {
            self.should_disable_channel = true;
        }
    }

    pub fn should_disable_channel(&self) -> bool {
        self.should_disable_channel
    }

    pub fn step(&mut self, period: &mut Period) {
        if self.timer > 0 {
            self.timer -= 1;
        }

        if self.timer == 0 {
            let sweep_period = self.initial_sweep_period();
            self.load_sweep_timer(sweep_period);

            if self.enabled && sweep_period > 0 {
                let new_frequency = self.calculate_frequency();

                if new_frequency <= 2047 && self.initial_sweep_shift() > 0 {
                    self.shadow_frequency = new_frequency;

                    let low_bits = (new_frequency & 0b11111111) as u8;
                    let high_bits = ((new_frequency & 0b11100000000) >> 8) as u8;

                    period.set_low(low_bits);

                    let current_high = period.high();
                    period.set_high((current_high & 0b11111000) | high_bits);
                    
                    self.calculate_frequency();
                }
            }
            else {
                self.frequency_calculated = false;
            }
        }
    }

    pub fn trigger(&mut self, period: &Period) {
        self.shadow_frequency = period.calculate_period_value();

        let sweep_period = self.initial_sweep_period();
        self.load_sweep_timer(sweep_period);

        let sweep_shift = self.initial_sweep_shift();

        self.enabled = sweep_period > 0 || sweep_shift > 0;

        self.should_disable_channel = false;
        
        if sweep_shift > 0 {
            self.calculate_frequency();
        }
        else {
            self.frequency_calculated = false;
        }
    }

    pub fn initial_settings(&self) -> u8 {
        self.initial_settings
    }

    pub fn set_initial_settings(&mut self, initial_settings: u8) {
        self.initial_settings = initial_settings;
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn timer(&self) -> u8 {
        self.timer
    }

    pub fn set_timer(&mut self, timer: u8) {
        self.timer = timer;
    }

    pub fn shadow_frequency(&self) -> u16 {
        self.shadow_frequency
    }

    pub fn set_shadow_frequency(&mut self, shadow_frequency: u16) {
        self.shadow_frequency = shadow_frequency;
    }
}
