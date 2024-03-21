use std::{env, fs::File, io::{BufRead, BufReader}, process::exit};

fn main() {
    let mut args = env::args().skip(1);

    let file_path = args.next();

    if let None = file_path {
        eprintln!("error: please provide an file!");
        exit(1);
    }

    let file_path = file_path.unwrap();

    let file = File::open(&file_path);

    if let Err(err) = file {
        eprintln!("error: opening file {}: {}", file_path, err);
        exit(1);
    }

    let file = file.unwrap();

    if file.metadata().unwrap().len() == 0 {
        eprintln!("Empty file!");
        exit(1);
    }

    let reader = BufReader::new(file);

    let mut braces_count = 0;
    let mut sq_braces_count = 0;

    for line in reader.lines() {
        let line = line.expect("Error reading line");
        let line = line.trim();
        
        braces_count += line.chars().filter(|&c| c == '{').count();
        braces_count -= line.chars().filter(|&c| c == '}').count();
        sq_braces_count += line.chars().filter(|&c| c == '[').count();
        sq_braces_count -= line.chars().filter(|&c| c == ']').count();

        let mut pair = line.split(':');

        let key = pair.next();

        if let Some(key) = key {
            let cases = vec!["{", "}", "[", "]"];
            if !cases.contains(&key) {
                if !key.starts_with('"') {
                    eprintln!("expecting STRING found: {key}");
                    exit(1);
                }

                println!("this is a valid key: {key}");
            }
        }    
    }    

    if braces_count != 0 || sq_braces_count != 0 {
        eprintln!("Json incomplete or malformed!");
        exit(1); 
    }
        
}
