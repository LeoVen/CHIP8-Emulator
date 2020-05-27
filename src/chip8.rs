use crate::cpu::Cpu;
use crate::memory::Memory;

pub struct Chip8 {
    // memory
    pub mem: Memory,
    // CPU
    cpu: Cpu,
    // stack
    pub stack: [u16; 16],
    // stack pointer
    pub sp: u16,
    // delay timer
    delay_timer: i32,
    // sound timer
    sound_timer: i32,
}

impl Chip8 {
    /// Creates a new Chip8
    pub fn new() -> Self {
        Self {
            mem: Memory::new(),
            cpu: Cpu::new(),
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    /// Loads a ROM provided by data
    pub fn load_rom(&mut self, data: &Vec<u8>) {
        for i in 0..data.len() {
            self.mem.write_byte(self.mem.offset + i as u16, data[i]);
        }
    }

    /// Runs an instruction for each cycle
    pub fn cycle(&mut self) {
        self.cpu.run_instruction(&mut self.mem);
    }

    #[allow(dead_code)]
    pub fn dump(&self) {
        self.cpu.dump();
        self.mem.dump();
    }
}
