use crate::emulator;
use crate::emulator::Emulator;
use crate::emulator::Mode;
use crate::emulator::CartridgeHeader;
use crate::keys;
use crate::keys::Key;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

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

    #[wasm_bindgen(js_name = canvasRender)]
    pub fn canvas_render(frame_buffer: &[u8]); 

    #[wasm_bindgen(js_name = playAudioSamples)]
    pub fn play_audio_samples(left_samples: &[f32], right_samples: &[f32]);
}

#[wasm_bindgen]
pub struct RomMetadata {
    title: String
}

#[wasm_bindgen]
impl RomMetadata {
    #[wasm_bindgen(getter)]
    pub fn title(&self) -> String {
        self.title.clone()
    }
}

fn initialize_web_emulator() -> Emulator {
    emulator::initialize_emulator(canvas_render)
}

thread_local! {
    pub static EMULATOR: RefCell<Emulator> = RefCell::new(initialize_web_emulator());
}

extern crate console_error_panic_hook;

fn as_mode(mode_text: &str) -> Mode {
    match mode_text {
        "DMG" => emulator::Mode::DMG,
        "CGB" => emulator::Mode::CGB,
        _ => panic!("Unsupported mode: {}", mode_text)
    }
}

fn as_rom_metadadta(header: CartridgeHeader) -> RomMetadata {
    RomMetadata {
        title: header.title
    }
}

#[wasm_bindgen(js_name = initializeEmulator)]
pub fn initialize_emulator(rom_buffer: &[u8], mode_text: &str) -> RomMetadata {
    EMULATOR.with(|emulator_cell: &RefCell<Emulator>| {
        console_error_panic_hook::set_once();

        let mut emulator = emulator_cell.borrow_mut();

        emulator::set_mode(&mut emulator, as_mode(mode_text));

        let cartridge_header = emulator::load_rom(&mut emulator, rom_buffer)
            .expect("An error occurred when trying to load the ROM."); 

        log("Emulator initialized!");

        as_rom_metadadta(cartridge_header)
    })
}

#[wasm_bindgen(js_name = resetEmulator)]
pub fn reset_emulator() {
    EMULATOR.with(|emulator_cell| {
         emulator_cell.replace(initialize_web_emulator());
    })
}

#[wasm_bindgen(js_name = stepUntilNextAudioBuffer)]
pub fn step_until_next_audio_buffer() {
    EMULATOR.with(|emulator_cell| {
        let mut emulator = emulator_cell.borrow_mut();

        let audio_buffers = emulator::step_until_next_audio_buffer(&mut emulator);
        let left_samples_slice = audio_buffers.0;
        let right_samples_slice = audio_buffers.1;

        play_audio_samples(left_samples_slice, right_samples_slice);
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
                keys::handle_key_press(&mut emulator, &key);
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
                keys::handle_key_release(&mut emulator, &key);
            })
        }
    );
}