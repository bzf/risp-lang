#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    OpeningParenthesis,
    ClosingParenthesis,
    NegativeSymbol,
    OpeningBracket,
    ClosingBracket,

    IfKeyword,
    DefnKeyword,

    String(String),
    Boolean(bool),
    Number(i64),
    Name(String),
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut cursor = input.chars().peekable();
    let mut tokens = vec![];

    while let Some(character) = cursor.next() {
        if character.is_whitespace() {
            continue;
        }

        if character.is_numeric() {
            let mut number_string = String::from(character);

            while let Some(next_character) = cursor.peek() {
                if next_character.is_numeric() && *next_character != ')' && *next_character != ']' {
                    number_string.push(cursor.next().unwrap());
                } else {
                    break;
                }
            }

            if let Ok(number) = number_string.parse::<i64>() {
                tokens.push(Token::Number(number));
            }

            continue;
        }

        match character {
            '(' => {
                tokens.push(Token::OpeningParenthesis);
            }

            ')' => {
                tokens.push(Token::ClosingParenthesis);
            }

            '"' => {
                let mut value_string = String::new();

                while let Some(next_character) = cursor.next() {
                    if next_character != '"' {
                        value_string.push(next_character);
                    } else {
                        break;
                    }
                }

                tokens.push(Token::String(value_string));
            }

            '[' => {
                tokens.push(Token::OpeningBracket);
            }

            ']' => {
                tokens.push(Token::ClosingBracket);
            }

            '-' => {
                tokens.push(Token::NegativeSymbol);
            }

            _ => {
                let mut name = String::from(character);

                while let Some(next_character) = cursor.peek() {
                    if !next_character.is_whitespace()
                        && *next_character != ')'
                        && *next_character != ']'
                    {
                        name.push(cursor.next().unwrap());
                    } else {
                        break;
                    }
                }

                match &name[..] {
                    "if" => tokens.push(Token::IfKeyword),
                    "defn" => tokens.push(Token::DefnKeyword),
                    "true" => tokens.push(Token::Boolean(true)),
                    "false" => tokens.push(Token::Boolean(false)),
                    _ => tokens.push(Token::Name(name)),
                }
            }
        }
    }

    return tokens;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_symbols() {
        assert_eq!(
            tokenize("()"),
            vec![Token::OpeningParenthesis, Token::ClosingParenthesis,]
        )
    }

    #[test]
    fn test_parsing_parameter_list() {
        assert_eq!(
            tokenize("defn [a b c]"),
            vec![
                Token::DefnKeyword,
                Token::OpeningBracket,
                Token::Name("a".to_string()),
                Token::Name("b".to_string()),
                Token::Name("c".to_string()),
                Token::ClosingBracket,
            ]
        )
    }

    #[test]
    fn test_parsing_numbers() {
        assert_eq!(
            tokenize("123 2"),
            vec![Token::Number(123), Token::Number(2),]
        )
    }

    #[test]
    fn test_parsing_names() {
        assert_eq!(
            tokenize("name my-var hello-there!"),
            vec![
                Token::Name("name".to_string()),
                Token::Name("my-var".to_string()),
                Token::Name("hello-there!".to_string()),
            ]
        )
    }

    #[test]
    fn test_parsing_names_with_special_characters() {
        assert_eq!(
            tokenize("name)"),
            vec![Token::Name("name".to_string()), Token::ClosingParenthesis]
        )
    }

    #[test]
    fn test_parsing_strings() {
        assert_eq!(tokenize(r#""""#), vec![Token::String("".to_string())]);

        assert_eq!(
            tokenize(r#""this is my string""#),
            vec![Token::String("this is my string".to_string())]
        )
    }

    #[test]
    fn test_parsing_numbers_with_special_characters() {
        assert_eq!(
            tokenize("123)"),
            vec![Token::Number(123), Token::ClosingParenthesis]
        )
    }

    #[test]
    fn test_true_false_literals() {
        assert_eq!(
            tokenize("true false"),
            vec![Token::Boolean(true), Token::Boolean(false)]
        )
    }

    #[test]
    fn test_ignore_whitespace() {
        assert_eq!(tokenize("               "), vec![],)
    }
}
