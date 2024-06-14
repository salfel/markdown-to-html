use std::borrow::BorrowMut;

use crate::lexer::Token;

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
                    Token::Word(content) => Statement::Plain(Self::parse_expression(
                        iterator.borrow_mut(),
                        Some(content),
                    )),
                };

                statements.push(statement);
            }
        }

        statements
    }

    fn parse_expression(
        iterator: &mut dyn Iterator<Item = Token>,
        current: Option<String>,
    ) -> Expression {
        let mut tokens = vec![];
        if let Some(word) = current {
            tokens.push(Token::Word(word));
        }

        for value in iterator {
            tokens.push(value);
        }

        let token = tokens
            .into_iter()
            .map(|token| {
                if let Token::Word(content) = token {
                    content
                } else {
                    String::from("")
                }
            })
            .collect::<Vec<String>>()
            .join(" ");

        Expression::Plain(token)
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
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
        let lexer = Lexer::new(String::from("# something\nsomething else"));
        let tokens = lexer.get_tokens();

        let parser = Parser::new(tokens);
        let statements = parser.get_statements();

        assert_eq!(
            statements,
            vec![
                Statement::Heading1(Expression::Plain(String::from("something"))),
                Statement::Plain(Expression::Plain(String::from("something else")))
            ]
        );
    }
}
