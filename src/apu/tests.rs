use crate::emulator::initialize_emulator;
use super::*;

#[test]
fn should_not_decrement_period_divider_when_apu_is_off() {
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
fn should_not_decrement_period_divider_if_channel_1_is_off() {
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
fn should_decrement_period_divider_for_channel_1() {
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
fn should_decrement_period_divider_for_channel_2() {
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
fn should_properly_wrap_period_divider_value_when_decrementing_it() {
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
    emulator.apu.channel1.wave_duty_position = 0;
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
fn should_trigger_channel_1_when_writing_to_channel_1_period_high() {
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
fn should_trigger_channel_2_when_writing_to_channel_2_period_high() {
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
fn should_not_trigger_channel_1_if_trigger_bit_is_not_set_when_writing_to_channel_1_period_high() {
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
fn should_not_trigger_channel_1_if_dac_is_disabled_when_writing_to_channel_1_period_high() {
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
fn should_disable_dac_and_channel_1_when_writing_to_channel_1_volume() {
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
fn should_disable_dac_and_chanel_2_when_writing_to_channel_2_volume() {
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
fn should_not_disable_dac_if_bits_three_through_seven_have_values_when_writing_to_channel_1_volume() {
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
fn should_update_channel_1_envelope_volume_and_reset_timer_when_timer_decrements_to_zero() {
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
fn should_decrement_channel_1_envelope_timer() {
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
fn should_not_step_channel_1_envelope_if_divider_apu_is_on_wrong_step() {
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

#[test]
fn should_step_channel_1_length_timer() {
    let mut emulator = initialize_emulator();

    emulator.apu.audio_master_control = 0b10000001;
    emulator.apu.divider_apu = 0;
    emulator.apu.last_divider_time = 0b10011111;
    emulator.timers.divider = 0b10100000;

    emulator.apu.channel1.enabled = true;
    emulator.apu.channel1.dac_enabled = true;
    
    emulator.apu.channel1.period.divider = 742;
    emulator.apu.channel1.period.low = 26;
    emulator.apu.channel1.period.high = 197;
    
    emulator.cpu.clock.instruction_clock_cycles = 4;
    
    emulator.apu.channel1.length.initial_settings = 0b01001110;
    emulator.apu.channel1.length.timer = 0b00000110;
    emulator.apu.channel1.period.high = 0b11000110;

    step(&mut emulator);
    
    assert_eq!(emulator.apu.channel1.length.timer, 0b00000101);
}

#[test]
fn should_not_step_channel_1_length_timer_if_divider_apu_is_on_wrong_step() {
    let mut emulator = initialize_emulator();

    emulator.apu.audio_master_control = 0b10000001;
    emulator.apu.divider_apu = 1;
    emulator.apu.last_divider_time = 0b10011111;
    emulator.timers.divider = 0b10100000;

    emulator.apu.channel1.enabled = true;
    emulator.apu.channel1.dac_enabled = true;
    
    emulator.apu.channel1.period.divider = 742;
    emulator.apu.channel1.period.low = 26;
    emulator.apu.channel1.period.high = 197;
    
    emulator.cpu.clock.instruction_clock_cycles = 4;
    
    emulator.apu.channel1.length.initial_settings = 0b01001110;
    emulator.apu.channel1.length.timer = 0b00000110;
    emulator.apu.channel1.period.high = 0b11000110;

    step(&mut emulator);
    
    assert_eq!(emulator.apu.channel1.length.timer, 0b00000110);
}

#[test]
fn should_disable_channel_1_when_length_timer_reaches_zero() {
    let mut emulator = initialize_emulator();

    emulator.apu.audio_master_control = 0b10000001;
    emulator.apu.divider_apu = 0;
    emulator.apu.last_divider_time = 0b10011111;
    emulator.timers.divider = 0b10100000;

    emulator.apu.channel1.enabled = true;
    emulator.apu.channel1.dac_enabled = true;
    
    emulator.apu.channel1.period.divider = 742;
    emulator.apu.channel1.period.low = 26;
    emulator.apu.channel1.period.high = 197;
    
    emulator.cpu.clock.instruction_clock_cycles = 4;
    
    emulator.apu.channel1.length.initial_settings = 0b01001110;
    emulator.apu.channel1.length.timer = 0b1;
    emulator.apu.channel1.period.high = 0b11000110;

    step(&mut emulator);
    
    assert_eq!(emulator.apu.channel1.length.timer, 0);
    assert_eq!(emulator.apu.channel1.enabled, false);
    assert_eq!(emulator.apu.channel1.dac_enabled, false);
}

#[test]
fn should_initialize_length_timer_when_channel_1_is_triggered() {
    let mut emulator = initialize_emulator();
    emulator.apu.audio_master_control = 0b10000000;
    emulator.apu.channel1.dac_enabled = true;
    emulator.apu.channel1.enabled = false;
    emulator.apu.channel1.length.initial_settings = 0b01101010;
    set_ch1_period_high(&mut emulator, 0b11000000);
    assert_eq!(emulator.apu.audio_master_control, 0b10000001);
    assert_eq!(emulator.apu.channel1.enabled, true);
    assert_eq!(emulator.apu.channel1.period.high, 0b11000000);
    assert_eq!(emulator.apu.channel1.length.timer, 0b00010110);
}

#[test]
fn should_decrement_channel_1_sweep_timer() {
    let mut emulator = initialize_emulator();

    emulator.apu.audio_master_control = 0b10000001;
    emulator.apu.divider_apu = 2;
    emulator.apu.last_divider_time = 0b10011111;
    emulator.timers.divider = 0b10100000;

    emulator.apu.channel1.enabled = true;
    emulator.apu.channel1.dac_enabled = true;
    
    emulator.apu.channel1.period.divider = 742;
    emulator.apu.channel1.period.low = 0b00011010;
    emulator.apu.channel1.period.high = 0b10000101;
    
    emulator.cpu.clock.instruction_clock_cycles = 4;

    emulator.apu.channel1.sweep.initial_settings = 0b00100010;
    emulator.apu.channel1.sweep.enabled = true;
    emulator.apu.channel1.sweep.timer = 0b10;
    emulator.apu.channel1.sweep.shadow_frequency = 0b10100011010;

    step(&mut emulator);
    
    assert_eq!(emulator.apu.channel1.sweep.timer, 0b01);
}

#[test]
fn should_disable_channel_1_on_sweep_overflow() {
    let mut emulator = initialize_emulator();

    emulator.apu.audio_master_control = 0b10000001;
    emulator.apu.divider_apu = 2;
    emulator.apu.last_divider_time = 0b10011111;
    emulator.timers.divider = 0b10100000;

    emulator.apu.channel1.enabled = true;
    emulator.apu.channel1.dac_enabled = true;
    
    emulator.apu.channel1.period.divider = 742;
    emulator.apu.channel1.period.low = 0b11111110;
    emulator.apu.channel1.period.high = 0b10000111;
    
    emulator.cpu.clock.instruction_clock_cycles = 4;

    emulator.apu.channel1.sweep.initial_settings = 0b00100010;
    emulator.apu.channel1.sweep.enabled = true;
    emulator.apu.channel1.sweep.timer = 0b01;
    emulator.apu.channel1.sweep.shadow_frequency = 0b11111111110;

    step(&mut emulator);
    
    assert_eq!(emulator.apu.channel1.enabled, false);
    assert_eq!(emulator.apu.channel1.dac_enabled, false);
}

#[test]
fn should_reload_sweep_timer_and_frequency_when_timer_reaches_zero() {
    let mut emulator = initialize_emulator();

    emulator.apu.audio_master_control = 0b10000001;
    emulator.apu.divider_apu = 2;
    emulator.apu.last_divider_time = 0b10011111;
    emulator.timers.divider = 0b10100000;

    emulator.apu.channel1.enabled = true;
    emulator.apu.channel1.dac_enabled = true;
    
    emulator.apu.channel1.period.divider = 742;
    emulator.apu.channel1.period.low = 0b00011010;
    emulator.apu.channel1.period.high = 0b10000101;
    
    emulator.cpu.clock.instruction_clock_cycles = 4;

    emulator.apu.channel1.sweep.initial_settings = 0b00100010;
    emulator.apu.channel1.sweep.enabled = true;
    emulator.apu.channel1.sweep.timer = 0b01;
    emulator.apu.channel1.sweep.shadow_frequency = 0b10100011010;

    step(&mut emulator);
    
    assert_eq!(emulator.apu.channel1.sweep.timer, 0b010);
    assert_eq!(emulator.apu.channel1.sweep.shadow_frequency, 0b11001100000);
    assert_eq!(emulator.apu.channel1.period.low, 0b01100000);
    assert_eq!(emulator.apu.channel1.period.high, 0b10000110);
}

#[test]
fn should_properly_initialize_sweep_timer_and_shadow_frequency_on_trigger() {
    let mut emulator = initialize_emulator();

    emulator.apu.audio_master_control = 0b10000000;
    emulator.apu.channel1.dac_enabled = true;
    emulator.apu.channel1.enabled = false;

    emulator.apu.channel1.period.divider = 742;
    emulator.apu.channel1.period.low = 0b00011010;
    
    emulator.cpu.clock.instruction_clock_cycles = 4;

    emulator.apu.channel1.sweep.initial_settings = 0b00100010;

    set_ch1_period_high(&mut emulator, 0b10100010);

    assert_eq!(emulator.apu.channel1.sweep.timer, 0b010);
    assert_eq!(emulator.apu.channel1.sweep.shadow_frequency, 0b01000011010);
    assert_eq!(emulator.apu.channel1.sweep.enabled, true);
}

#[test]
fn should_decrement_period_divider_for_channel_3() {
    let mut emulator = initialize_emulator();
    emulator.apu.audio_master_control = 0b10000100;
    emulator.apu.channel3.dac_enabled = true;
    emulator.apu.channel3.enabled = true;
    emulator.apu.channel3.period.divider = 742;
    emulator.apu.channel3.period.low = 26;
    emulator.apu.channel3.period.high = 197;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.apu.channel3.period.divider, 740);
}

#[test]
fn should_reload_period_divider_once_it_reaches_zero_for_channel_3() {
    let mut emulator = initialize_emulator();
    emulator.apu.audio_master_control = 0b10000001;
    emulator.apu.channel3.enabled = true;
    emulator.apu.channel3.dac_enabled = true;
    emulator.apu.channel3.period.divider = 2;
    emulator.apu.channel3.period.low = 26;
    emulator.apu.channel3.period.high = 197;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.apu.channel3.period.divider, 742);
}

#[test]
fn should_increment_wave_position_when_period_divider_reaches_zero_for_channel_3() {
    let mut emulator = initialize_emulator();
    emulator.apu.audio_master_control = 0b10000100;
    emulator.apu.channel3.enabled = true;
    emulator.apu.channel3.dac_enabled = true;
    emulator.apu.channel3.period.divider = 2;
    emulator.apu.channel3.period.low = 26;
    emulator.apu.channel3.period.high = 197;
    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    assert_eq!(emulator.apu.channel3.wave_position, 1);
}

#[test]
fn should_trigger_channel_3_when_writing_to_channel_3_period_high() {
    let mut emulator = initialize_emulator();
    emulator.apu.audio_master_control = 0b10000000;
    emulator.apu.channel3.dac_enabled = true;
    emulator.apu.channel3.enabled = false;
    set_ch3_period_high(&mut emulator, 0b10000000);
    assert_eq!(emulator.apu.audio_master_control, 0b10000100);
    assert_eq!(emulator.apu.channel3.enabled, true);
    assert_eq!(emulator.apu.channel3.period.high, 0b10000000);
}

#[test]
fn should_disable_channel_3_when_restting_bit_7_of_dac_enabled_register() {
    let mut emulator = initialize_emulator();
    emulator.apu.audio_master_control = 0b10000100;
    emulator.apu.channel3.dac_enabled = true;
    emulator.apu.channel3.enabled = true;
    set_ch3_dac_enabled(&mut emulator, 0);
    assert_eq!(emulator.apu.audio_master_control, 0b10000000);
    assert_eq!(emulator.apu.channel3.enabled, false);
}

#[test]
fn should_initialize_length_timer_when_channel_3_is_triggered() {
    let mut emulator = initialize_emulator();
    emulator.apu.audio_master_control = 0b10000000;
    emulator.apu.channel3.dac_enabled = true;
    emulator.apu.channel3.enabled = false;
    emulator.apu.channel3.length.initial_settings = 0b01101010;
    set_ch3_period_high(&mut emulator, 0b11000000);
    assert_eq!(emulator.apu.audio_master_control, 0b10000100);
    assert_eq!(emulator.apu.channel3.enabled, true);
    assert_eq!(emulator.apu.channel3.period.high, 0b11000000);
    assert_eq!(emulator.apu.channel3.length.timer, 0b10010110);
}

#[test]
fn should_step_channel_3_length_timer() {
    let mut emulator = initialize_emulator();

    emulator.apu.audio_master_control = 0b10000100;
    emulator.apu.divider_apu = 0;
    emulator.apu.last_divider_time = 0b10011111;
    emulator.timers.divider = 0b10100000;

    emulator.apu.channel3.enabled = true;
    emulator.apu.channel3.dac_enabled = true;
    
    emulator.apu.channel3.period.divider = 742;
    emulator.apu.channel3.period.low = 26;
    emulator.apu.channel3.period.high = 197;
    
    emulator.cpu.clock.instruction_clock_cycles = 4;
    
    emulator.apu.channel3.length.initial_settings = 0b01001110;
    emulator.apu.channel3.length.timer = 0b00000110;
    emulator.apu.channel3.period.high = 0b11000110;

    step(&mut emulator);
    
    assert_eq!(emulator.apu.channel3.length.timer, 0b00000101);
}

fn initialize_noise_channel(emulator: &mut Emulator) {
    emulator.apu.audio_master_control = 0b10001000;
    emulator.apu.channel4.dac_enabled = true;
    emulator.apu.channel4.enabled = true; 
}

fn initialize_disabled_noise_channel(emulator: &mut Emulator) {
    emulator.apu.audio_master_control = 0b10000000;
    emulator.apu.channel4.dac_enabled = true;
    emulator.apu.channel4.enabled = false;
}

fn step_apu_multiple_times(emulator: &mut Emulator, n: u8) {
    for _ in 0..n {
        emulator.cpu.clock.instruction_clock_cycles = 4;
        step(emulator);
    }
}

#[test]
fn should_not_decrement_period_divider_for_channel_4_if_only_four_instruction_cycles() {
    let mut emulator = initialize_emulator();
    initialize_noise_channel(&mut emulator);

    emulator.apu.channel4.period_divider = 742;

    emulator.cpu.clock.instruction_clock_cycles = 4;
    step(&mut emulator);
    
    assert_eq!(emulator.apu.channel4.period_divider, 742);
}


#[test]
fn should_decrement_period_divider_for_channel_4_after_sixteen_instruction_cycles() {
    let mut emulator = initialize_emulator();
    initialize_noise_channel(&mut emulator);

    emulator.apu.channel4.period_divider = 742;

    step_apu_multiple_times(&mut emulator, 4);
    
    assert_eq!(emulator.apu.channel4.period_divider, 741); 
}

#[test]
fn should_reload_period_divider_for_channel_4_once_it_decrements_to_zero() {
    let mut emulator = initialize_emulator();
    initialize_noise_channel(&mut emulator);
    emulator.apu.channel4.period_divider = 1;

    // Base Divisor = 0b110 = 6 which maps to 96
    // Shift Amount = 0b0011 = 3
    // 96 << 3 = 768
    emulator.apu.channel4.polynomial = 0b00110110;

    step_apu_multiple_times(&mut emulator, 4);

    assert_eq!(emulator.apu.channel4.period_divider, 768); 
}

#[test]
fn should_calculate_next_lfsr_value_correctly_for_channel_4() {
    let mut emulator = initialize_emulator();
    initialize_noise_channel(&mut emulator);

    emulator.apu.channel4.period_divider = 1;
    emulator.apu.channel4.polynomial = 0b00110110;
    emulator.apu.channel4.lfsr = 0b110010100101101;

    step_apu_multiple_times(&mut emulator, 4);
    
    assert_eq!(emulator.apu.channel4.lfsr, 0b111001010010110);
}

#[test]
fn should_calculate_next_lfsr_value_correctly_in_width_mode_for_channel_4() {
    let mut emulator = initialize_emulator();
    initialize_noise_channel(&mut emulator);

    emulator.apu.channel4.period_divider = 1;
    emulator.apu.channel4.polynomial = 0b00111110;
    emulator.apu.channel4.lfsr = 0b110010100101101;

    step_apu_multiple_times(&mut emulator, 4);
    
    assert_eq!(emulator.apu.channel4.lfsr, 0b111001011010110);
}

#[test]
fn should_trigger_channel_4() {
    let mut emulator = initialize_emulator();
    initialize_disabled_noise_channel(&mut emulator);

    set_ch4_control(&mut emulator, 0b10000000);

    assert_eq!(emulator.apu.channel4.enabled, true);
}

#[test]
fn should_set_ch4_control() {
    let mut emulator = initialize_emulator();
    initialize_disabled_noise_channel(&mut emulator);

    set_ch4_control(&mut emulator, 0b10000000);

    assert_eq!(emulator.apu.channel4.control, 0b10000000);
}