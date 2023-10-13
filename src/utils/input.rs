use std::io::Read;

pub fn get_char() -> char {
    std::io::stdin()
        .bytes() 
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as char)
        .expect("Failed to read from stdout!")
}
