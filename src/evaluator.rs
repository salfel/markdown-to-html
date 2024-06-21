use crate::parser::{Expression, Statement};

pub struct Evaluator {}

impl Evaluator {
    pub fn evaluate(statements: Vec<Statement>) -> String {
        let mut output = String::new();

        for statement in statements {
            let evaluated = match statement {
                Statement::Heading(count, expression) => format!(
                    "<h{}>{}</h{}>",
                    count,
                    Self::evaluate_expression(expression),
                    count
                ),
                Statement::Plain(expression) => {
                    format!("<p>{}</p>", Self::evaluate_expression(expression))
                }
            };

            output += &evaluated;
        }

        output
    }

    pub fn evaluate_expression(expression: Expression) -> String {
        match expression {
            Expression::Text(text) => text,
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
    use crate::{lexer::Lexer, parser::Parser};

    fn get_output(input: &str) -> String {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize(input.to_string());
        let statements = Parser::parse(tokens);
        Evaluator::evaluate(statements)
    }

    #[test]
    fn test_evaluate() {
        let output = get_output(
            "Hello, World!
## Hi there
#Hi",
        );

        assert_eq!(output, "<p>Hello, World!</p><h2>Hi there</h2><p>#Hi</p>");
    }
}
