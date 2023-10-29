use gameboy_emulator;
use std::io;

fn main() -> io::Result<()> {
    // Test loading the ROM.

    let mut cpu_state = gameboy_emulator::initialize_cpu_state();
    let filepath = "/Users/samuelparsons/development/gb-test-roms/cpu_instrs/cpu_instrs.gb";

    gameboy_emulator::load_rom_by_filepath(& mut cpu_state, filepath)
        .expect("An error occurred when trying to load the ROM.");

    println!("{:?} {:?} {:?}", cpu_state.registers, cpu_state.clock, cpu_state.memory.rom);
    
    Ok(())
}
