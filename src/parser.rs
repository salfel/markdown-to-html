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
            Some(Token::Asterisk(_)) => {
                let mut tokens = vec![];

                for token in &mut *iterator {
                    match token {
                        Token::Asterisk(_) => {
                            break;
                        }
                        _ => {
                            tokens.push(token);
                        }
                    }
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

                for token in &mut *iterator {
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

                Expression::Plain(tokens.join(" "))
            }
            None => {
                let current = iterator.next();
                Self::parse_expression(iterator, current)
            }
            _ => Expression::Plain(String::from("")),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Underline(Box<Expression>),
    Bold(Box<Expression>),
    Plain(String),
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

    #[test]
    fn parses_tokens() {
        let lexer = Lexer::new(String::from(
            "# **something**
__**something else**__",
        ));
        let tokens = lexer.get_tokens();

        let parser = Parser::new(tokens);
        let statements = parser.get_statements();

        assert_eq!(
            statements,
            vec![
                Statement::Heading1(Expression::Bold(Box::new(Expression::Plain(String::from(
                    "something"
                ))))),
                Statement::Plain(Expression::Underline(Box::new(Expression::Bold(Box::new(
                    Expression::Plain(String::from("something else"))
                )))))
            ]
        );
    }
}
