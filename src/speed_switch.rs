use crate::emulator::{is_cgb, Emulator};
use crate::utils::is_bit_set;
use bincode::{Encode, Decode};

#[derive(Clone, Encode, Decode)]
pub struct SpeedSwitch {
    pub cgb_double_speed: bool,
    pub armed: bool
}

pub fn initialize_speed_switch() -> SpeedSwitch {
    SpeedSwitch {
        cgb_double_speed: false,
        armed: false
    }
}

const SPEED_SWITCH_ARMED_INDEX: u8 = 0;

pub fn get_key1(emulator: &Emulator) -> u8 {
    if is_cgb(emulator) {
        let double_speed_bit = if emulator.speed_switch.cgb_double_speed { 1 } else { 0 };
        let speed_switch_armed_bit = if emulator.speed_switch.armed { 1 } else { 0 };
        double_speed_bit << 7 | speed_switch_armed_bit
    }
    else {
        0xFF
    }
}

pub fn set_key1(emulator: &mut Emulator, value: u8) {
    if is_cgb(emulator) {
        emulator.speed_switch.armed = is_bit_set(value, SPEED_SWITCH_ARMED_INDEX);
    }
}

pub fn toggle(emulator: &mut Emulator) {
    if is_cgb(emulator) && emulator.speed_switch.armed {
        emulator.speed_switch.armed = false;

        let new_cgb_double_speed = !emulator.speed_switch.cgb_double_speed;
        emulator.speed_switch.cgb_double_speed = new_cgb_double_speed;
        emulator.apu.set_cgb_double_speed(new_cgb_double_speed);
        emulator.gpu.set_cgb_double_speed(new_cgb_double_speed);
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::{initialize_screenless_emulator, Mode};
    use super::*;

    #[test]
    fn should_read_from_key1_in_double_seed_mode() {
        let mut emulator = initialize_screenless_emulator();
        emulator.mode = Mode::CGB;
        emulator.speed_switch.cgb_double_speed = true;
        assert_eq!(get_key1(&emulator), 0x80);
    }

    #[test]
    fn should_read_from_key1_while_speed_switch_is_armed() {
        let mut emulator = initialize_screenless_emulator();
        emulator.mode = Mode::CGB;
        emulator.speed_switch.armed = true;
        assert_eq!(get_key1(&emulator), 0x1);
    }

    #[test]
    fn should_write_to_key1() {
        let mut emulator = initialize_screenless_emulator();
        emulator.mode = Mode::CGB;
        set_key1(&mut emulator, 0x1);
        assert_eq!(emulator.speed_switch.armed, true);
    }

    #[test]
    fn should_toggle_speed_switch() {
        let mut emulator = initialize_screenless_emulator();
        emulator.mode = Mode::CGB;
        emulator.speed_switch.armed = true;
        emulator.speed_switch.cgb_double_speed = false;
        toggle(&mut emulator);
        assert_eq!(emulator.speed_switch.armed, false);
        assert_eq!(emulator.speed_switch.cgb_double_speed, true);
    }

    #[test]
    fn should_not_toggle_if_not_armed() {
        let mut emulator = initialize_screenless_emulator();
        emulator.mode = Mode::CGB;
        emulator.speed_switch.armed = false;
        emulator.speed_switch.cgb_double_speed = false;
        toggle(&mut emulator);
        assert_eq!(emulator.speed_switch.armed, false);
        assert_eq!(emulator.speed_switch.cgb_double_speed, false);
    }
}