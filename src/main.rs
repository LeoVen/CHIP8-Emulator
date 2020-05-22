use std::fs::File;
use std::io::Read;
use crate::cpu::Cpu;

mod cpu;
mod memory;
mod sprites;

fn main() {
    let file_name = "data/INVADERS";
    let mut file = match File::open(file_name) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Could not open file {}", file_name);
            return;
        },
    };
    let mut data = Vec::<u8>::new();
    match file.read_to_end(&mut data) {
        Ok(size) => println!("Read file {} with {} bytes", file_name, size),
        Err(_) => {
            eprintln!("Could not read file {}", file_name);
            return;
        },
    }

    let mut cpu = Cpu::new();
    cpu.load_rom(&data);

    cpu.dump();
}
