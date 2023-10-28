// use gameboy_emulator;
use std::io;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("/Users/samuelparsons/development/gb-test-roms/cpu_instrs/cpu_instrs.gb")?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    // Read file into vector.
    reader.read_to_end(&mut buffer)?;

    // Read.
    let mut i = 0;
    for value in buffer {
        println!("BYTE: {}", value);
        i = i + 1;
        if (i >= 64) {
            break;
        }
    }
    Ok(())
    // let result = gameboy_emulator::initialize_cpu_state();
    // println!("{:?} {:?}", result.registers, result.clock)
}
