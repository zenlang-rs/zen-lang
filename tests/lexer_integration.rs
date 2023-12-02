use zen::lexer::{lexer_util::Lexer, token_type::TokenType};

#[test]
fn test_lex_tokens() {
    let input = b"myVar 123 true false";
    let expected = Ok((
        &b""[..],
        vec![
            TokenType::Identifier("myVar".to_string()),
            TokenType::Number(123.0),
            TokenType::BooleanLiteral(true),
            TokenType::BooleanLiteral(false),
            TokenType::Eof,
        ],
    ));
    assert_eq!(Lexer::lex_tokens(input), expected);

    let input = b"anotherVar123 0 false true";
    let expected = Ok((
        &b""[..],
        vec![
            TokenType::Identifier("anotherVar123".to_string()),
            TokenType::Number(0.0),
            TokenType::BooleanLiteral(false),
            TokenType::BooleanLiteral(true),
            TokenType::Eof,
        ],
    ));
    assert_eq!(Lexer::lex_tokens(input), expected);
}

#[test]
fn test_program_start_and_end() {
    let input = b"    PAPARAMPARA    PARAMPARA PRATISHTA ANUSHASHAN KHATAM TATA   BYE BYE";
    let expected_output = Ok((
        &b""[..],
        vec![
            TokenType::Identifier("PAPARAMPARA".to_string()),
            TokenType::StartProgram,
            TokenType::EndProgram,
            TokenType::Eof,
        ],
    ));
    assert_eq!(Lexer::lex_tokens(input), expected_output);
}

#[test]
fn test_variable_initialization() {
    let input = b"A BOLE TOH 10";
    let expected_output = Ok((
        &b""[..],
        vec![
            TokenType::Identifier("A".to_string()),
            TokenType::Assign,
            TokenType::Number(10.0),
            TokenType::Eof,
        ],
    ));
    assert_eq!(Lexer::lex_tokens(input), expected_output);
}

#[test]
fn test_input_from_console() {
    let input = b"C BOLE TOH INPUT LE LE RE BABA";
    let expected_output = Ok((
        &b""[..],
        vec![
            TokenType::Identifier("C".to_string()),
            TokenType::Assign,
            TokenType::Input,
            TokenType::Eof,
        ],
    ));
    assert_eq!(Lexer::lex_tokens(input), expected_output);
}

#[test]
fn test_if_statement() {
    let input = b"AGAR A > 3 TAB PRINT BASANTI PRINT 3 BAS ITNA HI";
    let expected_output = Ok((
        &b""[..],
        vec![
            TokenType::If,
            TokenType::Identifier("A".to_string()),
            TokenType::GreaterThan,
            TokenType::Number(3.0),
            TokenType::Then,
            TokenType::Print,
            TokenType::Number(3.0),
            TokenType::EndIf,
            TokenType::Eof,
        ],
    ));
    assert_eq!(Lexer::lex_tokens(input), expected_output);
}

#[test]
fn test_else_if_statement() {
    let input = b"WARNA AGAR B > 3 TAB PRINT BASANTI PRINT 4.8";
    let expected_output = Ok((
        &b""[..],
        vec![
            TokenType::ElseIf,
            TokenType::Identifier("B".to_string()),
            TokenType::GreaterThan,
            TokenType::Number(3.0),
            TokenType::Then,
            TokenType::Print,
            TokenType::Number(4.8),
            TokenType::Eof,
        ],
    ));
    assert_eq!(Lexer::lex_tokens(input), expected_output);
}

#[test]
fn test_else_statement() {
    let input = b"NHI TOH PRINT BASANTI PRINT \"2\"";
    let expected_output = Ok((
        &b""[..],
        vec![
            TokenType::Else,
            TokenType::Print,
            TokenType::StringLiteral("2".to_owned()),
            TokenType::Eof,
        ],
    ));
    assert_eq!(Lexer::lex_tokens(input), expected_output);
}

#[test]
fn test_while_statement() {
    let input = b"JAB TAK HAI JAAN _12v > 3 TAB TAK PRINT BASANTI PRINT \"foobar\" JAHAN";
    let expected_output = Ok((
        &b""[..],
        vec![
            TokenType::While,
            TokenType::Identifier("_12v".to_string()),
            TokenType::GreaterThan,
            TokenType::Number(3.0),
            TokenType::Do,
            TokenType::Print,
            TokenType::StringLiteral("foobar".to_owned()),
            TokenType::EndWhile,
            TokenType::Eof,
        ],
    ));
    assert_eq!(Lexer::lex_tokens(input), expected_output);
}

#[test]
fn test_comment() {
    let input = b"@ This is a comment\n";
    let expected_output = Ok((&b""[..], vec![TokenType::Eof]));
    assert_eq!(Lexer::lex_tokens(input), expected_output);
}

#[test]
fn test_arithmetic_operations() {
    let input = b"Ab3 BOLE TOH 10 + 5 - 3 * 2 / 1 % 2";
    let expected_output = Ok((
        &b""[..],
        vec![
            TokenType::Identifier("Ab3".to_string()),
            TokenType::Assign,
            TokenType::Number(10.0),
            TokenType::Plus,
            TokenType::Number(5.0),
            TokenType::Minus,
            TokenType::Number(3.0),
            TokenType::Multiply,
            TokenType::Number(2.0),
            TokenType::Divide,
            TokenType::Number(1.0),
            TokenType::Modulo,
            TokenType::Number(2.0),
            TokenType::Eof,
        ],
    ));
    assert_eq!(Lexer::lex_tokens(input), expected_output);
}

#[test]
fn test_logical_operations() {
    let input = b"_A BOLE TOH true && false || true";
    let expected_output = Ok((
        &b""[..],
        vec![
            TokenType::Identifier("_A".to_string()),
            TokenType::Assign,
            TokenType::BooleanLiteral(true),
            TokenType::LogicalAnd,
            TokenType::BooleanLiteral(false),
            TokenType::LogicalOr,
            TokenType::BooleanLiteral(true),
            TokenType::Eof,
        ],
    ));
    assert_eq!(Lexer::lex_tokens(input), expected_output);
}

#[test]
fn test_punctuators() {
    let input = b"( )";
    let expected_output = Ok((
        &b""[..],
        vec![TokenType::LeftParen, TokenType::RightParen, TokenType::Eof],
    ));
    assert_eq!(Lexer::lex_tokens(input), expected_output);
}
#[test]
fn test_combined_operations() {
    let input = b"A_ BOLE TOH (10 + 5 - 3) * 2 / (1 % 2) && true || false";
    let expected_output = Ok((
        &b""[..],
        vec![
            TokenType::Identifier("A_".to_string()),
            TokenType::Assign,
            TokenType::LeftParen,
            TokenType::Number(10.0),
            TokenType::Plus,
            TokenType::Number(5.0),
            TokenType::Minus,
            TokenType::Number(3.0),
            TokenType::RightParen,
            TokenType::Multiply,
            TokenType::Number(2.0),
            TokenType::Divide,
            TokenType::LeftParen,
            TokenType::Number(1.0),
            TokenType::Modulo,
            TokenType::Number(2.0),
            TokenType::RightParen,
            TokenType::LogicalAnd,
            TokenType::BooleanLiteral(true),
            TokenType::LogicalOr,
            TokenType::BooleanLiteral(false),
            TokenType::Eof,
        ],
    ));
    assert_eq!(Lexer::lex_tokens(input), expected_output);
}

#[test]
fn is_identifier_token() {
    let input = "_AB".as_bytes();
    let expected_output = Ok((
        &b""[..],
        vec![TokenType::Identifier("_AB".to_string()), TokenType::Eof],
    ));
    assert_eq!(Lexer::lex_tokens(input), expected_output);
}

#[test]
fn test_lex_newline() {
    let input = b"A BOLE TOH 10\nB BOLE TOH 20\n";
    let expected_output = Ok((
        &b""[..],
        vec![
            TokenType::Identifier("A".to_string()),
            TokenType::Assign,
            TokenType::Number(10.0),
            TokenType::EndOfStatement,
            TokenType::Identifier("B".to_string()),
            TokenType::Assign,
            TokenType::Number(20.0),
            TokenType::EndOfStatement,
            TokenType::Eof,
        ],
    ));

    assert_eq!(Lexer::lex_tokens(input), expected_output);
}
