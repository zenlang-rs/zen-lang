mod token;

use std::str::Utf8Error;
use std::*;

use nom::branch::*;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{alpha1, alphanumeric1, char, digit1, multispace0, multispace1};
use nom::combinator::map;
use nom::combinator::map_res;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::*;

use crate::lexer::token::*;

macro_rules! syntax {
    ($func_name: ident, $tag_string: literal, $output_token: expr) => {
        fn $func_name<'a>(s: &'a [u8]) -> IResult<&[u8], Token> {
            map(tag($tag_string), |_| $output_token)(s)
        }
    };
}

// Operators
syntax! {plus_operator, "+", Token::Plus}
syntax! {minus_or_negation_operator, "-", Token::MinusOrNegation}
syntax! {multiply_operator, "*", Token::Multiply}
syntax! {divide_operator, "/", Token::Divide}
syntax! {modulo_operator, "%", Token::Modulo}
syntax! {greater_operator, ">", Token::GreaterThan}
syntax! {greater_equal_operator, ">=", Token::GreaterThanEqual}
syntax! {less_operator, "<", Token::LessThan}
syntax! {less_equal_operator, "<=", Token::LessThanEqual}
syntax! {equal_operator, "==", Token::Equal}
syntax! {not_equal_operator, "!=", Token::NotEqual}
syntax! {not_operator, "!", Token::Not}
syntax! {logical_and_operator, "&&", Token::LogicalAnd}
syntax! {logical_or_operator, "||", Token::LogicalOr}

pub fn lex_operator(input: &[u8]) -> IResult<&[u8], Token> {
    alt((
        logical_and_operator,
        logical_or_operator,
        assign_operator,
        plus_operator,
        minus_or_negation_operator,
        multiply_operator,
        divide_operator,
        modulo_operator,
        greater_operator,
        greater_equal_operator,
        less_operator,
        less_equal_operator,
        equal_operator,
        not_operator,
        not_equal_operator,
    ))(input)
}
// Keywords
fn start_program_keyword(input: &[u8]) -> IResult<&[u8], Token> {
    let (input, _) = multispace0(input)?; // Skip leading spaces
    let (input, _) = tag("PARAMPARA")(input)?;
    let (input, _) = multispace1(input)?; // Require at least one space
    let (input, _) = tag("PRATISHTA")(input)?;
    let (input, _) = multispace1(input)?; // Require at least one space
    let (input, _) = tag("ANUSHASHAN")(input)?;
    let (input, _) = multispace0(input)?; // Skip trailing spaces
    Ok((input, Token::StartProgram))
}

fn end_program_keyword(input: &[u8]) -> IResult<&[u8], Token> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("KHATAM")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("TATA")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("BYE")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("BYE")(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, Token::EndProgram))
}
syntax! {if_keyword, "AGAR", Token::If}
syntax! {then_keyword, "TAB", Token::Then}

fn else_if_keyword(input: &[u8]) -> IResult<&[u8], Token> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("WARNA")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("AGAR")(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, Token::ElseIf))
}

fn else_keyword(input: &[u8]) -> IResult<&[u8], Token> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("NHI")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("TOH")(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, Token::Else))
}

fn end_if_keyword(input: &[u8]) -> IResult<&[u8], Token> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("BAS")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("ITNA")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("HI")(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, Token::EndIf))
}

fn while_keyword(input: &[u8]) -> IResult<&[u8], Token> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("JAB")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("TAK")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("HAI")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("JAAN")(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, Token::While))
}

fn do_keyword(input: &[u8]) -> IResult<&[u8], Token> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("TAB")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("TAK")(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, Token::Do))
}

syntax! {end_while_keyword, "JAHAN", Token::EndWhile}

fn print_keyword(input: &[u8]) -> IResult<&[u8], Token> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("PRINT")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("BASANTI")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("PRINT")(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, Token::Print))
}

fn input_keyword(input: &[u8]) -> IResult<&[u8], Token> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("INPUT")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("LE")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("LE")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("RE")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("BABA")(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, Token::Input))
}

fn assign_operator(input: &[u8]) -> IResult<&[u8], Token> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("BOLE")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("TOH")(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, Token::Assign))
}
pub fn lex_keyword(input: &[u8]) -> IResult<&[u8], Token> {
    alt((
        start_program_keyword,
        end_program_keyword,
        if_keyword,
        do_keyword,
        then_keyword,
        else_if_keyword,
        else_keyword,
        end_if_keyword,
        while_keyword,
        end_while_keyword,
        print_keyword,
        input_keyword,
    ))(input)
}

syntax! {left_paren_punctuation, "(", Token::LeftParen}
syntax! {right_paren_punctuation, ")", Token::RightParen}

pub fn lex_punctuations(input: &[u8]) -> IResult<&[u8], Token> {
    alt((left_paren_punctuation, right_paren_punctuation))(input)
}
// Strings
fn pis(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    use std::result::Result::*;

    let (i1, c1) = take(1usize)(input)?;
    match c1.as_bytes() {
        b"\"" => Ok((input, vec![])),
        b"\\" => {
            let (i2, c2) = take(1usize)(i1)?;
            pis(i2).map(|(slice, done)| (slice, concat_slice_vec(c2, done)))
        }
        c => pis(i1).map(|(slice, done)| (slice, concat_slice_vec(c, done))),
    }
}

fn concat_slice_vec(c: &[u8], done: Vec<u8>) -> Vec<u8> {
    let mut new_vec = c.to_vec();
    new_vec.extend(&done);
    new_vec
}

fn convert_vec_utf8(v: Vec<u8>) -> Result<String, Utf8Error> {
    let slice = v.as_slice();
    str::from_utf8(slice).map(|s| s.to_owned())
}

fn string(input: &[u8]) -> IResult<&[u8], String> {
    delimited(tag("\""), map_res(pis, convert_vec_utf8), tag("\""))(input)
}

fn lex_string(input: &[u8]) -> IResult<&[u8], Token> {
    map(string, Token::StringLiteral)(input)
}

fn lex_ident(input: &[u8]) -> IResult<&[u8], Token> {
    map(
        |i| {
            recognize!(
                i,
                pair!(
                    alt((alpha1, tag("_"))),
                    many0_count!(alt((alphanumeric1, tag("_"))))
                )
            )
        },
        |ident: &[u8]| Token::Ident(std::str::from_utf8(ident).unwrap().to_string()),
    )(input)
}
// Number parsing(float,int)
fn lex_number(input: &[u8]) -> IResult<&[u8], Token> {
    map(
        |i| recognize!(i, pair!(digit1, opt!(pair!(char('.'), digit1)))),
        |digit_str: &[u8]| {
            let float_str = std::str::from_utf8(digit_str).unwrap();
            let float_val = float_str.parse::<f64>().unwrap();
            Token::Number(float_val)
        },
    )(input)
}
// Illegal tokens
fn lex_illegal(input: &[u8]) -> IResult<&[u8], Token> {
    map(take(1usize), |_| Token::Illegal)(input)
}

fn lex_bool_literal(input: &[u8]) -> IResult<&[u8], Token> {
    let parse_true = map(nom::bytes::complete::tag("true"), |_| {
        Token::BoolLiteral(true)
    });
    let parse_false = map(nom::bytes::complete::tag("false"), |_| {
        Token::BoolLiteral(false)
    });
    nom::branch::alt((parse_true, parse_false))(input)
}

fn lex_comment(input: &[u8]) -> IResult<&[u8], Token> {
    map(
        delimited(
            nom::bytes::complete::tag(b"@"),
            nom::bytes::complete::take_until("\n"),
            nom::bytes::complete::tag(b"\n"),
        ),
        |s: &[u8]| Token::Comment(std::str::from_utf8(s).unwrap().to_string()),
    )(input)
}

fn lex_token(input: &[u8]) -> IResult<&[u8], Token> {
    alt((
        lex_keyword,
        lex_operator,
        lex_punctuations,
        lex_number,
        lex_bool_literal,
        lex_ident,
        lex_string,
        lex_comment,
        lex_illegal,
    ))(input)
}

fn lex_tokens(input: &[u8]) -> IResult<&[u8], Vec<Token>> {
    many0(delimited(multispace0, lex_token, multispace0))(input)
}

pub struct Lexer;

impl Lexer {
    pub fn lex_tokens(bytes: &[u8]) -> IResult<&[u8], Vec<Token>> {
        lex_tokens(bytes)
            .map(|(slice, result)| (slice, [&result[..], &vec![Token::EOF][..]].concat()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_tokens() {
        let input = b"myVar 123 true false";
        let expected = Ok((
            &b""[..],
            vec![
                // Token::Comment("This is a comment".to_string()),
                Token::Ident("myVar".to_string()),
                Token::Number(123.0),
                Token::BoolLiteral(true),
                Token::BoolLiteral(false),
                Token::EOF,
            ],
        ));
        assert_eq!(Lexer::lex_tokens(input), expected);

        let input = b"anotherVar123 0 false true";
        let expected = Ok((
            &b""[..],
            vec![
                // Token::Comment("Another comment".to_string()),
                Token::Ident("anotherVar123".to_string()),
                Token::Number(0.0),
                Token::BoolLiteral(false),
                Token::BoolLiteral(true),
                Token::EOF,
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
                Token::Ident("PAPARAMPARA".to_string()),
                Token::StartProgram,
                Token::EndProgram,
                Token::EOF,
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
                Token::Ident("A".to_string()),
                Token::Assign,
                Token::Number(10.0),
                Token::EOF,
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
                Token::Ident("C".to_string()),
                Token::Assign,
                Token::Input,
                Token::EOF,
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
                Token::If,
                Token::Ident("A".to_string()),
                Token::GreaterThan,
                Token::Number(3.0),
                Token::Then,
                Token::Print,
                Token::Number(3.0),
                Token::EndIf,
                Token::EOF,
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
                Token::ElseIf,
                Token::Ident("B".to_string()),
                Token::GreaterThan,
                Token::Number(3.0),
                Token::Then,
                Token::Print,
                Token::Number(4.8),
                Token::EOF,
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
                Token::Else,
                Token::Print,
                Token::StringLiteral("2".to_owned()),
                Token::EOF,
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
                Token::While,
                Token::Ident("_12v".to_string()),
                Token::GreaterThan,
                Token::Number(3.0),
                Token::Do,
                Token::Print,
                Token::StringLiteral("foobar".to_owned()),
                Token::EndWhile,
                Token::EOF,
            ],
        ));
        assert_eq!(Lexer::lex_tokens(input), expected_output);
    }

    #[test]
    fn test_comment() {
        let input = b"@ This is a comment\n";
        let expected_output = Ok((
            &b""[..],
            vec![Token::Comment(" This is a comment".to_string()), Token::EOF],
        ));
        assert_eq!(Lexer::lex_tokens(input), expected_output);
    }

    #[test]
    fn test_arithmetic_operations() {
        let input = b"Ab3 BOLE TOH 10 + 5 - 3 * 2 / 1 % 2";
        let expected_output = Ok((
            &b""[..],
            vec![
                Token::Ident("Ab3".to_string()),
                Token::Assign,
                Token::Number(10.0),
                Token::Plus,
                Token::Number(5.0),
                Token::MinusOrNegation,
                Token::Number(3.0),
                Token::Multiply,
                Token::Number(2.0),
                Token::Divide,
                Token::Number(1.0),
                Token::Modulo,
                Token::Number(2.0),
                Token::EOF,
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
                Token::Ident("_A".to_string()),
                Token::Assign,
                Token::BoolLiteral(true),
                Token::LogicalAnd,
                Token::BoolLiteral(false),
                Token::LogicalOr,
                Token::BoolLiteral(true),
                Token::EOF,
            ],
        ));
        assert_eq!(Lexer::lex_tokens(input), expected_output);
    }

    #[test]
    fn test_punctuators() {
        let input = b"( )";
        let expected_output = Ok((
            &b""[..],
            vec![Token::LeftParen, Token::RightParen, Token::EOF],
        ));
        assert_eq!(Lexer::lex_tokens(input), expected_output);
    }
    #[test]
    fn test_combined_operations() {
        let input = b"A_ BOLE TOH (10 + 5 - 3) * 2 / (1 % 2) && true || false";
        let expected_output = Ok((
            &b""[..],
            vec![
                Token::Ident("A_".to_string()),
                Token::Assign,
                Token::LeftParen,
                Token::Number(10.0),
                Token::Plus,
                Token::Number(5.0),
                Token::MinusOrNegation,
                Token::Number(3.0),
                Token::RightParen,
                Token::Multiply,
                Token::Number(2.0),
                Token::Divide,
                Token::LeftParen,
                Token::Number(1.0),
                Token::Modulo,
                Token::Number(2.0),
                Token::RightParen,
                Token::LogicalAnd,
                Token::BoolLiteral(true),
                Token::LogicalOr,
                Token::BoolLiteral(false),
                Token::EOF,
            ],
        ));
        assert_eq!(Lexer::lex_tokens(input), expected_output);
    }

    #[test]
    fn is_identifier_token() {
        let input = "_AB".as_bytes();
        let expected_output = Ok((&b""[..], vec![Token::Ident("_AB".to_string()), Token::EOF]));
        assert_eq!(Lexer::lex_tokens(input), expected_output);
    }
}
