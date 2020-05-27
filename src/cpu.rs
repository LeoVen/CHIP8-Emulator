use crate::memory::Memory;
use crate::opcode::{Nibble, Opcode};

pub struct Cpu {
    // index register
    pub i: u16,
    // program counter
    pub pc: u16,
    // registers
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

    pub fn run_instruction(&mut self, mem: &mut Memory) {
        let l = mem.read_byte(self.pc) as u16;
        let h = mem.read_byte(self.pc + 1) as u16;

        // An instruction is based on two bytes
        let opcode = Opcode::new((l << 8) | h);

        println!("run {:b} -> {} -> {:#X} {:#X}", opcode, opcode, l, h);

        // Match the first nibble
        match opcode[Nibble::A] {
            0x0 => {
                // 0x00E0 -> Clear display
                // 0x00EE -> Return from subroutine
                match opcode.get(Nibble::CD) {
                    0xE0 => println!("Clear Display"),
                    0xEE => println!("Returns from a subroutine"),
                    _ => panic!("Unknown opcode for 0x0 at {} -> {}", self.pc, opcode),
                }
            },
            0x1 => {
                todo!()
            },
            _ => eprintln!("Unknown opcode at {} -> {}", self.pc, opcode),
        }

        self.pc += 2;
    }

    #[allow(dead_code)]
    pub fn dump(&self) {
        println!("{{ i : {} }} {{ pc: {} }}", self.i, self.pc);
    }
}
