use crate::chip8::Chip8;
use std::fs::File;
use std::io::Read;

mod chip8;
mod cpu;
mod memory;
mod opcode;
mod sprites;
mod stack;

#[cfg(test)]
mod tests;

fn main() {
    let file_name = "data/INVADERS";
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

    let mut chip8 = Chip8::new();

    chip8.load_rom(&data);

    for _ in 0..10 {
        chip8.cycle();
    }

    // chip8.dump();
}
