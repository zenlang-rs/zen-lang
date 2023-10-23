/// Just returns the same code as Ok!
/// TODO implement later
pub fn compile(mut code: String) -> Result<String, String> {
    code.push_str("\nBhag yha se!!\nOk?");
    Ok(code)
}

pub fn get_version() -> Result<String, &'static str> {
    Ok(String::from("testv4"))
}
