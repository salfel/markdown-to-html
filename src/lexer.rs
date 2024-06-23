use std::fmt::{self, Display};

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
            let token = Token::new(char);
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
                (Some(Token::Number(last_number)), Token::Number(number)) => {
                    *last_number = *last_number * 10 + number;
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
    LParen,
    RParen,
    LBracket,
    RBracket,
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            Token::Word(word) => word.to_string(),
            Token::Heading(count) => "#".repeat(*count),
            Token::WhiteSpace(count) => " ".repeat(*count),
            Token::Asterisk(count) => "*".repeat(*count),
            Token::Number(number) => number.to_string(),
            Token::Dot => ".".to_string(),
            Token::Hyphen => "-".to_string(),
            Token::NewLine => "\n".to_string(),
            Token::LParen => "(".to_string(),
            Token::RParen => ")".to_string(),
            Token::LBracket => "[".to_string(),
            Token::RBracket => "]".to_string(),
        };

        write!(f, "{}", output)
    }
}

impl Token {
    pub fn new(char: char) -> Token {
        match char {
            '\n' => Token::NewLine,
            '#' => Token::Heading(1),
            ' ' => Token::WhiteSpace(1),
            '*' => Token::Asterisk(1),
            '-' => Token::Hyphen,
            '.' => Token::Dot,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            '0'..='9' => Token::Number(char.to_digit(10).unwrap() as usize),
            _ => Token::Word(char.to_string()),
        }
    }

    pub fn to_word(&self) -> Token {
        Token::Word(self.to_string())
    }

    pub fn expect(&self, expected: &Token) -> bool {
        matches!(
            (self, expected),
            (Token::Word(_), Token::Word(_))
                | (Token::Heading(_), Token::Heading(_))
                | (Token::WhiteSpace(_), Token::WhiteSpace(_))
                | (Token::Asterisk(_), Token::Asterisk(_))
                | (Token::Number(_), Token::Number(_))
                | (Token::Dot, Token::Dot)
                | (Token::Hyphen, Token::Hyphen)
                | (Token::NewLine, Token::NewLine)
                | (Token::LParen, Token::LParen)
                | (Token::RParen, Token::RParen)
                | (Token::LBracket, Token::LBracket)
                | (Token::RBracket, Token::RBracket)
        )
    }
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

    #[test]
    fn lexes_list() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize(String::from("[title][https://example.test]"));

        assert_eq!(
            tokens,
            vec![
                Token::LBracket,
                Token::Word("title".to_string()),
                Token::RBracket,
                Token::LBracket,
                Token::Word("https://example".to_string()),
                Token::Dot,
                Token::Word("test".to_string()),
                Token::RBracket
            ]
        );
    }
}
