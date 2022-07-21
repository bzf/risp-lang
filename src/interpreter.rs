use crate::{ASTNode, Error, Value};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn evaluate(&mut self, expression: &ASTNode) -> Result<Value, Error> {
        match expression {
            ASTNode::NumberLiteral(number) => Ok(Value::Number(*number)),

            ASTNode::CallExpression(ref _name, ref _arguments) => {
                println!("{:?}", expression);
                return Ok(Value::Nil);
            }

            _ => Ok(Value::Nil),
        }
    }
}
