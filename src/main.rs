mod parser;
mod tests;

use parser::parse_json;
use std::{env, process::exit};

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
