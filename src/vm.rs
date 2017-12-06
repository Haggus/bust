use std::io;
use std::char::from_u32;
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

        'run: loop {
            match self.instructions.get(self.instruction_counter) {
                Some(instruction) => {

                    match *instruction {
                        Instruction::MoveRight => {
                            if self.memory_counter + 1 == MEMORY_SIZE {
                                self.memory_counter = 0;
                        } else {
                                self.memory_counter += 1;
                            }
                        }
                        Instruction::MoveLeft => if self.memory_counter == 0 {
                            self.memory_counter = MEMORY_SIZE - 1;
                        } else {
                            self.memory_counter -= 1;
                        },
                        Instruction::Increment => self.memory[self.memory_counter] += 1,
                        Instruction::Decrement => self.memory[self.memory_counter] -= 1,
                        Instruction::Output => {
                            output.push(from_u32(self.memory[self.memory_counter] as u32).unwrap())
                        }
                        Instruction::JumpRight(index) => {
                            if self.memory[self.memory_counter] == 0 {
                                self.instruction_counter = index + 1;
                            }
                        },
                        Instruction::JumpLeft(index) => {
                            if self.memory[self.memory_counter] != 0 {
                                self.instruction_counter = index;
                            }
                        },
                        _ => unimplemented!()
                    }

                    self.instruction_counter += 1;
                }
                None => break 'run
            }
        }

        Ok(output)
    }
}
