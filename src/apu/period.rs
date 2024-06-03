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