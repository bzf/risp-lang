mod environment;

use std::ops::Deref;

use crate::{value::Function, ASTNode, Error, ErrorType, Value};
use environment::EnvironmentStack;

pub struct Interpreter {
    environment_stack: EnvironmentStack,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment_stack: EnvironmentStack::new(),
        }
    }

    pub fn evaluate(&mut self, expression: &ASTNode) -> Result<Value, Error> {
        match expression {
            ASTNode::NumberLiteral(number) => Ok(Value::Number(*number)),

            ASTNode::CallExpression(ref name, ref arguments) => {
                return self.evaluate_call_expression(name, arguments);
            }

            ASTNode::FunctionDeclaration {
                identifier,
                parameter_list,
                body,
            } => {
                let function = Function::new(
                    identifier.to_string(),
                    parameter_list.clone(),
                    body.deref().clone(),
                );

                self.environment_stack
                    .set(identifier, Value::Function(function.clone()));

                return Ok(Value::Function(function));
            }

            ASTNode::Identifier(name) => {
                if let Some(value) = self.environment_stack.get(name) {
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
                    self.environment_stack.set(name, value.clone());
                    return Ok(value);
                }

                _ => {
                    return Err(Error::new(
                        "Wrong number of arguments",
                        ErrorType::ArgumentError,
                    ));
                }
            },

            name => {
                let value = {
                    let value = self.environment_stack.get(name);
                    value.clone().ok_or(Error::new(
                        "Undefined",
                        ErrorType::UndefinedFunction(name.to_string()),
                    ))?
                };

                if let Value::Function(function) = value {
                    if function.parameter_list().len() != arguments.len() {
                        return Err(Error::new("Too few arguments", ErrorType::TooFewArguments));
                    }

                    // TODO: Push the argument values onto the stack

                    let result = self.evaluate(&function.body().clone());

                    // TODO: Pop the call stack

                    return result;
                } else {
                    Err(Error::new(
                        "Not a function",
                        ErrorType::NotAFunction(name.to_string()),
                    ))
                }
            }
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
            .environment_stack
            .set("my-var", Value::Number(3));

        let result = interpreter.evaluate(&ASTNode::Identifier("my-var".to_string()));

        assert_eq!(result, Ok(Value::Number(3)));
    }
}
