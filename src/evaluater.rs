use crate::parser::{Expression, Parser, Statement};

pub struct Evaluator {
    statements: Vec<Statement>,
}

impl Evaluator {
    pub fn new(input: String) -> Evaluator {
        let parser = Parser::new(input);
        let statements = parser.get_statements();

        Evaluator { statements }
    }

    pub fn eval(self) -> String {
        let mut statements = vec![];

        for statement in self.statements {
            let content = match statement {
                Statement::Heading1(expression) => {
                    format!("<h1>{}</h1>", Self::eval_expression(expression))
                }
                Statement::Heading2(expression) => {
                    format!("<h2>{}</h2>", Self::eval_expression(expression))
                }
                Statement::Heading3(expression) => {
                    format!("<h3>{}</h3>", Self::eval_expression(expression))
                }
                Statement::OrderedList((_, expression)) => {
                    format!("<li>{}</li>", Self::eval_expression(expression))
                }
                Statement::Plain(expression) => {
                    format!("<p>{}</p>", Self::eval_expression(expression))
                }
            };

            statements.push(content);
        }

        statements.join("\n")
    }

    pub fn eval_expression(expression: Expression) -> String {
        match expression {
            Expression::Text(content) => content,
            Expression::Bold(expression) => {
                format!("<strong>{}</strong>", Self::eval_expression(*expression))
            }
            Expression::Italic(expression) => {
                format!("<i>{}</i>", Self::eval_expression(*expression))
            }
            Expression::BoldItalic(expression) => {
                format!(
                    "<strong><i>{}</i></strong>",
                    Self::eval_expression(*expression)
                )
            }
            Expression::Vec(expressions) => {
                let mut contents = vec![];

                for expression in expressions {
                    contents.push(Self::eval_expression(expression));
                }

                contents.join(" ")
            }
            Expression::None => String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluates_heading() {
        let evaluator = Evaluator::new(String::from(
            "# heading
## heading2
### heading3
plain
# **heading**
# *heading*
***paragraph***",
        ));
        let result = evaluator.eval();

        assert_eq!(
            result,
            "<h1>heading</h1>
<h2>heading2</h2>
<h3>heading3</h3>
<p>plain</p>
<h1><strong>heading</strong></h1>
<h1><i>heading</i></h1>
<p><strong><i>paragraph</i></strong></p>"
        )
    }

    #[test]
    fn evalutates_next() {
        let evaluator = Evaluator::new(String::from("**something** else"));
        let result = evaluator.eval();

        assert_eq!(result, "<p><strong>something</strong> else</p>")
    }
}
