#[derive(Debug)]
pub struct Cpu {
    /// index register
    pub i: u16,
    /// program counter
    pub pc: u16,
    /// registers
    pub v: [u8; 16],
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            i: 0,
            pc: 0x200,
            v: [0; 16],
        }
    }

    /// Goes to the next instruction
    pub fn next_instruction(&mut self) {
        self.pc += 2;
    }

    /// Skips one instruction and goes to the next one
    pub fn skip_instruction(&mut self) {
        self.pc += 4;
    }

    /// Writes a value val to a register v
    pub fn write_register(&mut self, v: u16, val: u8) {
        self.v[v as usize] = val;
    }

    #[allow(dead_code)]
    pub fn dump(&self) {
        println!(
            "Cpu {{ i: {}, pc: {:#X}, v: {:?} }}",
            self.i, self.pc, self.v
        );
    }
}
