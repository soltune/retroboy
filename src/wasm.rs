use crate::emulator;
use crate::emulator::Emulator;
use crate::emulator::Mode;
use crate::emulator::CartridgeEffects;
use crate::emulator::CartridgeHeader;
use crate::emulator::RTCState;
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

    #[wasm_bindgen(js_name = currentTimeMillis)]
    pub fn current_time_millis() -> f64;

    #[wasm_bindgen(js_name = loadRTCState)]
    fn load_rtc_state(key: &str) -> Option<WasmRTCState>;

    #[wasm_bindgen(js_name = saveRTCState)]
    fn save_rtc_state(key: &str, value: WasmRTCState);

    #[wasm_bindgen(js_name = loadRam)]
    fn load_ram(key: &str) -> Option<Vec<u8>>;

    #[wasm_bindgen(js_name = saveRam)]
    fn save_ram(key: &str, value: &[u8]);
}

#[derive(Clone)]
#[wasm_bindgen]
pub struct EmulatorSettings {
    mode: String,
    audio_sample_rate: u32
}

#[wasm_bindgen]
impl EmulatorSettings {
    #[wasm_bindgen(constructor)]
    pub fn new(mode: String, audio_sample_rate: u32) -> EmulatorSettings {
        EmulatorSettings {
            mode,
            audio_sample_rate
        }
    }
    
    #[wasm_bindgen(getter)]
    pub fn mode(&self) -> String {
        self.mode.clone()
    }

    #[wasm_bindgen(getter, js_name = audioSampleRate)]
    pub fn audio_sample_rate(&self) -> u32 {
        self.audio_sample_rate
    }
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

#[wasm_bindgen]
pub struct WasmRTCState {
    milliseconds: u16,
    seconds: u8,
    minutes: u8,
    hours: u8,
    days: u16,
    base_timestamp: f64,
    halted: bool,
    day_carry: bool
}

#[wasm_bindgen]
impl WasmRTCState {
    #[wasm_bindgen(constructor)]
    pub fn new(
        milliseconds: u16,
        seconds: u8,
        minutes: u8,
        hours: u8,
        days: u16,
        base_timestamp: f64,
        halted: bool,
        day_carry: bool
    ) -> WasmRTCState {
        WasmRTCState {
            milliseconds,
            seconds,
            minutes,
            hours,
            days,
            base_timestamp,
            halted,
            day_carry
        }
    }

    #[wasm_bindgen(getter)]
    pub fn milliseconds(&self) -> u16 {
        self.milliseconds
    }

    #[wasm_bindgen(getter)]
    pub fn seconds(&self) -> u8 {
        self.seconds
    }

    #[wasm_bindgen(getter)]
    pub fn minutes(&self) -> u8 {
        self.minutes
    }

    #[wasm_bindgen(getter)]
    pub fn hours(&self) -> u8 {
        self.hours
    }

    #[wasm_bindgen(getter)]
    pub fn days(&self) -> u16 {
        self.days
    }

    #[wasm_bindgen(getter)]
    pub fn base_timestamp(&self) -> f64 {
        self.base_timestamp
    }

    #[wasm_bindgen(getter)]
    pub fn halted(&self) -> bool {
        self.halted
    }

    #[wasm_bindgen(getter)]
    pub fn day_carry(&self) -> bool {
        self.day_carry
    }
}

pub struct WasmCartridgeEffects;

impl CartridgeEffects for WasmCartridgeEffects {
    fn current_time_millis(&self) -> f64 {
        current_time_millis()
    }

    fn load_rtc_state(&self, key: &str) -> Option<RTCState> {
        load_rtc_state(key).map(|state| {
            RTCState {
                milliseconds: state.milliseconds() as u16,
                seconds: state.seconds(),
                minutes: state.minutes(),
                hours: state.hours(),
                days: state.days(),
                base_timestamp: state.base_timestamp(),
                halted: state.halted(),
                day_carry: state.day_carry()
            }
        })
    }

    fn save_rtc_state(&self, key: &str, state: &RTCState) {
        save_rtc_state(key, WasmRTCState {
            milliseconds: state.milliseconds as u16,
            seconds: state.seconds,
            minutes: state.minutes,
            hours: state.hours,
            days: state.days,
            base_timestamp: state.base_timestamp,
            halted: state.halted,
            day_carry: state.day_carry
        });
    }

    fn load_ram(&self, key: &str) -> Option<Vec<u8>> {
        load_ram(key)
    }

    fn save_ram(&self, key: &str, ram: &[u8]) {
        save_ram(key, ram);
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
pub fn initialize_emulator(rom_buffer: &[u8], settings: EmulatorSettings) -> RomMetadataResult {
    EMULATOR.with(|emulator_cell: &RefCell<Emulator>| {
        console_error_panic_hook::set_once();

        let mut emulator = emulator_cell.borrow_mut();

        emulator::set_mode(&mut emulator, as_mode(settings.mode.as_str()));

        emulator::set_sample_rate(&mut emulator, settings.audio_sample_rate);

        let wasm_cartridge_effects= WasmCartridgeEffects {};

        match emulator::load_rom(&mut emulator, rom_buffer, Box::new(wasm_cartridge_effects)) {
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
