use crate::lexer::{Lexer, Token};
use std::{borrow::BorrowMut, cmp, vec};

pub struct Parser {
    tokens: Option<Vec<Token>>,
}

impl Parser {
    pub fn new(input: String) -> Self {
        let lexer = Lexer::new(input);
        let tokens = lexer.get_tokens();

        Parser {
            tokens: Some(tokens),
        }
    }

    pub fn get_statements(mut self) -> Vec<Statement> {
        let mut statements = vec![];
        let mut iterator = self.tokens.take().unwrap().into_iter();

        while let Some(value) = iterator.next() {
            let statement = match value {
                Token::Heading1 => {
                    Statement::Heading1(self.parse_expression(iterator.borrow_mut(), None))
                }
                Token::Heading2 => {
                    Statement::Heading2(self.parse_expression(iterator.borrow_mut(), None))
                }
                Token::Heading3 => {
                    Statement::Heading3(self.parse_expression(iterator.borrow_mut(), None))
                }
                Token::Asterisk(count) => Statement::Plain(
                    self.parse_expression(iterator.borrow_mut(), Some(Token::Asterisk(count))),
                ),
                Token::Underscore(count) => Statement::Plain(
                    self.parse_expression(iterator.borrow_mut(), Some(Token::Underscore(count))),
                ),
                Token::OrderedListItem(num) => Statement::OrderedList((
                    num,
                    self.parse_expression(iterator.borrow_mut(), None),
                )),
                Token::UnorderdListItem => {
                    Statement::UnorderedList(self.parse_expression(iterator.borrow_mut(), None))
                }
                Token::Word(content) => Statement::Plain(
                    self.parse_expression(iterator.borrow_mut(), Some(Token::Word(content))),
                ),
                Token::NewLine => continue,
            };

            statements.push(statement);
        }

        statements
    }

    fn parse_expression(
        &mut self,
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
                        Some(Token::NewLine) => break,
                        Some(token) => tokens.push(token),
                        None => break,
                    }
                }

                let next = iterator.next();
                let children = self.parse_children(tokens, left_count, right_count, "*");

                self.parse_vec(iterator, next, children)
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
                        Some(Token::NewLine) => break,
                        Some(token) => tokens.push(token),
                        None => break,
                    }
                }

                let next = iterator.next();
                let children = self.parse_children(tokens, left_count, right_count, "_");

                self.parse_vec(iterator, next, children)
            }
            Some(Token::Word(content)) => {
                let mut tokens = vec![content];
                let mut next = None;

                loop {
                    let value = iterator.next();
                    match value {
                        Some(Token::Word(content)) => tokens.push(content),
                        Some(Token::NewLine) => break,
                        Some(token) => {
                            next = Some(token);
                            break;
                        }
                        None => break,
                    }
                }

                self.parse_vec(iterator, next, Expression::Text(tokens.join(" ")))
            }
            Some(Token::Heading1) => {
                let token = iterator.next();

                Expression::Vec(vec![
                    Expression::Text(String::from("#")),
                    self.parse_expression(iterator, token),
                ])
            }
            Some(Token::Heading2) => {
                let token = iterator.next();

                Expression::Vec(vec![
                    Expression::Text(String::from("##")),
                    self.parse_expression(iterator, token),
                ])
            }
            Some(Token::Heading3) => {
                let token = iterator.next();

                Expression::Vec(vec![
                    Expression::Text(String::from("###")),
                    self.parse_expression(iterator, token),
                ])
            }
            Some(Token::OrderedListItem(num)) => {
                let token = iterator.next();

                Expression::Vec(vec![
                    Expression::Text(num.to_string()),
                    self.parse_expression(iterator, token),
                ])
            }
            Some(Token::UnorderdListItem) => {
                let token = iterator.next();

                Expression::Vec(vec![
                    Expression::Text(String::from("-")),
                    self.parse_expression(iterator, token),
                ])
            }
            Some(Token::NewLine) => Expression::None,
            None => {
                let token = iterator.next();
                self.parse_expression(iterator, token)
            }
        }
    }

    fn parse_vec(
        &mut self,
        iterator: &mut dyn Iterator<Item = Token>,
        next: Option<Token>,
        children: Expression,
    ) -> Expression {
        match next {
            Some(token) => {
                let mut expressions = vec![
                    Expression::None,
                    self.parse_expression(iterator, Some(token)),
                ];

                if (expressions.len() == 2 && expressions[1] == Expression::None)
                    || expressions.len() == 1
                {
                    children
                } else {
                    expressions[0] = children;
                    Expression::Vec(expressions)
                }
            }
            None => children,
        }
    }

    fn parse_children(
        &mut self,
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

        if current.is_none() {
            return Expression::None;
        }

        let expression = match (left_count, right_count) {
            (3..=u32::MAX, 3..=u32::MAX) => Expression::BoldItalic(Box::new(
                self.parse_expression(iterator.borrow_mut(), current),
            )),
            (2..=u32::MAX, 2..=u32::MAX) => Expression::Bold(Box::new(
                self.parse_expression(iterator.borrow_mut(), current),
            )),
            (1..=u32::MAX, 1..=u32::MAX) => Expression::Italic(Box::new(
                self.parse_expression(iterator.borrow_mut(), current),
            )),
            (_, _) => self.parse_expression(iterator.borrow_mut(), current),
        };

        expression
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
    None,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Heading1(Expression),
    Heading2(Expression),
    Heading3(Expression),
    OrderedList((u32, Expression)),
    UnorderedList(Expression),
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

        let statements = setup_parser(String::from(
            "- something
- something else",
        ));

        assert_eq!(
            statements,
            vec![
                Statement::UnorderedList(Expression::Text(String::from("something"))),
                Statement::UnorderedList(Expression::Text(String::from("something else")))
            ]
        );
    }
}
