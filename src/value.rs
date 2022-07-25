use crate::ASTNode;

#[derive(Debug, PartialEq)]
pub enum Type {
    Number,
    String,
    Boolean,
    Function,
    Nil,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i64),
    String(String),
    Function(Function),
    Boolean(bool),
    Nil,
}

impl Value {
    pub fn to_display_string(&self) -> String {
        match self {
            Value::Number(number) => format!("{}", number),
            Value::String(string) => string.clone(),
            Value::Function(function) => format!("#<Function:{}>", function.identifier()),
            Value::Boolean(value) => format!("{}", value),
            Value::Nil => format!("nil"),
        }
    }
}

impl Value {
    pub fn value_type(&self) -> Type {
        match self {
            Value::Number(_) => Type::Number,
            Value::String(_) => Type::String,
            Value::Boolean(_) => Type::Boolean,

            Value::Function(_) => Type::Function,
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
