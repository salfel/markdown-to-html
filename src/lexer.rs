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
            let token = match char {
                '\n' => Token::NewLine,
                '#' => Token::Heading(1),
                ' ' => Token::WhiteSpace(1),
                '*' => Token::Asterisk(1),
                '-' => Token::Hyphen,
                '.' => Token::Dot,
                '0'..='9' => Token::Number(char.to_digit(10).unwrap() as usize),
                _ => Token::Word(char.to_string()),
            };
            self.tokens.push(token);
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
    Number(usize),
    Dot,
    Hyphen,
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
## Hi** - - 1.",
        ));

        assert_eq!(
            tokens,
            vec![
                Token::Word("Hello".to_string()),
                Token::NewLine,
                Token::Heading(2),
                Token::WhiteSpace(1),
                Token::Word("Hi".to_string()),
                Token::Asterisk(2),
                Token::WhiteSpace(1),
                Token::Hyphen,
                Token::WhiteSpace(1),
                Token::Hyphen,
                Token::WhiteSpace(1),
                Token::Number(1),
                Token::Dot,
            ]
        );
    }
}
