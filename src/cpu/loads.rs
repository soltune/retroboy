use crate::cpu::{Register, RegisterPair, Cpu};

impl Cpu {
    pub fn load_immediate_value(&mut self, register: Register) {
        let immediate_byte = self.read_next_instruction_byte();
        self.store_in_register(register, immediate_byte);
    }

    pub fn load_source_register_in_destination_register(&mut self, source: Register, destination: Register) {
        let source_value = self.read_from_register(&source);
        self.store_in_register(destination, source_value);
    }

    pub fn load_memory_byte_in_destination_register(&mut self, address: u16, destination: Register) {
        let byte = self.read_byte_from_memory(address);
        self.store_in_register(destination, byte);
    }

    pub fn load_source_register_in_memory(&mut self, source: Register, address: u16) {
        let byte = self.read_from_register(&source);
        self.store_byte_in_memory(address, byte);
    }

    pub fn load_immediate_value_in_memory(&mut self, register_pair: RegisterPair) {
        let address = self.read_from_register_pair(&register_pair);
        let immediate_byte = self.read_next_instruction_byte();
        self.store_byte_in_memory(address, immediate_byte);
    }

    pub fn push_word_to_stack(&mut self, word: u16) {
        self.step_one_machine_cycle();
        self.registers.stack_pointer = self.registers.stack_pointer - 1;
        self.store_byte_in_memory(self.registers.stack_pointer, (word >> 8) as u8);
        self.registers.stack_pointer = self.registers.stack_pointer - 1;
        self.store_byte_in_memory(self.registers.stack_pointer, (word & 0xFF) as u8);
    }

    pub fn push_register_pair_to_stack(&mut self, register_pair: RegisterPair) {
        let word = self.read_from_register_pair(&register_pair);
        self.push_word_to_stack(word);
    }

    pub fn pop_word_from_stack(&mut self) -> u16 {
        let first_byte = self.read_byte_from_memory(self.registers.stack_pointer) as u16;
        self.registers.stack_pointer = self.registers.stack_pointer + 1;
        let second_byte = self.read_byte_from_memory(self.registers.stack_pointer) as u16;
        self.registers.stack_pointer = self.registers.stack_pointer + 1;
        (second_byte << 8) + first_byte
    }

    pub fn pop_word_into_register_pair_from_stack(&mut self, register_pair: RegisterPair) {
        let word = self.pop_word_from_stack();
        self.store_in_register_pair(register_pair, word);
    }
}