mod cpu;
mod mmu;

pub use cpu::initialize_cpu_state;

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = initialize_cpu_state();
    //     assert_eq!(result, 4);
    // }
}
