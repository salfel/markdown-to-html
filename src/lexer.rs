pub struct Lexer {
    lines: Vec<String>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            lines: input.lines().map(|line| line.to_string()).collect(),
        }
    }

    pub fn get_tokens(self) -> Vec<Vec<Token>> {
        let mut lines = vec![];

        for line in self.lines {
            let mut tokens = vec![];

            for token in line.split(' ').filter(|token| !token.is_empty()) {
                tokens.push(match token {
                    "#" => Token::Heading1,
                    content => Token::Word(content.to_string()),
                });
            }

            lines.push(tokens);
        }

        lines
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
        let tokens = tokenizer.get_tokens();

        assert_eq!(
            tokens,
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
