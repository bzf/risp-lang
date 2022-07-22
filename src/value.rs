use crate::ASTNode;

#[derive(Debug, PartialEq)]
pub enum Type {
    Number,
    Nil,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i64),
    Function(Function),
    Nil,
}

impl Value {
    pub fn value_type(&self) -> Type {
        match self {
            Value::Number(_) => Type::Number,

            Value::Function(_) => Type::Nil,
            Value::Nil => Type::Nil,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    identifier: String,
}

impl Function {
    pub fn new(identifier: String, parameter_list: Vec<String>, body: ASTNode) -> Self {
        Self { identifier }
    }

    pub fn identifier(&self) -> &str {
        &self.identifier
    }
}
