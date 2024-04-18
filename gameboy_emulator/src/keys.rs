use minifb::{Key, KeyRepeat, Window};

use crate::utils::{reset_bit, set_bit};

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

const KEYS: [Key; 8] = [Key::Down, Key::Up, Key::Left, Key::Right, Key::Enter, Key::Space, Key::X, Key::Z];

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
        _ => ()
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
        _ => ()
    }
}

pub fn detect_key_presses(key_state: &mut KeyState, window: &Window) {
    for key in KEYS.iter() {
        if window.is_key_pressed(*key, KeyRepeat::No) {
            handle_key_press(key_state, key);
        }
    }
}

pub fn detect_key_releases(key_state: &mut KeyState, window: &Window) {
    for key in KEYS.iter() {
        if window.is_key_released(*key) {
            handle_key_release(key_state, key);
        }
    }
}

#[cfg(test)]
mod tests;
