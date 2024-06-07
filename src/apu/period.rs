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

pub fn calculate_period_value(period: &Period) -> u16 {
    let period_high_bits = (period.high & 0b111) as u16;
    let period_low_bits = period.low as u16;
    (period_high_bits << 8) | period_low_bits
}

pub fn calculate_period_divider(period: &Period) -> u16 {
    2048 - calculate_period_value(period)
}