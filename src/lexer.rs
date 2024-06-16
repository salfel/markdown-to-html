pub struct Lexer {
    content: String,
}

const MARKERS: &[char] = &['*', '_'];

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer { content: input }
    }

    pub fn get_tokens(self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        for line in self.content.lines() {
            for token in line.split(' ').filter(|token| !token.is_empty()) {
                Self::parse_word(&mut tokens, token);
            }
            tokens.push(Token::NewLine);
        }

        tokens
    }

    fn parse_word(tokens: &mut Vec<Token>, token: &str) {
        match token {
            "#" => tokens.push(Token::Heading1),
            "##" => tokens.push(Token::Heading2),
            "###" => tokens.push(Token::Heading3),
            "-" => tokens.push(Token::UnorderdListItem),
            token if Self::is_ordered_list_item(token) => {
                let number = token.trim_end_matches('.').parse().unwrap();
                tokens.push(Token::OrderedListItem(number));
            }
            content => {
                let mut modifiers: Vec<(String, u32)> = vec![];
                let mut last: char = ' ';

                for char in content.chars() {
                    // insert word into modifiers
                    if !MARKERS.contains(&char) {
                        last = char;
                        let value = modifiers.last_mut();
                        if let Some(token) = value {
                            if token.1 == 0 {
                                token.0 += &char.to_string();
                            } else {
                                modifiers.push((char.to_string(), 0));
                            }
                        } else {
                            modifiers.push((char.to_string(), 0));
                        }
                        continue;
                    }

                    // insert modifiers into vec
                    if char == last {
                        let value = modifiers.last_mut();
                        if let Some(modifier) = value {
                            *modifier = (char.to_string(), modifier.1 + 1);
                        }
                    } else {
                        modifiers.push((char.to_string(), 1));
                        last = char;
                    }
                }

                for modifier in modifiers {
                    tokens.push(match modifier {
                        (char, count) if char == *"*" => Token::Asterisk(count),
                        (char, count) if char == *"_" => Token::Underscore(count),
                        (word, _) => Token::Word(word),
                    });
                }
            }
        }
    }

    fn is_ordered_list_item(token: &str) -> bool {
        if !token.ends_with('.') {
            false
        } else {
            token.trim_end_matches('.').parse::<u32>().is_ok()
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Token {
    NewLine,
    Heading1,
    Heading2,
    Heading3,
    OrderedListItem(u32),
    UnorderdListItem,
    Asterisk(u32),
    Underscore(u32),
    Word(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexes_tokens() {
        let input = "# **something**
## ###
# **__something__** else";

        let lexer = Lexer::new(input.to_string());
        let tokens = lexer.get_tokens();

        assert_eq!(
            tokens,
            vec![
                Token::Heading1,
                Token::Asterisk(2),
                Token::Word("something".to_string()),
                Token::Asterisk(2),
                Token::NewLine,
                Token::Heading2,
                Token::Heading3,
                Token::NewLine,
                Token::Heading1,
                Token::Asterisk(2),
                Token::Underscore(2),
                Token::Word("something".to_string()),
                Token::Underscore(2),
                Token::Asterisk(2),
                Token::Word("else".to_string()),
                Token::NewLine
            ]
        );
    }

    #[test]
    fn lexes_list() {
        let lexer = Lexer::new(String::from("1. 2. 3."));
        let tokens = lexer.get_tokens();

        assert!(Lexer::is_ordered_list_item("1."));

        assert_eq!(
            tokens,
            vec![
                Token::OrderedListItem(1),
                Token::OrderedListItem(2),
                Token::OrderedListItem(3),
                Token::NewLine
            ]
        );

        let lexer = Lexer::new(String::from("- -"));
        let tokens = lexer.get_tokens();

        assert_eq!(
            tokens,
            vec![
                Token::UnorderdListItem,
                Token::UnorderdListItem,
                Token::NewLine
            ]
        );
    }
}
