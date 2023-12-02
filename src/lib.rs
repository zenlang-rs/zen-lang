use crate::evaluator::interpreter::{Interpreter, InterpreterError};
use crate::evaluator::interpreter::InterpreterErrorType::UnknownParserError;
use crate::lexer::lexer_util::Lexer;
use crate::lexer::tokens::Tokens;
use crate::parser::ast::Statement;
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

pub fn run_program(code: String, input: &str, is_on_console: bool) -> Result<String, InterpreterError> {
    let (_, r) = Lexer::lex_tokens(code.as_bytes()).unwrap();
    let tokens = Tokens::new(&r);
    let (_, result) = Parser::parse_tokens(tokens).unwrap();

    if !r.is_empty() && (result.statements.is_empty() || !result.statements.contains(&Statement::ProgramEnd)) {
        return Err(InterpreterError::new("Some error in parsing the language!", UnknownParserError));
    }

    Interpreter::new(input, is_on_console).run_code(result)
}
