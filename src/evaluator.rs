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

        for statement in self.statements {
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
}
