mod error;
mod interpreter;
mod parser;
mod tokenizer;
mod value;

pub use error::{Error, ErrorType};
pub use interpreter::Interpreter;
pub use parser::ASTNode;
pub use tokenizer::{tokenize, Token};
pub use value::{Type, Value};

pub fn parse_and_evaluate(input: &str) -> Result<Value, Error> {
    let interpreter = Interpreter::new();
    let tokens = tokenize(input);
    let expression = parser::parse_node(&mut tokens.into_iter().peekable())?;

    return interpreter.evaluate(&expression);
}
