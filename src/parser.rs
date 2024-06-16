use crate::lexer::{Lexer, Token};
use std::borrow::BorrowMut;
use std::cmp;

pub struct Parser {
    tokens: Vec<Vec<Token>>,
}

impl Parser {
    pub fn new(input: String) -> Self {
        let lexer = Lexer::new(input);
        let tokens = lexer.get_tokens();

        Parser { tokens }
    }

    pub fn get_statements(self) -> Vec<Statement> {
        let mut statements = vec![];

        for line in self.tokens {
            let mut iterator = line.into_iter();

            while let Some(value) = iterator.next() {
                let statement = match value {
                    Token::Heading1 => Statement::Heading1(Self::parse_expression(
                        iterator.borrow_mut(),
                        Some(Token::Heading1),
                    )),
                    Token::Heading2 => Statement::Heading2(Self::parse_expression(
                        iterator.borrow_mut(),
                        Some(Token::Heading1),
                    )),
                    Token::Heading3 => Statement::Heading3(Self::parse_expression(
                        iterator.borrow_mut(),
                        Some(Token::Heading1),
                    )),
                    Token::Asterisk(count) => Statement::Plain(Self::parse_expression(
                        iterator.borrow_mut(),
                        Some(Token::Asterisk(count)),
                    )),
                    Token::Underscore(count) => Statement::Plain(Self::parse_expression(
                        iterator.borrow_mut(),
                        Some(Token::Underscore(count)),
                    )),
                    Token::OrderedListItem(num) => Statement::OrderedList((
                        num,
                        Self::parse_expression(
                            iterator.borrow_mut(),
                            Some(Token::OrderedListItem(num)),
                        ),
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

                loop {
                    let token = iterator.next();
                    match token {
                        Some(Token::Asterisk(count)) => {
                            right_count = count;
                            break;
                        }
                        Some(token) => tokens.push(token),
                        None => break,
                    }
                }

                let next = iterator.next();
                let children = Self::parse_children(tokens, left_count, right_count, "*");

                match next {
                    Some(token) => Expression::Vec(vec![
                        children,
                        Self::parse_expression(iterator, Some(token)),
                    ]),
                    None => children,
                }
            }
            Some(Token::Underscore(left_count)) => {
                let mut tokens = vec![];
                let mut right_count = 0;

                loop {
                    let token = iterator.next();
                    match token {
                        Some(Token::Underscore(count)) => {
                            right_count = count;
                            break;
                        }
                        Some(token) => tokens.push(token),
                        None => break,
                    }
                }

                let next = iterator.next();
                let children = Self::parse_children(tokens, left_count, right_count, "_");

                match next {
                    Some(token) => Expression::Vec(vec![
                        children,
                        Self::parse_expression(iterator, Some(token)),
                    ]),
                    None => children,
                }
            }
            Some(Token::Word(content)) => {
                let mut tokens = vec![content];
                let mut next = None;

                loop {
                    let value = iterator.next();
                    match value {
                        Some(Token::Word(content)) => tokens.push(content),
                        Some(token) => {
                            next = Some(token);
                            break;
                        }
                        None => break,
                    }
                }

                match next {
                    Some(token) => Expression::Vec(vec![
                        Expression::Text(tokens.join(" ")),
                        Self::parse_expression(iterator, Some(token)),
                    ]),
                    None => Expression::Text(tokens.join(" ")),
                }
            }
            Some(_) => {
                let token = iterator.next();
                Self::parse_expression(iterator.borrow_mut(), token)
            }
            None => Expression::Text(String::new()),
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
    Vec(Vec<Expression>),
    Italic(Box<Expression>),
    Bold(Box<Expression>),
    BoldItalic(Box<Expression>),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Heading1(Expression),
    Heading2(Expression),
    Heading3(Expression),
    OrderedList((u32, Expression)),
    Plain(Expression),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_parser(input: String) -> Vec<Statement> {
        let parser = Parser::new(input);
        parser.get_statements()
    }

    #[test]
    fn parses_tokens() {
        let statements = setup_parser(String::from(
            "# **something**
## something
### else
    _something_
    **_something else_**",
        ));

        assert_eq!(
            statements,
            vec![
                Statement::Heading1(Expression::Bold(Box::new(Expression::Text(String::from(
                    "something"
                ))))),
                Statement::Heading2(Expression::Text(String::from("something"))),
                Statement::Heading3(Expression::Text(String::from("else"))),
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
    fn check_vec() {
        let statements = setup_parser(String::from("# something **else**"));

        assert_eq!(
            statements,
            vec![Statement::Heading1(Expression::Vec(vec![
                Expression::Text(String::from("something")),
                Expression::Bold(Box::new(Expression::Text(String::from("else"))))
            ]))]
        )
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

    #[test]
    fn parses_list() {
        let statements = setup_parser(String::from(
            "1. something
2. something
3 something",
        ));

        assert_eq!(
            statements,
            vec![
                Statement::OrderedList((1, Expression::Text(String::from("something")))),
                Statement::OrderedList((2, Expression::Text(String::from("something")))),
                Statement::Plain(Expression::Text(String::from("3 something")))
            ],
        );
    }
}
