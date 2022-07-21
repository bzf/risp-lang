mod interpreter;
mod parser;
mod tokenizer;
mod value;

pub use interpreter::Interpreter;
pub use parser::ASTNode;
pub use tokenizer::{tokenize, Token};
pub use value::Value;

pub fn parse_and_evaluate(input: &str) -> Option<Value> {
    let mut interpreter = Interpreter::new();
    let tokens = tokenize(input);
    let expression = parser::parse_node(&mut tokens.into_iter().peekable())?;
    return Some(interpreter.evaluate(&expression));
}
