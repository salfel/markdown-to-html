use crate::lexer::Token;

pub struct Parser {}

impl Parser {
    pub fn parse(tokens: Vec<Token>) -> Vec<Statement> {
        let lines = Self::prepare_lines(&mut tokens.into_iter());
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
                    let last = expressions.last_mut();
                    if let Some(Expression::Text(last_text)) = last {
                        *last_text += &word;
                    } else {
                        expressions.push(Expression::Text(word));
                    }
                }
                Token::WhiteSpace(count) => {
                    let last = expressions.last_mut();
                    if let Some(Expression::Text(last_text)) = last {
                        *last_text += &" ".repeat(count);
                    } else {
                        expressions.push(Expression::Text(" ".repeat(count)))
                    }
                }
                Token::Heading(count) => {
                    let last = expressions.last_mut();
                    if let Some(Expression::Text(last_text)) = last {
                        *last_text += &"#".repeat(count);
                    } else {
                        expressions.push(Expression::Text("#".repeat(count)));
                    }
                }
                Token::NewLine => break,
            }
        }

        Self::tidy_expressions(expressions)
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
    use crate::lexer::Lexer;

    #[test]
    fn parses_heading() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize(String::from(
            "## Hello
#Hi",
        ));
        let statements = Parser::parse(tokens);

        assert_eq!(
            statements,
            vec![
                Statement::Heading(2, Expression::Text("Hello".to_string())),
                Statement::Plain(Expression::Text("#Hi".to_string()))
            ]
        )
    }
}
