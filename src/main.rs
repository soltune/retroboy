use gameboy_emulator::{emulator::{initialize_emulator_by_filepath, step, Emulator}, keys::{detect_key_presses, detect_key_releases}};
use std::io;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

fn open_gameboy_emulator_window(emulator: &mut Emulator) {
    let mut window = Window::new(
        "Gameboy Emulator",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create window");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        detect_key_presses(&mut emulator.keys, &window);
        detect_key_releases(&mut emulator.keys, &window);

        let minifb_renderer = |buffer: &Vec<u32>| {
            window
                .update_with_buffer(&buffer, WIDTH, HEIGHT)
                .expect("Unable to update window"); 
        };

        step(emulator, minifb_renderer);
    }
}

fn main() -> io::Result<()> {
    let rom_filepath = "/Users/samuelparsons/development/gb-test-roms/cpu_instrs/individual/08-misc instrs.gb";
    let bios_filepath = "/Users/samuelparsons/development/rusty-gameboy-emulator/bios.bin";
    let mut emulator = initialize_emulator_by_filepath(rom_filepath, bios_filepath)
        .expect("An error occurred when trying to load the ROM");

    open_gameboy_emulator_window(&mut emulator);
    
    Ok(())
}
