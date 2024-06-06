use std::{env, fs, io, process::exit};

#[derive(Debug)]
enum JsonValue {
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
fn read_file(path: String) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

fn tokenize(input: String) -> Vec<Token> {
    todo!("Implement tokenization")
}

fn parse(tokens: &[Token]) -> Result<JsonValue, String> {
    todo!("Implement parsing")
}

fn main() {
    let mut args = env::args().skip(1);

    let file_path = args.next();

    if let None = file_path {
        eprintln!("error: please provide an file!");
        exit(1);
    }

    let file_path = file_path.unwrap();

    match read_file(file_path) {
        Ok(content) => {
            let tokens = tokenize(content);
            match parse(&tokens) {
                Ok(json_value) => println!("{:#?}", json_value),
                Err(e) => eprintln!("Fail to parse JSON: {}", e),
            }
        },
        Err(e) => eprintln!("Failed to read file: {}", e),
    }        
}
