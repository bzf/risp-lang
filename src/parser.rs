use std::{iter::Peekable, vec::IntoIter};

use crate::{Error, ErrorType, Token};

#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    NumberLiteral(i64),
    BooleanLiteral(bool),
    Identifier(String),
    StringLiteral(String),

    CallExpression(String, Vec<ASTNode>),

    IfExpression {
        expression: Box<ASTNode>,
        when_true: Box<ASTNode>,
        when_false: Box<ASTNode>,
    },

    FunctionDeclaration {
        identifier: String,
        parameter_list: Vec<String>,
        body: Box<ASTNode>,
    },
}

pub fn parse_node(tokens: &mut Peekable<std::vec::IntoIter<Token>>) -> Result<ASTNode, Error> {
    if let Some(token) = tokens.next() {
        match token {
            Token::Number(number) => Ok(ASTNode::NumberLiteral(number)),
            Token::Boolean(value) => Ok(ASTNode::BooleanLiteral(value)),
            Token::String(value) => Ok(ASTNode::StringLiteral(value)),

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
                        Token::IfKeyword => parse_if_expression(tokens),

                        Token::DefnKeyword => parse_function_declaration(tokens),

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

fn parse_if_expression(tokens: &mut Peekable<IntoIter<Token>>) -> Result<ASTNode, Error> {
    let token = tokens
        .next()
        .ok_or(Error::new("Missing tokens", ErrorType::MissingToken))?;

    assert_eq!(Token::IfKeyword, token);

    return Ok(ASTNode::IfExpression {
        expression: Box::new(parse_node(tokens)?),
        when_true: Box::new(parse_node(tokens)?),
        when_false: Box::new(parse_node(tokens)?),
    });
}

fn parse_function_declaration(tokens: &mut Peekable<IntoIter<Token>>) -> Result<ASTNode, Error> {
    let token = tokens
        .next()
        .ok_or(Error::new("Missing tokens", ErrorType::MissingToken))?;

    assert_eq!(Token::DefnKeyword, token);

    let identifier = {
        if let Ok(ASTNode::Identifier(name)) = parse_node(tokens) {
            name
        } else {
            return Err(Error::new(
                "Expected an identifier following defn keyword",
                ErrorType::UnexpectedToken(token),
            ));
        }
    };

    let mut parameter_list: Vec<String> = vec![];

    {
        let token = tokens
            .next()
            .ok_or(Error::new("Missing tokens", ErrorType::MissingToken))?;
        assert_eq!(Token::OpeningBracket, token);
    }

    while let Some(next_token) = tokens.next() {
        match next_token {
            Token::Name(name) => {
                parameter_list.push(name);
            }

            Token::ClosingBracket => break,

            _ => {
                return Err(Error::new(
                    "Unexpected token",
                    ErrorType::UnexpectedToken(next_token),
                ));
            }
        }
    }

    let body = parse_node(tokens)?;

    return Ok(ASTNode::FunctionDeclaration {
        identifier,
        parameter_list,
        body: Box::new(body),
    });
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

    #[test]
    fn test_parsing_true_literals() {
        assert_eq!(
            parse_node(&mut vec![Token::Boolean(true)].into_iter().peekable()),
            Ok(ASTNode::BooleanLiteral(true))
        )
    }

    #[test]
    fn test_parsing_false_literals() {
        assert_eq!(
            parse_node(&mut vec![Token::Boolean(false)].into_iter().peekable()),
            Ok(ASTNode::BooleanLiteral(false))
        )
    }

    #[test]
    fn test_parsing_function_declaration() {
        assert_eq!(
            parse_node(
                &mut vec![
                    Token::OpeningParenthesis,
                    Token::DefnKeyword,
                    Token::Name("hello-there".to_string()),
                    Token::OpeningBracket,
                    Token::Name("a".to_string()),
                    Token::ClosingBracket,
                    Token::Number(123),
                    Token::ClosingParenthesis,
                ]
                .into_iter()
                .peekable()
            ),
            Ok(ASTNode::FunctionDeclaration {
                identifier: "hello-there".to_string(),
                parameter_list: vec!["a".to_string()],
                body: Box::new(ASTNode::NumberLiteral(123)),
            })
        );
    }

    #[test]
    fn test_parsing_if_else_expression() {
        assert_eq!(
            parse_node(
                &mut vec![
                    Token::OpeningParenthesis,
                    Token::IfKeyword,
                    Token::Boolean(true),
                    Token::Number(321),
                    Token::Number(123),
                    Token::ClosingParenthesis,
                ]
                .into_iter()
                .peekable()
            ),
            Ok(ASTNode::IfExpression {
                expression: Box::new(ASTNode::BooleanLiteral(true)),
                when_true: Box::new(ASTNode::NumberLiteral(321)),
                when_false: Box::new(ASTNode::NumberLiteral(123)),
            })
        );
    }
}
