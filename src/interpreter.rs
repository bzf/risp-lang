use std::collections::HashMap;

use crate::{ASTNode, Error, ErrorType, Value};

pub struct Interpreter {
    variables: HashMap<String, Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn evaluate(&mut self, expression: &ASTNode) -> Result<Value, Error> {
        match expression {
            ASTNode::NumberLiteral(number) => Ok(Value::Number(*number)),

            ASTNode::CallExpression(ref name, ref arguments) => {
                return self.evaluate_call_expression(name, arguments);
            }

            ASTNode::Identifier(name) => {
                if let Some(value) = self.variables.get(name) {
                    return Ok(value.clone());
                } else {
                    return Ok(Value::Nil);
                }
            }
        }
    }

    fn evaluate_call_expression(
        &mut self,
        name: &str,
        arguments: &Vec<ASTNode>,
    ) -> Result<Value, Error> {
        match &name[..] {
            "add" => {
                let numbers = self.number_arguments(arguments)?;

                if numbers.len() == 0 {
                    return Err(Error::new("Too few arguments", ErrorType::TooFewArguments));
                }

                return Ok(Value::Number(numbers.iter().sum()));
            }

            "subtract" => {
                let numbers = self.number_arguments(arguments)?;

                if numbers.len() == 0 {
                    return Err(Error::new("Too few arguments", ErrorType::TooFewArguments));
                }

                return Ok(Value::Number(
                    numbers.into_iter().reduce(|acc, a| acc - a).unwrap_or(0),
                ));
            }

            "define" => match &arguments[..] {
                [ASTNode::Identifier(name), value_node] => {
                    let value = self.evaluate(value_node)?;
                    self.variables.insert(name.to_string(), value.clone());
                    return Ok(value);
                }

                _ => {
                    return Err(Error::new(
                        "Wrong number of arguments",
                        ErrorType::ArgumentError,
                    ));
                }
            },

            _ => Err(Error::new(
                "Undefined function",
                ErrorType::UndefinedFunction(name.to_string()),
            )),
        }
    }

    fn number_arguments(&mut self, arguments: &Vec<ASTNode>) -> Result<Vec<i64>, Error> {
        let values = arguments.iter().map(|argument| self.evaluate(argument));
        let mut numbers: Vec<i64> = vec![];

        for value in values {
            match value {
                Ok(Value::Number(number)) => numbers.push(number),

                Ok(value) => {
                    return Err(Error::new(
                        "add requires all arguments to be Numbers",
                        ErrorType::TypeError {
                            expected_type: crate::Type::Number,
                            actual_type: value.value_type(),
                        },
                    ));
                }

                Err(error) => return Err(error),
            }
        }

        return Ok(numbers);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading_variables() {
        let mut interpreter = Interpreter::new();
        interpreter
            .variables
            .insert("my-var".to_string(), Value::Number(3));

        let result = interpreter.evaluate(&ASTNode::Identifier("my-var".to_string()));

        assert_eq!(result, Ok(Value::Number(3)));
    }
}
