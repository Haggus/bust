const MEMORY_SIZE: usize = 30_000;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Output,
    Input,
    JumpRight(usize),
    JumpLeft(usize),
}

pub struct VirtualMachine {
    memory: [u8; MEMORY_SIZE],
}

impl VirtualMachine {
    fn new() -> VirtualMachine {
        VirtualMachine {
            memory: [0; MEMORY_SIZE]
        }
    }
}
