use crate::utils;
use crate::cpu::{BusActivityEntry, BusActivityType, Register, RegisterPair, CpuState};
use crate::utils::get_t_cycle_increment;

pub fn step_one_machine_cycle(cpu_state: &mut CpuState) {
    let double_speed_mode = cpu_state.address_bus.speed_switch().cgb_double_speed();
    let t_cycle_increment = get_t_cycle_increment(double_speed_mode);
    cpu_state.instruction_clock_cycles = cpu_state.instruction_clock_cycles.wrapping_add(t_cycle_increment);
    cpu_state.address_bus.sync();
}

pub fn step_machine_cycles(cpu_state: &mut CpuState, cycles: u8) {
    for _ in 0..cycles {
        step_one_machine_cycle(cpu_state);
    }
}

fn record_bus_activity(cpu_state: &mut CpuState, bus_activity_entry: BusActivityEntry) {
    let double_speed_mode = cpu_state.address_bus.speed_switch().cgb_double_speed();
    let t_cycle_increment = get_t_cycle_increment(double_speed_mode);
    let current_machine_cycle = (cpu_state.instruction_clock_cycles / t_cycle_increment) as usize;
    let recorded_cycles = cpu_state.opcode_bus_activity.len();
    let cycles_with_no_activity = (current_machine_cycle - 1) - recorded_cycles;

    for _ in 0..cycles_with_no_activity {
        cpu_state.opcode_bus_activity.push(None);
    }

    cpu_state.opcode_bus_activity.push(Some(bus_activity_entry));
}

fn record_bus_read(cpu_state: &mut CpuState, address: u16, value: u8) {
    let bus_activity_entry = BusActivityEntry {
        address,
        value,
        activity_type: BusActivityType::Read
    };
    record_bus_activity(cpu_state, bus_activity_entry);
}

fn record_bus_write(cpu_state: &mut CpuState, address: u16, value: u8) {
    let bus_activity_entry = BusActivityEntry {
        address,
        value,
        activity_type: BusActivityType::Write
    };
    record_bus_activity(cpu_state, bus_activity_entry);
}

pub fn read_byte_from_memory(cpu_state: &mut CpuState, address: u16) -> u8 {
    step_one_machine_cycle(cpu_state);
    let byte = cpu_state.address_bus.read_byte(address);

    if cpu_state.address_bus.processor_test_mode() {
        record_bus_read(cpu_state, address, byte);
    }
    
    byte
}

pub fn read_word_from_memory(cpu_state: &mut CpuState, address: u16) -> u16 {
    let first_byte = read_byte_from_memory(cpu_state, address);
    let second_byte = read_byte_from_memory(cpu_state, address + 1);
    utils::as_word(first_byte, second_byte)
}

pub fn store_byte_in_memory(cpu_state: &mut CpuState, address: u16, byte: u8) {
    step_one_machine_cycle(cpu_state);
    cpu_state.address_bus.write_byte(address, byte);
    
    if cpu_state.address_bus.processor_test_mode() {
        record_bus_write(cpu_state, address, byte);
    }
}

pub fn store_word_in_memory(cpu_state: &mut CpuState, address: u16, word: u16) {
    let (first_byte, second_byte) = utils::as_bytes(word);
    store_byte_in_memory(cpu_state, address, first_byte);
    store_byte_in_memory(cpu_state, address + 1, second_byte);
}

pub fn read_from_register(cpu_state: &CpuState, register: &Register) -> u8 {
    match register {
        Register::A => cpu_state.registers.a,
        Register::B => cpu_state.registers.b,
        Register::C => cpu_state.registers.c,
        Register::D => cpu_state.registers.d,
        Register::E => cpu_state.registers.e,
        Register::F => cpu_state.registers.f,
        Register::H => cpu_state.registers.h,
        Register::L => cpu_state.registers.l
    } 
}

pub fn store_in_register(cpu_state: &mut CpuState, register: Register, value: u8) {
    match register {
        Register::A => cpu_state.registers.a = value,
        Register::B => cpu_state.registers.b = value,
        Register::C => cpu_state.registers.c = value,
        Register::D => cpu_state.registers.d = value,
        Register::E => cpu_state.registers.e = value,
        Register::F => cpu_state.registers.f = value,
        Register::H => cpu_state.registers.h = value,
        Register::L => cpu_state.registers.l = value
    } 
}

pub fn read_from_register_pair(cpu_state: &mut CpuState, register_pair: &RegisterPair) -> u16 {
    let first_byte = read_from_register(cpu_state, &register_pair.first);
    let second_byte = read_from_register(cpu_state, &register_pair.second);
    ((first_byte as u16) << 8) | (second_byte as u16 & 0xFF)
}

pub fn store_in_register_pair(cpu_state: &mut CpuState, register_pair: RegisterPair, value: u16) {
    store_in_register(cpu_state, register_pair.first, ((value >> 8) & 0xFF) as u8);
    store_in_register(cpu_state, register_pair.second, (value & 0xFF) as u8);
}

pub fn set_flag_z(cpu_state: &mut CpuState, flag: bool) {
    if flag {
        cpu_state.registers.f = cpu_state.registers.f | 0x80;
    } else {
        cpu_state.registers.f = cpu_state.registers.f & 0x7F;
    }
}

pub fn set_flag_n(cpu_state: &mut CpuState, flag: bool) {
    if flag {
        cpu_state.registers.f = cpu_state.registers.f | 0x40;
    } else {
        cpu_state.registers.f = cpu_state.registers.f & 0xBF;
    }
}

pub fn set_flag_h(cpu_state: &mut CpuState, flag: bool) {
    if flag {
        cpu_state.registers.f = cpu_state.registers.f | 0x20;
    } else {
        cpu_state.registers.f = cpu_state.registers.f & 0xDF;
    }
}

pub fn set_flag_c(cpu_state: &mut CpuState, flag: bool) {
    if flag {
        cpu_state.registers.f = cpu_state.registers.f | 0x10;
    } else {
        cpu_state.registers.f = cpu_state.registers.f & 0xEF;
    }
}

pub fn is_z_flag_set(cpu_state: &CpuState) -> bool {
    let value = read_from_register(cpu_state, &Register::F);
    (value & 0x80) == 0x80
}

pub fn is_n_flag_set(cpu_state: &CpuState) -> bool {
    let value = read_from_register(cpu_state, &Register::F);
    (value & 0x40) == 0x40
}

pub fn is_h_flag_set(cpu_state: &CpuState) -> bool {
    let value = read_from_register(cpu_state, &Register::F);
    (value & 0x20) == 0x20
}

pub fn is_c_flag_set(cpu_state: &CpuState) -> bool {
    let value = read_from_register(cpu_state, &Register::F);
    (value & 0x10) == 0x10
}