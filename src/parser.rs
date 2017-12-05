use vm::Instruction;

pub struct Parser {}

impl Parser {
    pub fn parse(source: &str) -> Vec<Instruction> {
        let mut stack: Vec<usize> = Vec::new();
        let mut conditional_jumps: Vec<(usize, usize)> = Vec::new();

        // filter source code to only contain command characters
        let source_filtered: Vec<char> = source
            .chars()
            .filter(|&c| match c {
                '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' => true,
                _ => false,
            })
            .collect();

        // find conditional jumps
        source_filtered.iter().enumerate().for_each(
            |(idx, &c)| match c {
                '[' => stack.push(idx),
                ']' => conditional_jumps.push((stack.pop().expect("No matching parenthesis"), idx)),
                _ => (),
            },
        );

        source_filtered
            .iter()
            .enumerate()
            .map(|(idx, &c)| match c {
                '>' => Instruction::MoveRight,
                '<' => Instruction::MoveLeft,
                '+' => Instruction::Increment,
                '-' => Instruction::Decrement,
                '.' => Instruction::Output,
                ',' => Instruction::Input,
                '[' => {
                    Instruction::JumpRight(
                        conditional_jumps
                            .iter()
                            .find(|&&(s, _)| s == idx)
                            .map(|&(_, e)| e)
                            .unwrap(),
                    )
                }
                ']' => {
                    Instruction::JumpLeft(
                        conditional_jumps
                            .iter()
                            .find(|&&(_, e)| e == idx)
                            .map(|&(s, _)| s)
                            .unwrap(),
                    )
                }
                _ => panic!("Unknown instruction"),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_out_unnecessary_characters() {
        assert_eq!(
            Parser::parse("><hello..[.]"),
            vec![
                Instruction::MoveRight,
                Instruction::MoveLeft,
                Instruction::Output,
                Instruction::Output,
                Instruction::JumpRight(6),
                Instruction::Output,
                Instruction::JumpLeft(4),
            ]
        );
    }

    #[test]
    fn translates_chars_to_instructions() {
        assert_eq!(
            Parser::parse("+++"),
            vec![
                Instruction::Increment,
                Instruction::Increment,
                Instruction::Increment,
            ]
        );
    }

    #[test]
    fn calculates_jump_right_correctly() {
        assert_eq!(
            Parser::parse("++[+]"),
            vec![
                Instruction::Increment,
                Instruction::Increment,
                Instruction::JumpRight(4),
                Instruction::Increment,
                Instruction::JumpLeft(2),
            ]
        );
    }

    #[test]
    #[should_panic]
    fn panics_if_matching_left_paren_is_missing() {
        assert_eq!(
            Parser::parse("..]"),
            vec![
                Instruction::Output,
                Instruction::Output,
                Instruction::JumpLeft(0),
            ]
        );
    }

    #[test]
    #[should_panic]
    fn panics_if_matching_right_paren_is_missing() {
        assert_eq!(
            Parser::parse("+.+[+"),
            vec![
                Instruction::Increment,
                Instruction::Output,
                Instruction::Increment,
                Instruction::JumpRight(0),
                Instruction::Increment,
            ]
        );
    }

    #[test]
    fn matches_two_bracket_pairs_within_one_another() {
        assert_eq!(
            Parser::parse("++[..[-+]>>+]"),
            vec![
                Instruction::Increment,
                Instruction::Increment,
                Instruction::JumpRight(12),
                Instruction::Output,
                Instruction::Output,
                Instruction::JumpRight(8),
                Instruction::Decrement,
                Instruction::Increment,
                Instruction::JumpLeft(5),
                Instruction::MoveRight,
                Instruction::MoveRight,
                Instruction::Increment,
                Instruction::JumpLeft(2),
            ]
        );
    }

    #[test]
    fn matches_two_bracket_pairs_separate_from_each_other() {
        assert_eq!(
            Parser::parse("+[.].[-+]>"),
            vec![
                Instruction::Increment,
                Instruction::JumpRight(3),
                Instruction::Output,
                Instruction::JumpLeft(1),
                Instruction::Output,
                Instruction::JumpRight(8),
                Instruction::Decrement,
                Instruction::Increment,
                Instruction::JumpLeft(5),
                Instruction::MoveRight,
            ]
        );
    }
}
