use crate::lexer::Token;
use std::borrow::BorrowMut;
use std::cmp;

pub struct Parser {
    lines: Vec<Vec<Token>>,
}

impl Parser {
    pub fn new(lines: Vec<Vec<Token>>) -> Self {
        Parser { lines }
    }

    pub fn get_statements(self) -> Vec<Statement> {
        let mut statements = vec![];

        for line in self.lines {
            let mut iterator = line.into_iter();

            while let Some(value) = iterator.next() {
                let statement = match value {
                    Token::Heading1 => {
                        Statement::Heading1(Self::parse_expression(iterator.borrow_mut(), None))
                    }
                    Token::Asterisk(count) => Statement::Plain(Self::parse_expression(
                        iterator.borrow_mut(),
                        Some(Token::Asterisk(count)),
                    )),
                    Token::Underscore(count) => Statement::Plain(Self::parse_expression(
                        iterator.borrow_mut(),
                        Some(Token::Underscore(count)),
                    )),
                    Token::Word(content) => Statement::Plain(Self::parse_expression(
                        iterator.borrow_mut(),
                        Some(Token::Word(content)),
                    )),
                };

                statements.push(statement);
            }
        }

        statements
    }

    fn parse_expression(
        iterator: &mut dyn Iterator<Item = Token>,
        current: Option<Token>,
    ) -> Expression {
        match current {
            Some(Token::Asterisk(left_count)) => {
                let mut tokens = vec![];
                let mut right_count = 0;

                for token in iterator {
                    match token {
                        Token::Asterisk(count) => {
                            right_count = count;
                            break;
                        }
                        _ => {
                            tokens.push(token);
                        }
                    }
                }

                for current in &tokens {
                    match &current {
                        Token::Underscore(_) => println!("underscore"),
                        Token::Asterisk(_) => println!("asterisk"),
                        Token::Word(content) => println!("word: {}", &content),
                        Token::Heading1 => println!("heading"),
                    }
                }
                Self::parse_children(tokens, left_count, right_count, "*")
            }
            Some(Token::Underscore(left_count)) => {
                let mut tokens = vec![];
                let mut right_count = 0;

                for token in iterator {
                    match token {
                        Token::Underscore(count) => {
                            right_count = count;
                            break;
                        }
                        _ => {
                            tokens.push(token);
                        }
                    }
                }

                Self::parse_children(tokens, left_count, right_count, "_")
            }
            Some(Token::Word(content)) => {
                let mut tokens = vec![content];
                while let Some(Token::Word(content)) = iterator.next() {
                    tokens.push(content);
                }

                Expression::Text(tokens.join(" "))
            }
            Some(Token::Heading1) => {
                let mut tokens = vec![String::from("#")];

                while let Some(Token::Word(content)) = iterator.next() {
                    tokens.push(content);
                }

                Expression::Text(tokens.join(" "))
            }
            None => {
                let current = iterator.next();
                Self::parse_expression(iterator, current)
            }
        }
    }

    fn parse_children(
        mut tokens: Vec<Token>,
        left_count: u32,
        right_count: u32,
        character: &str,
    ) -> Expression {
        let min = cmp::min(left_count, right_count);
        Self::prepend_string(
            &mut tokens,
            String::from(character),
            (left_count - min) as usize,
        );
        Self::append_string(
            &mut tokens,
            String::from(character),
            (right_count - min) as usize,
        );

        for token in &tokens {
            match &token {
                Token::Underscore(_) => println!("underscore"),
                Token::Asterisk(_) => println!("asterisk"),
                Token::Word(content) => println!("word: '{}'", &content),
                Token::Heading1 => println!("heading"),
            }
        }

        let mut iterator = tokens.into_iter();
        let current = iterator.next();

        match (left_count, right_count) {
            (3..=u32::MAX, 3..=u32::MAX) => Expression::BoldItalic(Box::new(
                Self::parse_expression(iterator.borrow_mut(), current),
            )),
            (2..=u32::MAX, 2..=u32::MAX) => Expression::Bold(Box::new(Self::parse_expression(
                iterator.borrow_mut(),
                current,
            ))),
            (1..=u32::MAX, 1..=u32::MAX) => Expression::Italic(Box::new(Self::parse_expression(
                iterator.borrow_mut(),
                current,
            ))),
            (_, _) => Self::parse_expression(iterator.borrow_mut(), current),
        }
    }

    fn append_string(tokens: &mut Vec<Token>, string: String, count: usize) {
        if count == 0 {
            return;
        }
        if let Some(last_token) = tokens.last_mut() {
            if let Token::Word(content) = last_token {
                *last_token = Token::Word(content.to_string() + &string.repeat(count));
            } else {
                tokens.push(Token::Word(string.repeat(count)));
            }
        }
    }

    fn prepend_string(tokens: &mut [Token], string: String, count: usize) {
        if count == 0 {
            return;
        }
        if let Some(first_token) = tokens.first_mut() {
            if let Token::Word(content) = first_token {
                *first_token = Token::Word(string.repeat(count) + content);
            } else {
                tokens.rotate_right(1);
                tokens[0] = Token::Word(string.repeat(count));
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Italic(Box<Expression>),
    Bold(Box<Expression>),
    BoldItalic(Box<Expression>),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Heading1(Expression),
    Plain(Expression),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn setup_parser(input: String) -> Vec<Statement> {
        let lexer = Lexer::new(input);
        let tokens = lexer.get_tokens();

        let parser = Parser::new(tokens);
        parser.get_statements()
    }

    #[test]
    fn parses_tokens() {
        let statements = setup_parser(String::from(
            "# **something**
_something_
**_something else_**",
        ));

        assert_eq!(
            statements,
            vec![
                Statement::Heading1(Expression::Bold(Box::new(Expression::Text(String::from(
                    "something"
                ))))),
                Statement::Plain(Expression::Italic(Box::new(Expression::Text(
                    String::from("something")
                )))),
                Statement::Plain(Expression::Bold(Box::new(Expression::Italic(Box::new(
                    Expression::Text(String::from("something else"))
                )))))
            ]
        );
    }

    #[test]
    fn check_conditions() {
        let statements = setup_parser(String::from(
            "*something*
**something**
***something***
**something",
        ));

        assert_eq!(
            statements,
            vec![
                Statement::Plain(Expression::Italic(Box::new(Expression::Text(
                    String::from("something")
                )))),
                Statement::Plain(Expression::Bold(Box::new(Expression::Text(String::from(
                    "something"
                ))))),
                Statement::Plain(Expression::BoldItalic(Box::new(Expression::Text(
                    String::from("something")
                )))),
                Statement::Plain(Expression::Text(String::from("**something"))),
            ]
        )
    }

    #[test]
    fn check_overflow() {
        let statements = setup_parser(String::from(
            "**something*
*something**
***something**",
        ));

        assert_eq!(
            statements,
            vec![
                Statement::Plain(Expression::Italic(Box::new(Expression::Text(
                    String::from("*something")
                )))),
                Statement::Plain(Expression::Italic(Box::new(Expression::Text(
                    String::from("something*")
                )))),
                Statement::Plain(Expression::Bold(Box::new(Expression::Text(String::from(
                    "*something"
                ))))),
            ]
        )
    }
}
