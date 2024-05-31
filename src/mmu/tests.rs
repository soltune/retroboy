use crate::emulator::initialize_emulator;

use super::*;

fn setup_emulator_with_test_memory() -> Emulator {
    let mut emulator = initialize_emulator();

    emulator.memory.bios[0] = 0xaf;
    emulator.memory.bios[1] = 0xF1;
    emulator.memory.bios[2] = 0x03;
    emulator.memory.bios[3] = 0x4D;

    emulator.memory.rom.resize(0x8000, 0);

    emulator.memory.rom[0] = 0x1E;
    emulator.memory.rom[1] = 0xF2;
    emulator.memory.rom[2] = 0x01;
    emulator.memory.rom[3] = 0x09;

    emulator.memory.rom[0x20AF] = 0x11;
    emulator.memory.rom[0x20B0] = 0x17;
    emulator.memory.rom[0x20B1] = 0xEE;

    emulator.memory.rom[0x5ACC] = 0x13;
    emulator.memory.rom[0x5ACD] = 0x9C;
    emulator.memory.rom[0x5ACE] = 0x55;

    emulator.memory.video_ram[0] = 0xB1;
    emulator.memory.video_ram[1] = 0xD2;
    emulator.memory.video_ram[2] = 0xAA;

    emulator.memory.external_ram[0] = 0xC2;
    emulator.memory.external_ram[1] = 0x22;
    emulator.memory.external_ram[2] = 0x35;

    emulator.memory.working_ram[0] = 0xF1;
    emulator.memory.working_ram[1] = 0x22;
    emulator.memory.working_ram[2] = 0x2B;

    emulator.memory.working_ram[0x15F0] = 0x2B;
    emulator.memory.working_ram[0x15F1] = 0x7C;

    emulator.memory.object_attribute_memory[0x7A] = 0x44;
    emulator.memory.object_attribute_memory[0x7B] = 0x45;
    emulator.memory.object_attribute_memory[0x7C] = 0x9B;

    emulator.memory.zero_page_ram[0x20] = 0xBB;
    emulator.memory.zero_page_ram[0x21] = 0x44;
    emulator.memory.zero_page_ram[0x5B] = 0x5F;

    emulator.interrupts.enabled = 0x1F;
    emulator.interrupts.flags = 0xA;

    emulator.timers.divider = 0x3A;
    emulator.timers.counter = 0x04;
    emulator.timers.modulo = 0x02;
    emulator.timers.control = 0x07;

    emulator.gpu.registers.lcdc = 0x80;
    emulator.gpu.registers.scy = 0x55;
    emulator.gpu.registers.scx = 0xA1;
    emulator.gpu.registers.wy = 0xBB;
    emulator.gpu.registers.wx = 0xDD;
    emulator.gpu.registers.palette = 0xC1;
    emulator.gpu.registers.ly = 0x2B;
    emulator.gpu.registers.lyc = 0xAB;
    emulator.gpu.registers.stat = 0xD2;
    emulator.gpu.registers.obp0 = 0x1B;
    emulator.gpu.registers.obp1 = 0xE4;

    emulator.keys.column = 0x10;
    emulator.keys.select_buttons = 0x04;

    emulator.apu.audio_master_control = 0xB1;
    emulator.apu.sound_panning = 0xF2;
    emulator.apu.master_volume = 0xC1;
    emulator.apu.ch1_sweep = 0xDD;
    emulator.apu.ch1_length_and_duty = 0xB0;
    emulator.apu.ch1_volume = 0xAA;
    emulator.apu.ch1_period_low = 0xB2;
    emulator.apu.ch1_period_high = 0xC2;

    emulator.memory.in_bios = false;

    emulator
}

#[test]
fn reads_from_bios() {
    let mut emulator = setup_emulator_with_test_memory();
    emulator.memory.in_bios = true;
    assert_eq!(read_byte(&emulator, 0x02), 0x03);
}

#[test]
fn reads_from_rom_in_bank_zero() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0x02), 0x01);
}

#[test]
fn reads_from_rom_in_bank_zero_scenario_two() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0x20B1), 0xEE);
}

#[test]
fn reads_from_rom_in_subsequent_bank() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0x5ACE), 0x55);
}

#[test]
fn reads_from_video_ram() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0x8002), 0xAA);
}

#[test]
fn reads_from_external_ram() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xA001), 0x22);
}

#[test]
fn reads_from_working_ram() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xC002), 0x2B);
}

#[test]
fn reads_from_working_ram_shadow() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xE002), 0x2B);
}

#[test]
fn reads_from_working_ram_shadow_scenario_two() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xF5F0), 0x2B);
}

#[test]
fn reads_from_object_attribute_memory() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFE7B), 0x45);
}

#[test]
fn reads_zero_values_outside_of_object_attribute_memory() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFEEE), 0x00);
}

#[test]
fn reads_from_zero_page_ram() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFFA0), 0xBB);
}

#[test]
fn reads_from_interrupts_enabled_register() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFFFF), 0x1F);
}

#[test]
fn reads_from_interrupt_flags_register() {
    let emulator= setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF0F), 0xA);
}

#[test]
fn reads_from_timer_divider_register() {
    let emulator= setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF04), 0x3A);
}

#[test]
fn reads_from_timer_counter_register() {
    let emulator= setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF05), 0x04);
}

#[test]
fn reads_from_timer_modulo_register() {
    let emulator= setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF06), 0x02);
}

#[test]
fn reads_from_timer_control_register() {
    let emulator= setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF07), 0x07);
}

#[test]
fn reads_lcdc_register_from_gpu() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF40), 0x80);
}

#[test]
fn reads_scroll_registers_from_gpu() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF42), 0x55);
    assert_eq!(read_byte(&emulator, 0xFF43), 0xA1);
}

#[test]
fn reads_window_position_registers_from_gpu() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF4A), 0xBB);
    assert_eq!(read_byte(&emulator, 0xFF4B), 0xDD);
}

#[test]
fn reads_palette_from_gpu() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF47), 0xC1);
}

#[test]
fn reads_ly_from_gpu() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF44), 0x2B);
}

#[test]
fn reads_lyc_from_gpu() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF45), 0xAB);
}

#[test]
fn reads_stat_from_gpu() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF41), 0xD2);
}

#[test]
fn reads_obp0_from_gpu() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF48), 0x1B);
}

#[test]
fn reads_obp1_from_gpu() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF49), 0xE4);
}

#[test]
fn writes_obp0_value_to_gpu() {
    let mut emulator = setup_emulator_with_test_memory();
    write_byte(&mut emulator, 0xFF48, 0xE4);
    assert_eq!(read_byte(&emulator, 0xFF48), 0xE4);
}

#[test]
fn writes_obp1_value_to_gpu() {
    let mut emulator = setup_emulator_with_test_memory();
    write_byte(&mut emulator, 0xFF49, 0x1B);
    assert_eq!(read_byte(&emulator, 0xFF49), 0x1B);
}

#[test]
fn initiates_dma_transfer() {
    let mut emulator = setup_emulator_with_test_memory();

    for byte_offset in 0..DMA_TRANSFER_BYTES {
        write_byte(&mut emulator, 0xC100 + (byte_offset as u16), 0xAA);
    }

    // Write to FF46 to initiate DMA transfer to OAM memory
    write_byte(&mut emulator, 0xFF46, 0xC1);

    for byte_offset in 0..DMA_TRANSFER_BYTES {
        assert_eq!(read_byte(&emulator, 0xFE00 + (byte_offset as u16)), 0xAA);
    }
}

#[test]
fn reads_joyp_register() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF00), 0x04);
}

#[test]
fn reads_word_from_emulator() {
    let emulator= setup_emulator_with_test_memory();
    assert_eq!(read_word(&emulator, 0x20AF), 0x1711);
}

#[test]
fn loads_rom_buffer_into_emulator() {
    let mut emulator = setup_emulator_with_test_memory();

    let mut rom_buffer = vec![0; 0xA000];
    rom_buffer[0] = 0xA0;
    rom_buffer[1] = 0xCC;
    rom_buffer[2] = 0x3B;
    rom_buffer[3] = 0x4C;
    rom_buffer[0x146] = 0x00;
    rom_buffer[0x147] = 0x01;
    rom_buffer[0x7FFF] = 0xD4;
    rom_buffer[0x8000] = 0xBB;
    rom_buffer[0x8001] = 0xD1;

    load_rom_buffer(&mut emulator.memory, rom_buffer);

    assert_eq!(read_byte(&emulator, 0x0000), 0xA0);
    assert_eq!(read_byte(&emulator, 0x0001), 0xCC);
    assert_eq!(read_byte(&emulator, 0x0002), 0x3B);
    assert_eq!(read_byte(&emulator, 0x0003), 0x4C);
    assert_eq!(read_byte(&emulator, 0x7FFF), 0xD4);
    assert_eq!(read_byte(&emulator, 0x8000), 0xB1);

    assert_eq!(emulator.memory.cartridge_header.sgb_support, false);
    assert_eq!(emulator.memory.cartridge_header.type_code, 0x01);
}

#[test]
fn writes_to_video_ram() {
    let mut emulator = setup_emulator_with_test_memory();
    write_byte(&mut emulator, 0x8002, 0xC1);
    assert_eq!(emulator.memory.video_ram[2], 0xC1);
}

#[test]
fn writes_word_to_video_ram() {
    let mut emulator = setup_emulator_with_test_memory();
    write_word(&mut emulator, 0x8002, 0xC1DD);
    assert_eq!(emulator.memory.video_ram[2], 0xDD);
    assert_eq!(emulator.memory.video_ram[3], 0xC1);
}

#[test]
fn enable_external_ram_if_correct_cartridge_type() {
    let mut emulator = setup_emulator_with_test_memory();
    emulator.memory.cartridge_header.type_code = CART_TYPE_MBC1_WITH_RAM;
    write_byte(&mut emulator, 0x0000, 0xA);
    assert_eq!(emulator.memory.ram_enabled, true);
}

#[test]
fn enable_external_ram_if_correct_cartridge_type_scenario_two() {
    let mut emulator = setup_emulator_with_test_memory();
    emulator.memory.cartridge_header.type_code = CART_TYPE_MBC1_WITH_RAM_PLUS_BATTERY;
    write_byte(&mut emulator, 0x0000, 0xA);
    assert_eq!(emulator.memory.ram_enabled, true);
}

#[test]
fn not_enable_external_ram_if_incorrect_cartridge_type() {
    let mut emulator = setup_emulator_with_test_memory();
    emulator.memory.cartridge_header.type_code = CART_TYPE_MBC1;
    write_byte(&mut emulator, 0x0000, 0xA);
    assert_eq!(emulator.memory.ram_enabled, false); 
}

#[test]
fn disable_external_ram_if_correct_cartridge_type() {
    let mut emulator = setup_emulator_with_test_memory();
    emulator.memory.cartridge_header.type_code = CART_TYPE_MBC1_WITH_RAM;
    emulator.memory.ram_enabled = true;
    write_byte(&mut emulator, 0x0000, 0xB);
    assert_eq!(emulator.memory.ram_enabled, false);
}

#[test]
fn set_rom_bank_number() {
    let mut emulator = setup_emulator_with_test_memory();
    emulator.memory.cartridge_header.type_code = CART_TYPE_MBC1;
    write_byte(&mut emulator, 0x2000, 0x4);
    assert_eq!(emulator.memory.rom_bank_number, 0x04);
}

#[test]
fn sets_the_lower_five_bits_of_the_rom_bank_number() {
    let mut emulator = setup_emulator_with_test_memory();
    emulator.memory.cartridge_header.type_code = CART_TYPE_MBC1;
    emulator.memory.mbc_mode = MBCMode::ROM;
    emulator.memory.rom_bank_number = 0x41;
    write_byte(&mut emulator, 0x2000, 0x4);
    assert_eq!(emulator.memory.rom_bank_number, 0x44);
}

#[test]
fn treats_setting_bank_zero_as_bank_one() {
    let mut emulator = setup_emulator_with_test_memory();
    emulator.memory.cartridge_header.type_code = CART_TYPE_MBC1;
    emulator.memory.mbc_mode = MBCMode::ROM;
    write_byte(&mut emulator, 0x2000, 0x0);
    assert_eq!(emulator.memory.rom_bank_number, 0x1);
}

#[test]
fn sets_ram_bank_number() {
    let mut emulator = setup_emulator_with_test_memory();
    emulator.memory.cartridge_header.type_code = CART_TYPE_MBC1;
    emulator.memory.mbc_mode = MBCMode::RAM;
    write_byte(&mut emulator, 0x4000, 0x2);
    assert_eq!(emulator.memory.ram_bank_number, 0x2);
}

#[test]
fn sets_high_two_bits_of_rom_bank_number() {
    let mut emulator = setup_emulator_with_test_memory();
    emulator.memory.cartridge_header.type_code = CART_TYPE_MBC1;
    emulator.memory.mbc_mode = MBCMode::ROM;
    emulator.memory.rom_bank_number = 0x41;
    write_byte(&mut emulator, 0x4000, 0x3);
    assert_eq!(emulator.memory.rom_bank_number, 0x61);
}

#[test]
fn switch_mbc_mode_from_rom_mode_to_ram_mode() {
    let mut emulator = setup_emulator_with_test_memory();
    emulator.memory.cartridge_header.type_code = CART_TYPE_MBC1_WITH_RAM;
    emulator.memory.ram_enabled = true;
    emulator.memory.mbc_mode = MBCMode::ROM;
    write_byte(&mut emulator, 0x6010, 0x01);
    assert_eq!(emulator.memory.mbc_mode, MBCMode::RAM); 
}

#[test]
fn switch_mbc_mode_from_ram_mode_to_rom_mode() {
    let mut emulator = setup_emulator_with_test_memory();
    emulator.memory.cartridge_header.type_code = CART_TYPE_MBC1_WITH_RAM;
    emulator.memory.ram_enabled = true;
    emulator.memory.mbc_mode = MBCMode::RAM;
    write_byte(&mut emulator, 0x6010, 0x00);
    assert_eq!(emulator.memory.mbc_mode, MBCMode::ROM); 
}

#[test]
fn reads_from_different_rom_bank() {
    let mut emulator = setup_emulator_with_test_memory();
    emulator.memory.cartridge_header.type_code = CART_TYPE_MBC1;
    emulator.memory.mbc_mode = MBCMode::ROM;
    emulator.memory.rom_bank_number = 3;
    emulator.memory.rom.resize(0x16000, 0);
    emulator.memory.rom[0xC005] = 0xA1;
    let result = read_byte(&emulator, 0x4005);
    assert_eq!(result, 0xA1);
}

#[test]
fn reads_from_different_ram_bank() {
    let mut emulator = setup_emulator_with_test_memory();
    emulator.memory.cartridge_header.type_code = CART_TYPE_MBC1_WITH_RAM;
    emulator.memory.mbc_mode = MBCMode::RAM;
    emulator.memory.ram_bank_number = 3;
    emulator.memory.external_ram[0x6005] = 0xA1;
    let result = read_byte(&emulator, 0xA005);
    assert_eq!(result, 0xA1);
}

// emulator.apu.audio_master_control = 0xB1;
// emulator.apu.sound_panning = 0xF2;
// emulator.apu.master_volume = 0xC1;
// emulator.apu.ch1_sweep = 0xDD;
// emulator.apu.ch1_length_and_duty = 0xB0;
// emulator.apu.ch1_volume = 0xAA;
// emulator.apu.ch1_period_low = 0xB2;
// emulator.apu.ch1_period_high = 0xC2;

#[test]
fn reads_from_audio_master_control() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF26), 0xB1);
}

#[test]
fn writes_to_audio_master_control() {
    let mut emulator = setup_emulator_with_test_memory();
    write_byte(&mut emulator, 0xFF26, 0xDA);
    assert_eq!(emulator.apu.audio_master_control, 0xDA);
}

#[test]
fn reads_from_sound_panning() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF25), 0xF2);
}

#[test]
fn reads_from_master_volume() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF24), 0xC1);
}

#[test]
fn reads_from_ch1_sweep() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF10), 0xDD);
}

#[test]
fn reads_from_ch1_length_and_duty() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF11), 0xB0);
}

#[test]
fn reads_from_ch1_volume() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF12), 0xAA);
}

#[test]
fn reads_from_ch1_period_low() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF13), 0xB2);
}

#[test]
fn reads_from_ch1_period_high() {
    let emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&emulator, 0xFF14), 0xC2);
}
