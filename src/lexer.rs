pub struct Lexer {
    lines: Vec<Vec<Token>>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lines = vec![];

        for line in input.lines() {
            let mut tokens = vec![];

            for token in line.split(' ').filter(|token| !token.is_empty()) {
                tokens.push(match token {
                    "#" => Token::Heading1,
                    content => Token::Word(content.to_string()),
                });
            }

            lines.push(tokens);
        }

        Lexer { lines }
    }

    pub fn get_lines(&self) -> &Vec<Vec<Token>> {
        &self.lines
    }
}

#[derive(PartialEq, Debug)]
pub enum Token {
    Heading1,
    Word(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_tokens() {
        let input = "# something\n# something else";

        let tokenizer = Lexer::new(input.to_string());

        assert_eq!(
            tokenizer.lines,
            vec![
                vec![Token::Heading1, Token::Word("something".to_string()),],
                vec![
                    Token::Heading1,
                    Token::Word("something".to_string()),
                    Token::Word("else".to_string())
                ]
            ]
        );
    }
}
