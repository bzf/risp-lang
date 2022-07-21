use std::iter::Peekable;

use crate::Token;

#[derive(Debug, PartialEq)]
pub enum ASTNode {
    NumberLiteral(i64),
    Identifier(String),

    CallExpression(String, Vec<ASTNode>),
}

pub fn parse_node(tokens: &mut Peekable<std::vec::IntoIter<Token>>) -> Option<ASTNode> {
    match tokens.next()? {
        Token::Number(number) => Some(ASTNode::NumberLiteral(number)),
        Token::Name(name) => Some(ASTNode::Identifier(name)),

        Token::OpeningParenthesis => {
            let identifier = {
                if let Some(ASTNode::Identifier(name)) = parse_node(tokens) {
                    name
                } else {
                    panic!("Expected an identifier after parsing an '('");
                }
            };

            let mut arguments: Vec<ASTNode> = vec![];

            while let Some(next_token) = tokens.peek() {
                if Token::ClosingParenthesis != *next_token {
                    if let Some(argument) = parse_node(tokens) {
                        arguments.push(argument);
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            return Some(ASTNode::CallExpression(identifier, arguments));
        }

        Token::ClosingParenthesis => {
            panic!("Unexpected closing parenthesis");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_number_literal() {
        assert_eq!(
            parse_node(&mut vec![Token::Number(123)].into_iter().peekable()),
            Some(ASTNode::NumberLiteral(123))
        )
    }

    #[test]
    fn test_parsing_identifier() {
        assert_eq!(
            parse_node(
                &mut vec![Token::Name("hello-there".to_string())]
                    .into_iter()
                    .peekable()
            ),
            Some(ASTNode::Identifier("hello-there".to_string()))
        )
    }

    #[test]
    fn test_parsing_call_expression() {
        assert_eq!(
            parse_node(
                &mut vec![
                    Token::OpeningParenthesis,
                    Token::Name("hello-there".to_string()),
                    Token::Number(123),
                ]
                .into_iter()
                .peekable()
            ),
            Some(ASTNode::CallExpression(
                "hello-there".to_string(),
                vec![ASTNode::NumberLiteral(123)]
            ))
        )
    }
}
