#[allow(dead_code)]
use crate::memory::Memory;
use crate::{memory, registers::Registers};

struct Emulator {
    memory: Memory,
    registers: Registers,
}

pub(crate) fn start(rom: Vec<u8>) {
    let memory: Memory = match Memory::new(rom) {
        Some(mem) => mem,
        None => return,
    };
    let registers: Registers = Registers::new();
    let mut emulator = Emulator { memory, registers };
    emulator.run();
}

impl Emulator {
    fn run(&mut self) {
        let reset_vector = self.get_reset_vector();
        println!("val at reset_vector: {}", self.memory.read_byte(reset_vector));
    }

    fn get_reset_vector(&self) -> u32 {
        let mut reset_vector: u32 = self.memory.read_byte(0) as u32;
        reset_vector = reset_vector << 8;
        reset_vector += self.memory.read_byte(1) as u32;
        reset_vector = reset_vector << 8;
        reset_vector += self.memory.read_byte(2) as u32;
        reset_vector = reset_vector << 8;
        reset_vector += self.memory.read_byte(3) as u32;
        return reset_vector;
    }
}
