use gameboy_emulator::emulator::initialize_emulator_by_filepath;
use std::io;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

fn open_window_and_render_pixel() {
    let mut window = Window::new(
        "Rust Pixel Rendering",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create window");

    let pixel_x = 80;
    let pixel_y = 72;

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    buffer[pixel_y * WIDTH + pixel_x] = 0xFFFFFF;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .expect("Unable to update window");

        // Handle input events or perform other tasks as needed

        // You can also sleep for a short duration to control the rendering speed
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

fn main() -> io::Result<()> {
    let rom_filepath = "/Users/samuelparsons/development/gb-test-roms/GBTICTAC.gb";
    let bios_filepath = "/Users/samuelparsons/development/rusty-gameboy-emulator/bios.bin";
    let emulator = initialize_emulator_by_filepath(rom_filepath, bios_filepath)
        .expect("An error occurred when trying to load the ROM");

    println!("{:?} {:?} {:?}", emulator.cpu.registers, emulator.cpu.clock, emulator.memory.rom);

    for byte in &emulator.memory.bios {
        println!("{:02X}", byte); // Print each byte as hexadecimal
    }
    
    open_window_and_render_pixel();
    
    Ok(())
}
