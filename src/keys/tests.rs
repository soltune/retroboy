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

