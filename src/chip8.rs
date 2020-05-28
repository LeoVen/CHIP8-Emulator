use crate::cpu::Cpu;
use crate::memory::Memory;
use crate::opcode::{Nibble, Opcode};
use crate::stack::Stack;

pub struct Chip8 {
    /// Memory
    pub mem: Memory,
    /// CPU
    pub cpu: Cpu,
    /// Call stack
    pub stack: Stack,
    /// Delay timer
    delay_timer: i32,
    /// Sound timer
    sound_timer: i32,
}

impl Chip8 {
    /// Creates a new Chip8
    pub fn new() -> Self {
        Self {
            mem: Memory::new(),
            cpu: Cpu::new(),
            stack: Stack::new(),
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
        let l = self.mem.read_byte(self.cpu.pc) as u16;
        let h = self.mem.read_byte(self.cpu.pc + 1) as u16;

        // An instruction is based on two u8 (one u16)
        let opcode = Opcode::new((l << 8) | h);

        // Match the first nibble
        match opcode[Nibble::A] {
            0x0 => {
                match opcode.get(Nibble::CD) {
                    0xE0 => {
                        // 0x00E0 -> Clear display
                        println!("{:#X}\tCLS", self.cpu.pc);
                        // todo
                    }
                    0xEE => {
                        // 0x00EE -> Return from subroutine
                        println!("{:#X}\tRET", self.cpu.pc);
                        self.cpu.pc = self.stack.pop();
                    }
                    _ => eprintln!("Unknown opcode for 0x0 at {} -> {}", self.cpu.pc, opcode),
                }
                self.cpu.next_instruction();
            }
            0x1 => {
                // 0x1NNN -> Jump to address NNN
                println!("{:#X}:\tJMP\t{:#X}", self.cpu.pc, opcode.get(Nibble::BCD));
                self.cpu.pc = opcode.get(Nibble::BCD);
            }
            0x2 => {
                // 0x2NNN -> Call subroutine at NNN
                println!("{:#X}:\tCALL\t{:#X}", self.cpu.pc, opcode.get(Nibble::BCD));
                self.cpu.next_instruction();
                self.stack.push(self.cpu.pc);
                self.cpu.pc = opcode.get(Nibble::BCD);
            }
            0x3 => {
                // 0x3XNN -> Skips the next instruction if VX equals NN
                println!(
                    "{:#X}:\tSE\tV{},\t{:#X}",
                    self.cpu.pc,
                    opcode[Nibble::B],
                    opcode.get(Nibble::CD)
                );
                if self.cpu.v[opcode[Nibble::B] as usize] as u16 == opcode.get(Nibble::CD) {
                    self.cpu.skip_instruction();
                } else {
                    self.cpu.next_instruction();
                }
            }
            0x4 => {
                // 0x4XNN -> Skips the next instruction if VX doesn't equal NN
                println!(
                    "{:#X}:\tSNE\tV{},\t{:#X}",
                    self.cpu.pc,
                    opcode[Nibble::B],
                    opcode.get(Nibble::CD)
                );
                if self.cpu.v[opcode[Nibble::B] as usize] as u16 != opcode.get(Nibble::CD) {
                    self.cpu.skip_instruction();
                } else {
                    self.cpu.next_instruction();
                }
            }
            0x5 => {
                // 0x5XY0 -> Skips the next instruction if VX == VY
                // Here, 0x000X is ignored
                println!(
                    "{:#X}:\tSE\tV{},\tV{}",
                    self.cpu.pc,
                    opcode[Nibble::B],
                    opcode[Nibble::C]
                );
                // todo
                self.cpu.next_instruction();
            }
            0x6 => {
                // 0x6XNN -> VX = NN
                println!(
                    "{:#X}:\tLD\tV{},\t{:#X}",
                    self.cpu.pc,
                    opcode[Nibble::B],
                    opcode.get(Nibble::CD)
                );
                self.cpu
                    .write_register(opcode[Nibble::B], opcode.get(Nibble::CD) as u8);
                self.cpu.next_instruction();
            }
            0x7 => {
                // 0x7XNN -> VX += NN
                println!(
                    "{:#X}:\tADD\tV{},\t{:#X}",
                    self.cpu.pc,
                    opcode[Nibble::B],
                    opcode.get(Nibble::CD)
                );
                // todo replace by self.cup.write_instruction
                self.cpu.v[opcode[Nibble::B] as usize] += opcode.get(Nibble::CD) as u8;
                self.cpu.next_instruction();
            }
            0x8 => {
                let vx = opcode[Nibble::B];
                let vy = opcode[Nibble::C];
                let vx_value = self.cpu.v[vx as usize];
                let vy_value = self.cpu.v[vy as usize];
                match opcode[Nibble::D] {
                    // [0, 1, 2, 3, 4, 5, 6, 7, E]
                    0x0 => {
                        // 0x8XY0 -> VX = VY
                        println!("{:#X}:\tLD\tV{},\tV{}", self.cpu.pc, vx, vy);
                        self.cpu.write_register(vx, vy_value);
                    }
                    0x1 => {
                        // 0x8XY1 -> VX = VX | VY
                        println!("{:#X}:\tOR\tV{},\tV{}", self.cpu.pc, vx, vy);
                        self.cpu.write_register(vx, vx_value | vy_value);
                    }
                    0x2 => {
                        // 0x8XY2 -> VX = VX & VY
                        println!("{:#X}:\tAND\tV{},\tV{}", self.cpu.pc, vx, vy);
                        self.cpu.write_register(vx, vx_value & vy_value);
                    }
                    0x3 => {
                        // 0x8XY3 -> VX = VX ^ VY
                        println!("{:#X}:\tXOR\tV{},\tV{}", self.cpu.pc, vx, vy);
                        self.cpu.write_register(vx, vx_value ^ vy_value);
                    }
                    0x4 => {
                        // 0x8XY4 -> VX += VY
                        println!("{:#X}:\tADD\tV{},\tV{}", self.cpu.pc, vx, vy);
                        let sum: u16 = vx_value as u16 + vy_value as u16;
                        self.cpu.write_register(vx, sum as u8);
                        self.cpu.set_carry(sum > 0xFF); // todo don't use this anymore
                    }
                    0x5 => {
                        // 0x8XY5 -> VX -= VY
                        println!("{:#X}:\tSUB\tV{},\tV{}", self.cpu.pc, vx, vy);
                        let sub: i8 = vx_value as i8 - vy_value as i8;
                        self.cpu.write_register(vx, sub as u8);
                        self.cpu.set_borrow(vx_value < vy_value); // todo don't use this anymore
                    }
                    0x6 => {
                        // 0x8XY6 -> VX >>= 1
                        println!("{:#X}:\tSHR\tV{},\tV{}", self.cpu.pc, vx, vy);
                        self.cpu.write_register(vx, vx_value >> 1);
                        self.cpu.write_register(0xF, vx_value & 0x1);
                    }
                    0x7 => {
                        // 0x8XY7 -> VX = VY - VX
                        println!("{:#X}:\tSUBN\tV{},\tV{}", self.cpu.pc, vx, vy);
                        let sub: i8 = vy_value as i8 - vx_value as i8;
                        self.cpu.write_register(vx, sub as u8);
                        self.cpu.set_borrow(vy_value < vx_value); // todo don't use this anymore
                    }
                    0xE => {
                        // 0x8XYE -> VX <<= 1
                        println!("{:#X}:\tSHL\tV{},\tV{}", self.cpu.pc, vx, vy);
                        self.cpu.write_register(vx, vx_value << 1);
                        self.cpu.write_register(0xF, vx_value >> 7);
                    }
                    _ => eprintln!("Unknown opcode for 0x8 at {} -> {}", self.cpu.pc, opcode),
                }
            }
            0x9 => {
                // 0x9XY0 -> Skip next instruction if VX != VY
                todo!();
            }
            0xA => {
                // 0xANNN -> Set I to NNN
                println!(
                    "{:#X}:\tLD\tI,\t{:#X}",
                    self.cpu.pc,
                    opcode.get(Nibble::BCD)
                );
                self.cpu.i = opcode.get(Nibble::BCD);
                self.cpu.next_instruction();
            }
            // todo
            _ => {
                eprintln!("Unknown opcode at {:#X} -> {}", self.cpu.pc, opcode);
                self.cpu.next_instruction();
            }
        }
    }

    #[allow(dead_code)]
    pub fn dump(&self) {
        self.mem.dump();
        self.cpu.dump();
        self.stack.dump();
    }
}
