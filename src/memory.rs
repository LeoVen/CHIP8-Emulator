use crate::sprites::SPRITES;

pub struct Memory {
    /// Actual memory
    pub mem: [u8; 4096],

    /// Offset for the start of a program
    /// The first 512 bytes is where the original interpreter was located
    pub offset: u16,
}

impl Memory {
    pub fn new() -> Self {
        let mut memory = Memory {
            mem: [0; 4096],
            offset: 0x200,
        };

        // Initialize memory at 0x0000 with pre-defined sprites
        let mut i = 0;
        for sprite in &SPRITES {
            for byte in sprite {
                memory.mem[i] = *byte;
                i += 1;
            }
        }

        memory
    }

    /// Writes a byte to a memory region
    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }

    /// Reads a byte from a memory region
    pub fn read_byte(&mut self, address: u16) -> u8 {
        self.mem[address as usize]
    }

    /// Gets a slice from memory starting at i up to i + n
    pub fn get_slice(&self, i: u16, n: u16) -> &[u8] {
        &self.mem[i as usize..(i + n) as usize]
    }

    #[allow(dead_code)]
    pub fn dump(&self) {
        for (i, b) in self.mem.iter().enumerate() {
            if i % 16 == 0 {
                print!("\n0x{:0>3X}: ", i);
            } else if i % 8 == 0 {
                print!(" ");
            }
            print!("{}{:0>2X}", if i % 2 == 0 { " " } else { "" }, b);
        }
        println!("");
    }
}
