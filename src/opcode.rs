use std::fmt;
use std::ops;

#[allow(dead_code)]
pub enum Nibble {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    AB,
    BC,
    CD,
    ABC,
    BCD,
    ABCD,
}

pub struct Opcode {
    /// Nibbles of an opcode
    nib: [u16; 4],
    /// The opcode itself
    opcode: u16,
}

impl Opcode {
    pub fn new(opcode: u16) -> Self {
        Self {
            nib: [
                (opcode & 0xF000) >> 12,
                (opcode & 0x0F00) >> 8,
                (opcode & 0x00F0) >> 4,
                opcode & 0x000F,
            ],
            opcode,
        }
    }

    /// Gets one of the possible combinations of Nibble
    pub fn get(&self, nib: Nibble) -> u16 {
        match nib {
            Nibble::A => self[Nibble::A] << 12,
            Nibble::B => self[Nibble::B] << 8,
            Nibble::C => self[Nibble::C] << 4,
            Nibble::D => self[Nibble::D],
            Nibble::AB => (self[Nibble::A] << 12) | (self[Nibble::B] << 8),
            Nibble::BC => (self[Nibble::B] << 8) | (self[Nibble::C] << 4),
            Nibble::CD => (self[Nibble::C] << 4) | self[Nibble::D],
            Nibble::ABC => {
                (self[Nibble::A] << 12)
                    | (self[Nibble::B] << 8)
                    | (self[Nibble::C] << 4)
            }
            Nibble::BCD => {
                (self[Nibble::B] << 8)
                    | (self[Nibble::C] << 4)
                    | self[Nibble::D]
            }
            Nibble::ABCD => self.opcode,
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[ {:#X}, {:#X}, {:#X}, {:#X} ]",
            self.nib[0], self.nib[1], self.nib[2], self.nib[3]
        )
    }
}

impl fmt::Binary for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Binary::fmt(&self.opcode, f)
    }
}

impl ops::Index<Nibble> for Opcode {
    type Output = u16;

    fn index(&self, nib: Nibble) -> &Self::Output {
        match nib {
            Nibble::A | Nibble::B | Nibble::C | Nibble::D => {
                &self.nib[nib as usize]
            }
            _ => panic!(
                "Can't index Opcode with a Nibble that is not A, B, C or D"
            ),
        }
    }
}
