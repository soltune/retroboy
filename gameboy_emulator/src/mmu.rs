#[derive(Debug)]
pub struct Memory {
    pub in_bios: bool,
    pub bios: [u8; 0x100],
    pub rom: [u8; 0x8000],
    pub video_ram: [u8; 0x2000],
    pub object_attribute_memory: [u8; 0xa0],
    pub working_ram: [u8; 0x3e00],
    pub external_ram: [u8; 0x2000],
    pub zero_page_ram: [u8; 0x80]
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = initialize_cpu_state();
    //     assert_eq!(result, 4);
    // }
}
