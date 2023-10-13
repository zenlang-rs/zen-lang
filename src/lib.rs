use std::cell::RefCell;

mod utils;
use crate::utils::input;

#[derive(Debug, PartialEq)]
pub enum Token {
    Eof,
    // Commands
    Def,
    Extern,

    // Primary
    Identifier,
    Number,

    Invalid(char),
}

const EOF: u8 = 0;

thread_local!(static IDENTIFIER_STRING: RefCell<String> = RefCell::new(String::new())); // Stores the names of the current identifier
thread_local!(static NUM_VALUE: RefCell<f64> = RefCell::new(0.0)); // Holds current state of parsing token
thread_local!(static LAST_CHAR: RefCell<char> = RefCell::new(' '));

pub fn get_tok() -> Token {
    let mut detect_token = Token::Invalid(' ');
    LAST_CHAR.with(|l_char| {
        let mut last_char = l_char.borrow_mut();
        while last_char.is_whitespace() {
            *last_char = input::get_char();
        }
        detect_token = IDENTIFIER_STRING.with(|i_string| {
            // identifier : [a-zA-Z][a-zA-Z0-9]*
            if last_char.is_alphabetic() {
                let mut identifier_str = i_string.borrow_mut();
                *identifier_str = last_char.to_string();

                loop {
                    *last_char = input::get_char();
                    if !last_char.is_alphabetic() {
                        break;
                    }
                    identifier_str.push(*last_char);
                }

                if *identifier_str == "def" {
                    return Token::Def;
                }
                if *identifier_str == "extern" {
                    return Token::Extern;
                }
                return Token::Identifier;
            }
            Token::Invalid(' ')
        });

        if detect_token != Token::Invalid(' ') {
            return detect_token;
        }

        // Number [0-9.]+
        if last_char.is_numeric() || *last_char == '.' {
            let mut num_str = String::new();
            loop {
                num_str.push(*last_char);
                *last_char = input::get_char();

                if !(last_char.is_numeric() || *last_char == '.') {
                    break;
                }
            }
            NUM_VALUE.with(|n_value| {
                let mut num_value = n_value.borrow_mut();
                *num_value = num_str.parse::<f64>().expect("Error parsing number!");
            });
            return Token::Number;
        }
        // Comment
        if *last_char == '#' {
            loop {
                *last_char = input::get_char();

                if *last_char == '\n' || *last_char == '\r' || *last_char == EOF as char {
                    break;
                }

                if *last_char != EOF as char {
                    return get_tok();
                }
            }
        }

        if *last_char == EOF as char {
            return Token::Eof;
        }

        let this_char = Token::Invalid(*last_char);
        *last_char = input::get_char();
        this_char
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_test() {
        
    }
}