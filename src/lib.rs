use crate::evaluator::interpreter::{Interpreter, InterpreterError};
use crate::lexer::lexer_util::Lexer;
use crate::lexer::tokens::Tokens;
use crate::parser::parser_util::Parser;

pub mod evaluator;
pub mod lexer;
pub mod parser;

/// Just returns the same code as Ok!
/// TODO implement later
pub fn compile(mut code: String) -> Result<String, String> {
    code.push_str("\nBhag yha se!!");
    Ok(code)
}

pub fn run_program(code: String, input: &str) -> Result<String, InterpreterError> {
    let (_, r) = Lexer::lex_tokens(code.as_bytes()).unwrap();
    let tokens = Tokens::new(&r);
    let (_, result) = Parser::parse_tokens(tokens).unwrap();


    Interpreter::new(input).run_code(result)
}