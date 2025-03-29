use crate::emulator::{is_cgb, Emulator};
use crate::utils::is_bit_set;
use bincode::{Encode, Decode};

#[derive(Clone, Encode, Decode)]
pub struct SerialState {
    pub data: u8,
    pub clock: u16,
    pub is_high_speed_clock: bool,
    pub is_master: bool,
    pub transfer_enabled: bool,
    pub bits_transferred: u8,
}

fn serial_disconnected_exchange(_: bool) -> bool {
    // When no device is connected, it will always read 1 for each bit transfer,
    // which means it will receive 0xFF bytes.
   true
}

pub fn initialize_serial() -> SerialState {
    SerialState {
        data: 0,
        clock: 0,
        is_high_speed_clock: false,
        is_master: false,
        transfer_enabled: false,
        bits_transferred: 0,
    }
}

fn get_m_cycle_clock_rate(emulator: &Emulator) -> u16 {
    if emulator.serial.is_high_speed_clock && is_cgb(emulator) {
        if emulator.speed_switch.cgb_double_speed { 8 } else { 16 }
    } else {
        if emulator.speed_switch.cgb_double_speed { 256 } else { 512 }
    }
}

fn fire_serial_interrupt(emulator: &mut Emulator) {
    emulator.interrupts.flags |= 0x8;
}

fn exchange_bits(emulator: &mut Emulator) {
    let outgoing_bit = is_bit_set(emulator.serial.data, 7);
    emulator.serial.data <<= 1;
    let incoming_bit = serial_disconnected_exchange(outgoing_bit);
    if incoming_bit {
        emulator.serial.data |= 1;
    }
}

/*
    This is a very bare bones serial implementation that always assumes there is no
    serial device connected and doesn't know how to operate in slave mode.
*/
pub fn step(emulator: &mut Emulator) {
    if emulator.serial.transfer_enabled && emulator.serial.is_master {
        emulator.serial.clock += 1;

        let clock_rate = get_m_cycle_clock_rate(emulator);
        if emulator.serial.clock >= clock_rate {
            emulator.serial.clock = 0;

            exchange_bits(emulator);

            emulator.serial.bits_transferred += 1;
            if emulator.serial.bits_transferred >= 8 {
                emulator.serial.transfer_enabled = false;
                emulator.serial.bits_transferred = 0;
                fire_serial_interrupt(emulator);
            }
        }
    }
}

pub fn get_data(emulator: &Emulator) -> u8 {
    emulator.serial.data
}

pub fn get_control(emulator: &Emulator) -> u8 {
    let transfer_enabled_bit = if emulator.serial.transfer_enabled { 1 } else { 0 };
    let high_speed_clock_bit = if emulator.serial.is_high_speed_clock { 1 } else { 0 };
    let master_bit = if emulator.serial.is_master { 1 } else { 0 };
    (transfer_enabled_bit << 7)
        | (high_speed_clock_bit << 1)
        | master_bit
}

pub fn set_data(emulator: &mut Emulator, value: u8) {
    emulator.serial.data = value;
}

pub fn set_control(emulator: &mut Emulator, value: u8) {
    emulator.serial.transfer_enabled = is_bit_set(value, 7);
    if is_cgb(emulator) {
        emulator.serial.is_high_speed_clock = is_bit_set(value, 1);
    }
    emulator.serial.is_master = is_bit_set(value, 0);
}

#[cfg(test)]
mod tests {
    use crate::emulator::{initialize_screenless_emulator, Mode};
    use super::*;

    #[test]
    fn should_get_control_byte() {
        let mut emulator = initialize_screenless_emulator();
        emulator.mode = Mode::CGB;
        emulator.serial.is_high_speed_clock = true;
        emulator.serial.is_master = true;
        emulator.serial.transfer_enabled = true;
        assert_eq!(get_control(&emulator), 0x83);
    }

    #[test]
    fn should_get_control_byte_when_high_speed_clock_disabled() {
        let mut emulator = initialize_screenless_emulator();
        emulator.mode = Mode::CGB;
        emulator.serial.is_high_speed_clock = false;
        emulator.serial.is_master = true;
        emulator.serial.transfer_enabled = true;
        assert_eq!(get_control(&emulator), 0x81);
    }

    #[test]
    fn should_set_control_byte() {
        let mut emulator = initialize_screenless_emulator();
        emulator.mode = Mode::CGB;
        set_control(&mut emulator, 0x83);
        assert_eq!(emulator.serial.is_high_speed_clock, true);
        assert_eq!(emulator.serial.is_master, true);
        assert_eq!(emulator.serial.transfer_enabled, true);
    }

    #[test]
    fn should_not_be_able_to_set_high_speed_clock_in_monochrome_mode() {
        let mut emulator = initialize_screenless_emulator();
        emulator.mode = Mode::DMG;
        set_control(&mut emulator, 0x83);
        assert_eq!(emulator.serial.is_high_speed_clock, false);
    }

    #[test]
    fn should_increment_clock_while_transfer_in_progress() {
        let mut emulator = initialize_screenless_emulator();
        emulator.serial.transfer_enabled = true;
        emulator.serial.is_master = true;
        emulator.serial.clock = 0;
        step(&mut emulator);
        assert_eq!(emulator.serial.clock, 1);
    }

    #[test]
    fn should_transfer_bit_when_512_cycles_have_passed() {
        let mut emulator = initialize_screenless_emulator();
        emulator.serial.transfer_enabled = true;
        emulator.serial.is_master = true;
        emulator.serial.data = 0b10011010;
        emulator.serial.clock = 511;
        step(&mut emulator);
        assert_eq!(emulator.serial.clock, 0);
        assert_eq!(emulator.serial.data, 0b00110101);
    }

    #[test]
    fn should_transfer_bit_when_16_cycles_have_passed_if_high_speed_clock_enabled() {
        let mut emulator = initialize_screenless_emulator();
        emulator.mode = Mode::CGB;
        emulator.serial.is_high_speed_clock = true;
        emulator.serial.transfer_enabled = true;
        emulator.serial.is_master = true;
        emulator.serial.data = 0b10011010;
        emulator.serial.clock = 15;
        step(&mut emulator);
        assert_eq!(emulator.serial.clock, 0);
        assert_eq!(emulator.serial.data, 0b00110101);
    }

    #[test]
    fn should_transfer_bit_when_8_cycles_have_passed_if_high_speed_clock_and_double_speed_mode_are_enabled() {
        let mut emulator = initialize_screenless_emulator();
        emulator.mode = Mode::CGB;
        emulator.speed_switch.cgb_double_speed = true;
        emulator.serial.is_high_speed_clock = true;
        emulator.serial.transfer_enabled = true;
        emulator.serial.is_master = true;
        emulator.serial.data = 0b10011010;
        emulator.serial.clock = 7;
        step(&mut emulator);
        assert_eq!(emulator.serial.clock, 0);
        assert_eq!(emulator.serial.data, 0b00110101);
    }

    #[test]
    fn should_complete_transfer_and_fire_interrupt_when_8_bits_have_been_transferred() {
        let mut emulator = initialize_screenless_emulator();
        emulator.serial.transfer_enabled = true;
        emulator.serial.is_master = true;
        emulator.serial.bits_transferred = 7;
        emulator.serial.clock = 511;
        step(&mut emulator);
        assert_eq!(emulator.serial.transfer_enabled, false);
        assert_eq!(emulator.interrupts.flags, 0x08);
    }
}