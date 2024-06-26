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
                let token = iterator.next();
                let tokens = iterator.collect();
                match token {
                    Some(Token::WhiteSpace(_)) => {
                        Statement::Heading(count, Self::parse_expression(tokens))
                    }
                    Some(token) => Statement::Plain(Self::parse_expression(Self::prepend_array(
                        tokens,
                        vec![Token::Heading(count), token],
                    ))),
                    None => Statement::Plain(Self::parse_expression(Self::prepend_array(
                        tokens,
                        vec![Token::Heading(count)],
                    ))),
                }
            }
            Some(Token::Number(number)) => {
                let mut next = iterator.next();
                match next {
                    Some(token) => {
                        if !token.expect(&Token::Dot) {
                            return Self::get_plain_statement(
                                &mut iterator,
                                vec![Token::Number(number), token],
                            );
                        }

                        next = iterator.next();
                        let next_token = match next {
                            Some(token) => token,
                            None => {
                                return Self::get_plain_statement(
                                    &mut iterator,
                                    vec![Token::Number(number), Token::Dot],
                                );
                            }
                        };

                        if !next_token.expect(&Token::WhiteSpace(1)) {
                            return Self::get_plain_statement(
                                &mut iterator,
                                vec![Token::Number(number), Token::Dot, next_token],
                            );
                        }

                        Statement::OrderedListItem(
                            number,
                            Self::parse_expression(iterator.collect()),
                        )
                    }
                    None => Self::get_plain_statement(&mut iterator, vec![Token::Number(number)]),
                }
            }
            Some(Token::Hyphen) => {
                let token = iterator.next();
                let tokens: Vec<Token> = iterator.collect();
                match token {
                    Some(Token::WhiteSpace(_)) => {
                        let mut iterator = tokens.into_iter();

                        match iterator.next() {
                            Some(Token::LBracket) => {
                                let next = iterator.next();
                                let mut checked = false;

                                match next {
                                    Some(Token::WhiteSpace(1)) => {}
                                    Some(Token::Word(word)) if word == "x" => {
                                        checked = true;
                                    }
                                    Some(token) => {
                                        return Statement::Plain(Self::parse_expression(
                                            Self::prepend_array(
                                                iterator.collect(),
                                                vec![Token::LBracket, token],
                                            ),
                                        ));
                                    }
                                    None => {
                                        return Statement::Plain(Self::parse_expression(vec![
                                            Token::LBracket,
                                        ]))
                                    }
                                }

                                let checked_token = if checked {
                                    Token::Word("x".to_string())
                                } else {
                                    Token::WhiteSpace(1)
                                };

                                match iterator.next() {
                                    Some(Token::RBracket) => {}
                                    Some(token) => {
                                        return Statement::UnorderedListItem(
                                            Self::parse_expression(Self::prepend_array(
                                                iterator.collect(),
                                                vec![Token::LBracket, checked_token, token],
                                            )),
                                        )
                                    }
                                    None => {
                                        return Statement::UnorderedListItem(
                                            Self::parse_expression(Self::prepend_array(
                                                iterator.collect(),
                                                vec![Token::LBracket, checked_token],
                                            )),
                                        )
                                    }
                                }

                                match iterator.next() {
                                    Some(Token::WhiteSpace(_)) => {}
                                    Some(token) => {
                                        return Statement::UnorderedListItem(
                                            Self::parse_expression(Self::prepend_array(
                                                iterator.collect(),
                                                vec![
                                                    Token::LBracket,
                                                    checked_token,
                                                    Token::RBracket,
                                                    token,
                                                ],
                                            )),
                                        )
                                    }
                                    None => {
                                        return Statement::UnorderedListItem(
                                            Self::parse_expression(vec![
                                                Token::LBracket,
                                                checked_token,
                                                Token::RBracket,
                                            ]),
                                        )
                                    }
                                }

                                Statement::TaskListItem(
                                    checked,
                                    Self::parse_expression(iterator.collect()),
                                )
                            }
                            Some(token) => Statement::UnorderedListItem(Self::parse_expression(
                                Self::prepend_array(iterator.collect(), vec![token]),
                            )),
                            None => Statement::UnorderedListItem(Self::parse_expression(
                                iterator.collect(),
                            )),
                        }
                    }
                    Some(token) => Statement::Plain(Self::parse_expression(Self::prepend_array(
                        tokens,
                        vec![Token::Hyphen, token],
                    ))),
                    None => Statement::Plain(Self::parse_expression(Self::prepend_array(
                        tokens,
                        vec![Token::Hyphen],
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

    fn get_plain_statement(
        iterator: &mut dyn Iterator<Item = Token>,
        prepend: Vec<Token>,
    ) -> Statement {
        Statement::Plain(Self::parse_expression(Self::prepend_array(
            iterator.collect(),
            prepend,
        )))
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
                Token::LBracket => {
                    let mut title_tokens = Vec::new();
                    let mut link_tokens = Vec::new();

                    let mut found = false;
                    for token in iterator.by_ref() {
                        match token {
                            Token::RBracket => {
                                found = true;
                                break;
                            }
                            token => title_tokens.push(token),
                        }
                    }

                    if !found {
                        expressions.push(Self::parse_expression(Self::prepend_array(
                            title_tokens,
                            vec![Token::LBracket.to_word()],
                        )));
                        continue;
                    }

                    let next = iterator.next();
                    let mut found = false;
                    if let Some(Token::LParen) = &next {
                        for token in iterator.by_ref() {
                            match token {
                                Token::RParen => {
                                    found = true;
                                    break;
                                }
                                token => link_tokens.push(token),
                            }
                        }
                    }

                    if !found {
                        let mut tokens =
                            Self::prepend_array(title_tokens, vec![Token::LBracket.to_word()]);
                        tokens.push(Token::RBracket.to_word());
                        if let Some(next) = next {
                            tokens.push(next);
                        }
                        tokens.append(&mut link_tokens);
                        expressions.push(Self::parse_expression(tokens));
                        continue;
                    }

                    expressions.push(Expression::Link(
                        Box::new(Self::parse_expression(title_tokens)),
                        Box::new(Self::parse_expression(link_tokens)),
                    ));
                }
                Token::NewLine => break,
                token => {
                    Self::append_to_last(&mut expressions, token.to_string());
                }
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
    OrderedListItem(usize, Expression),
    UnorderedListItem(Expression),
    TaskListItem(bool, Expression),
    Plain(Expression),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Vec(Vec<Expression>),
    Bold(Box<Expression>),
    Italic(Box<Expression>),
    BoldItalic(Box<Expression>),
    Link(Box<Expression>, Box<Expression>),
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

    #[test]
    fn parses_list() {
        let parser = Parser::new(String::from(
            "- Hi
1. Hello
1.Hello
1 Hi
1.",
        ));
        let statements = parser.parse();

        assert_eq!(
            statements,
            vec![
                Statement::UnorderedListItem(Expression::Text("Hi".to_string())),
                Statement::OrderedListItem(1, Expression::Text("Hello".to_string())),
                Statement::Plain(Expression::Text("1.Hello".to_string())),
                Statement::Plain(Expression::Text("1 Hi".to_string())),
                Statement::Plain(Expression::Text("1.".to_string())),
            ]
        )
    }

    #[test]
    fn parses_link() {
        let parser = Parser::new(String::from(
            "[title](https://example.test)
[title something else
[title]https://example.test
[title](https://example.test]
[title]https://example.test",
        ));
        let statements = parser.parse();

        assert_eq!(
            statements,
            vec![
                Statement::Plain(Expression::Link(
                    Box::new(Expression::Text("title".to_string())),
                    Box::new(Expression::Text("https://example.test".to_string()))
                )),
                Statement::Plain(Expression::Text("[title something else".to_string())),
                Statement::Plain(Expression::Text("[title]https://example.test".to_string())),
                Statement::Plain(Expression::Text(
                    "[title](https://example.test]".to_string()
                )),
                Statement::Plain(Expression::Text("[title]https://example.test".to_string())),
            ]
        )
    }

    #[test]
    fn parses_task_list() {
        let parsers = Parser::new(String::from(
            "- [ ] Hi
- [x] Hello
- [ hi there
- [x hi there
- [x]hi there",
        ));
        let statements = parsers.parse();

        assert_eq!(
            statements,
            vec![
                Statement::TaskListItem(false, Expression::Text("Hi".to_string())),
                Statement::TaskListItem(true, Expression::Text("Hello".to_string())),
                Statement::UnorderedListItem(Expression::Text("[ hi there".to_string())),
                Statement::UnorderedListItem(Expression::Text("[x hi there".to_string())),
                Statement::UnorderedListItem(Expression::Text("[x]hi there".to_string())),
            ]
        )
    }
}
