use crate::utils::{reset_bit, set_bit};

#[derive(Debug)]
pub struct KeyState {
    pub column: u8,
    pub select_buttons: u8,
    pub directional_buttons: u8
}

pub enum Key {
    Z,
    X,
    Enter,
    Space,
    Down,
    Up,
    Left,
    Right
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
        key_state.directional_buttons
    }
    else if key_state.column == 0x10 {
        key_state.select_buttons
    }
    else {
        0x0
    }
}

pub fn handle_key_press(key_state: &mut KeyState, key: &Key) {
    match key {
        Key::Down =>
            key_state.directional_buttons = reset_bit(key_state.directional_buttons, DOWN_BIT),
        Key::Up =>
            key_state.directional_buttons = reset_bit(key_state.directional_buttons, UP_BIT),
        Key::Left =>
            key_state.directional_buttons = reset_bit(key_state.directional_buttons, LEFT_BIT),
        Key::Right =>
            key_state.directional_buttons = reset_bit(key_state.directional_buttons, RIGHT_BIT),
        Key::Enter =>
            key_state.select_buttons = reset_bit(key_state.select_buttons, START_BIT),
        Key::Space =>
            key_state.select_buttons = reset_bit(key_state.select_buttons, SELECT_BIT),
        Key::X =>
            key_state.select_buttons = reset_bit(key_state.select_buttons, B_BIT),
        Key::Z =>
            key_state.select_buttons = reset_bit(key_state.select_buttons, A_BIT),
    }
}

pub fn handle_key_release(key_state: &mut KeyState, key: &Key) {
    match key {
        Key::Down =>
            key_state.directional_buttons = set_bit(key_state.directional_buttons, DOWN_BIT),
        Key::Up =>
            key_state.directional_buttons = set_bit(key_state.directional_buttons, UP_BIT),
        Key::Left =>
            key_state.directional_buttons = set_bit(key_state.directional_buttons, LEFT_BIT),
        Key::Right =>
            key_state.directional_buttons = set_bit(key_state.directional_buttons, RIGHT_BIT),
        Key::Enter =>
            key_state.select_buttons = set_bit(key_state.select_buttons, START_BIT),
        Key::Space =>
            key_state.select_buttons = set_bit(key_state.select_buttons, SELECT_BIT),
        Key::X =>
            key_state.select_buttons = set_bit(key_state.select_buttons, B_BIT),
        Key::Z =>
            key_state.select_buttons = set_bit(key_state.select_buttons, A_BIT),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_from_directional_keys() {
        let state = KeyState { column: 0x20, directional_buttons: 0x4, select_buttons: 0x2 };
        let result = read_joyp_byte(&state);
        assert_eq!(result, 0x4);
    }

    #[test]
    fn reads_from_select_keys() {
        let state = KeyState { column: 0x10, directional_buttons: 0x4, select_buttons: 0x2 };
        let result = read_joyp_byte(&state);
        assert_eq!(result, 0x2);
    }

    #[test]
    fn writes_to_joyp() {
        let mut state = KeyState { column: 0x0, directional_buttons: 0x4, select_buttons: 0x2 };
        write_joyp_byte(&mut state, 0x20);
        assert_eq!(state.column, 0x20);
    }

    #[test]
    fn stores_down_key_press() {
        let mut state = KeyState { column: 0x0, directional_buttons: 0xF, select_buttons: 0xF };
        handle_key_press(&mut state, &Key::Down);
        assert_eq!(state.directional_buttons, 0x7);
    }

    #[test]
    fn stores_down_key_release() {
        let mut state = KeyState { column: 0x0, directional_buttons: 0x7, select_buttons: 0xF };
        handle_key_release(&mut state, &Key::Down);
        assert_eq!(state.directional_buttons, 0xF); 
    }

    #[test]
    fn stores_up_key_press() {
        let mut state = KeyState { column: 0x0, directional_buttons: 0xF, select_buttons: 0xF };
        handle_key_press(&mut state, &Key::Up);
        assert_eq!(state.directional_buttons, 0xB);
    }

    #[test]
    fn stores_up_key_release() {
        let mut state = KeyState { column: 0x0, directional_buttons: 0xB, select_buttons: 0xF };
        handle_key_release(&mut state, &Key::Up);
        assert_eq!(state.directional_buttons, 0xF); 
    }

    #[test]
    fn stores_left_key_press() {
        let mut state = KeyState { column: 0x0, directional_buttons: 0xF, select_buttons: 0xF };
        handle_key_press(&mut state, &Key::Left);
        assert_eq!(state.directional_buttons, 0xD);
    }

    #[test]
    fn stores_left_key_release() {
        let mut state = KeyState { column: 0x0, directional_buttons: 0xD, select_buttons: 0xF };
        handle_key_release(&mut state, &Key::Left);
        assert_eq!(state.directional_buttons, 0xF); 
    }

    #[test]
    fn stores_right_key_press() {
        let mut state = KeyState { column: 0x0, directional_buttons: 0xF, select_buttons: 0xF };
        handle_key_press(&mut state, &Key::Right);
        assert_eq!(state.directional_buttons, 0xE);
    }

    #[test]
    fn stores_right_key_release() {
        let mut state = KeyState { column: 0x0, directional_buttons: 0xE, select_buttons: 0xF };
        handle_key_release(&mut state, &Key::Right);
        assert_eq!(state.directional_buttons, 0x0F); 
    }

    #[test]
    fn stores_start_key_press() {
        let mut state = KeyState { column: 0x0, directional_buttons: 0xF, select_buttons: 0xF };
        handle_key_press(&mut state, &Key::Enter);
        assert_eq!(state.select_buttons, 0x7);
    }

    #[test]
    fn stores_start_key_release() {
        let mut state = KeyState { column: 0x0, directional_buttons: 0xF, select_buttons: 0x7 };
        handle_key_release(&mut state, &Key::Enter);
        assert_eq!(state.select_buttons, 0xF);
    }

    #[test]
    fn stores_select_key_press() {
        let mut state = KeyState { column: 0x0, directional_buttons: 0xF, select_buttons: 0xF };
        handle_key_press(&mut state, &Key::Space);
        assert_eq!(state.select_buttons, 0xB);
    }

    #[test]
    fn stores_select_key_release() {
        let mut state = KeyState { column: 0x0, directional_buttons: 0xF, select_buttons: 0xB };
        handle_key_release(&mut state, &Key::Space);
        assert_eq!(state.select_buttons, 0xF);
    }

    #[test]
    fn stores_b_key_press() {
        let mut state = KeyState { column: 0x0, directional_buttons: 0xF, select_buttons: 0xF };
        handle_key_press(&mut state, &Key::X);
        assert_eq!(state.select_buttons, 0xD);
    }

    #[test]
    fn stores_b_key_release() {
        let mut state = KeyState { column: 0x0, directional_buttons: 0xF, select_buttons: 0xD };
        handle_key_release(&mut state, &Key::X);
        assert_eq!(state.select_buttons, 0xF);
    }

    #[test]
    fn stores_a_key_press() {
        let mut state = KeyState { column: 0x0, directional_buttons: 0xF, select_buttons: 0xF };
        handle_key_press(&mut state, &Key::Z);
        assert_eq!(state.select_buttons, 0xE);
    }

    #[test]
    fn stores_a_key_release() {
        let mut state = KeyState { column: 0x0, directional_buttons: 0xF, select_buttons: 0xE };
        handle_key_release(&mut state, &Key::Z);
        assert_eq!(state.select_buttons, 0xF);
    }
}
