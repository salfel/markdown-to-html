use crate::parser::{Parser, Statement};

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
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluates_heading() {
        let evaluator = Evaluator::new(String::from("# heading"));
        let result = evaluator.eval();

        assert_eq!(result, "<h1>heading<h1>")
    }
}
