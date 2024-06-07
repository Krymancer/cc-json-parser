use anyhow::{Context, Result};
use std::{env, fs, process::exit};

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

fn parse_string(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
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

fn parse_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> f64 {
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

fn parse_bool(chars: &mut std::iter::Peekable<std::str::Chars>) -> bool {
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

fn parse_null(chars: &mut std::iter::Peekable<std::str::Chars>) {
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

fn tokenize(input: String) -> Vec<Token> {
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
                tokens.push(Token::String(parse_string(&mut chars)));
            }
            '0'..='9' | '-' => {
                tokens.push(Token::Number(parse_number(&mut chars)));
            }
            't' | 'f' => {
                tokens.push(Token::Bool(parse_bool(&mut chars)));
            }
            'n' => {
                parse_null(&mut chars);
                tokens.push(Token::Null);
            }
            _ if ch.is_whitespace() => {
                chars.next();
            }
            _ => panic!("Unexpected character: {}", ch),
        }
    }

    tokens
}

fn parse(tokens: Vec<Token>) -> Result<JsonValue> {
    todo!("Implement parsing")
}

pub fn parse_json(path: String) -> Result<JsonValue> {
    let input = read_file(path)?;
    let tokens = tokenize(input);
    println!("{:?}", tokens);
    parse(tokens).context("Falied to parse JSON")
}

fn main() {
    let mut args = env::args().skip(1);

    let file_path = args.next();

    if file_path.is_none() {
        eprintln!("error: please provide an file!");
        exit(1);
    }

    let file_path = file_path.unwrap();

    let parsed = parse_json(file_path);
    println!("{:#?}", parsed);
}

#[test]
fn test_invalid_path() {
    let path = String::from("invalid/path");
    let result = parse_json(path);
    assert_eq!(result.is_err(), true)
}

#[test]
fn test_step1_valid() {
    let path = String::from("./tests/step1/valid.json");
    let result = parse_json(path);
    assert_eq!(result.is_ok(), true)
}

#[test]
fn test_step1_invalid() {
    let path = String::from("./tests/step1/invalid.json");
    let result = parse_json(path);
    assert_eq!(result.is_err(), true)
}

#[test]
fn test_step2_valid() {
    let path = String::from("./tests/step1/valid.json");
    let result = parse_json(path);
    assert_eq!(result.is_ok(), true)
}

#[test]
fn test_step2_invalid() {
    let path = String::from("./tests/step1/invalid.json");
    let result = parse_json(path);
    assert_eq!(result.is_err(), true)
}

#[test]
fn test_step2_valid1() {
    let path = String::from("./tests/step1/valid2.json");
    let result = parse_json(path);
    assert_eq!(result.is_ok(), true)
}

#[test]
fn test_step2_invalid2() {
    let path = String::from("./tests/step1/invalid2.json");
    let result = parse_json(path);
    assert_eq!(result.is_err(), true)
}
