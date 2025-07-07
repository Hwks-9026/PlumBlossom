#[allow(dead_code)]
use crate::memory::Memory;
use crate::rasterize;
use crate::registers::Register;
use crate::registers::Registers;
use crate::instruction::*;

pub(crate) struct Emulator {
    halt: bool,
    pub(crate) memory: Memory,
    pub(crate) registers: Registers,
}

pub(crate) fn start(rom: Vec<u8>) {
    let memory: Memory = match Memory::new(rom) {
        Some(mem) => mem,
        None => return,
    };
    let registers: Registers = Registers::new();
    let mut emulator = Emulator {
        halt: false,
        memory,
        registers,
    };
    emulator.run();
}

impl Emulator {
    fn run(&mut self) {
        //self.memory.debug_print();
        let reset_vector = self.memory.read_word(0x00000000);
        self.registers.write(Register::IP as u32, reset_vector);
        while !self.halt {
            self.tick();
        }
    }

    fn tick(&mut self) {
        let instruction: u8 = self
            .memory
            .read_byte(self.registers.read(Register::IP as u32));
        let mut execution_words: Vec<u32> = Vec::new();
        match instruction >> 4 {
            0x00 => {}
            0x01 => execution_words.push(
                self.memory
                    .read_word(self.registers.read(Register::IP as u32) + 1),
            ),
            0x02 => {
                execution_words.push(
                    self.memory
                        .read_word(self.registers.read(Register::IP as u32) + 1),
                );
                execution_words.push(
                    self.memory
                        .read_word(self.registers.read(Register::IP as u32) + 5),
                )
            }
            0x03 => {
                execution_words.push(
                    self.memory
                        .read_word(self.registers.read(Register::IP as u32) + 1),
                );
                execution_words.push(
                    self.memory
                        .read_word(self.registers.read(Register::IP as u32) + 5),
                );
                execution_words.push(
                    self.memory
                        .read_word(self.registers.read(Register::IP as u32) + 9),
                )
            }
            _ => {}
        }
        self.registers.write(
            Register::IP as u32,
            self.registers.read(Register::IP as u32) + (execution_words.len() * 4) as u32 + 1);
        match execute_instruction(self, instruction, execution_words) {
            Ok(()) => {self.registers.write(Register::PC as u32, self.registers.read(Register::PC as u32) + 1);}
            Err(()) => self.halt = true,
        }

        if self.registers.read(Register::PC as u32) > 300 {
            self.halt = true;
        }
    }
}

fn execute_instruction(
    em: &mut Emulator,
    instruction: u8,
    execution_words: Vec<u32>,
) -> Result<(), ()> {
    match instruction {
    0x00=> {nop(em, execution_words)},
    0x01=> {jmp(em, execution_words)},
    0x02=> {cal(em, execution_words)},
    0x03=> {ret(em, execution_words)},
    0x04=> {
        let buffer = em.memory.get_frame();
        rasterize::blit(buffer);
    }
    0x05=> {println!("CPU HALTING!\nRegister Content: {:?}", em.registers); return Err(())},
    0x06=> {println!("Debug Register Print: {:?}", em.registers)},
    0x07=> {em.memory.debug_print();}
    0x10=> {jnz(em, execution_words)},
    0x11=> {jiz(em, execution_words)},
    0x12=> {psh(em, execution_words)},
    0x13=> {pop(em, execution_words)},
    0x14=> {not(em, execution_words)},
    0x15=> {shl(em, execution_words)},
    0x16=> {shr(em, execution_words)},
    0x20=> {mov(em, execution_words)},
    0x21=> {mvi(em, execution_words)},
    0x22=> {lda(em, execution_words)},
    0x23=> {sta(em, execution_words)},
    0x30=> {add(em, execution_words)},
    0x31=> {sub(em, execution_words)},
    0x32=> {mul(em, execution_words)},
    0x33=> {div(em, execution_words)},
    0x34=> {and(em, execution_words)},
    0x35=> {xor(em, execution_words)},
    _ => {return Err(())}
    }
    Ok(())
}
