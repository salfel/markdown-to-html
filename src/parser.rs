use crate::lexer::Token;
use std::borrow::BorrowMut;

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

                if right_count <= 1 {
                    Self::prepend_string(&mut tokens, String::from("*"), left_count as usize);
                    Self::append_string(&mut tokens, String::from("*"), right_count as usize);
                    let mut iterator = tokens.into_iter();
                    let current = iterator.next();
                    return Self::parse_expression(iterator.borrow_mut(), current);
                } else if left_count >= 3 {
                    Self::prepend_string(&mut tokens, String::from("*"), (left_count - 2) as usize);
                } else if left_count == 1 {
                    Self::prepend_string(&mut tokens, String::from("*"), 1);
                    Self::append_string(&mut tokens, String::from("*"), right_count as usize);

                    let mut iterator = tokens.into_iter();
                    let current = iterator.next();
                    return Self::parse_expression(iterator.borrow_mut(), current);
                }

                if right_count >= 3 {
                    Self::append_string(&mut tokens, String::from("*"), (right_count - 2) as usize);
                } else if right_count == 1 {
                    Self::append_string(&mut tokens, String::from("*"), 1);
                }

                let mut iterator = tokens.into_iter();
                let current = iterator.next();
                Expression::Bold(Box::new(Self::parse_expression(
                    iterator.borrow_mut(),
                    current,
                )))
            }
            Some(Token::Underscore(_)) => {
                let mut tokens = vec![];

                for token in iterator {
                    match token {
                        Token::Underscore(_) => {
                            break;
                        }
                        _ => {
                            tokens.push(token);
                        }
                    }
                }

                let mut iterator = tokens.into_iter();
                let current = iterator.next();
                Expression::Underline(Box::new(Self::parse_expression(
                    iterator.borrow_mut(),
                    current,
                )))
            }
            Some(Token::Word(content)) => {
                let mut tokens = vec![content];
                while let Some(Token::Word(content)) = iterator.next() {
                    tokens.push(content);
                }

                Expression::Text(tokens.join(" "))
            }
            None => {
                let current = iterator.next();
                Self::parse_expression(iterator, current)
            }
            _ => Expression::Text(String::from("")),
        }
    }
    fn append_string(tokens: &mut Vec<Token>, string: String, count: usize) {
        if let Some(last_token) = tokens.last_mut() {
            if let Token::Word(content) = last_token {
                *last_token = Token::Word(content.to_string() + &string.repeat(count));
            } else {
                tokens.push(Token::Word(string.repeat(count)));
            }
        }
    }

    fn prepend_string(tokens: &mut [Token], string: String, count: usize) {
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
    Underline(Box<Expression>),
    Bold(Box<Expression>),
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
            "# ***something**
# **something***
__**something else**__",
        ));

        assert_eq!(
            statements,
            vec![
                Statement::Heading1(Expression::Bold(Box::new(Expression::Text(String::from(
                    "*something"
                ))))),
                Statement::Heading1(Expression::Bold(Box::new(Expression::Text(String::from(
                    "something*"
                ))))),
                Statement::Plain(Expression::Underline(Box::new(Expression::Bold(Box::new(
                    Expression::Text(String::from("something else"))
                )))))
            ]
        );
    }

    #[test]
    fn check_overflow() {
        let statements = setup_parser(String::from(
            "**something*
***something**
*something**
**something***
***something***",
        ));

        assert_eq!(
            statements,
            vec![
                Statement::Plain(Expression::Text(String::from("**something*"))),
                Statement::Plain(Expression::Bold(Box::new(Expression::Text(String::from(
                    "*something"
                ))))),
                Statement::Plain(Expression::Text(String::from("*something**"))),
                Statement::Plain(Expression::Bold(Box::new(Expression::Text(String::from(
                    "something*"
                ))))),
                Statement::Plain(Expression::Bold(Box::new(Expression::Text(String::from(
                    "*something*"
                ))))),
            ]
        )
    }
}
