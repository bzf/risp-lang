#[derive(Debug, PartialEq)]
pub enum Type {
    Number,
    Nil,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Number(i64),
    Nil,
}

impl Value {
    pub fn value_type(&self) -> Type {
        match self {
            Value::Number(_) => Type::Number,
            Value::Nil => Type::Nil,
        }
    }
}
