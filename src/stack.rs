const STACK_SIZE: usize = 16;

/// The call stack of the CHIP-8
#[derive(Debug)]
pub struct Stack {
    /// the call stack
    stack: [u16; STACK_SIZE],
    /// stack pointer
    sp: u16,
}

impl Stack {
    /// Gets a new empty stack
    pub fn new() -> Self {
        Self {
            stack: [0; STACK_SIZE],
            sp: 0,
        }
    }

    pub fn push(&mut self, addr: u16) {
        if self.sp >= STACK_SIZE as u16 {
            panic!("Stack Overflow");
        }

        self.stack[self.sp as usize] = addr;
        self.sp += 1;
    }

    pub fn pop(&mut self) -> u16 {
        if self.sp == 0 {
            panic!("Stack Underflow");
        }

        self.sp -= 1;

        self.stack[self.sp as usize]
    }

    #[allow(dead_code)]
    pub fn dump(&self) {
        println!("{:?}", self);
    }
}
