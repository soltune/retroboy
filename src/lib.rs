use emulator::Emulator;
use keys::Key;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

thread_local! {
    pub static EMULATOR: RefCell<Emulator> = RefCell::new(emulator::initialize_emulator());
}

const DOWN_KEY_CODE: u8 = 40;
const UP_KEY_CODE: u8 = 38;
const LEFT_KEY_CODE: u8 = 37;
const RIGHT_KEY_CODE: u8 = 39;

const START_KEY_CODE: u8 = 13;
const SELECT_KEY_CODE: u8 = 32;
const B_KEY_CODE: u8 = 88;
const A_KEY_CODE: u8 = 90;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    pub fn render(frame_buffer: &[u8]); 
}

#[wasm_bindgen]
pub fn load_rom(rom_buffer: &[u8]) {
    EMULATOR.with(|emulator_cell| {
        let emulator = emulator_cell.borrow_mut();

        emulator::load_rom(emulator, rom_buffer)
            .expect("An error occurred when trying to load the ROM.");
        
        log("ROM loaded!");
    });
}

#[wasm_bindgen]
pub fn load_bios(bios_buffer: &[u8]) {
    EMULATOR.with(|emulator_cell| {
        let emulator = emulator_cell.borrow_mut();

        emulator::load_bios(emulator, bios_buffer);
        
        log("BIOS loaded!")
    })
}

#[wasm_bindgen]
pub fn step_frame() {
    EMULATOR.with(|emulator_cell| {
        let mut emulator = emulator_cell.borrow_mut();

        let mut gpu_render = false;
        loop {
            emulator::step(&mut emulator, |buffer: &Vec<u8>| {
                render(buffer.as_slice());
                gpu_render = true;
             });
             if gpu_render {
                break;
             }
        }
    })
}

fn as_maybe_key(key_code: u8) -> Option<Key> {
    match key_code {
        DOWN_KEY_CODE => Some(Key::Down),
        UP_KEY_CODE => Some(Key::Up),
        LEFT_KEY_CODE => Some(Key::Left),
        RIGHT_KEY_CODE => Some(Key::Right),
        START_KEY_CODE => Some(Key::Enter),
        SELECT_KEY_CODE => Some(Key::Space),
        B_KEY_CODE => Some(Key::X),
        A_KEY_CODE => Some(Key::Z),
        _ => None
    }
}

#[wasm_bindgen]
pub fn press_key(key_code: u8) {
    let maybe_key = as_maybe_key(key_code);

    maybe_key.map(
        |key| {
            EMULATOR.with(|emulator_cell| {
                let mut emulator = emulator_cell.borrow_mut();
                keys::handle_key_press(&mut emulator.keys, &key);
            })
        }
    );
}

#[wasm_bindgen]
pub fn release_key(key_code: u8) {
    let maybe_key = as_maybe_key(key_code);

    maybe_key.map(
        |key| {
            EMULATOR.with(|emulator_cell| {
                let mut emulator = emulator_cell.borrow_mut();
                keys::handle_key_release(&mut emulator.keys, &key);
            })
        }
    );
}

pub mod cpu;
pub mod mmu;
pub mod gpu;
pub mod utils;
pub mod keys;
pub mod emulator;
