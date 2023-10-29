mod cpu;
mod mmu;

pub use cpu::initialize_cpu_state;
pub use cpu::load_rom_by_filepath;

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = initialize_cpu_state();
    //     assert_eq!(result, 4);
    // }
}
