pub struct Memory {
    // actual memory
    pub mem: [u8; 4096],

    // offset for the start of a program
    // the first 512 bytes is where the original interpreter was located
    pub offset: u16,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            mem: [0; 4096],
            offset: 0x200,
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        self.mem[address as usize]
    }

    #[allow(dead_code)]
    pub fn dump(&self) {
        for (i, b) in self.mem.iter().enumerate() {
            if i % 8 == 0 {
                print!("\n{:0>4X}", i);
            }

            print!("{}{:0>2X}", if i % 2 == 0 { " " } else { "" }, b);
        }
    }
}
