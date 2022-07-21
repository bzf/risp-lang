use crate::{ASTNode, Error, ErrorType, Value};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn evaluate(&self, expression: &ASTNode) -> Result<Value, Error> {
        match expression {
            ASTNode::NumberLiteral(number) => Ok(Value::Number(*number)),

            ASTNode::CallExpression(ref name, ref arguments) => {
                return self.evaluate_call_expression(name, arguments);
            }

            _ => Ok(Value::Nil),
        }
    }

    fn evaluate_call_expression(
        &self,
        name: &str,
        arguments: &Vec<ASTNode>,
    ) -> Result<Value, Error> {
        match &name[..] {
            "add" => {
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

                if numbers.len() == 0 {
                    return Err(Error::new("Too few arguments", ErrorType::TooFewArguments));
                }

                return Ok(Value::Number(numbers.iter().sum()));
            }

            _ => Err(Error::new(
                "Undefined function",
                ErrorType::UndefinedFunction(name.to_string()),
            )),
        }
    }
}
