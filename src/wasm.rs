use crate::emulator;
use crate::emulator::Emulator;
use crate::emulator::Mode;
use crate::emulator::CartridgeHeader;
use crate::keys::{self, Key};
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_name = canvasRender)]
    pub fn canvas_render(frame_buffer: &[u8]); 

    #[wasm_bindgen(js_name = playAudioSamples)]
    pub fn play_audio_samples(left_samples: &[f32], right_samples: &[f32]);
}

#[derive(Clone)]
#[wasm_bindgen]
pub struct RomMetadata {
    title: String,
    has_battery: bool
}

#[wasm_bindgen]
impl RomMetadata {
    #[wasm_bindgen(getter)]
    pub fn title(&self) -> String {
        self.title.clone()
    }

    #[wasm_bindgen(getter, js_name = hasBattery)]
    pub fn has_battery(&self) -> bool {
        self.has_battery
    }
}

#[wasm_bindgen]
pub struct RomMetadataResult {
    error: Option<String>,
    metadata: Option<RomMetadata>
}

#[wasm_bindgen]
impl RomMetadataResult {
    #[wasm_bindgen(getter)]
    pub fn error(&self) -> Option<String> {
        self.error.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn metadata(&self) -> Option<RomMetadata> {
        self.metadata.clone()
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

fn as_rom_metadata(header: CartridgeHeader) -> RomMetadata {
    RomMetadata {
        title: header.title,
        has_battery: header.has_battery
    }
}

#[wasm_bindgen(js_name = initializeEmulator)]
pub fn initialize_emulator(rom_buffer: &[u8], mode_text: &str) -> RomMetadataResult {
    EMULATOR.with(|emulator_cell: &RefCell<Emulator>| {
        console_error_panic_hook::set_once();

        let mut emulator = emulator_cell.borrow_mut();

        emulator::set_mode(&mut emulator, as_mode(mode_text));

        match emulator::load_rom(&mut emulator, rom_buffer) {
            Ok(result) => {
                log("Emulator initialized!");
                RomMetadataResult {
                    error: None,
                    metadata: Some(as_rom_metadata(result))
                }
            }
            Err(error) => {
                log(&format!("Error loading ROM: {}", error.to_string()));
                RomMetadataResult {
                    error: Some(error.to_string()),
                    metadata: None
                }
            }
        }
    })
}

#[wasm_bindgen(js_name = setCartridgeRam)]
pub fn set_cartridge_ram(ram: &[u8]) {
    EMULATOR.with(|emulator_cell| {
        let mut emulator = emulator_cell.borrow_mut();
        emulator::set_cartridge_ram(&mut emulator, ram);
    })
}

#[wasm_bindgen(js_name = getCartridgeRam)]
pub fn get_cartridge_ram() -> Vec<u8> {
    EMULATOR.with(|emulator_cell| {
        let emulator = emulator_cell.borrow();
        emulator::get_cartridge_ram(&emulator)
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

const UP_CODE: &str = "Up";
const DOWN_CODE: &str = "Down";
const LEFT_CODE: &str = "Left";
const RIGHT_CODE: &str = "Right";
const START_CODE: &str = "Start";
const SELECT_CODE: &str = "Select";
const B_CODE: &str = "B";
const A_CODE: &str = "A";

fn as_maybe_key(key_code: &str) -> Option<Key> {
    match key_code {
        UP_CODE => Some(Key::Up),
        DOWN_CODE => Some(Key::Down),
        LEFT_CODE => Some(Key::Left),
        RIGHT_CODE => Some(Key::Right),
        START_CODE => Some(Key::Start),
        SELECT_CODE => Some(Key::Select),
        B_CODE => Some(Key::B),
        A_CODE => Some(Key::A),
        _ => None
    }
}

#[wasm_bindgen(js_name = pressKey)]
pub fn press_key(key_code: &str) {
    as_maybe_key(key_code).map(|key| {
        EMULATOR.with(|emulator_cell| {
            let mut emulator = emulator_cell.borrow_mut();
            keys::handle_key_press(&mut emulator, &key);
        })
    });
}

#[wasm_bindgen(js_name = releaseKey)]
pub fn release_key(key_code: &str) {
    as_maybe_key(key_code).map(|key| {
        EMULATOR.with(|emulator_cell| {
            let mut emulator = emulator_cell.borrow_mut();
            keys::handle_key_release(&mut emulator, &key);
        })
    });
}
