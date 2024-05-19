use emulator::Emulator;
use keys::Key;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

thread_local! {
    pub static EMULATOR: RefCell<Emulator> = RefCell::new(emulator::initialize_emulator());
}

const DOWN_KEY_CODE: &str = "ArrowDown";
const UP_KEY_CODE: &str = "ArrowUp";
const LEFT_KEY_CODE: &str = "ArrowLeft";
const RIGHT_KEY_CODE: &str = "ArrowRight";

const START_KEY_CODE: &str = "Enter";
const SELECT_KEY_CODE: &str = "Space";
const B_KEY_CODE: &str = "KeyX";
const A_KEY_CODE: &str = "KeyZ";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    pub fn render(frame_buffer: &[u8]); 
}

#[wasm_bindgen(js_name = initializeEmulator)]
pub fn initialize_emulator(rom_buffer: &[u8], bios_buffer: &[u8]) {
    EMULATOR.with(|emulator_cell| {
        let mut emulator = emulator_cell.borrow_mut();

        emulator::load_rom(&mut emulator, rom_buffer)
            .expect("An error occurred when trying to load the ROM."); 

        emulator::load_bios(&mut emulator, bios_buffer);

        log("Emulator initialized!");
    })
}

#[wasm_bindgen(js_name = initializeEmulatorWithoutBios)]
pub fn initialize_emulator_without_bios(rom_buffer: &[u8]) {
    EMULATOR.with(|emulator_cell| {
        let mut emulator = emulator_cell.borrow_mut();

        emulator::load_rom(&mut emulator, rom_buffer)
            .expect("An error occurred when trying to load the ROM."); 

        emulator::skip_bios(&mut emulator);

        log("Emulator initialized!");
    }) 
}

#[wasm_bindgen(js_name = resetEmulator)]
pub fn reset_emulator() {
    EMULATOR.with(|emulator_cell| {
         emulator_cell.replace(emulator::initialize_emulator());
    })
}

#[wasm_bindgen(js_name = stepFrame)]
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

fn as_maybe_key(key_code: &str) -> Option<Key> {
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

#[wasm_bindgen(js_name = pressKey)]
pub fn press_key(key_code: &str) {
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

#[wasm_bindgen(js_name = releaseKey)]
pub fn release_key(key_code: &str) {
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
