use gameboy_emulator;

fn main() {
    let result = gameboy_emulator::initialize_cpu_state();
    println!("{:?} {:?}", result.registers, result.clock)
}
