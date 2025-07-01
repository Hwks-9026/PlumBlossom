use crate::debugtools::mem_dump;
pub(crate) struct Memory {
    raw_memory: Vec<u8>,
}

impl Memory {
    pub fn new(rom: Vec<u8>) -> Option<Memory> {
        if rom.len() != 4_294_967_296 {
            eprintln!("Rom Size Incorrect | Binary File to Load to Memory");
            return None;
        }
        return Some(Memory { raw_memory: rom });
    }
    pub fn read_byte(&self, index: u32) -> u8 {
        return self.raw_memory[index as usize];
    }

    pub fn read_word(&self, index: u32) -> u32 {
        let mut word: u32 = self.read_byte(index) as u32;
        word = word << 8;
        word += self.read_byte(index + 1) as u32;
        word = word << 8;
        word += self.read_byte(index + 2) as u32;
        word = word << 8;
        word += self.read_byte(index + 3) as u32;
        return word;
    }

    pub fn write_word(&mut self, index: u32, word: u32) {
        self.write_byte(index, ((word >> 24) & 0xFF) as u8);
        self.write_byte(index + 1, ((word >> 16) & 0xFF) as u8);
        self.write_byte(index + 2, ((word >> 8) & 0xFF) as u8);
        self.write_byte(index + 3, (word & 0xFF) as u8);
    }

    pub fn write_byte(&mut self, index: u32, byte: u8) {
        self.raw_memory[index as usize] = byte;
    }
    pub(crate) fn debug_print(&self) {
        mem_dump(&self.raw_memory);
    }
    pub(crate) fn get_frame(&self) -> Vec<u8> {
        let mem_offset = 0x40;
        println!(
            "{}, {}",
            self.raw_memory[mem_offset],
            self.raw_memory[mem_offset + 1228799]
        );
        let data = &self.raw_memory[mem_offset..(mem_offset + 1228800)];
        data.to_vec()
    }
}
