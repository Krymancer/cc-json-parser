use anyhow::{anyhow, Context, Result};
use std::fs;

#[derive(Debug)]
pub enum JsonValue {
    Object(Vec<(String, JsonValue)>),
    Array(Vec<JsonValue>),
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

#[derive(Debug)]
enum Token {
    CurlyOpen,
    CurlyClose,
    SquareOpen,
    SquareClose,
    Colon,
    Comma,
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

// TODO: Reading a file to memory is not great
// If the file is too big this cloud crash or be really slow
// Try a stream or something later!!
fn read_file(path: String) -> Result<String> {
    fs::read_to_string(path).context("Falied to read File")
}

fn tokenize_string(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut result = String::new();
    chars.next(); // Skip opening (") quote

    while let Some(&ch) = chars.peek() {
        match ch {
            '"' => {
                chars.next(); // Skip closing (") quote
                break;
            }
            _ => {
                result.push(ch);
                chars.next();
            }
        }
    }

    result
}

fn tokenize_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> f64 {
    let mut result = String::new();

    while let Some(&ch) = chars.peek() {
        match ch {
            '0'..='9' | '.' | '-' => {
                result.push(ch);
                chars.next();
            }
            _ => break,
        }
    }

    result.parse().unwrap() // Assuming valid number input
}

fn tokenize_bool(chars: &mut std::iter::Peekable<std::str::Chars>) -> bool {
    let mut result = String::new();

    while let Some(&ch) = chars.peek() {
        match ch {
            't' | 'r' | 'u' | 'e' | 'f' | 'a' | 'l' | 's' => {
                result.push(ch);
                chars.next();
            }
            _ => break,
        }
    }

    result == "true"
}

fn tokenize_null(chars: &mut std::iter::Peekable<std::str::Chars>) {
    let mut result = String::new();

    while let Some(&ch) = chars.peek() {
        match ch {
            'n' | 'u' | 'l' => {
                result.push(ch);
                chars.next();
            }
            _ => break,
        }
    }

    assert_eq!(result, "null");
}

fn tokenize(input: String) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            '{' => {
                tokens.push(Token::CurlyOpen);
                chars.next();
            }
            '}' => {
                tokens.push(Token::CurlyClose);
                chars.next();
            }
            '[' => {
                tokens.push(Token::SquareOpen);
                chars.next();
            }
            ']' => {
                tokens.push(Token::SquareClose);
                chars.next();
            }
            ':' => {
                tokens.push(Token::Colon);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            '"' => {
                tokens.push(Token::String(tokenize_string(&mut chars)));
            }
            '0'..='9' | '-' => {
                tokens.push(Token::Number(tokenize_number(&mut chars)));
            }
            't' | 'f' => {
                tokens.push(Token::Bool(tokenize_bool(&mut chars)));
            }
            'n' => {
                tokenize_null(&mut chars);
                tokens.push(Token::Null);
            }
            _ if ch.is_whitespace() => {
                chars.next();
            }
            _ => return Err(anyhow!("Unexpected character: {}", ch)),
        }
    }

    Ok(tokens)
}

fn parse_tokens(tokens: Vec<Token>) -> Result<JsonValue> {
    let mut iter = tokens.iter().peekable();
    parse_value(&mut iter)
}

fn parse_value<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<JsonValue>
where
    I: Iterator<Item = &'a Token>,
{
    match tokens.peek() {
        Some(Token::CurlyOpen) => parse_object(tokens),
        Some(Token::SquareOpen) => parse_array(tokens),
        Some(Token::String(_)) => {
            if let Some(Token::String(s)) = tokens.next() {
                Ok(JsonValue::String(s.clone()))
            } else {
                Err(anyhow!("Expected a string"))
            }
        }
        Some(Token::Number(_)) => {
            if let Some(Token::Number(n)) = tokens.next() {
                Ok(JsonValue::Number(*n))
            } else {
                Err(anyhow!("Expected a number"))
            }
        }
        Some(Token::Bool(_)) => {
            if let Some(Token::Bool(b)) = tokens.next() {
                Ok(JsonValue::Bool(*b))
            } else {
                Err(anyhow!("Expected a boolean"))
            }
        }
        Some(Token::Null) => {
            tokens.next(); // Consume the Null token
            Ok(JsonValue::Null)
        }
        _ => Err(anyhow!("Unexpected token")),
    }
}

fn parse_object<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<JsonValue>
where
    I: Iterator<Item = &'a Token>,
{
    let mut object = Vec::new();
    tokens.next(); // Consume the '{' (Open curly bracket)

    loop {
        match tokens.peek() {
            Some(Token::CurlyClose) => {
                tokens.next(); // Consume the '}' (Close curly bracket)
                break;
            }
            Some(Token::String(_)) => {
                if let Some(Token::String(key)) = tokens.next() {
                    if let Some(Token::Colon) = tokens.next() {
                        let value = parse_value(tokens)?;
                        object.push((key.clone(), value));
                        match tokens.peek() {
                            Some(Token::Comma) => {
                                tokens.next(); // Consume the ',' (Comma)
                            }
                            Some(Token::CurlyClose) => {
                                tokens.next(); // Consume the '}' (Close curly bracket)
                                break;
                            }
                            _ => return Err(anyhow!("Expected ',' or '}}'")),
                        }
                    }
                } else {
                    return Err(anyhow!("Expected ':'"));
                }
            }
            _ => return Err(anyhow!("Expected string key or '}}'")),
        }
    }

    Ok(JsonValue::Object(object))
}

fn parse_array<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<JsonValue>
where
    I: Iterator<Item = &'a Token>,
{
    let mut array = Vec::new();
    tokens.next(); // Consume the '[' (Open bracket)

    loop {
        match tokens.peek() {
            Some(Token::SquareClose) => {
                tokens.next(); // Consume the ']' (Close bracket) end of array
                break;
            }
            Some(_) => {
                let value = parse_value(tokens)?;
                array.push(value);
                match tokens.peek() {
                    Some(Token::Comma) => tokens.next(), // Consume the ',' (Comma)
                    Some(Token::SquareClose) => {
                        tokens.next(); // Consume the ']' (Clase bracket) end of array
                        break;
                    }
                    _ => return Err(anyhow!("Expected ',' or ']'")),
                };
            }
            _ => return Err(anyhow!("Expected value or ]")),
        };
    }

    Ok(JsonValue::Array(array))
}

pub fn parse_json(path: String) -> Result<JsonValue> {
    let input = read_file(path)?;
    let tokens = tokenize(input)?;
    parse_tokens(tokens)
}
