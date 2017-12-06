use std::io;
use parser::Parser;

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
    memory_counter: usize,

    instructions: Vec<Instruction>,
    instruction_counter: usize,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            memory: [0; MEMORY_SIZE],
            memory_counter: 0,

            instructions: vec![],
            instruction_counter: 0,
        }
    }

    pub fn add_instructions(&mut self, source: &str) {
        self.instructions.append(&mut Parser::parse(source));
    }

    pub fn run(&mut self) -> Result<String, io::Error> {
        let mut output = String::new();

        while let Some(instruction) = self.instructions.get(self.instruction_counter) {
            println!("{:?}", instruction);

            self.instruction_counter += 1;
        }

        Ok(output)
    }
}
