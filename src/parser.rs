use crate::lexer::{Lexer, Token};

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize(input);
        Parser { tokens }
    }

    pub fn parse(self) -> Vec<Statement> {
        let lines = Self::prepare_lines(&mut self.tokens.into_iter());
        let mut statements = Vec::new();

        for line in lines {
            statements.push(Self::parse_line(line));
        }

        statements
    }

    fn parse_line(tokens: Vec<Token>) -> Statement {
        let mut iterator = tokens.into_iter();
        let first = iterator.next();

        match first {
            Some(Token::Heading(count)) => {
                let next = iterator.next();
                match next {
                    Some(Token::WhiteSpace(white_space_count)) => {
                        if white_space_count > 1 {
                            Statement::Heading(
                                count,
                                Self::parse_expression(Self::prepend_array(
                                    iterator.collect(),
                                    vec![Token::WhiteSpace(white_space_count - 1)],
                                )),
                            )
                        } else {
                            Statement::Heading(count, Self::parse_expression(iterator.collect()))
                        }
                    }
                    Some(token) => Statement::Plain(Self::parse_expression(Self::prepend_array(
                        iterator.collect(),
                        vec![Token::Heading(count), token],
                    ))),
                    None => Statement::Plain(Self::parse_expression(Self::prepend_array(
                        iterator.collect(),
                        vec![Token::Heading(count)],
                    ))),
                }
            }
            Some(token) => Statement::Plain(Self::parse_expression(Self::prepend_array(
                iterator.collect(),
                vec![token],
            ))),
            None => Statement::Plain(Expression::Text(String::new())),
        }
    }

    fn parse_expression(tokens: Vec<Token>) -> Expression {
        let mut expressions = Vec::new();

        for token in tokens {
            match token {
                Token::Word(word) => {
                    Self::append_to_last(&mut expressions, word);
                }
                Token::WhiteSpace(count) => {
                    Self::append_to_last(&mut expressions, " ".repeat(count));
                }
                Token::Heading(count) => {
                    Self::append_to_last(&mut expressions, "#".repeat(count));
                }
                Token::NewLine => break,
            }
        }

        Self::tidy_expressions(expressions)
    }

    fn append_to_last(expressions: &mut Vec<Expression>, string: String) {
        let last = expressions.last_mut();
        if let Some(Expression::Text(last_text)) = last {
            *last_text += &string;
        } else {
            expressions.push(Expression::Text(string));
        }
    }

    fn prepare_lines(iterator: &mut dyn Iterator<Item = Token>) -> Vec<Vec<Token>> {
        let mut lines = vec![Vec::new()];
        for token in iterator {
            match token {
                Token::NewLine => lines.push(Vec::new()),
                token => {
                    if let Some(last) = lines.last_mut() {
                        last.push(token);
                    }
                }
            }
        }

        lines
    }

    fn tidy_expressions(mut expressions: Vec<Expression>) -> Expression {
        if expressions.len() == 1 {
            expressions.remove(0)
        } else {
            Expression::Vec(expressions)
        }
    }

    fn prepend_array<T>(mut array: Vec<T>, items: Vec<T>) -> Vec<T> {
        let length = items.len();
        for item in items {
            array.push(item);
        }
        array.rotate_right(length);
        array
    }
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Heading(usize, Expression),
    Plain(Expression),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Vec(Vec<Expression>),
    Text(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_heading() {
        let parser = Parser::new(String::from(
            "## Hello
#Hi",
        ));
        let statements = parser.parse();

        assert_eq!(
            statements,
            vec![
                Statement::Heading(2, Expression::Text("Hello".to_string())),
                Statement::Plain(Expression::Text("#Hi".to_string()))
            ]
        )
    }
}
