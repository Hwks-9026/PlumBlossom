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
    pub fn write_byte(&mut self, index: u32, byte: u8) {
        self.raw_memory[index as usize] = byte;
    }
}
