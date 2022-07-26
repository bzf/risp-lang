use crate::ASTNode;

#[derive(Debug, PartialEq)]
pub enum Type {
    Number,
    String,
    List,
    Boolean,
    Function,
    Nil,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i64),
    String(String),
    List(Vec<Value>),
    Function(Function),
    Boolean(bool),
    Nil,
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Number(number) => number > &0,
            Value::Boolean(value) => *value,
            Value::String(value) => value.len() > 0,
            Value::List(value) => !value.is_empty(),
            Value::Function(_) => true,
            Value::Nil => false,
        }
    }

    pub fn to_display_string(&self) -> String {
        match self {
            Value::Number(number) => format!("{}", number),
            Value::String(string) => string.clone(),
            Value::Function(function) => format!("#<Function:{}>", function.identifier()),
            Value::Boolean(value) => format!("{}", value),
            Value::List(value) => {
                format!(
                    "({})",
                    value
                        .iter()
                        .map(|v| v.to_display_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }
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
            Value::List(_) => Type::List,

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
