pub mod lexer;
/// Just returns the same code as Ok!
/// TODO implement later
pub fn compile(mut code: String) -> Result<String, String> {
    code.push_str("\nBhag yha se!!");
    Ok(code)
}
