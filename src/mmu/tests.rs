use crate::emulator::initialize_screenless_emulator;
use crate::emulator::Mode;

use super::*;

fn setup_emulator_with_test_memory() -> Emulator {
    let mut emulator = initialize_screenless_emulator();

    emulator.memory.bios[0] = 0xaf;
    emulator.memory.bios[1] = 0xF1;
    emulator.memory.bios[2] = 0x03;
    emulator.memory.bios[3] = 0x4D;

    emulator.memory.cartridge.rom.resize(0x8000, 0);

    emulator.memory.cartridge.rom[0] = 0x1E;
    emulator.memory.cartridge.rom[1] = 0xF2;
    emulator.memory.cartridge.rom[2] = 0x01;
    emulator.memory.cartridge.rom[3] = 0x09;

    emulator.memory.cartridge.rom[0x20AF] = 0x11;
    emulator.memory.cartridge.rom[0x20B0] = 0x17;
    emulator.memory.cartridge.rom[0x20B1] = 0xEE;

    emulator.memory.cartridge.rom[0x5ACC] = 0x13;
    emulator.memory.cartridge.rom[0x5ACD] = 0x9C;
    emulator.memory.cartridge.rom[0x5ACE] = 0x55;

    emulator.gpu.video_ram[0] = 0xB1;
    emulator.gpu.video_ram[1] = 0xD2;
    emulator.gpu.video_ram[2] = 0xAA;

    emulator.memory.cartridge.ram[0] = 0xC2;
    emulator.memory.cartridge.ram[1] = 0x22;
    emulator.memory.cartridge.ram[2] = 0x35;

    emulator.memory.working_ram[0] = 0xF1;
    emulator.memory.working_ram[1] = 0x22;
    emulator.memory.working_ram[2] = 0x2B;
    
    emulator.memory.working_ram[0x1001] = 0x11;

    emulator.memory.working_ram[0x15F0] = 0x2B;
    emulator.memory.working_ram[0x15F1] = 0x7C;

    emulator.memory.working_ram[0x2001] = 0x22;

    emulator.gpu.object_attribute_memory[0x7A] = 0x44;
    emulator.gpu.object_attribute_memory[0x7B] = 0x45;
    emulator.gpu.object_attribute_memory[0x7C] = 0x9B;

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
    emulator.gpu.registers.palettes.bgp = 0xC1;
    emulator.gpu.registers.ly = 0x2B;
    emulator.gpu.registers.lyc = 0xAB;
    emulator.gpu.registers.stat = 0xD2;
    emulator.gpu.registers.palettes.obp0 = 0x1B;
    emulator.gpu.registers.palettes.obp1 = 0xE4;

    emulator.keys.column = 0x10;
    emulator.keys.select_buttons = 0x04;

    emulator.apu.enabled = true;
    emulator.apu.sound_panning = 0xF2;
    emulator.apu.master_volume = 0xC1;
    emulator.apu.audio_buffer_clock = 84;

    emulator.apu.channel1.sweep.initial_settings = 0xDD;
    emulator.apu.channel1.length.initial_settings = 0xB0;
    emulator.apu.channel1.envelope.initial_settings = 0xAA;
    emulator.apu.channel1.period.low = 0xB2;
    emulator.apu.channel1.period.high = 0xC2;

    emulator.apu.channel2.length.initial_settings = 0xC0;
    emulator.apu.channel2.envelope.initial_settings = 0xC1;
    emulator.apu.channel2.period.low = 0x14;
    emulator.apu.channel2.period.high = 0x24;

    emulator.apu.channel3.enabled = true;
    emulator.apu.channel3.dac_enabled = true;
    emulator.apu.channel3.volume = 0x60;
    emulator.apu.channel3.period.low = 0xFF;
    emulator.apu.channel3.period.high = 0x44;
    emulator.apu.channel3.period.divider = 0x301;
    emulator.apu.channel3.period.reloaded = true;
    emulator.apu.channel3.wave_pattern_ram[0x0] = 0xB1;
    emulator.apu.channel3.wave_pattern_ram[0x1] = 0xD2;

    emulator.apu.channel4.length.initial_settings = 0x1A;
    emulator.apu.channel4.envelope.initial_settings = 0xD2;
    emulator.apu.channel4.polynomial = 0xCE;
    emulator.apu.channel4.control = 0xC0;

    emulator.memory.in_bios = false;

    emulator
}

#[test]
fn reads_from_bios() {
    let mut emulator = setup_emulator_with_test_memory();
    emulator.memory.in_bios = true;
    assert_eq!(read_byte(&mut emulator, 0x02), 0x03);
}

#[test]
fn reads_from_rom_in_bank_zero() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0x02), 0x01);
}

#[test]
fn reads_from_rom_in_bank_zero_scenario_two() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0x20B1), 0xEE);
}

#[test]
fn reads_from_rom_in_subsequent_bank() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0x5ACE), 0x55);
}

#[test]
fn reads_from_video_ram() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0x8002), 0xAA);
}

#[test]
fn disallow_access_to_external_ram_if_not_enabled() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xA001), 0xFF);
}

#[test]
fn reads_from_working_ram() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xC002), 0x2B);
}

#[test]
fn reads_from_working_ram_shadow() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xE002), 0x2B);
}

#[test]
fn reads_from_working_ram_shadow_scenario_two() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xF5F0), 0x2B);
}

#[test]
fn reads_from_separate_working_ram_bank() {
    let mut emulator = setup_emulator_with_test_memory();
    emulator.mode = Mode::CGB;

    write_byte(&mut emulator, 0xFF70, 0x02);

    assert_eq!(read_byte(&mut emulator, 0xD001), 0x22);
    assert_eq!(read_byte(&mut emulator, 0xF001), 0x22); 
}

#[test]
fn reads_from_object_attribute_memory() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFE7B), 0x45);
}

#[test]
fn reads_empty_values_outside_of_object_attribute_memory() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFEEE), 0xFF);
}

#[test]
fn reads_from_zero_page_ram() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFFA0), 0xBB);
}

#[test]
fn reads_from_interrupts_enabled_register() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFFFF), 0x1F);
}

#[test]
fn reads_from_interrupt_flags_register() {
    let mut emulator= setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF0F), 0xA);
}

#[test]
fn reads_from_timer_divider_register() {
    let mut emulator= setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF04), 0x3A);
}

#[test]
fn reads_from_timer_counter_register() {
    let mut emulator= setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF05), 0x04);
}

#[test]
fn reads_from_timer_modulo_register() {
    let mut emulator= setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF06), 0x02);
}

#[test]
fn reads_from_timer_control_register() {
    let mut emulator= setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF07), 0x07);
}

#[test]
fn reads_lcdc_register_from_gpu() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF40), 0x80);
}

#[test]
fn reads_scroll_registers_from_gpu() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF42), 0x55);
    assert_eq!(read_byte(&mut emulator, 0xFF43), 0xA1);
}

#[test]
fn reads_window_position_registers_from_gpu() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF4A), 0xBB);
    assert_eq!(read_byte(&mut emulator, 0xFF4B), 0xDD);
}

#[test]
fn reads_palette_from_gpu() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF47), 0xC1);
}

#[test]
fn reads_ly_from_gpu() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF44), 0x2B);
}

#[test]
fn reads_lyc_from_gpu() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF45), 0xAB);
}

#[test]
fn reads_stat_from_gpu() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF41), 0xD2);
}

#[test]
fn reads_obp0_from_gpu() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF48), 0x1B);
}

#[test]
fn reads_obp1_from_gpu() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF49), 0xE4);
}

#[test]
fn writes_obp0_value_to_gpu() {
    let mut emulator = setup_emulator_with_test_memory();
    write_byte(&mut emulator, 0xFF48, 0xE4);
    assert_eq!(read_byte(&mut emulator, 0xFF48), 0xE4);
}

#[test]
fn writes_obp1_value_to_gpu() {
    let mut emulator = setup_emulator_with_test_memory();
    write_byte(&mut emulator, 0xFF49, 0x1B);
    assert_eq!(read_byte(&mut emulator, 0xFF49), 0x1B);
}

#[test]
fn reads_joyp_register() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF00), 0x14);
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

    load_rom_buffer(&mut emulator.memory, rom_buffer).unwrap();

    assert_eq!(read_byte(&mut emulator, 0x0000), 0xA0);
    assert_eq!(read_byte(&mut emulator, 0x0001), 0xCC);
    assert_eq!(read_byte(&mut emulator, 0x0002), 0x3B);
    assert_eq!(read_byte(&mut emulator, 0x0003), 0x4C);
    assert_eq!(read_byte(&mut emulator, 0x7FFF), 0xD4);
    assert_eq!(read_byte(&mut emulator, 0x8000), 0xB1);

    assert_eq!(emulator.memory.cartridge.header.sgb_support, false);
    assert_eq!(emulator.memory.cartridge.header.type_code, 0x01);
}

#[test]
fn writes_to_video_ram() {
    let mut emulator = setup_emulator_with_test_memory();
    write_byte(&mut emulator, 0x8002, 0xC1);
    assert_eq!(emulator.gpu.video_ram[2], 0xC1);
}

#[test]
fn reads_from_audio_master_control() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF26), 0xF4);
}

#[test]
fn writes_to_audio_master_control() {
    let mut emulator = setup_emulator_with_test_memory();
    write_byte(&mut emulator, 0xFF26, 0x0);
    assert_eq!(emulator.apu.enabled, false);
}

#[test]
fn reads_from_sound_panning() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF25), 0xF2);
}

#[test]
fn reads_from_master_volume() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF24), 0xC1);
}

#[test]
fn reads_from_ch1_sweep() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF10), 0xDD);
}

#[test]
fn reads_from_ch1_length_and_duty() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF11), 0xBF);
}

#[test]
fn reads_from_ch1_volume() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF12), 0xAA);
}

#[test]
fn reads_from_ch1_period_low() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF13), 0xFF);
}

#[test]
fn reads_from_ch1_period_high() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF14), 0xFF);
}

#[test]
fn reads_from_ch2_length_and_duty() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF16), 0xFF);
}

#[test]
fn reads_from_ch2_volume() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF17), 0xC1);
}

#[test]
fn reads_from_ch2_period_low() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF18), 0xFF);
}

#[test]
fn reads_from_ch2_period_high() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF19), 0xBF);
}

#[test]
fn reads_from_ch3_dac_enabled() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF1A), 0xFF);
}

#[test]
fn reads_from_ch3_output() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF1C), 0xFF);
}

#[test]
fn reads_from_ch3_period_high() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF1E), 0xFF);
}

#[test]
fn reads_from_wave_pattern_ram() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF30), 0xB1);
    assert_eq!(read_byte(&mut emulator, 0xFF31), 0xB1);
}

#[test]
fn reads_from_ch4_length() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF20), 0xFF);
}

#[test]
fn reads_from_ch4_envelope() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF21), 0xD2);
}

#[test]
fn reads_from_ch4_polynomial() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF22), 0xCE);
}

#[test]
fn reads_from_ch4_control() {
    let mut emulator = setup_emulator_with_test_memory();
    assert_eq!(read_byte(&mut emulator, 0xFF23), 0xFF);
}

#[test]
fn reads_from_key1() {
    let mut emulator = setup_emulator_with_test_memory();
    emulator.mode = Mode::CGB;
    assert_eq!(read_byte(&mut emulator, 0xFF4D), 0x0);
}