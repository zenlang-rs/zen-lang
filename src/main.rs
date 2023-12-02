use std::process;
use std::{env, fs, io, io::Write};
use zen::run_program;
use colored::Colorize;

pub mod lexer;
pub mod parser;
use crate::lexer::lexer_util::Lexer;
use crate::lexer::tokens::Tokens;
use crate::parser::parser_util::Parser;

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

        let runnable = run_program(contents, "", true);
        match runnable {
            Ok(output) => {
                println!("{}", output);
            }
            Err(e) => {
                println!("{}\nMessage: {}\nError Type: {}", "Runtime Error occurred!".red(), e.msg.blue(), e.error_type.to_string().yellow());
            }
        }

    } else {
        println!("Welcome To Zen world!");
        println!("Generate AST from code right here!");
        print!("> ");
        std::io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut contents)
            .expect("Failed to read line");

        let (_, r) = Lexer::lex_tokens(contents.as_bytes()).unwrap();
        let tokens = Tokens::new(&r);
        let (_, result) = Parser::parse_tokens(tokens).unwrap();

        println!("Here is your AST:\n {:#?}", result);
    }

}
