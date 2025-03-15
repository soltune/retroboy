use std::collections::HashMap;
use std::io::{Error, ErrorKind, Result};

use crate::emulator::Emulator;
use crate::mmu;

pub struct Cheat {
    pub address: u16,
    pub new_data: u8,
    pub maybe_old_data: Option<u8>,
    pub maybe_bank: Option<u8>,
}

pub struct CheatState {
    pub registered: HashMap<String, Cheat>,
}

pub fn initialize_cheats() -> CheatState {
    CheatState {
        registered: HashMap::new(),
    }
}

fn parse_hex_byte(slice: &str, field: &str) -> Result<u8> {
    u8::from_str_radix(slice, 16)
        .map_err(|_|
            Error::new(
                ErrorKind::InvalidInput,
                format!("Invalid {} byte: {}", field, slice)))
}

fn parse_hex_word(slice: &str, field: &str) -> Result<u16> {
    u16::from_str_radix(slice, 16)
        .map_err(|_|
            Error::new(
                ErrorKind::InvalidInput,
                format!("Invalid {} word: {}", field, slice)))
}

pub fn parse_gameshark_code(gameshark_code: &str) -> Result<Cheat> {
    if gameshark_code.len() != 8 {
        Err(Error::new(
            ErrorKind::InvalidInput, 
            "Gameshark codes must be eight digits long."))
    }
    else {
        let parsed_bank = parse_hex_byte(&gameshark_code[..2], "bank")?;
        let parsed_new_data = parse_hex_byte(&gameshark_code[2..4], "new data")?;
        let parsed_addr_ls = parse_hex_byte(&gameshark_code[4..6], "address")?;
        let parsed_addr_ms = parse_hex_byte(&gameshark_code[6..8], "address")?;
    
        let address = ((parsed_addr_ms as u16) << 8) | (parsed_addr_ls as u16);

        if address < 0xA000 || address > 0xDFFF {
            Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Invalid address: {}", address)))
        }
        else {
            Ok(Cheat {
                address,
                new_data: parsed_new_data,
                maybe_old_data: None,
                maybe_bank: Some(parsed_bank),
            }) 
        }
    }
}

pub fn parse_gamegenie_code(gamegenie_code: &str) -> Result<Cheat> {
    if gamegenie_code.len() != 11 && gamegenie_code.len() != 7 {
        Err(Error::new(
            ErrorKind::InvalidInput, 
            "Game Genie codes must be either eleven or seven digits long."))
    }
    else {
        let without_dashes = gamegenie_code.replace("-", "");
        let stripped_code = if without_dashes.len() > 6 {
            format!("{}{}", &without_dashes[..7], &without_dashes[8..])
        }
        else {
            without_dashes
        };

        let parsed_new_data = parse_hex_byte(&stripped_code[..2], "new data")?;
        let parsed_addr_ls = parse_hex_word(&stripped_code[2..5], "address")?;
        let parsed_addr_ms = parse_hex_byte(&stripped_code[5..6], "address")?;
    
        let address = (((parsed_addr_ms as u16) << 12) | (parsed_addr_ls)) ^ 0xF000;

        if address > 0x7FFF {
            Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Invalid address: {}", address)))
        }
        else {
            let maybe_old_data = if stripped_code.len() > 6 {
                let parsed_old_data = parse_hex_byte(&stripped_code[6..8], "old data")?;
                Some((parsed_old_data.rotate_right(2)) ^ 0xBA)
            }
            else {
                None
            };
    
            Ok(Cheat {
                address,
                new_data: parsed_new_data,
                maybe_old_data,
                maybe_bank: None,
            }) 
        }
    }
}

const CHEAT_LIMIT: usize = 10;

pub fn register_cheat(emulator: &mut Emulator, cheat_id: &str, cheat: Cheat) -> Option<String> {
    if emulator.cheats.registered.len() >= CHEAT_LIMIT {
        Some(format!("You cannot register more than {CHEAT_LIMIT} cheats at a time."))
    }
    else {
        emulator.cheats.registered.insert(cheat_id.to_string(), cheat); 
        None
    }
}

pub fn unregister_cheat(emulator: &mut Emulator, cheat_id: &str) {
    emulator.cheats.registered.remove(cheat_id);
}

pub fn validate_gameshark_code(cheat_code: &str) -> Option<String> {
    match parse_gameshark_code(cheat_code) {
        Ok(_) => None,
        Err(error) => Some(error.to_string())
    }
}

pub fn validate_gamegenie_code(cheat_code: &str) -> Option<String> {
    match parse_gamegenie_code(cheat_code) {
        Ok(_) => None,
        Err(error) => Some(error.to_string())
    }
}

pub fn register_gameshark_cheat(emulator: &mut Emulator, cheat_id: &str, cheat_code: &str) -> Option<String> {
    match parse_gameshark_code(cheat_code) {
        Ok(cheat) => {
            register_cheat(emulator, cheat_id, cheat)
        },
        Err(error) => Some(error.to_string())
    }
}

pub fn register_gamegenie_cheat(emulator: &mut Emulator, cheat_id: &str, cheat_code: &str) -> Option<String> {
    match parse_gamegenie_code(cheat_code) {
        Ok(cheat) => {
            register_cheat(emulator, cheat_id, cheat)
        },
        Err(error) => Some(error.to_string())
    }
}

fn get_ram_bank_for_address(emulator: &Emulator, address: u16) -> u8 {
    match address & 0xF000 {
        0xA000..=0xBFFF => emulator.memory.cartridge_mapper.get_ram_bank(),
        0xC000..=0xDFFF => mmu::get_working_ram_bank(emulator),
        _ => 0
    }
}

pub fn apply_cheat_if_needed(emulator: &Emulator, address: u16, old_data: u8) -> u8 {
    let mut maybe_found_cheat = None;

    for (_, cheat) in &emulator.cheats.registered {
        if cheat.address == address {
            let apply_cheat = if cheat.maybe_bank.is_some() {
                let cheat_bank = cheat.maybe_bank.unwrap();
                let current_bank = get_ram_bank_for_address(emulator, address);
                cheat_bank == current_bank
            }
            else if cheat.maybe_bank.is_none() && cheat.maybe_old_data.is_some() {
                let cheat_old_data = cheat.maybe_old_data.unwrap();
                old_data == cheat_old_data
            }
            else {
                true
            };
        
            if apply_cheat {
                maybe_found_cheat = Some(cheat);
                break;
            }
        }
    };

    maybe_found_cheat
        .map(|cheat| cheat.new_data)
        .unwrap_or_else(|| old_data)
}

#[cfg(test)]
mod tests {
    use crate::emulator::{initialize_screenless_emulator, Mode};
    use crate::mmu;
    use crate::mmu::constants::*;
    use crate::mmu::test_utils::*;
    use crate::mmu::effects::empty_cartridge_effects;

    use super::*;

    #[test]
    fn should_parse_gameshark_code() {
        let gameshark_code = "01FF56D3";
        let cheat = parse_gameshark_code(gameshark_code).unwrap();
        assert_eq!(cheat.address, 0xD356);
        assert_eq!(cheat.new_data, 0xFF);
        assert_eq!(cheat.maybe_old_data, None);
        assert_eq!(cheat.maybe_bank, Some(1));
    }

    #[test]
    fn should_apply_gameshark_code() {
        let mut emulator = initialize_screenless_emulator();
        emulator.mode = Mode::CGB;

        let cheat = Cheat {
            address: 0xD356,
            new_data: 0xFF,
            maybe_old_data: None,
            maybe_bank: Some(1),
        };

        register_cheat(&mut emulator, "da9a7056-ad9b-4ca1-9049-688931b279a3", cheat);

        // Switch to working RAM bank 1
        mmu::write_byte(&mut emulator, 0xFF70, 1);
        
        let byte = mmu::read_byte(&mut emulator, 0xD356);

        assert_eq!(byte, 0xFF);
    }

    #[test]
    fn should_parse_gamegenie_code() {
        let gamegenie_code = "CED-56A-D50";
        let cheat = parse_gamegenie_code(gamegenie_code).unwrap();
        assert_eq!(cheat.new_data, 0xCE);
        assert_eq!(cheat.address, 0x5D56);
        assert_eq!(cheat.maybe_old_data, Some(0x8E));
        assert_eq!(cheat.maybe_bank, None);
    }

    #[test]
    fn should_apply_gamegenie_code() {
        let mut emulator = initialize_screenless_emulator();
        emulator.mode = Mode::CGB;

        let mut test_rom = build_rom(CART_TYPE_MBC1_WITH_RAM, ROM_SIZE_64KB, RAM_SIZE_2KB);
        test_rom[0x5D56] = 0x8E;
        mmu::load_rom_buffer(&mut emulator.memory, test_rom, empty_cartridge_effects()).unwrap();

        let cheat = Cheat {
            address: 0x5D56,
            new_data: 0xCE,
            maybe_old_data: Some(0x8E),
            maybe_bank: None,
        };

        register_cheat(&mut emulator, "da9a7056-ad9b-4ca1-9049-688931b279a3", cheat);

        let byte = mmu::read_byte(&mut emulator, 0x5D56);

        assert_eq!(byte, 0xCE);
    }

    #[test]
    fn should_parse_gamegenie_code_without_compare_value() {
        let gamegenie_code = "CED-56A";
        let cheat = parse_gamegenie_code(gamegenie_code).unwrap();
        assert_eq!(cheat.new_data, 0xCE);
        assert_eq!(cheat.address, 0x5D56);
        assert_eq!(cheat.maybe_old_data, None);
        assert_eq!(cheat.maybe_bank, None);
    }
}