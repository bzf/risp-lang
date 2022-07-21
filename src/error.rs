use crate::Token;

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
}
