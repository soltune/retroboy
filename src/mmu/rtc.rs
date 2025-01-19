#[derive(Debug)]
pub struct RTC {
    pub halted: bool,
    pub seconds: u8,
    pub minutes: u8,
    pub hours: u8,
    pub days_lower: u8,
    pub days_upper: u8
}

pub fn empty_clock() -> RTC {
    RTC {
        halted: false,
        seconds: 0,
        minutes: 0,
        hours: 0,
        days_lower: 0,
        days_upper: 0
    }
}