use crate::emulator::Emulator;
use crate::utils::{reset_bit, set_bit};
use crate::wasm::Key;

#[derive(Debug)]
pub struct KeyState {
    pub column: u8,
    pub select_buttons: u8,
    pub directional_buttons: u8
}

const DOWN_BIT: u8 = 3;
const UP_BIT: u8 = 2;
const LEFT_BIT: u8 = 1;
const RIGHT_BIT: u8 = 0;

const START_BIT: u8 = 3;
const SELECT_BIT: u8 = 2;
const B_BIT: u8 = 1;
const A_BIT: u8 = 0;

pub fn initialize_keys() -> KeyState {
    KeyState {
        column: 0x0,
        select_buttons: 0xF,
        directional_buttons: 0xF
    }
}

pub fn write_joyp_byte(key_state: &mut KeyState, value: u8) {
    key_state.column = value & 0x30;
}

pub fn read_joyp_byte(key_state: &KeyState) -> u8 {
    if key_state.column == 0x20 {
        0x20 | key_state.directional_buttons
    }
    else if key_state.column == 0x10 {
        0x10 | key_state.select_buttons
    }
    else {
        0x3F
    }
}

fn fire_joyp_interrupt(emulator: &mut Emulator) {
    emulator.interrupts.flags |= 0x10;
}

pub fn handle_key_press(emulator: &mut Emulator, key: &Key) {
    match key {
        Key::Down =>
            emulator.keys.directional_buttons = reset_bit(emulator.keys.directional_buttons, DOWN_BIT),
        Key::Up =>
            emulator.keys.directional_buttons = reset_bit(emulator.keys.directional_buttons, UP_BIT),
        Key::Left =>
            emulator.keys.directional_buttons = reset_bit(emulator.keys.directional_buttons, LEFT_BIT),
        Key::Right =>
            emulator.keys.directional_buttons = reset_bit(emulator.keys.directional_buttons, RIGHT_BIT),
        Key::Start =>
            emulator.keys.select_buttons = reset_bit(emulator.keys.select_buttons, START_BIT),
        Key::Select =>
            emulator.keys.select_buttons = reset_bit(emulator.keys.select_buttons, SELECT_BIT),
        Key::B =>
            emulator.keys.select_buttons = reset_bit(emulator.keys.select_buttons, B_BIT),
        Key::A =>
            emulator.keys.select_buttons = reset_bit(emulator.keys.select_buttons, A_BIT),
    }
    
    fire_joyp_interrupt(emulator);
}

pub fn handle_key_release(emulator: &mut Emulator, key: &Key) {
    match key {
        Key::Down =>
            emulator.keys.directional_buttons = set_bit(emulator.keys.directional_buttons, DOWN_BIT),
        Key::Up =>
            emulator.keys.directional_buttons = set_bit(emulator.keys.directional_buttons, UP_BIT),
        Key::Left =>
            emulator.keys.directional_buttons = set_bit(emulator.keys.directional_buttons, LEFT_BIT),
        Key::Right =>
            emulator.keys.directional_buttons = set_bit(emulator.keys.directional_buttons, RIGHT_BIT),
        Key::Start =>
            emulator.keys.select_buttons = set_bit(emulator.keys.select_buttons, START_BIT),
        Key::Select =>
            emulator.keys.select_buttons = set_bit(emulator.keys.select_buttons, SELECT_BIT),
        Key::B =>
            emulator.keys.select_buttons = set_bit(emulator.keys.select_buttons, B_BIT),
        Key::A =>
            emulator.keys.select_buttons = set_bit(emulator.keys.select_buttons, A_BIT),
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::initialize_screenless_emulator;

    use super::*;

    #[test]
    fn reads_from_directional_keys() {
        let state = KeyState { column: 0x20, directional_buttons: 0x4, select_buttons: 0x2 };
        let result = read_joyp_byte(&state);
        assert_eq!(result, 0x24);
    }

    #[test]
    fn reads_from_select_keys() {
        let state = KeyState { column: 0x10, directional_buttons: 0x4, select_buttons: 0x2 };
        let result = read_joyp_byte(&state);
        assert_eq!(result, 0x12);
    }

    #[test]
    fn writes_to_joyp() {
        let mut state = KeyState { column: 0x0, directional_buttons: 0x4, select_buttons: 0x2 };
        write_joyp_byte(&mut state, 0x20);
        assert_eq!(state.column, 0x20);
    }

    #[test]
    fn stores_down_key_press() {
        let mut emulator = initialize_screenless_emulator();
        emulator.keys = KeyState { column: 0x0, directional_buttons: 0xF, select_buttons: 0xF };
        handle_key_press(&mut emulator, &Key::Down);
        assert_eq!(emulator.keys.directional_buttons, 0x7);
    }

    #[test]
    fn stores_down_key_release() {
        let mut emulator = initialize_screenless_emulator();
        emulator.keys = KeyState { column: 0x0, directional_buttons: 0x7, select_buttons: 0xF };
        handle_key_release(&mut emulator, &Key::Down);
        assert_eq!(emulator.keys.directional_buttons, 0xF); 
    }

    #[test]
    fn stores_up_key_press() {
        let mut emulator = initialize_screenless_emulator();
        emulator.keys = KeyState { column: 0x0, directional_buttons: 0xF, select_buttons: 0xF };
        handle_key_press(&mut emulator, &Key::Up);
        assert_eq!(emulator.keys.directional_buttons, 0xB);
    }

    #[test]
    fn stores_up_key_release() {
        let mut emulator = initialize_screenless_emulator();
        emulator.keys = KeyState { column: 0x0, directional_buttons: 0xB, select_buttons: 0xF };
        handle_key_release(&mut emulator, &Key::Up);
        assert_eq!(emulator.keys.directional_buttons, 0xF); 
    }

    #[test]
    fn stores_left_key_press() {
        let mut emulator = initialize_screenless_emulator();
        emulator.keys = KeyState { column: 0x0, directional_buttons: 0xF, select_buttons: 0xF };
        handle_key_press(&mut emulator, &Key::Left);
        assert_eq!(emulator.keys.directional_buttons, 0xD);
    }

    #[test]
    fn stores_left_key_release() {
        let mut emulator = initialize_screenless_emulator();
        emulator.keys = KeyState { column: 0x0, directional_buttons: 0xD, select_buttons: 0xF };
        handle_key_release(&mut emulator, &Key::Left);
        assert_eq!(emulator.keys.directional_buttons, 0xF); 
    }

    #[test]
    fn stores_right_key_press() {
        let mut emulator = initialize_screenless_emulator();
        emulator.keys = KeyState { column: 0x0, directional_buttons: 0xF, select_buttons: 0xF };
        handle_key_press(&mut emulator, &Key::Right);
        assert_eq!(emulator.keys.directional_buttons, 0xE);
    }

    #[test]
    fn stores_right_key_release() {
        let mut emulator = initialize_screenless_emulator();
        emulator.keys = KeyState { column: 0x0, directional_buttons: 0xE, select_buttons: 0xF };
        handle_key_release(&mut emulator, &Key::Right);
        assert_eq!(emulator.keys.directional_buttons, 0x0F); 
    }

    #[test]
    fn stores_start_key_press() {
        let mut emulator = initialize_screenless_emulator();
        emulator.keys = KeyState { column: 0x0, directional_buttons: 0xF, select_buttons: 0xF };
        handle_key_press(&mut emulator, &Key::Start);
        assert_eq!(emulator.keys.select_buttons, 0x7);
    }

    #[test]
    fn stores_start_key_release() {
        let mut emulator = initialize_screenless_emulator();
        emulator.keys = KeyState { column: 0x0, directional_buttons: 0xF, select_buttons: 0x7 };
        handle_key_release(&mut emulator, &Key::Start);
        assert_eq!(emulator.keys.select_buttons, 0xF);
    }

    #[test]
    fn stores_select_key_press() {
        let mut emulator = initialize_screenless_emulator();
        emulator.keys = KeyState { column: 0x0, directional_buttons: 0xF, select_buttons: 0xF };
        handle_key_press(&mut emulator, &Key::Select);
        assert_eq!(emulator.keys.select_buttons, 0xB);
    }

    #[test]
    fn stores_select_key_release() {
        let mut emulator = initialize_screenless_emulator();
        emulator.keys = KeyState { column: 0x0, directional_buttons: 0xF, select_buttons: 0xB };
        handle_key_release(&mut emulator, &Key::Select);
        assert_eq!(emulator.keys.select_buttons, 0xF);
    }

    #[test]
    fn stores_b_key_press() {
        let mut emulator = initialize_screenless_emulator();
        emulator.keys = KeyState { column: 0x0, directional_buttons: 0xF, select_buttons: 0xF };
        handle_key_press(&mut emulator, &Key::B);
        assert_eq!(emulator.keys.select_buttons, 0xD);
    }

    #[test]
    fn stores_b_key_release() {
        let mut emulator = initialize_screenless_emulator();
        emulator.keys = KeyState { column: 0x0, directional_buttons: 0xF, select_buttons: 0xD };
        handle_key_release(&mut emulator, &Key::B);
        assert_eq!(emulator.keys.select_buttons, 0xF);
    }

    #[test]
    fn stores_a_key_press() {
        let mut emulator = initialize_screenless_emulator();
        emulator.keys = KeyState { column: 0x0, directional_buttons: 0xF, select_buttons: 0xF };
        handle_key_press(&mut emulator, &Key::A);
        assert_eq!(emulator.keys.select_buttons, 0xE);
    }

    #[test]
    fn stores_a_key_release() {
        let mut emulator = initialize_screenless_emulator();
        emulator.keys = KeyState { column: 0x0, directional_buttons: 0xF, select_buttons: 0xE };
        handle_key_release(&mut emulator, &Key::A);
        assert_eq!(emulator.keys.select_buttons, 0xF);
    }
}
