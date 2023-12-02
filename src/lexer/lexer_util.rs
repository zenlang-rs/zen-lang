use std::str::Utf8Error;
use std::*;

use nom::branch::*;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{
    alpha1, alphanumeric1, char, digit1, multispace0, multispace1, space0,
};
use nom::combinator::{map, map_res, opt, recognize, value};
use nom::multi::many0;
use nom::sequence::pair;
use nom::sequence::{delimited, preceded};
use nom::IResult;
use nom::*;

use crate::lexer::token_type::TokenType;

macro_rules! syntax {
    ($func_name: ident, $tag_string: literal, $output_token: expr) => {
        fn $func_name(s: &[u8]) -> IResult<&[u8], TokenType> {
            map(tag($tag_string), |_| $output_token)(s)
        }
    };
}

pub struct Lexer;

impl Lexer {
    pub fn lex_tokens(bytes: &[u8]) -> IResult<&[u8], Vec<TokenType>> {
        lex_tokens(bytes)
            .map(|(slice, result)| (slice, [&result[..], &vec![TokenType::Eof][..]].concat()))
    }
}

// Operators
syntax! {plus_operator, "+", TokenType::Plus}
syntax! {minus_or_negation_operator, "-", TokenType::Minus}
syntax! {multiply_operator, "*", TokenType::Multiply}
syntax! {divide_operator, "/", TokenType::Divide}
syntax! {modulo_operator, "%", TokenType::Modulo}
syntax! {greater_operator, ">", TokenType::GreaterThan}
syntax! {greater_equal_operator, ">=", TokenType::GreaterThanEqual}
syntax! {less_operator, "<", TokenType::LessThan}
syntax! {less_equal_operator, "<=", TokenType::LessThanEqual}
syntax! {equal_operator, "==", TokenType::Equal}
syntax! {not_equal_operator, "!=", TokenType::NotEqual}
syntax! {not_operator, "!", TokenType::Not}
syntax! {logical_and_operator, "&&", TokenType::LogicalAnd}
syntax! {logical_or_operator, "||", TokenType::LogicalOr}

pub fn lex_operator(input: &[u8]) -> IResult<&[u8], TokenType> {
    alt((
        logical_and_operator,
        logical_or_operator,
        assign_operator,
        plus_operator,
        minus_or_negation_operator,
        multiply_operator,
        divide_operator,
        modulo_operator,
        greater_equal_operator,
        less_equal_operator,
        greater_operator,
        less_operator,
        equal_operator,
        not_equal_operator,
        not_operator,
    ))(input)
}
// Keywords
fn start_program_keyword(input: &[u8]) -> IResult<&[u8], TokenType> {
    let (input, _) = multispace0(input)?; // Skip leading spaces
    let (input, _) = tag("PARAMPARA")(input)?;
    let (input, _) = multispace1(input)?; // Require at least one space
    let (input, _) = tag("PRATISHTA")(input)?;
    let (input, _) = multispace1(input)?; // Require at least one space
    let (input, _) = tag("ANUSHASHAN")(input)?;
    let (input, _) = multispace0(input)?; // Skip trailing spaces
    Ok((input, TokenType::StartProgram))
}

fn end_program_keyword(input: &[u8]) -> IResult<&[u8], TokenType> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("KHATAM")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("TATA")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("BYE")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("BYE")(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, TokenType::EndProgram))
}
syntax! {if_keyword, "AGAR", TokenType::If}
syntax! {then_keyword, "TAB", TokenType::Then}

fn else_if_keyword(input: &[u8]) -> IResult<&[u8], TokenType> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("WARNA")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("AGAR")(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, TokenType::ElseIf))
}

fn else_keyword(input: &[u8]) -> IResult<&[u8], TokenType> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("NHI")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("TOH")(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, TokenType::Else))
}

fn end_if_keyword(input: &[u8]) -> IResult<&[u8], TokenType> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("BAS")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("ITNA")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("HI")(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, TokenType::EndIf))
}

fn while_keyword(input: &[u8]) -> IResult<&[u8], TokenType> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("JAB")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("TAK")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("HAI")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("JAAN")(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, TokenType::While))
}

fn do_keyword(input: &[u8]) -> IResult<&[u8], TokenType> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("TAB")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("TAK")(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, TokenType::Do))
}

syntax! {end_while_keyword, "JAHAN", TokenType::EndWhile}

fn print_keyword(input: &[u8]) -> IResult<&[u8], TokenType> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("PRINT")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("BASANTI")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("PRINT")(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, TokenType::Print))
}

fn input_keyword(input: &[u8]) -> IResult<&[u8], TokenType> {
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
    Ok((input, TokenType::Input))
}

fn assign_operator(input: &[u8]) -> IResult<&[u8], TokenType> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("BOLE")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("TOH")(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, TokenType::Assign))
}
pub fn lex_keyword(input: &[u8]) -> IResult<&[u8], TokenType> {
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

syntax! {left_paren_punctuation, "(", TokenType::LeftParen}
syntax! {right_paren_punctuation, ")", TokenType::RightParen}

pub fn lex_punctuations(input: &[u8]) -> IResult<&[u8], TokenType> {
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

fn lex_string(input: &[u8]) -> IResult<&[u8], TokenType> {
    map(string, TokenType::StringLiteral)(input)
}

fn lex_ident(input: &[u8]) -> IResult<&[u8], TokenType> {
    map(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        |ident: &[u8]| TokenType::Identifier(str::from_utf8(ident).unwrap().to_string()),
    )(input)
}
// Number parsing(float,int)
fn lex_number(input: &[u8]) -> IResult<&[u8], TokenType> {
    let (input, digit_str) = recognize(pair(digit1, opt(pair(char('.'), digit1))))(input)?;
    let float_str = std::str::from_utf8(digit_str).unwrap();
    let float_val = float_str.parse::<f64>().unwrap();
    Ok((input, TokenType::Number(float_val)))
}
// Illegal tokens
fn lex_illegal(input: &[u8]) -> IResult<&[u8], TokenType> {
    map(take(1usize), |_| TokenType::Illegal)(input)
}

fn lex_bool_literal(input: &[u8]) -> IResult<&[u8], TokenType> {
    let parse_true = map(bytes::complete::tag("true"), |_| {
        TokenType::BooleanLiteral(true)
    });
    let parse_false = map(bytes::complete::tag("false"), |_| {
        TokenType::BooleanLiteral(false)
    });
    branch::alt((parse_true, parse_false))(input)
}

fn lex_comment(input: &[u8]) -> IResult<&[u8], TokenType> {
    map(
        delimited(
            bytes::complete::tag(b"@"),
            bytes::complete::take_until("\n"),
            bytes::complete::tag(b"\n"),
        ),
        |_| TokenType::EndOfStatement, // Ignore the comment
    )(input)
}

fn lex_token(input: &[u8]) -> IResult<&[u8], TokenType> {
    alt((
        lex_newline,
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

fn lex_newline(input: &[u8]) -> IResult<&[u8], TokenType> {
    value(TokenType::EndOfStatement, tag("\n"))(input)
}

pub fn lex_tokens(input: &[u8]) -> IResult<&[u8], Vec<TokenType>> {
    let (remaining_input, tokens) = many0(preceded(space0, lex_token))(input)?;

    Ok((remaining_input, tokens))
}
