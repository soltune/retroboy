#[derive(Debug)]
pub struct Period {
    pub low: u8,
    pub high: u8,
    pub divider: u16
}

pub fn initalize_period() -> Period {
    Period {
        low: 0,
        high: 0,
        divider: 0
    }
}

pub fn step(period: &mut Period, last_instruction_clock_cycles: u8, cycle_rate: u8, mut handle_divider_reload: impl FnMut()) {
    let mut period_divider_increment = (last_instruction_clock_cycles / cycle_rate) as u16;
    while period_divider_increment > 0 {
        period.divider -= 1;
        if period.divider == 0 {
            period.divider = calculate_period_divider(&period);
            handle_divider_reload();
        }
        period_divider_increment -= 1;
    } 
}

pub fn calculate_period_value(period: &Period) -> u16 {
    let period_high_bits = (period.high & 0b111) as u16;
    let period_low_bits = period.low as u16;
    (period_high_bits << 8) | period_low_bits
}

pub fn calculate_period_divider(period: &Period) -> u16 {
    2048 - calculate_period_value(period)
}