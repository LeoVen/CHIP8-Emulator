use crate::chip8::Chip8;
use crate::display::{Chip8Display, TextDisplay};
use crate::opcode::{Nibble, Opcode};

#[test]
fn opcode_index() {
    let opcode = Opcode::new(0xABCD);

    assert_eq!(opcode[Nibble::A], 0xA);
    assert_eq!(opcode[Nibble::B], 0xB);
    assert_eq!(opcode[Nibble::C], 0xC);
    assert_eq!(opcode[Nibble::D], 0xD);
}

#[test]
fn opcode_get() {
    let opcode = Opcode::new(0xABCD);

    assert_eq!(0xA000, opcode.get(Nibble::A));
    assert_eq!(0x0B00, opcode.get(Nibble::B));
    assert_eq!(0x00C0, opcode.get(Nibble::C));
    assert_eq!(0x000D, opcode.get(Nibble::D));
    assert_eq!(0xAB00, opcode.get(Nibble::AB));
    assert_eq!(0x0BC0, opcode.get(Nibble::BC));
    assert_eq!(0x00CD, opcode.get(Nibble::CD));
    assert_eq!(0xABC0, opcode.get(Nibble::ABC));
    assert_eq!(0x0BCD, opcode.get(Nibble::BCD));
    assert_eq!(0xABCD, opcode.get(Nibble::ABCD));
}

#[test]
fn chip_test_instruction_0x5() {
    // 0x5XY0 -> SE VX, VY
    let mut chip = Chip8::<TextDisplay>::new().debug().no_display();

    // 0x200 LD V0, 0x0
    // 0x202 LD V1, 0x0
    // 0x204 SE V0, V1
    // 0x206 LD V0, 0xFF
    // 0x208 LD V0, 0x0C
    // 0x20A SE V0, V1
    // 0x20C LD V1, 0x0A
    // 0x20E SE V0, V1
    // 0x210 LD V0, 0x0A
    // 0x212 SE V0, V1
    // 0x214 LD V0, 0x0B
    let data: Vec<u8> = vec![
        0x60, 0x00, 0x61, 0x00, 0x50, 0x10, 0x60, 0xFF, 0x60, 0x0C, 0x50, 0x10, 0x61, 0x0A, 0x50,
        0x10, 0x60, 0x0A, 0x50, 0x10, 0x50, 0x0B,
    ];

    chip.load_rom(&data);

    chip.cycle(); // LD V0, 0x0
    chip.cycle(); // LD V1, 0x0
    assert_eq!(chip.cpu.v[0x0], 0x0);
    assert_eq!(chip.cpu.v[0x1], 0x0);
    chip.cycle(); // Skip
    chip.cycle(); // V0 = 0x0C
    assert_eq!(chip.cpu.v[0x0], 0x0C);
    chip.cycle(); // No Skip
    chip.cycle(); // V1 = 0xA
    assert_eq!(chip.cpu.v[0x1], 0x0A);
    chip.cycle(); // No Skip
    chip.cycle(); // V0 = 0xA
    assert_eq!(chip.cpu.v[0x0], 0x0A);
    chip.cycle(); // Skip
    chip.cycle(); // NOP
    assert_eq!(chip.cpu.v[0x0], 0x0A);
    assert_eq!(chip.cpu.v[0x1], 0x0A);
}

#[test]
fn chip_test_instruction_0x6() {
    // 0x6XNN -> LD VX, NN
    let mut chip = Chip8::<TextDisplay>::new().debug().no_display();

    // 0x200: LD V0, 0x0A
    // 0x202: LD V1, 0x0B
    let data: Vec<u8> = vec![0x60, 0x0A, 0x61, 0x0B];

    chip.load_rom(&data);

    chip.cycle();
    chip.cycle();

    assert_eq!(chip.cpu.v[0x0], 0x0A);
    assert_eq!(chip.cpu.v[0x1], 0x0B);
}

#[test]
fn chip_test_instruction_0x7() {
    // 0x6XNN -> LD VX, NN
    let mut chip = Chip8::<TextDisplay>::new().debug().no_display();

    // 0x200:  LD V0, 0x0
    // 0x202:  LD V1, 0x0
    // 0x204: ADD V0, 0x1
    // 0x206: ADD V0, 0x1
    // .. x8 total
    let data: Vec<u8> = vec![
        0x60, 0x00, 0x61, 0x00, 0x70, 0x01, 0x70, 0x01, 0x70, 0x01, 0x70, 0x01, 0x70, 0x01, 0x70,
        0x01, 0x70, 0x01, 0x70, 0x01,
    ];

    chip.load_rom(&data);

    chip.cycle(); // LD V0, 0x0
    chip.cycle(); // LD V1, 0x0

    for i in 0..9 {
        assert_eq!(chip.cpu.v[0x0], i);
        chip.cycle();
    }
}

// #[test]
// fn chip_test_display() {
//     // A visual display test

//     let mut chip = Chip8::<TextDisplay>::new().debug();

//     // 0x200  LD I, 0x000
//     // 0x202 DRW V0, V1, 0x5
//     let data: Vec<u8> = vec![0xA0, 0x00, 0xD0, 0x15];

//     chip.load_rom(&data);

//     chip.cycle();
//     chip.cycle();

//     std::thread::sleep(std::time::Duration::from_secs(2));
// }
