pub const ENTRY_POINT_ADDRESS: usize = 0x100;
pub const SGB_SUPPORT_ADDRESS: usize = 0x146;
pub const CARTRIDGE_TYPE_ADDRESS: usize = 0x147;
pub const ROM_SIZE_ADDRESS: usize = 0x148;
pub const RAM_SIZE_ADDRESS: usize = 0x149;

pub const CART_TYPE_ROM_ONLY: u8 = 0x0;
pub const CART_TYPE_MBC1: u8 = 0x1;
pub const CART_TYPE_MBC1_WITH_RAM: u8 = 0x2;
pub const CART_TYPE_MBC1_WITH_RAM_PLUS_BATTERY: u8 = 0x3;
pub const CART_TYPE_MBC3_TIMER_BATTERY: u8 = 0xF;
pub const CART_TYPE_MBC3_TIMER_RAM_BATTERY: u8 = 0x10;
pub const CART_TYPE_MBC3: u8 = 0x11;
pub const CART_TYPE_MBC3_RAM: u8 = 0x12;
pub const CART_TYPE_MBC3_RAM_BATTERY: u8 = 0x13;
pub const CART_TYPE_MBC5: u8 = 0x19;
pub const CART_TYPE_MBC5_RAM: u8 = 0x1A;
pub const CART_TYPE_MBC5_RAM_BATTERY: u8 = 0x1B;
pub const CART_TYPE_MBC5_RUMBLE: u8 = 0x1C;
pub const CART_TYPE_MBC5_RUMBLE_RAM: u8 = 0x1D;
pub const CART_TYPE_MBC5_RUMBLE_RAM_BATTERY: u8 = 0x1E;

pub const TITLE_START_ADDRESS: usize = 0x134;
pub const TITLE_END_ADDRESS: usize = 0x143;

pub const CGB_COMPATABILITY_INDEX: usize = 15;

pub const ROM_SIZE_32KB: u8 = 0x0;
pub const ROM_SIZE_64KB: u8 = 0x1;
pub const ROM_SIZE_128KB: u8 = 0x2;
pub const ROM_SIZE_256KB: u8 = 0x3;
pub const ROM_SIZE_512KB: u8 = 0x4;
pub const ROM_SIZE_1MB: u8 = 0x5;
pub const ROM_SIZE_2MB: u8 = 0x6;
pub const ROM_SIZE_4MB: u8 = 0x7;
pub const ROM_SIZE_8MB: u8 = 0x8;

pub const RAM_SIZE_0KB: u8 = 0x0;
pub const RAM_SIZE_2KB: u8 = 0x1;
pub const RAM_SIZE_8KB: u8 = 0x2;
pub const RAM_SIZE_32KB: u8 = 0x3;
pub const RAM_SIZE_128KB: u8 = 0x4;