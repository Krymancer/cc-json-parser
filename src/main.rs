use std::{env, fs, process::exit};
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
