use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Integer(i64),
    Float(f64),
    String(String),
    Symbol(String),
    LParen,
    RParen,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Integer(n) => write!(f, "{n}"),
            Token::Float(n) => write!(f, "{}", n),
            Token::String(s) => write!(f, "{}", s),
            Token::Symbol(s) => write!(f, "{s}"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
        }
    }
}

#[derive(Debug)]
pub struct TokenError {
    err: String,
}

impl Error for TokenError {}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tokenization error: {}", self.err)
    }
}

// 功能：将用户的输入解析成Vec<Token>
pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars: Vec<char> = input.chars().collect();

    if chars.is_empty() {
        return Ok(tokens);
    }

    while !chars.is_empty() {
        let mut ch = chars.remove(0);
        match ch {
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '"' => {
                let mut word = String::new();
                while !chars.is_empty() && chars[0] != '"' {
                    word.push(chars.remove(0));
                }

                if !chars.is_empty() && chars[0] == '"' {
                    chars.remove(0);
                } else {
                    return Err(TokenError {
                        err: format!("Unterminated string: {}", word),
                    });
                }

                tokens.push(Token::String(word));
            }
            _ => {
                // Symbol Float Int
                let mut word = String::new();
                while !chars.is_empty() && !ch.is_whitespace() && ch != '(' && ch != ')' {
                    word.push(ch);

                    let peek = chars[0];
                    if peek == '(' || peek == ')' {
                        break;
                    }

                    ch = chars.remove(0);
                }
                // 过滤空字符
                if word.is_empty() {
                    continue;
                }

                if let Ok(i) = word.parse::<i64>() {
                    tokens.push(Token::Integer(i));
                    continue;
                }

                if let Ok(f) = word.parse::<f64>() {
                    tokens.push(Token::Float(f));
                    continue;
                }

                tokens.push(Token::Symbol(word.to_string()));
            }
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let tokens = tokenize("(+ 1 2)").unwrap_or_default();
    
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Symbol("+".to_string()),
                Token::Integer(1),
                Token::Integer(2),
                Token::RParen,
            ]
        );
    }

    #[test]
    fn test_area_of_a_circle() {
        let program = "
            (
                (define r 10)
                (define pi 314)
                (* pi (* r r))
            )
        ";
        let tokens = tokenize(program).unwrap_or_default();
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::LParen,
                Token::Symbol("define".to_string()),
                Token::Symbol("r".to_string()),
                Token::Integer(10),
                Token::RParen,
                Token::LParen,
                Token::Symbol("define".to_string()),
                Token::Symbol("pi".to_string()),
                Token::Integer(314),
                Token::RParen,
                Token::LParen,
                Token::Symbol("*".to_string()),
                Token::Symbol("pi".to_string()),
                Token::LParen,
                Token::Symbol("*".to_string()),
                Token::Symbol("r".to_string()),
                Token::Symbol("r".to_string()),
                Token::RParen,
                Token::RParen,
                Token::RParen
            ]
        );
    }

    #[test]
    fn test_string() {
        let input = r#"(+ "abc" "123")"#;
        let tokens = tokenize(input).unwrap_or_default();

        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Symbol("+".into()),
                Token::String("abc".into()),
                Token::String("123".into()),
                Token::RParen,
            ]
        )
    }
}