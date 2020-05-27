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
