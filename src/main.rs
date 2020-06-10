use std::fs::File;
use std::io::Read;

use crate::chip8::Chip8;
use crate::display::{Chip8Display, TextDisplay};

mod chip8;
mod cpu;
mod display;
mod memory;
mod opcode;
mod sprites;
mod stack;

#[cfg(test)]
mod tests;

fn main() {
    let file_name = "data/TETRIS";
    let mut file = match File::open(file_name) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Could not open file {}", file_name);
            return;
        }
    };
    let mut data = Vec::<u8>::new();
    match file.read_to_end(&mut data) {
        Ok(size) => println!("Read file {} with {} bytes", file_name, size),
        Err(_) => {
            eprintln!("Could not read file {}", file_name);
            return;
        }
    }

    let mut chip8 = Chip8::<Chip8Display>::new().debug();

    chip8.load_rom(&data);
    chip8.run();

    // chip8.dump();
}
