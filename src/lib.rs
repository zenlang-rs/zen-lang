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

    Uninitialized,
}

const EOF: u8 = 0;

thread_local!(static IDENTIFIER_STRING: RefCell<String> = RefCell::new(String::new())); // Stores the names of the current identifier
thread_local!(static NUM_VALUE: RefCell<f64> = RefCell::new(0.0)); // Holds current state of parsing token
thread_local!(static LAST_CHAR: RefCell<char> = RefCell::new(' '));

pub fn get_tok() -> Token {
    let mut last_char_dup = ' ';
    let mut detect_token;
    LAST_CHAR.with(|l_char| {
        let mut last_char = l_char.borrow_mut();
        while last_char.is_whitespace() {
            *last_char = input::get_char();
        }
        last_char_dup = *last_char;
    });

    detect_token = IDENTIFIER_STRING.with(|i_string| {
        // identifier : [a-zA-Z][a-zA-Z0-9]*
        if last_char_dup.is_alphabetic() {
            let mut identifier_str = i_string.borrow_mut();
            *identifier_str = last_char_dup.to_string();

            loop {
                last_char_dup = input::get_char();
                if !last_char_dup.is_alphabetic() {
                    break;
                }
                identifier_str.push(last_char_dup);
            }

            if *identifier_str == "def" {
                return Token::Def;
            }
            if *identifier_str == "extern" {
                return Token::Extern;
            }
            return Token::Identifier;
        }
        Token::Uninitialized
    });

    // Number [0-9.]+
    if last_char_dup.is_numeric() || last_char_dup == '.' {
        let mut num_str = String::new();
        loop {
            num_str.push(last_char_dup);
            last_char_dup = input::get_char();

            if !last_char_dup.is_numeric() || last_char_dup == '.' {
                break;
            }
        }
        NUM_VALUE.with(|n_value| {
            let mut num_value = n_value.borrow_mut();
            *num_value = num_str.parse::<f64>().expect("Error parsing number!");
        });
        detect_token = Token::Number;
    }
    // Comment
    if last_char_dup == '#' {
        loop {
            last_char_dup = input::get_char();

            if last_char_dup == '\n' || last_char_dup == '\r' || last_char_dup == EOF as char {
                break;
            }
        }
        
        if last_char_dup != EOF as char {
            LAST_CHAR.with(|l_char| {
                let mut last_char = l_char.borrow_mut();
                *last_char = last_char_dup;
            });
            detect_token = get_tok();
        }
    }

    if last_char_dup == EOF as char {
        detect_token = Token::Eof;
    }
    if detect_token == Token::Uninitialized {
        let this_char = Token::Invalid(last_char_dup);
        last_char_dup = input::get_char();
        detect_token = this_char;    
    }

    LAST_CHAR.with(|l_char| {
        let mut last_char = l_char.borrow_mut();
        *last_char = last_char_dup;
        detect_token
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_test() {}
}
