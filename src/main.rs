use gameboy_emulator::emulator::initialize_emulator_by_filepath;
use std::io;

fn main() -> io::Result<()> {
    let filepath = "/Users/samuelparsons/development/gb-test-roms/cpu_instrs/cpu_instrs.gb";
    let emulator = initialize_emulator_by_filepath(filepath)
        .expect("An error occurred when trying to load the ROM.");

    println!("{:?} {:?} {:?}", emulator.cpu.registers, emulator.cpu.clock, emulator.memory.rom);
    
    Ok(())
}
