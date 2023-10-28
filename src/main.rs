use std::process;
use std::{env, fs, io, io::Write};

pub mod lexer;
use crate::lexer::lexer_util::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut contents = String::new();

    if args.len() > 1 {
        let filename = &args[1];
        contents = match fs::read_to_string(filename) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading the file {}\nError {}", filename, e);
                process::exit(1);
            }
        };
    } else {
        print!("> ");
        std::io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut contents)
            .expect("Failed to read line");
    }

    println!("{:?}", Lexer::lex_tokens(contents.as_bytes()));
}
