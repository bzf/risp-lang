use std::{iter::Peekable, vec::IntoIter};

use crate::{Error, ErrorType, Token};

#[derive(Debug, PartialEq)]
pub enum ASTNode {
    NumberLiteral(i64),
    Identifier(String),

    CallExpression(String, Vec<ASTNode>),
}

pub fn parse_node(tokens: &mut Peekable<std::vec::IntoIter<Token>>) -> Result<ASTNode, Error> {
    if let Some(token) = tokens.next() {
        match token {
            Token::Number(number) => Ok(ASTNode::NumberLiteral(number)),

            Token::Name(name) => Ok(ASTNode::Identifier(name)),

            Token::NegativeSymbol => {
                if let Ok(ASTNode::NumberLiteral(number)) = parse_node(tokens) {
                    Ok(ASTNode::NumberLiteral(-number))
                } else {
                    return Err(Error::new(
                        "Expected a number followed by a - symbol",
                        ErrorType::UnexpectedToken(token),
                    ));
                }
            }

            Token::OpeningParenthesis => {
                if let Some(token) = tokens.peek() {
                    match token {
                        Token::Name(_name) => parse_call_expression(tokens),

                        _ => Err(Error::new(
                            "Unexpected token",
                            ErrorType::UnexpectedToken(token.clone()),
                        )),
                    }
                } else {
                    return Err(Error::new("Expected more tokens", ErrorType::MissingToken));
                }
            }

            _ => Err(Error::new(
                "Unexpected token",
                ErrorType::UnexpectedToken(token),
            )),
        }
    } else {
        return Err(Error::new("Expected more tokens", ErrorType::MissingToken));
    }
}

fn parse_call_expression(tokens: &mut Peekable<IntoIter<Token>>) -> Result<ASTNode, Error> {
    let identifier = {
        if let Ok(ASTNode::Identifier(name)) = parse_node(tokens) {
            name
        } else {
            return Err(Error::new(
                "Expected an identifier following '('",
                ErrorType::UnexpectedToken(Token::OpeningParenthesis),
            ));
        }
    };

    let mut arguments: Vec<ASTNode> = vec![];

    while let Some(next_token) = tokens.peek() {
        if *next_token == Token::ClosingParenthesis {
            tokens.next().unwrap();
            return Ok(ASTNode::CallExpression(identifier, arguments));
        } else {
            arguments.push(parse_node(tokens)?);
        }
    }

    return Err(Error::new("Expected missing ')'", ErrorType::MissingToken));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_negative_number_literals() {
        assert_eq!(
            parse_node(
                &mut vec![Token::NegativeSymbol, Token::Number(123)]
                    .into_iter()
                    .peekable()
            ),
            Ok(ASTNode::NumberLiteral(-123))
        )
    }

    #[test]
    fn test_parsing_number_literal() {
        assert_eq!(
            parse_node(&mut vec![Token::Number(123)].into_iter().peekable()),
            Ok(ASTNode::NumberLiteral(123))
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
            Ok(ASTNode::Identifier("hello-there".to_string()))
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
                    Token::ClosingParenthesis,
                ]
                .into_iter()
                .peekable()
            ),
            Ok(ASTNode::CallExpression(
                "hello-there".to_string(),
                vec![ASTNode::NumberLiteral(123)]
            ))
        )
    }
}
