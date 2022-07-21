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

                if name == "defn" {
                    tokens.push(Token::DefnKeyword);
                } else {
                    tokens.push(Token::Name(name));
                }
            }
        }
    }

    return tokens;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    OpeningParenthesis,
    ClosingParenthesis,
    NegativeSymbol,
    OpeningBracket,
    ClosingBracket,

    DefnKeyword,

    Number(i64),
    Name(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_parens() {
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
    fn test_parsing_numbers_with_special_characters() {
        assert_eq!(
            tokenize("123)"),
            vec![Token::Number(123), Token::ClosingParenthesis]
        )
    }

    #[test]
    fn test_ignore_whitespace() {
        assert_eq!(tokenize("               "), vec![],)
    }
}
