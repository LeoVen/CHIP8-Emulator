use rand::prelude::{thread_rng, Rng, ThreadRng};

use crate::cpu::Cpu;
use crate::display::Display;
use crate::memory::Memory;
use crate::opcode::{Nibble, Opcode};
use crate::stack::Stack;

pub struct Chip8<D: Display + Default> {
    /// Memory
    pub mem: Memory,
    /// CPU
    pub cpu: Cpu,
    /// Call stack
    pub stack: Stack,
    /// The emulator's display
    pub display: D,
    /// A random number generator provided by Rust
    pub rng: ThreadRng,
    /// Delay timer
    delay_timer: i32,
    /// Sound timer
    sound_timer: i32,
    /// Useful debugging information
    debug: bool,
}

impl<D> Chip8<D>
where
    D: Display + Default,
{
    /// Creates a new Chip8
    pub fn new() -> Self {
        Self {
            mem: Memory::new(),
            cpu: Cpu::new(),
            stack: Stack::new(),
            display: D::default(),
            rng: thread_rng(),
            delay_timer: 0,
            sound_timer: 0,
            debug: false,
        }
    }

    pub fn debug(mut self) -> Self {
        self.debug = true;
        self
    }

    pub fn no_display(mut self) -> Self {
        self.display.should_update(false);
        self
    }

    pub fn run(&mut self) {
        // todo add loop based on display
        let max = 5000;
        let mut i = 0;
        while self.display.is_open() {
            self.cycle();
            i += 1;
            if i == max {
                break;
            }
        }
    }

    /// Loads a ROM provided by data
    pub fn load_rom(&mut self, data: &Vec<u8>) {
        for i in 0..data.len() {
            self.mem.write_byte(self.mem.offset + i as u16, data[i]);
        }
    }

    /// Sets the display on or off
    #[allow(dead_code)]
    pub fn set_display(&mut self, on: bool) {
        self.display.should_update(on);
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
                        if self.debug {
                            println!("{:#X}\tCLS", self.cpu.pc);
                        }
                        self.display.clear();
                    }
                    0xEE => {
                        // 0x00EE -> Return from subroutine
                        if self.debug {
                            println!("{:#X}\tRET", self.cpu.pc);
                        }
                        self.cpu.pc = self.stack.pop();
                    }
                    _ => match opcode.get(Nibble::ABCD) {
                        0x0000 => {
                            if self.debug {
                                println!("{:#X}:\tNOP", self.cpu.pc);
                            }
                        }
                        _ => {
                            if self.debug {
                                eprintln!("Unknown opcode for 0x0 at {} -> {}", self.cpu.pc, opcode)
                            }
                        }
                    },
                }
                self.cpu.next_instruction();
            }
            0x1 => {
                // 0x1NNN -> Jump to address NNN
                if self.debug {
                    println!("{:#X}:\tJP\t{:#X}", self.cpu.pc, opcode.get(Nibble::BCD));
                }
                self.cpu.pc = opcode.get(Nibble::BCD);
            }
            0x2 => {
                // 0x2NNN -> Call subroutine at NNN
                if self.debug {
                    println!("{:#X}:\tCALL\t{:#X}", self.cpu.pc, opcode.get(Nibble::BCD));
                }
                self.cpu.next_instruction();
                self.stack.push(self.cpu.pc);
                self.cpu.pc = opcode.get(Nibble::BCD);
            }
            0x3 => {
                // 0x3XNN -> Skips the next instruction if VX equals NN
                if self.debug {
                    println!(
                        "{:#X}:\tSE\tV{},\t{:#X}",
                        self.cpu.pc,
                        opcode[Nibble::B],
                        opcode.get(Nibble::CD)
                    );
                }
                if self.cpu.v[opcode[Nibble::B] as usize] as u16 == opcode.get(Nibble::CD) {
                    self.cpu.skip_instruction();
                } else {
                    self.cpu.next_instruction();
                }
            }
            0x4 => {
                // 0x4XNN -> Skips the next instruction if VX doesn't equal NN
                if self.debug {
                    println!(
                        "{:#X}:\tSNE\tV{},\t{:#X}",
                        self.cpu.pc,
                        opcode[Nibble::B],
                        opcode.get(Nibble::CD)
                    );
                }
                if self.cpu.v[opcode[Nibble::B] as usize] as u16 != opcode.get(Nibble::CD) {
                    self.cpu.skip_instruction();
                } else {
                    self.cpu.next_instruction();
                }
            }
            0x5 => {
                // 0x5XY0 -> Skips the next instruction if VX == VY
                // Here, 0x000X is ignored
                if self.debug {
                    println!(
                        "{:#X}:\tSE\tV{},\tV{}",
                        self.cpu.pc,
                        opcode[Nibble::B],
                        opcode[Nibble::C]
                    );
                }
                if self.cpu.v[opcode[Nibble::B] as usize] == self.cpu.v[opcode[Nibble::C] as usize]
                {
                    self.cpu.skip_instruction();
                } else {
                    self.cpu.next_instruction();
                }
            }
            0x6 => {
                // 0x6XNN -> VX = NN
                if self.debug {
                    println!(
                        "{:#X}:\tLD\tV{},\t{:#X}",
                        self.cpu.pc,
                        opcode[Nibble::B],
                        opcode.get(Nibble::CD)
                    );
                }
                self.cpu
                    .write_register(opcode[Nibble::B], opcode.get(Nibble::CD) as u8);
                self.cpu.next_instruction();
            }
            0x7 => {
                // 0x7XNN -> VX += NN
                if self.debug {
                    println!(
                        "{:#X}:\tADD\tV{},\t{:#X}",
                        self.cpu.pc,
                        opcode[Nibble::B],
                        opcode.get(Nibble::CD)
                    );
                }
                self.cpu.write_register(
                    opcode[Nibble::B],
                    (self.cpu.v[opcode[Nibble::B] as usize] as u16 + opcode.get(Nibble::CD)) as u8,
                );
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
                        if self.debug {
                            println!("{:#X}:\tLD\tV{},\tV{}", self.cpu.pc, vx, vy);
                        }
                        self.cpu.write_register(vx, vy_value);
                    }
                    0x1 => {
                        // 0x8XY1 -> VX = VX | VY
                        if self.debug {
                            println!("{:#X}:\tOR\tV{},\tV{}", self.cpu.pc, vx, vy);
                        }
                        self.cpu.write_register(vx, vx_value | vy_value);
                    }
                    0x2 => {
                        // 0x8XY2 -> VX = VX & VY
                        if self.debug {
                            println!("{:#X}:\tAND\tV{},\tV{}", self.cpu.pc, vx, vy);
                        }
                        self.cpu.write_register(vx, vx_value & vy_value);
                    }
                    0x3 => {
                        // 0x8XY3 -> VX = VX ^ VY
                        if self.debug {
                            println!("{:#X}:\tXOR\tV{},\tV{}", self.cpu.pc, vx, vy);
                        }
                        self.cpu.write_register(vx, vx_value ^ vy_value);
                    }
                    0x4 => {
                        // 0x8XY4 -> VX += VY
                        if self.debug {
                            println!("{:#X}:\tADD\tV{},\tV{}", self.cpu.pc, vx, vy);
                        }
                        let sum: u16 = vx_value as u16 + vy_value as u16;
                        self.cpu.write_register(vx, sum as u8);
                        self.cpu.v[0xF] = (sum > 0xFF) as u8;
                    }
                    0x5 => {
                        // 0x8XY5 -> VX = VX - VY
                        if self.debug {
                            println!("{:#X}:\tSUB\tV{},\tV{}", self.cpu.pc, vx, vy);
                        }
                        let sub: i8 = vx_value as i8 - vy_value as i8;
                        self.cpu.write_register(vx, sub as u8);
                        self.cpu.write_register(0xF, (vx_value < vy_value) as u8);
                    }
                    0x6 => {
                        // 0x8XY6 -> VX >>= 1
                        if self.debug {
                            println!("{:#X}:\tSHR\tV{},\tV{}", self.cpu.pc, vx, vy);
                        }
                        self.cpu.write_register(vx, vx_value >> 1);
                        self.cpu.write_register(0xF, vx_value & 0x1);
                    }
                    0x7 => {
                        // 0x8XY7 -> VX = VY - VX
                        if self.debug {
                            println!("{:#X}:\tSUBN\tV{},\tV{}", self.cpu.pc, vx, vy);
                        }
                        let sub: i8 = vy_value as i8 - vx_value as i8;
                        self.cpu.write_register(vx, sub as u8);
                        self.cpu.write_register(0xF, (vy_value < vx_value) as u8);
                    }
                    0xE => {
                        // 0x8XYE -> VX <<= 1
                        if self.debug {
                            println!("{:#X}:\tSHL\tV{},\tV{}", self.cpu.pc, vx, vy);
                        }
                        self.cpu.write_register(vx, vx_value << 1);
                        self.cpu.write_register(0xF, vx_value >> 7);
                    }
                    _ => {
                        if self.debug {
                            eprintln!("Unknown opcode for 0x8 at {} -> {}", self.cpu.pc, opcode);
                        }
                    }
                }
                self.cpu.next_instruction();
            }
            0x9 => {
                // 0x9XY0 -> Skip next instruction if VX != VY
                // Here, 0x000X is ignored
                if self.debug {
                    println!(
                        "{:#X}:\tSNE\tV{},\tV{}",
                        self.cpu.pc,
                        opcode[Nibble::B],
                        opcode[Nibble::C]
                    );
                }
                if self.cpu.v[opcode[Nibble::B] as usize] != self.cpu.v[opcode[Nibble::C] as usize]
                {
                    self.cpu.skip_instruction();
                } else {
                    self.cpu.next_instruction();
                }
            }
            0xA => {
                // 0xANNN -> Set I to NNN
                if self.debug {
                    println!(
                        "{:#X}:\tLD\tI,\t{:#X}",
                        self.cpu.pc,
                        opcode.get(Nibble::BCD)
                    );
                }
                self.cpu.i = opcode.get(Nibble::BCD);
                self.cpu.next_instruction();
            }
            0xB => {
                // 0xBNNN -> Jump to location V0 + NNN
                if self.debug {
                    println!(
                        "{:#X}:\tJP\tV0,\t{:#X}",
                        self.cpu.pc,
                        opcode.get(Nibble::BCD)
                    );
                }
                self.cpu.pc = self.cpu.v[0x0] as u16 + opcode[Nibble::BCD];
            }
            0xC => {
                // 0xCXKK -> VX = random() & KK
                if self.debug {
                    println!(
                        "{:#X}:\tRND\tV{},\t{:#X}",
                        self.cpu.pc,
                        opcode[Nibble::B],
                        opcode.get(Nibble::CD)
                    );
                }
                self.cpu.v[Nibble::B as usize] = self.rng.gen::<u8>();
                self.cpu.next_instruction();
            }
            0xD => {
                // 0xDXYN -> Draw sprite at (VX, VY) with width 8 and height N
                if self.debug {
                    println!(
                        "{:#X}:\tDRW\tV{},\tV{},\t{:#X}",
                        self.cpu.pc,
                        opcode[Nibble::B],
                        opcode[Nibble::C],
                        opcode[Nibble::D],
                    );
                }
                self.cpu.v[0xF] = self.display.display(
                    opcode[Nibble::B],
                    opcode[Nibble::C],
                    opcode[Nibble::D],
                    self.mem.get_slice(self.cpu.i, opcode[Nibble::D]),
                ) as u8;
                self.display.update();
                self.cpu.next_instruction();
            }
            // todo 0xE
            0xF => {
                // [07, 0A, 15, 18, 1E, 29, 33, 55, 65]
                match opcode.get(Nibble::CD) {
                    // todo
                    _ => {
                        if self.debug {
                            eprintln!("Unknown opcode for 0xF at {} -> {}", self.cpu.pc, opcode)
                        }
                    }
                }
                self.cpu.next_instruction();
            }
            _ => {
                if self.debug {
                    eprintln!("Unknown opcode at {:#X} -> {}", self.cpu.pc, opcode);
                }
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
