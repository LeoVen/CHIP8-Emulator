use crate::memory::Memory;

pub struct Cpu {
    // index register
    pub i: u16,
    // program counter
    pub pc: u16,
    // memory
    pub memory: Memory,
    // registers
    pub reg: [u8; 16],
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            i: 0,
            pc: 512,
            memory: Memory::new(),
            reg: [0; 16]
        }
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) {
        for i in 0..data.len() {
            self.memory.write_byte(self.memory.offset + i as u16, data[i]);
        }
    }

    pub fn dump(&self) {
        self.memory.dump();
    }
}
