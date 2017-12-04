pub struct Parser {}

impl Parser {
    pub fn parse(source: &str) -> Vec<char> {
        source
            .chars()
            .filter(|&c| match c {
                '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' => true,
                _ => false,
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
            Parser::parse("><hello..[]"),
            vec!['>', '<', '.', '.', '[', ']']
        );
    }
}
