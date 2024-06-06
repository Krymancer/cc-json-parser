use std::{env, fs, process::exit, str::FromStr};
use anyhow::{Result, Context };

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

fn tokenize(input: String) -> Vec<Token> {
    todo!("Implement tokenization")
}

fn parse(tokens: Vec<Token>) -> Result<JsonValue> {
    todo!("Implement parsing")
}

pub fn parse_json(path: String) -> Result<JsonValue> {
    let input = read_file(path)?;
    let tokens = tokenize(input);
    parse(tokens).context("Falied to parse JSON")
}

fn main() {
    let mut args = env::args().skip(1);

    let file_path = args.next();

    if let None = file_path {
        eprintln!("error: please provide an file!");
        exit(1);
    }

    let file_path = file_path.unwrap();

    let parsed = parse_json(file_path);
    println!("{:#?}", parsed);
}

#[test]
fn test_invalid_path() {
    let path = String::from_str("invalid/path").unwrap();
    let result = parse_json(path);
    assert_eq!(result.is_err(), true)
}

#[test]
fn test_step1_valid() {
    let path = String::from_str("./tests/step1/valid.json").unwrap();
    let result = parse_json(path);
    assert_eq!(result.is_ok(), true)
}

#[test]
fn test_step1_invalid() {
    let path = String::from_str("./tests/step1/invalid.json").unwrap();
    let result = parse_json(path);
    assert_eq!(result.is_err(), true)
}

#[test]
fn test_step2_valid() {
    let path = String::from_str("./tests/step1/valid.json").unwrap();
    let result = parse_json(path);
    assert_eq!(result.is_ok(), true)
}

#[test]
fn test_step2_invalid() {
    let path = String::from_str("./tests/step1/invalid.json").unwrap();
    let result = parse_json(path);
    assert_eq!(result.is_err(), true)
}

#[test]
fn test_step2_valid1() {
    let path = String::from_str("./tests/step1/valid2.json").unwrap();
    let result = parse_json(path);
    assert_eq!(result.is_ok(), true)
}

#[test]
fn test_step2_invalid2() {
    let path = String::from_str("./tests/step1/invalid2.json").unwrap();
    let result = parse_json(path);
    assert_eq!(result.is_err(), true)
}
