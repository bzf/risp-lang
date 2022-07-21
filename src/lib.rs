mod parser;
mod tokenizer;

pub use parser::ASTNode;
pub use tokenizer::{tokenize, Token};

pub fn parse_and_evaluate(input: &str) -> Option<ASTNode> {
    let tokens = tokenize(input);
    return parser::parse_node(&mut tokens.into_iter().peekable());
}
