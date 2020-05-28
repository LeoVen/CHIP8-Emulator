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

    /// The register 0xF is set when there is a carry during +
    pub fn set_carry(&mut self, val: bool) {
        self.v[0xF] = val as u8;
    }

    /// The register 0xF is set to 0 when there is a borrow
    pub fn set_borrow(&mut self, val: bool) {
        self.v[0xF] = val as u8;
    }

    #[allow(dead_code)]
    pub fn dump(&self) {
        println!(
            "Cpu {{ i: {}, pc: {:#X}, v: {:?} }}",
            self.i, self.pc, self.v
        );
    }
}
