use gameboy_emulator;
use std::io;

fn main() -> io::Result<()> {
    let cpu_state = gameboy_emulator::cpu::initialize_cpu_state();
    let filepath = "/Users/samuelparsons/development/gb-test-roms/cpu_instrs/cpu_instrs.gb";

    let updated_cpu_state = gameboy_emulator::cpu::load_rom_by_filepath(cpu_state, filepath)
        .expect("An error occurred when trying to load the ROM.");

    println!("{:?} {:?} {:?}", updated_cpu_state.registers, updated_cpu_state.clock, updated_cpu_state.memory.rom);
    
    Ok(())
}
