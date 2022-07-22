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
    parameter_list: Vec<String>,
    body: ASTNode,
}

impl Function {
    pub fn new(identifier: String, parameter_list: Vec<String>, body: ASTNode) -> Self {
        Self {
            identifier,
            parameter_list,
            body,
        }
    }

    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    pub fn parameter_list(&self) -> &Vec<String> {
        &self.parameter_list
    }

    pub fn body(&self) -> &ASTNode {
        &self.body
    }
}
