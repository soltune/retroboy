use super::*;

fn setup_test_memory_data() -> Memory {
    let mut memory = initialize_memory();

    memory.bios[0] = 0xAF;
    memory.bios[1] = 0xF1;
    memory.bios[2] = 0x03;
    memory.bios[3] = 0x4D;

    memory.rom[0] = 0x1E;
    memory.rom[1] = 0xF2;
    memory.rom[2] = 0x01;
    memory.rom[3] = 0x09;

    memory.rom[0x20AF] = 0x11;
    memory.rom[0x20B0] = 0x17;
    memory.rom[0x20B1] = 0xEE;

    memory.rom[0x5ACC] = 0x13;
    memory.rom[0x5ACD] = 0x9C;
    memory.rom[0x5ACE] = 0x55;

    memory.video_ram[0] = 0xB1;
    memory.video_ram[1] = 0xD2;
    memory.video_ram[2] = 0xAA;

    memory.external_ram[0] = 0xC2;
    memory.external_ram[1] = 0x22;
    memory.external_ram[2] = 0x35;

    memory.working_ram[0] = 0xF1;
    memory.working_ram[1] = 0x22;
    memory.working_ram[2] = 0x2B;

    memory.working_ram[0x15F0] = 0x2B;
    memory.working_ram[0x15F1] = 0x7C;

    memory.object_attribute_memory[0x7A] = 0x44;
    memory.object_attribute_memory[0x7B] = 0x45;
    memory.object_attribute_memory[0x7C] = 0x9B;

    memory.zero_page_ram[0x20] = 0xBB;
    memory.zero_page_ram[0x21] = 0x44;
    memory.zero_page_ram[0x5B] = 0x5F;

    memory.interrupt_registers.enabled = 0x1F;
    memory.interrupt_registers.flags = 0xA;

    memory.timer_registers.divider = 0x3A;
    memory.timer_registers.counter = 0x04;
    memory.timer_registers.modulo = 0x02;
    memory.timer_registers.control = 0x07;

    memory.in_bios = false;

    memory
}

#[test]
fn reads_from_bios() {
    let mut memory = setup_test_memory_data();
    memory.in_bios = true;
    assert_eq!(read_byte(&memory, 0x02), 0x03);
}

#[test]
fn reads_from_rom_in_bank_zero() {
    let memory = setup_test_memory_data();
    assert_eq!(read_byte(&memory, 0x02), 0x01);
}

#[test]
fn reads_from_rom_in_bank_zero_scenario_two() {
    let memory = setup_test_memory_data();
    assert_eq!(read_byte(&memory, 0x20B1), 0xEE);
}

#[test]
fn reads_from_rom_in_subsequent_bank() {
    let memory = setup_test_memory_data();
    assert_eq!(read_byte(&memory, 0x5ACE), 0x55);
}

#[test]
fn reads_from_video_ram() {
    let memory = setup_test_memory_data();
    assert_eq!(read_byte(&memory, 0x8002), 0xAA);
}

#[test]
fn reads_from_external_ram() {
    let memory = setup_test_memory_data();
    assert_eq!(read_byte(&memory, 0xA001), 0x22);
}

#[test]
fn reads_from_working_ram() {
    let memory = setup_test_memory_data();
    assert_eq!(read_byte(&memory, 0xC002), 0x2B);
}

#[test]
fn reads_from_working_ram_shadow() {
    let memory = setup_test_memory_data();
    assert_eq!(read_byte(&memory, 0xE002), 0x2B);
}

#[test]
fn reads_from_working_ram_shadow_scenario_two() {
    let memory = setup_test_memory_data();
    assert_eq!(read_byte(&memory, 0xF5F0), 0x2B);
}

#[test]
fn reads_from_object_attribute_memory() {
    let memory = setup_test_memory_data();
    assert_eq!(read_byte(&memory, 0xFE7B), 0x45);
}

#[test]
fn reads_zero_values_outside_of_object_attribute_memory() {
    let memory = setup_test_memory_data();
    assert_eq!(read_byte(&memory, 0xFEEE), 0x00);
}

#[test]
fn reads_from_zero_page_ram() {
    let memory = setup_test_memory_data();
    assert_eq!(read_byte(&memory, 0xFFA0), 0xBB);
}

#[test]
fn reads_from_interrupts_enabled_register() {
    let memory: Memory = setup_test_memory_data();
    assert_eq!(read_byte(&memory, 0xFFFF), 0x1F);
}

#[test]
fn reads_from_interrupt_flags_register() {
    let memory: Memory = setup_test_memory_data();
    assert_eq!(read_byte(&memory, 0xFF0F), 0xA);
}

#[test]
fn reads_from_timer_divider_register() {
    let memory: Memory = setup_test_memory_data();
    assert_eq!(read_byte(&memory, 0xFF04), 0x3A);
}

#[test]
fn reads_from_timer_counter_register() {
    let memory: Memory = setup_test_memory_data();
    assert_eq!(read_byte(&memory, 0xFF05), 0x04);
}

#[test]
fn reads_from_timer_modulo_register() {
    let memory: Memory = setup_test_memory_data();
    assert_eq!(read_byte(&memory, 0xFF06), 0x02);
}

#[test]
fn reads_from_timer_control_register() {
    let memory: Memory = setup_test_memory_data();
    assert_eq!(read_byte(&memory, 0xFF07), 0x07);
}

#[test]
fn reads_word_from_memory() {
    let memory = setup_test_memory_data();
    assert_eq!(read_word(&memory, 0x20AF), 0x1711);
}

#[test]
fn loads_rom_buffer_into_memory() {
    let memory = setup_test_memory_data();

    let mut rom_buffer = vec![0; 0xA000];
    rom_buffer[0] = 0xA0;
    rom_buffer[1] = 0xCC;
    rom_buffer[2] = 0x3B;
    rom_buffer[3] = 0x4C;
    rom_buffer[0x7FFF] = 0xD4;
    rom_buffer[0x8000] = 0xBB;
    rom_buffer[0x8001] = 0xD1;

    let loaded_memory = load_rom_buffer(memory, rom_buffer);

    assert_eq!(read_byte(&loaded_memory, 0x0000), 0xA0);
    assert_eq!(read_byte(&loaded_memory, 0x0001), 0xCC);
    assert_eq!(read_byte(&loaded_memory, 0x0002), 0x3B);
    assert_eq!(read_byte(&loaded_memory, 0x0003), 0x4C);
    assert_eq!(read_byte(&loaded_memory, 0x7FFF), 0xD4);
    assert_eq!(read_byte(&loaded_memory, 0x8000), 0xB1);
}

#[test]
fn writes_to_video_ram() {
    let mut memory = setup_test_memory_data();
    write_byte(&mut memory, 0x8002, 0xC1);
    assert_eq!(memory.video_ram[2], 0xC1);
}

#[test]
fn writes_word_to_video_ram() {
    let mut memory = setup_test_memory_data();
    write_word(&mut memory, 0x8002, 0xC1DD);
    assert_eq!(memory.video_ram[2], 0xDD);
    assert_eq!(memory.video_ram[3], 0xC1);
}
