use crate::{ASTNode, Value};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn evaluate(&mut self, expression: &ASTNode) -> Value {
        match expression {
            ASTNode::NumberLiteral(number) => Value::Number(*number),

            ASTNode::CallExpression(ref _name, ref _arguments) => {
                println!("{:?}", expression);
                return Value::Nil;
            }

            _ => Value::Nil,
        }
    }
}
