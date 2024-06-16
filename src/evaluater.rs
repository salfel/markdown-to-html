use crate::parser::{Expression, Parser, Statement};

pub struct Evaluator {
    iterator: Box<dyn Iterator<Item = Statement>>,
}

impl Evaluator {
    pub fn new(input: String) -> Evaluator {
        let parser = Parser::new(input);
        let statements = parser.get_statements();

        Evaluator {
            iterator: Box::new(statements.into_iter()),
        }
    }

    pub fn eval(mut self) -> String {
        let mut statements = vec![];

        while let Some(statement) = self.iterator.next() {
            let content = self.match_statement(statement);

            statements.push(content);
        }

        statements.join("\n")
    }

    fn match_statement(&mut self, statement: Statement) -> String {
        match statement {
            Statement::Heading1(expression) => {
                format!("<h1>{}</h1>", Self::eval_expression(expression))
            }
            Statement::Heading2(expression) => {
                format!("<h2>{}</h2>", Self::eval_expression(expression))
            }
            Statement::Heading3(expression) => {
                format!("<h3>{}</h3>", Self::eval_expression(expression))
            }
            Statement::OrderedList((num, expression)) => self.eval_ordered_list(num, expression),

            Statement::Plain(expression) => {
                format!("<p>{}</p>", Self::eval_expression(expression))
            }
        }
    }

    fn format_list_item(expression: Expression) -> String {
        format!("<li>{}</li>", Self::eval_expression(expression))
    }

    fn eval_ordered_list(&mut self, num: u32, expression: Expression) -> String {
        if num != 1 {
            return format!("{num}. {}", Self::eval_expression(expression));
        }

        let mut last_num = num;
        let mut expressions = vec![Self::format_list_item(expression)];
        let mut next = None;

        for statement in self.iterator.by_ref() {
            match statement {
                Statement::OrderedList((num, expression)) if num == last_num + 1 => {
                    last_num = num;
                    expressions.push(Self::format_list_item(expression));
                }
                _ => {
                    next = Some(statement);
                    break;
                }
            }
        }

        let mut content = format!("<ol>\n{}\n</ol>", expressions.join("\n"));

        if let Some(statement) = next {
            content.push('\n');
            content.push_str(&self.match_statement(statement));
        }

        content
    }

    fn eval_expression(expression: Expression) -> String {
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

    #[test]
    fn evaluates_ordered_list() {
        let evaluator = Evaluator::new(String::from(
            "1. first
2. second",
        ));
        let result = evaluator.eval();

        assert_eq!(
            result,
            String::from(
                "<ol>
<li>first</li>
<li>second</li>
</ol>"
            )
        );

        let evaluator = Evaluator::new(String::from(
            "1. first
2. second
# heading",
        ));
        let result = evaluator.eval();

        assert_eq!(
            result,
            String::from(
                "<ol>
<li>first</li>
<li>second</li>
</ol>
<h1>heading</h1>"
            )
        );

        let evaluator = Evaluator::new(String::from(
            "1. first
3. second
# heading",
        ));
        let result = evaluator.eval();

        assert_eq!(
            result,
            "<ol>
<li>first</li>
</ol>
3. second
<h1>heading</h1>"
        )
    }
}
