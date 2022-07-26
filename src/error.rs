use crate::{Token, Type};

#[derive(Debug, PartialEq)]
pub struct Error {
    error_type: ErrorType,
}

impl Error {
    pub fn new(_message: &str, error_type: ErrorType) -> Self {
        return Self { error_type };
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorType {
    UnexpectedToken(Token),
    MissingToken,
    UndefinedFunction(String),
    ArgumentError,
    NotAFunction(String),
    TooFewArguments,
    IOError(std::io::ErrorKind),
    TypeError {
        expected_type: Type,
        actual_type: Type,
    },
}
