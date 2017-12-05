use vm::Instruction;

pub struct Parser {}

impl Parser {
    pub fn parse(source: &str) -> Vec<Instruction> {
        source
            .chars()
            .filter(|&c| match c {
                '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' => true,
                _ => false,
            })
            .map(|c| match c {
                '>' => Instruction::MoveRight,
                '<' => Instruction::MoveLeft,
                '+' => Instruction::Increment,
                '-' => Instruction::Decrement,
                '.' => Instruction::Output,
                ',' => Instruction::Input,
                '[' => Instruction::JumpRight(0),
                ']' => Instruction::JumpLeft(0),
                _ => panic!("Unknown instruction"),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translates_chars_to_instructions() {
        assert_eq!(
            Parser::parse("+++"),
            vec![Instruction::Increment, Instruction::Increment, Instruction::Increment]
        );
    }
    #[test]

    fn filters_out_unnecessary_characters() {
        assert_eq!(
            Parser::parse("><hello..[]"),
            vec![Instruction::MoveRight, Instruction::MoveLeft, Instruction::Output, Instruction::Output, Instruction::JumpRight(0), Instruction::JumpLeft(0)]
        );
    }
}
