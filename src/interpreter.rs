mod environment;

use std::{
    collections::{HashMap, VecDeque},
    ops::Deref,
};

use crate::{parser::parse, tokenize, value::Function, ASTNode, Error, ErrorType, Type, Value};
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

    pub fn evaluate_file(&mut self, filepath: &str) -> Result<(), Error> {
        let file_content = std::fs::read_to_string(filepath)
            .map_err(|error| Error::new("IO error", ErrorType::IOError(error.kind())))?;

        let tokens = tokenize(&file_content);
        let expressions = parse(&mut tokens.into_iter().peekable())?;

        for expression in expressions.iter() {
            self.evaluate(expression)?;
        }

        return Ok(());
    }

    pub fn evaluate(&mut self, expression: &ASTNode) -> Result<Value, Error> {
        match expression {
            ASTNode::NumberLiteral(number) => Ok(Value::Number(*number)),

            ASTNode::ListExpression(expressions) => {
                let mut values = vec![];

                for expression in expressions.iter() {
                    values.push(self.evaluate(expression)?);
                }

                Ok(Value::List(values))
            }

            ASTNode::BooleanLiteral(value) => Ok(Value::Boolean(*value)),

            ASTNode::CallExpression(ref name, ref arguments) => {
                return self.evaluate_call_expression(name, arguments);
            }

            ASTNode::IfExpression {
                ref expression,
                ref when_true,
                ref when_false,
            } => {
                if self.evaluate(expression)?.is_truthy() {
                    self.evaluate(when_true)
                } else {
                    self.evaluate(when_false)
                }
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

            ASTNode::StringLiteral(string) => {
                return Ok(Value::String(string.to_string()));
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

            "car" => match &arguments[..] {
                [value_node] => {
                    let value = self.evaluate(value_node)?;

                    match value {
                        Value::List(values) => {
                            let mut cloned_values: VecDeque<Value> = VecDeque::from(values);
                            return Ok(cloned_values.pop_front().unwrap_or(Value::Nil));
                        }

                        _ => {
                            return Err(Error::new(
                                "Type error",
                                ErrorType::TypeError {
                                    expected_type: Type::List,
                                    actual_type: value.value_type(),
                                },
                            ))
                        }
                    }
                }

                _ => {
                    return Err(Error::new(
                        "Wrong number of arguments",
                        ErrorType::ArgumentError,
                    ));
                }
            },

            "is-empty" => match &arguments[..] {
                [value_node] => {
                    let value = self.evaluate(value_node)?;

                    match value {
                        Value::List(values) => Ok(Value::Boolean(values.is_empty())),

                        _ => {
                            return Err(Error::new(
                                "Type error",
                                ErrorType::TypeError {
                                    expected_type: Type::List,
                                    actual_type: value.value_type(),
                                },
                            ))
                        }
                    }
                }

                _ => {
                    return Err(Error::new(
                        "Wrong number of arguments",
                        ErrorType::ArgumentError,
                    ));
                }
            },

            "append" => match &arguments[..] {
                [list_node, value_node] => {
                    let list = self.evaluate(list_node)?;
                    let value = self.evaluate(value_node)?;

                    match (list, &value) {
                        (Value::List(values), value) => {
                            let mut new_values = values.clone();
                            new_values.push(value.clone());
                            Ok(Value::List(new_values))
                        }

                        _ => {
                            return Err(Error::new(
                                "Type error",
                                ErrorType::TypeError {
                                    expected_type: Type::List,
                                    actual_type: value.value_type(),
                                },
                            ))
                        }
                    }
                }

                _ => {
                    return Err(Error::new(
                        "Wrong number of arguments",
                        ErrorType::ArgumentError,
                    ));
                }
            },

            "prepend" => match &arguments[..] {
                [list_node, value_node] => {
                    let list = self.evaluate(list_node)?;
                    let value = self.evaluate(value_node)?;

                    match (list, &value) {
                        (Value::List(values), value) => {
                            let mut new_values = VecDeque::from(values.clone());
                            new_values.push_front(value.clone());
                            Ok(Value::List(new_values.into_iter().collect()))
                        }

                        _ => {
                            return Err(Error::new(
                                "Type error",
                                ErrorType::TypeError {
                                    expected_type: Type::List,
                                    actual_type: value.value_type(),
                                },
                            ))
                        }
                    }
                }

                _ => {
                    return Err(Error::new(
                        "Wrong number of arguments",
                        ErrorType::ArgumentError,
                    ));
                }
            },

            "is-nil" => match &arguments[..] {
                [value_node] => Ok(Value::Boolean(self.evaluate(value_node)?.is_nil())),

                _ => {
                    return Err(Error::new(
                        "Wrong number of arguments",
                        ErrorType::ArgumentError,
                    ));
                }
            },

            "cdr" => match &arguments[..] {
                [value_node] => {
                    let value = self.evaluate(value_node)?;

                    match value {
                        Value::List(values) => {
                            let mut cloned_values: VecDeque<Value> = VecDeque::from(values);
                            cloned_values.pop_front();

                            return Ok(Value::List(cloned_values.into_iter().collect()));
                        }

                        _ => {
                            return Err(Error::new(
                                "Type error",
                                ErrorType::TypeError {
                                    expected_type: Type::List,
                                    actual_type: value.value_type(),
                                },
                            ))
                        }
                    }
                }

                _ => {
                    return Err(Error::new(
                        "Wrong number of arguments",
                        ErrorType::ArgumentError,
                    ));
                }
            },

            "println" => match &arguments[..] {
                nodes => {
                    let mut values = Vec::new();

                    for node in nodes.iter() {
                        values.push(self.evaluate(node)?);
                    }

                    println!(
                        "{}",
                        values
                            .iter()
                            .map(|a| a.to_display_string())
                            .collect::<Vec<String>>()
                            .join(" ")
                    );

                    return Ok(Value::List(values));
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

                    // Push the argument values onto the stack
                    let mut values = Vec::new();

                    for argument in arguments.iter() {
                        values.push(self.evaluate(argument)?);
                    }

                    let mut arguments: HashMap<String, Value> = HashMap::new();

                    for (index, key) in function.parameter_list().iter().enumerate() {
                        let value = &values[index];

                        arguments.insert(key.to_string(), value.clone());
                    }

                    self.environment_stack.push_environment(arguments);

                    let result = self.evaluate(&function.body().clone());

                    // Pop the call stack
                    self.environment_stack.pop_environment();

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
