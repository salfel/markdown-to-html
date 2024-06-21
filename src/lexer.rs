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

    pub fn tokenize(&mut self, contents: String) -> Vec<Token> {
        let mut tokens = Vec::new();
        for char in contents.chars() {
            match char {
                '\n' => tokens.push(Token::NewLine),
                _ => tokens.push(Token::Word(char.to_string())),
            }
        }

        self.combine_tokens(tokens)
    }

    pub fn combine_tokens(&mut self, tokens: Vec<Token>) -> Vec<Token> {
        let mut combined_tokens = Vec::new();
        let iterator = tokens.into_iter();

        for token in iterator {
            let last = combined_tokens.last_mut();

            match (last, token) {
                (Some(Token::Word(last_word)), Token::Word(word)) => {
                    *last_word += &word;
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
    NewLine,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_word() {
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("Hello".to_string());

        assert_eq!(tokens, vec![Token::Word("Hello".to_string())]);
    }
}
