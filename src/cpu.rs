use crate::memory::Memory;
use crate::opcode::{Nibble, Opcode};
use crate::stack::Stack;

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

    pub fn run_instruction(&mut self, mem: &mut Memory, st: &mut Stack) {
        let l = mem.read_byte(self.pc) as u16;
        let h = mem.read_byte(self.pc + 1) as u16;

        // An instruction is based on two bytes
        let opcode = Opcode::new((l << 8) | h);

        // Match the first nibble
        match opcode[Nibble::A] {
            0x0 => {
                match opcode.get(Nibble::CD) {
                    0xE0 => {
                        // 0x00E0 -> Clear display
                        println!("{:#X}  CLS", self.pc);
                    }
                    0xEE => {
                        // 0x00EE -> Return from subroutine
                        println!("{:#X}  RET", self.pc);
                        self.pc = st.pop();
                    },
                    _ => eprintln!("Unknown opcode for 0x0 at {} -> {}", self.pc, opcode),
                }
                self.next_i();
            },
            0x1 => {
                // 0x1NNN -> Jump to address NNN
                println!("{:#X}:  JMP     {:#X}", self.pc, opcode.get(Nibble::BCD));
                self.pc = opcode.get(Nibble::BCD);
            },
            0x2 => {
                // 0x2NNN -> Call subroutine at NNN
                println!("{:#X}: CALL     {:#X}", self.pc, opcode.get(Nibble::BCD));
                st.push(self.pc + 2);
                self.pc = opcode.get(Nibble::BCD);
            },
            0x3 => {
                // 0x3XNN -> Skips the next instruction if VX equals NN
                println!("{:#X}:   SE    V{}, {:#X}", self.pc, opcode[Nibble::B], opcode.get(Nibble::CD));
                if self.v[opcode[Nibble::B] as usize] as u16 == opcode.get(Nibble::CD) {
                    self.skip_i();
                } else {
                    self.next_i();
                }
            },
            0x4 => {
                // 0x4XNN -> Skips the next instruction if VX doesn't equal NN
                println!("{:#X}:  SNE    V{}, {:#X}", self.pc, opcode[Nibble::B], opcode.get(Nibble::CD));
                if self.v[opcode[Nibble::B] as usize] as u16 != opcode.get(Nibble::CD) {
                    self.skip_i();
                } else {
                    self.next_i();
                }
            },
            0x5 => {
                // 0x5XY0 -> Skips the next instruction if VX == VY
                // Here, 0x000X is ignored
                println!("{:#X}:   SE    V{}, V{}", self.pc, opcode[Nibble::B], opcode[Nibble::C]);
                self.next_i();
            },
            0x6 => {
                // 0x6XNN -> Sets VX to NN
                println!("{:#X}:   LD    V{}, {:#X}", self.pc, opcode[Nibble::B], opcode.get(Nibble::CD));
                self.v[opcode[Nibble::B] as usize] = opcode.get(Nibble::CD) as u8;
                self.next_i();
            },
            0x7 => {
                // 0x7XNN -> Add NN to VX
                println!("{:#X}:  ADD    V{}, {:#X}", self.pc, opcode[Nibble::B], opcode.get(Nibble::CD));
                self.next_i();
            },
            0xA => {
                // 0xANNN -> Set I to NNN
                println!("{:#X}:   LD    I to {:#X}", self.pc, opcode.get(Nibble::BCD));
                self.next_i();
            },
            _ => {
                eprintln!("Unknown opcode at {:#X} -> {}", self.pc, opcode);
                self.next_i();
            }
        }
    }

    fn next_i(&mut self) {
        self.pc += 2;
    }

    fn skip_i(&mut self) {
        self.pc += 4;
    }

    #[allow(dead_code)]
    pub fn dump(&self) {
        println!("{{ i : {} }} {{ pc: {} }}", self.i, self.pc);
        println!("{:?}", self.v);
    }
}
