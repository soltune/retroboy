use crate::emulator::initialize_emulator;
use super::*;

#[test]
fn should_decrement_period_divider() {
    let mut emulator = initialize_emulator();
    emulator.apu.audio_master_control = 0b10000001;
    emulator.apu.channel1.enabled = true;
    emulator.apu.channel1.period.divider = 742;
    emulator.apu.channel1.period.low = 26;
    emulator.apu.channel1.period.high = 197;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.apu.channel1.period.divider, 741);
}

#[test]
fn should_increment_period_divider_for_channel_2() {
    let mut emulator = initialize_emulator();
    emulator.apu.audio_master_control = 0b10000010;
    emulator.apu.channel2.enabled = true;
    emulator.apu.channel2.period.divider = 742;
    emulator.apu.channel2.period.low = 26;
    emulator.apu.channel2.period.high = 197;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.apu.channel2.period.divider, 741); 
}

#[test]
fn should_do_nothing_if_apu_is_off() {
    let mut emulator = initialize_emulator();
    emulator.apu.audio_master_control = 0;
    emulator.apu.channel1.enabled = true;
    emulator.apu.channel1.period.divider = 742;
    emulator.apu.channel1.period.low = 26;
    emulator.apu.channel1.period.high = 197;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.apu.channel1.period.divider, 742); 
}

#[test]
fn should_do_nothing_if_ch1_is_off() {
    let mut emulator = initialize_emulator();
    emulator.apu.audio_master_control = 0b10000000;
    emulator.apu.channel1.enabled = false;
    emulator.apu.channel1.period.divider = 742;
    emulator.apu.channel1.period.low = 26;
    emulator.apu.channel1.period.high = 197;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.apu.channel1.period.divider, 742); 
}

#[test]
fn should_decrement_period_divider_twice_with_eight_instruction_cycles() {
    let mut emulator = initialize_emulator();
    emulator.apu.audio_master_control = 0b10000001;
    emulator.apu.channel1.enabled = true;
    emulator.apu.channel1.period.divider = 742;
    emulator.apu.channel1.period.low = 26;
    emulator.apu.channel1.period.high = 197;
    emulator.cpu.clock.instruction_clock_cycles = 8;
    step(&mut emulator);
    assert_eq!(emulator.apu.channel1.period.divider, 740); 
}

#[test]
fn should_reload_period_divider_once_it_reaches_zero() {
    let mut emulator = initialize_emulator();
    emulator.apu.audio_master_control = 0b10000001;
    emulator.apu.channel1.enabled = true;
    emulator.apu.channel1.period.divider = 1;
    emulator.apu.channel1.period.low = 26;
    emulator.apu.channel1.period.high = 197;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.apu.channel1.period.divider, 742);
}

#[test]
fn should_properly_wrap_period_divider_value_with_eight_instruction_cycles() {
    let mut emulator = initialize_emulator();
    emulator.apu.audio_master_control = 0b10000001;
    emulator.apu.channel1.enabled = true;
    emulator.apu.channel1.period.divider = 1;
    emulator.apu.channel1.period.low = 26;
    emulator.apu.channel1.period.high = 197;
    emulator.cpu.clock.instruction_clock_cycles = 8;
    step(&mut emulator);
    assert_eq!(emulator.apu.channel1.period.divider, 741);  
}

#[test]
fn should_increment_wave_duty_position_when_period_divider_reaches_zero() {
    let mut emulator = initialize_emulator();
    emulator.apu.audio_master_control = 0b10000001;
    emulator.apu.channel1.enabled = true;
    emulator.apu.channel1.period.divider = 1;
    emulator.apu.channel1.period.low = 26;
    emulator.apu.channel1.period.high = 197;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.apu.channel1.wave_duty_position, 1);
}

#[test]
fn should_reset_wave_duty_position_to_zero_when_increased_above_seven() {
    let mut emulator = initialize_emulator();
    emulator.apu.audio_master_control = 0b10000001;
    emulator.apu.channel1.enabled = true;
    emulator.apu.channel1.period.divider = 1;
    emulator.apu.channel1.period.low = 26;
    emulator.apu.channel1.period.high = 197;
    emulator.apu.channel1.wave_duty_position = 7;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.apu.channel1.wave_duty_position, 0);
}

#[test]
fn should_increment_divider_apu_every_time_bit_four_of_divider_timer_goes_from_one_to_zero() {
    let mut emulator = initialize_emulator();
    emulator.apu.audio_master_control = 0b10000001;
    emulator.apu.channel1.enabled = true;
    emulator.apu.last_divider_time = 0b10011111;
    emulator.timers.divider = 0b10100000;
    step(&mut emulator);
    assert_eq!(emulator.apu.divider_apu, 1);
}

#[test]
fn should_not_increment_divider_apu_if_bit_four_of_divider_timer_remains_unchanged() {
    let mut emulator = initialize_emulator();
    emulator.apu.audio_master_control = 0b10000001;
    emulator.apu.channel1.enabled = true;
    emulator.apu.last_divider_time = 0b10010000;
    emulator.timers.divider = 0b10010001;
    step(&mut emulator);
    assert_eq!(emulator.apu.divider_apu, 0); 
}

#[test]
fn should_wrap_div_apu_to_zero_when_increased_above_seven() {
    let mut emulator = initialize_emulator();
    emulator.apu.audio_master_control = 0b10000001;
    emulator.apu.channel1.enabled = true;
    emulator.apu.last_divider_time = 0b10011111;
    emulator.timers.divider = 0b10100000;
    emulator.apu.divider_apu = 7;
    step(&mut emulator);
    assert_eq!(emulator.apu.divider_apu, 0);
}

#[test]
fn should_trigger_ch1_when_writing_to_ch1_period_high() {
    let mut emulator = initialize_emulator();
    emulator.apu.audio_master_control = 0b10000000;
    emulator.apu.channel1.dac_enabled = true;
    emulator.apu.channel1.enabled = false;
    emulator.apu.channel1.envelope.initial_settings = 0b10100101;
    set_ch1_period_high(&mut emulator, 0b10000000);
    assert_eq!(emulator.apu.audio_master_control, 0b10000001);
    assert_eq!(emulator.apu.channel1.enabled, true);
    assert_eq!(emulator.apu.channel1.period.high, 0b10000000);
    assert_eq!(emulator.apu.channel1.envelope.current_volume, 0b1010);
    assert_eq!(emulator.apu.channel1.envelope.timer, 0b101)
}

#[test]
fn should_trigger_ch2_when_writing_to_ch2_period_high() {
    let mut emulator = initialize_emulator();
    emulator.apu.audio_master_control = 0b10000000;
    emulator.apu.channel2.dac_enabled = true;
    emulator.apu.channel2.enabled = false;
    set_ch2_period_high(&mut emulator, 0b10000000);
    assert_eq!(emulator.apu.audio_master_control, 0b10000010);
    assert_eq!(emulator.apu.channel2.enabled, true);
    assert_eq!(emulator.apu.channel2.period.high, 0b10000000);
}

#[test]
fn should_not_trigger_ch1_if_trigger_bit_is_not_set_when_writing_to_ch1_period_high() {
    let mut emulator = initialize_emulator();
    emulator.apu.audio_master_control = 0b10000000;
    emulator.apu.channel1.dac_enabled = true;
    emulator.apu.channel1.enabled = false;
    set_ch1_period_high(&mut emulator, 0b00000001);
    assert_eq!(emulator.apu.audio_master_control, 0b10000000);
    assert_eq!(emulator.apu.channel1.enabled, false);
    assert_eq!(emulator.apu.channel1.period.high, 0b00000001); 
}

#[test]
fn should_not_trigger_ch1_if_dac_is_disabled_when_writing_to_ch1_period_high() {
    let mut emulator = initialize_emulator();
    emulator.apu.audio_master_control = 0b10000000;
    emulator.apu.channel1.dac_enabled = false;
    emulator.apu.channel1.enabled = false;
    set_ch1_period_high(&mut emulator, 0b10000000);
    assert_eq!(emulator.apu.audio_master_control, 0b10000000);
    assert_eq!(emulator.apu.channel1.enabled, false);
    assert_eq!(emulator.apu.channel1.period.high, 0b10000000); 
}

#[test]
fn should_disable_dac_and_ch1_when_writing_to_ch1_volume() {
    let mut emulator = initialize_emulator();
    emulator.apu.audio_master_control = 0b10000001;
    emulator.apu.channel1.dac_enabled = true;
    emulator.apu.channel1.enabled = true;
    set_ch1_envelope_settings(&mut emulator, 0b00000001);
    assert_eq!(emulator.apu.audio_master_control, 0b10000000);
    assert_eq!(emulator.apu.channel1.dac_enabled, false);
    assert_eq!(emulator.apu.channel1.enabled, false);
    assert_eq!(emulator.apu.channel1.envelope.initial_settings, 0b00000001);
}

#[test]
fn should_disable_dac_and_ch2_when_writing_to_ch2_volume() {
    let mut emulator = initialize_emulator();
    emulator.apu.audio_master_control = 0b10000010;
    emulator.apu.channel2.dac_enabled = true;
    emulator.apu.channel2.enabled = true;
    set_ch2_envelope_settings(&mut emulator, 0b00000001);
    assert_eq!(emulator.apu.audio_master_control, 0b10000000);
    assert_eq!(emulator.apu.channel2.dac_enabled, false);
    assert_eq!(emulator.apu.channel2.enabled, false);
    assert_eq!(emulator.apu.channel2.envelope.initial_settings, 0b00000001);
}

#[test]
fn should_not_disable_dac_if_bits_three_through_seven_have_values_when_writing_to_ch1_volume() {
    let mut emulator = initialize_emulator();
    emulator.apu.audio_master_control = 0b10000001;
    emulator.apu.channel1.dac_enabled = true;
    emulator.apu.channel1.enabled = true;
    set_ch1_envelope_settings(&mut emulator, 0b00101001);
    assert_eq!(emulator.apu.audio_master_control, 0b10000001);
    assert_eq!(emulator.apu.channel1.dac_enabled, true);
    assert_eq!(emulator.apu.channel1.enabled, true);
    assert_eq!(emulator.apu.channel1.envelope.initial_settings, 0b00101001); 
}

#[test]
fn should_update_ch1_envelope_volume_and_reset_timer_when_timer_decrements_to_zero() {
    let mut emulator = initialize_emulator();

    emulator.apu.audio_master_control = 0b10000001;
    emulator.apu.divider_apu = 7;
    emulator.apu.last_divider_time = 0b10011111;
    emulator.timers.divider = 0b10100000;

    emulator.apu.channel1.enabled = true;
    emulator.apu.channel1.dac_enabled = true;
    
    emulator.apu.channel1.period.divider = 742;
    emulator.apu.channel1.period.low = 26;
    emulator.apu.channel1.period.high = 197;
    
    emulator.cpu.clock.instruction_clock_cycles = 4;
    
    emulator.apu.channel1.envelope.initial_settings = 0b10100101;
    emulator.apu.channel1.envelope.current_volume = 0b1010;
    emulator.apu.channel1.envelope.timer = 0b1;

    step(&mut emulator);
    
    assert_eq!(emulator.apu.channel1.envelope.current_volume, 0b1001);
    assert_eq!(emulator.apu.channel1.envelope.timer, 0b101);
}

#[test]
fn should_decrement_ch1_envelope_timer() {
    let mut emulator = initialize_emulator();

    emulator.apu.audio_master_control = 0b10000001;
    emulator.apu.divider_apu = 7;
    emulator.apu.last_divider_time = 0b10011111;
    emulator.timers.divider = 0b10100000;

    emulator.apu.channel1.enabled = true;
    emulator.apu.channel1.dac_enabled = true;
    
    emulator.apu.channel1.period.divider = 742;
    emulator.apu.channel1.period.low = 26;
    emulator.apu.channel1.period.high = 197;
    
    emulator.cpu.clock.instruction_clock_cycles = 4;
    
    emulator.apu.channel1.envelope.initial_settings = 0b10100101;
    emulator.apu.channel1.envelope.current_volume = 0b1010;
    emulator.apu.channel1.envelope.timer = 0b101;

    step(&mut emulator);
    
    assert_eq!(emulator.apu.channel1.envelope.current_volume, 0b1010);
    assert_eq!(emulator.apu.channel1.envelope.timer, 0b100);
}


#[test]
fn should_not_step_ch1_envelope_if_divider_apu_is_on_wrong_step() {
    let mut emulator = initialize_emulator();

    emulator.apu.audio_master_control = 0b10000001;
    emulator.apu.divider_apu = 4;
    emulator.apu.last_divider_time = 0b10011111;
    emulator.timers.divider = 0b10100000;

    emulator.apu.channel1.enabled = true;
    emulator.apu.channel1.dac_enabled = true;
    
    emulator.apu.channel1.period.divider = 742;
    emulator.apu.channel1.period.low = 26;
    emulator.apu.channel1.period.high = 197;
    
    emulator.cpu.clock.instruction_clock_cycles = 4;
    
    emulator.apu.channel1.envelope.initial_settings = 0b10100101;
    emulator.apu.channel1.envelope.current_volume = 0b1010;
    emulator.apu.channel1.envelope.timer = 0b101;

    step(&mut emulator);
    
    assert_eq!(emulator.apu.channel1.envelope.current_volume, 0b1010);
    assert_eq!(emulator.apu.channel1.envelope.timer, 0b101);
}
