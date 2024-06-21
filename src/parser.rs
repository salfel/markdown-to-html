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
        let mut iterator = tokens.into_iter();

        while let Some(token) = iterator.next() {
            match token {
                Token::Word(word) => {
                    Self::append_to_last(&mut expressions, word);
                }
                Token::Asterisk(left_count) => {
                    let mut tokens = Vec::new();
                    let mut right_count = 0;

                    for token in iterator.by_ref() {
                        match token {
                            Token::Asterisk(count) => {
                                right_count = count;
                                break;
                            }
                            token => tokens.push(token),
                        }
                    }

                    if left_count > right_count {
                        expressions.push(Expression::Text("*".repeat(left_count - right_count)));
                    }

                    let expression = match (left_count, right_count) {
                        (3..=usize::MAX, 3..=usize::MAX) => {
                            Expression::BoldItalic(Box::new(Self::parse_expression(tokens)))
                        }
                        (2..=usize::MAX, 2..=usize::MAX) => {
                            Expression::Bold(Box::new(Self::parse_expression(tokens)))
                        }
                        (1..=usize::MAX, 1..=usize::MAX) => {
                            Expression::Italic(Box::new(Self::parse_expression(tokens)))
                        }
                        _ => Expression::Text("*".repeat(left_count + right_count)),
                    };
                    expressions.push(expression);

                    if right_count > left_count {
                        expressions.push(Expression::Text("*".repeat(right_count - left_count)));
                    }
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
    Bold(Box<Expression>),
    Italic(Box<Expression>),
    BoldItalic(Box<Expression>),
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
                Statement::Plain(Expression::Text("#Hi".to_string())),
            ]
        )
    }

    #[test]
    fn parses_bold_italic() {
        let parser = Parser::new(String::from(
            "*Hi* **there**
***Hello**",
        ));
        let statements = parser.parse();

        assert_eq!(
            statements,
            vec![
                Statement::Plain(Expression::Vec(vec![
                    Expression::Italic(Box::new(Expression::Text("Hi".to_string()))),
                    Expression::Text(" ".to_string()),
                    Expression::Bold(Box::new(Expression::Text("there".to_string())))
                ])),
                Statement::Plain(Expression::Vec(vec![
                    Expression::Text("*".to_string()),
                    Expression::Bold(Box::new(Expression::Text("Hello".to_string())))
                ]))
            ]
        )
    }
}
