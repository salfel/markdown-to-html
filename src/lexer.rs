pub struct Lexer {
    tokens: Vec<Token>,
}

impl Default for Lexer {
    fn default() -> Self {
        Lexer::new()
    }
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer { tokens: Vec::new() }
    }

    pub fn tokenize(mut self, contents: String) -> Vec<Token> {
        for char in contents.chars() {
            match char {
                '\n' => self.tokens.push(Token::NewLine),
                '#' => self.tokens.push(Token::Heading(1)),
                ' ' => self.tokens.push(Token::WhiteSpace(1)),
                '*' => self.tokens.push(Token::Asterisk(1)),
                _ => self.tokens.push(Token::Word(char.to_string())),
            }
        }

        self.combine_tokens()
    }

    pub fn combine_tokens(self) -> Vec<Token> {
        let mut combined_tokens: Vec<Token> = Vec::new();
        let iterator = self.tokens.into_iter();

        for token in iterator {
            let last = combined_tokens.last_mut();

            match (last, token) {
                (Some(Token::Word(last_word)), Token::Word(word)) => {
                    *last_word += &word;
                }
                (Some(Token::Asterisk(last_count)), Token::Asterisk(count)) => {
                    *last_count += count;
                }
                (Some(Token::Heading(last_count)), Token::Heading(count)) => {
                    *last_count += count;
                }
                (Some(Token::WhiteSpace(last_count)), Token::WhiteSpace(count)) => {
                    *last_count += count;
                }
                (_, token) => combined_tokens.push(token),
            }
        }

        combined_tokens
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Word(String),
    Heading(usize),
    WhiteSpace(usize),
    Asterisk(usize),
    NewLine,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_word() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize(String::from(
            "Hello
## Hi**",
        ));

        assert_eq!(
            tokens,
            vec![
                Token::Word("Hello".to_string()),
                Token::NewLine,
                Token::Heading(2),
                Token::WhiteSpace(1),
                Token::Word("Hi".to_string()),
                Token::Asterisk(2)
            ]
        );
    }
}
