use crate::parser::{Expression, Parser, Statement};

pub struct Evaluator {
    statements: Vec<Statement>,
}

impl Evaluator {
    pub fn new(input: String) -> Evaluator {
        let parser = Parser::new(input);
        let statements = parser.parse();

        Evaluator { statements }
    }

    pub fn evaluate(self) -> String {
        let mut output = String::new();
        let mut iterator = self.statements.into_iter();

        while let Some(statement) = iterator.by_ref().next() {
            let evaluated = Self::evaluate_statement(statement, &mut iterator);
            output += &evaluated;
        }

        output
    }

    pub fn evaluate_statement(
        statement: Statement,
        iterator: &mut dyn Iterator<Item = Statement>,
    ) -> String {
        match statement {
            Statement::Heading(count, expression) => format!(
                "<h{}>{}</h{}>",
                count,
                Self::evaluate_expression(expression),
                count
            ),
            Statement::UnorderedListItem(expression) => {
                let mut items = format!("<li>{}</li>", Self::evaluate_expression(expression));

                while let Some(statement) = iterator.next() {
                    match statement {
                        Statement::UnorderedListItem(expression) => items.push_str(&format!(
                            "<li>{}</li>",
                            Self::evaluate_expression(expression)
                        )),
                        statement => {
                            return format!(
                                "<ul>{}</ul>{}",
                                items,
                                Self::evaluate_statement(statement, iterator),
                            )
                        }
                    }
                }

                format!("<ul>{}</ul>", items)
            }
            Statement::Plain(expression) => {
                format!("<p>{}</p>", Self::evaluate_expression(expression))
            }
        }
    }

    pub fn evaluate_expression(expression: Expression) -> String {
        match expression {
            Expression::Text(text) => text,
            Expression::Bold(expression) => {
                format!(
                    "<strong>{}</strong>",
                    Self::evaluate_expression(*expression)
                )
            }
            Expression::Italic(expression) => {
                format!("<i>{}</i>", Self::evaluate_expression(*expression))
            }
            Expression::BoldItalic(expression) => {
                format!(
                    "<strong><i>{}</i></strong>",
                    Self::evaluate_expression(*expression)
                )
            }
            Expression::Vec(expressions) => {
                let mut output = String::new();

                for expression in expressions {
                    output += &Self::evaluate_expression(expression);
                }

                output
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        let evaluator = Evaluator::new(String::from(
            "Hello, World!
## Hi there
#Hi",
        ));
        let output = evaluator.evaluate();

        assert_eq!(output, "<p>Hello, World!</p><h2>Hi there</h2><p>#Hi</p>");
    }

    #[test]
    fn evaluates_bold_italic() {
        let evaluator = Evaluator::new(String::from(
            "*Hi* **there**
# *Hi there**",
        ));
        let output = evaluator.evaluate();

        assert_eq!(
            output,
            "<p><i>Hi</i> <strong>there</strong></p><h1><i>Hi there</i>*</h1>"
        )
    }

    #[test]
    fn evaluates_list() {
        let evaluator = Evaluator::new(String::from(
            "- Hi
- there
- # fake heading
# heading",
        ));
        let output = evaluator.evaluate();

        assert_eq!(
            output,
            "<ul><li>Hi</li><li>there</li><li># fake heading</li></ul><h1>heading</h1>"
        )
    }
}
