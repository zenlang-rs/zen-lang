use nom::branch::alt;
use nom::error::{context, Error, ErrorKind};
use nom::multi::many0;
use nom::sequence::{preceded, tuple};
use nom::{error_position, IResult};

use super::ast::*;
use crate::lexer::token_type::TokenType;
use crate::lexer::tokens::Tokens;
use nom;
use nom::bytes::complete::take;
use nom::combinator::{map, opt, peek, verify};
use nom::Err;
use std::result::Result::*;
macro_rules! tag_token (
    ($func_name:ident, $tag: expr) => (
        fn $func_name(tokens: Tokens) -> IResult<Tokens, Tokens> {
            verify(take(1usize), |t: &Tokens| t.tok[0] == $tag)(tokens)
        }
    )
  );
fn parse_literal(input: Tokens) -> IResult<Tokens, Literal> {
    let (i1, t1) = take(1usize)(input)?;
    if t1.tok.is_empty() {
        Err(Err::Error(Error::new(input, ErrorKind::Fail)))
    } else {
        match t1.tok[0].clone() {
            TokenType::Number(name) => Ok((i1, Literal::Number(name))),
            TokenType::StringLiteral(s) => Ok((i1, Literal::StringLiteral(s))),
            TokenType::BooleanLiteral(b) => Ok((i1, Literal::BoolLiteral(b))),
            _ => Err(Err::Error(Error::new(input, ErrorKind::Fail))),
        }
    }
}
fn parse_ident(input: Tokens) -> IResult<Tokens, Ident> {
    let (i1, t1) = take(1usize)(input)?;
    if t1.tok.is_empty() {
        Err(Err::Error(Error::new(input, ErrorKind::Fail)))
    } else {
        match t1.tok[0].clone() {
            TokenType::Identifier(name) => Ok((i1, Ident(name))),
            _ => Err(Err::Error(Error::new(input, ErrorKind::Fail))),
        }
    }
}
tag_token!(lparen_tag, TokenType::LeftParen);
tag_token!(rparen_tag, TokenType::RightParen);
tag_token!(assign_tag, TokenType::Assign);
tag_token!(if_tag, TokenType::If);
tag_token!(then_tag, TokenType::Then);
tag_token!(else_tag, TokenType::Else);
tag_token!(elseif_tag, TokenType::ElseIf);

fn infix_op(t: &TokenType) -> (Precedence, Option<Infix>) {
    match *t {
        TokenType::Equal => (Precedence::PEquals, Some(Infix::Equal)),
        TokenType::NotEqual => (Precedence::PEquals, Some(Infix::NotEqual)),
        TokenType::LessThanEqual => (Precedence::PLessGreater, Some(Infix::LessThanEqual)),
        TokenType::GreaterThanEqual => (Precedence::PLessGreater, Some(Infix::GreaterThanEqual)),
        TokenType::LessThan => (Precedence::PLessGreater, Some(Infix::LessThan)),
        TokenType::GreaterThan => (Precedence::PLessGreater, Some(Infix::GreaterThan)),
        TokenType::Plus => (Precedence::PSum, Some(Infix::Plus)),
        TokenType::Minus => (Precedence::PSum, Some(Infix::Minus)),
        TokenType::Multiply => (Precedence::PProduct, Some(Infix::Multiply)),
        TokenType::Divide => (Precedence::PProduct, Some(Infix::Divide)),
        TokenType::Modulo => (Precedence::PProduct, Some(Infix::Modulo)),
        TokenType::LogicalAnd => (Precedence::PLogicalAnd, Some(Infix::LogicalAnd)),
        TokenType::LogicalOr => (Precedence::PLogicalOr, Some(Infix::LogicalOr)),
        _ => (Precedence::PLowest, None),
    }
}

pub struct Parser;

impl Parser {
    pub fn parse_tokens(tokens: Tokens) -> IResult<Tokens, Program> {
        parse_program(tokens)
    }
}

fn parse_program(input: Tokens) -> IResult<Tokens, Program> {
    let (remaining_tokens, statements) = context("parse_program", many0(parse_statement))(input)?;
    Ok((remaining_tokens, Program { statements }))
}

fn parse_statement(input: Tokens) -> IResult<Tokens, Statement> {
    let (remaining_tokens, statement) = alt((
        parse_program_start,
        parse_program_end,
        parse_let_statement,
        parse_if_statement,
        parse_while_statement,
        parse_print_statement,
        parse_expression_statement,
    ))(input)?;

    let remaining_tokens = if let Ok((remaining_tokens, _)) =
        opt(many0(tag_token(TokenType::EndOfStatement)))(remaining_tokens)
    {
        remaining_tokens
    } else {
        remaining_tokens
    };

    Ok((remaining_tokens, statement))
}

fn parse_expression_statement(input: Tokens) -> IResult<Tokens, Statement> {
    map(parse_expr, Statement::Expression)(input)
}

fn parse_program_start(input: Tokens) -> IResult<Tokens, Statement> {
    map(tag_token(TokenType::StartProgram), |_| {
        Statement::ProgramStart
    })(input)
}

fn parse_program_end(input: Tokens) -> IResult<Tokens, Statement> {
    map(tag_token(TokenType::EndProgram), |_| Statement::ProgramEnd)(input)
}
fn tag_token(token: TokenType) -> impl Fn(Tokens) -> IResult<Tokens, Tokens> {
    move |input: Tokens| {
        let (remaining_tokens, first_token) = take(1usize)(input)?;
        if first_token.tok[0] == token {
            Ok((remaining_tokens, first_token))
        } else {
            Err(nom::Err::Failure(nom::error::Error {
                input,
                code: nom::error::ErrorKind::Fail,
            }))
        }
    }
}

fn parse_let_statement(input: Tokens) -> IResult<Tokens, Statement> {
    map(
        tuple((parse_ident, assign_tag, parse_expr)),
        |(ident, _, expr)| Statement::Let {
            name: ident,
            value: expr,
        },
    )(input)
}

fn parse_while_statement(input: Tokens) -> IResult<Tokens, Statement> {
    map(
        tuple((
            tag_token(TokenType::While),
            parse_expr,
            tag_token(TokenType::Do),
            many0(parse_statement),
            tag_token(TokenType::EndWhile),
        )),
        |(_, condition, _, body, _)| Statement::While {
            condition: Box::new(condition),
            body,
        },
    )(input)
}

fn parse_print_statement(input: Tokens) -> IResult<Tokens, Statement> {
    map(preceded(tag_token(TokenType::Print), parse_expr), |expr| {
        Statement::Print(Box::new(expr))
    })(input)
}

fn parse_expr(input: Tokens) -> IResult<Tokens, Expression> {
    parse_pratt_expr(input, Precedence::PLowest)
}

fn parse_pratt_expr(input: Tokens, precedence: Precedence) -> IResult<Tokens, Expression> {
    let (i1, left) = parse_atom_expr(input)?;
    go_parse_pratt_expr(i1, precedence, left)
}

fn go_parse_pratt_expr(
    input: Tokens,
    precedence: Precedence,
    left: Expression,
) -> IResult<Tokens, Expression> {
    let (i1, t1) = take(1usize)(input)?;

    if t1.tok.is_empty() {
        Ok((i1, left))
    } else {
        let preview = &t1.tok[0];
        let p = infix_op(preview);
        match p {
            (ref peek_precedence, _) if precedence < *peek_precedence => {
                let (i2, left2) = parse_infix_expr(input, left)?;
                go_parse_pratt_expr(i2, precedence, left2)
            }
            _ => Ok((input, left)),
        }
    }
}

fn parse_infix_expr(input: Tokens, left: Expression) -> IResult<Tokens, Expression> {
    let (i1, t1) = take(1usize)(input)?;
    if t1.tok.is_empty() {
        Err(Err::Error(error_position!(input, ErrorKind::Fail)))
    } else {
        let next = &t1.tok[0];
        let (precedence, maybe_op) = infix_op(next);
        match maybe_op {
            None => Err(Err::Error(error_position!(input, ErrorKind::Fail))),
            Some(op) => {
                let (i2, right) = parse_pratt_expr(i1, precedence)?;
                Ok((
                    i2,
                    Expression::InfixExpr {
                        left: Box::new(left),
                        operator: op,
                        right: Box::new(right),
                    },
                ))
            }
        }
    }
}

fn parse_atom_expr(input: Tokens) -> IResult<Tokens, Expression> {
    let (input, expr) = alt((
        parse_literal_expr,
        parse_identifier_expr,
        parse_prefix_expr,
        parse_paren_expr,
        parse_input_expr,
    ))(input)?;
    let (remaining_input, _) = opt(many0(tag_token(TokenType::EndOfStatement)))(input)?;

    Ok((remaining_input, expr))
}
fn parse_paren_expr(input: Tokens) -> IResult<Tokens, Expression> {
    let (i1, _) = lparen_tag(input)?;
    let (i2, expr) = parse_expr(i1)?;
    let (i3, _) = rparen_tag(i2)?;
    Ok((i3, expr))
}
fn parse_literal_expr(input: Tokens) -> IResult<Tokens, Expression> {
    let (i1, lit) = parse_literal(input)?;
    let (_i2, next) = peek(take(1usize))(i1)?;
    if next.tok.is_empty() || !matches!(next.tok[0], TokenType::Identifier(_)) {
        Ok((i1, Expression::LiteralExpr(lit)))
    } else {
        Err(Err::Error(Error::new(input, ErrorKind::Fail)))
    }
}
fn parse_identifier_expr(input: Tokens) -> IResult<Tokens, Expression> {
    map(parse_ident, Expression::IdentifierExpr)(input)
}

fn parse_prefix_expr(input: Tokens) -> IResult<Tokens, Expression> {
    map(
        tuple((parse_prefix_operator, parse_expr)),
        |(operator, right)| Expression::PrefixExpr {
            operator,
            right: Box::new(right),
        },
    )(input)
}

fn parse_input_expr(input: Tokens) -> IResult<Tokens, Expression> {
    map(tag_token(TokenType::Input), |_| Expression::Input)(input)
}

fn parse_if_statement(input: Tokens) -> IResult<Tokens, Statement> {
    let (remaining_tokens, result) = context(
        "parse_if_statement",
        tuple((
            if_tag,
            opt(many0(tag_token(TokenType::EndOfStatement))),
            parse_expr,
            opt(many0(tag_token(TokenType::EndOfStatement))),
            then_tag,
            opt(many0(tag_token(TokenType::EndOfStatement))),
            many0(parse_statement),
            opt(many0(tag_token(TokenType::EndOfStatement))),
            opt(parse_else_elif),
            opt(many0(tag_token(TokenType::EndOfStatement))),
            opt(parse_else),
            tag_token(TokenType::EndIf),
        )),
    )(input)?;

    match result {
        (_, _, condition, _, _, _, consequence, _, elif, _, else_, _) => Ok((
            remaining_tokens,
            Statement::If {
                condition: Box::new(condition),
                consequence,
                alternative: else_.or(elif),
            },
        )),
        _ => Err(nom::Err::Failure(nom::error::Error {
            input,
            code: nom::error::ErrorKind::Fail,
        })),
    }
}

fn parse_else_elif(input: Tokens) -> IResult<Tokens, Vec<Statement>> {
    map(
        tuple((
            elseif_tag,
            opt(many0(tag_token(TokenType::EndOfStatement))),
            parse_expr,
            opt(many0(tag_token(TokenType::EndOfStatement))),
            tag_token(TokenType::Then),
            opt(many0(tag_token(TokenType::EndOfStatement))),
            many0(parse_statement),
            opt(many0(tag_token(TokenType::EndOfStatement))),
            opt(parse_else_elif),
            opt(many0(tag_token(TokenType::EndOfStatement))),
            opt(parse_else),
        )),
        |(_, _, condition, _, _, _, consequence, _, elif, _, else_)| {
            vec![Statement::If {
                condition: Box::new(condition),
                consequence,
                alternative: elif.or(else_),
            }]
        },
    )(input)
}

fn parse_else(input: Tokens) -> IResult<Tokens, Vec<Statement>> {
    map(preceded(else_tag, many0(parse_statement)), |statements| {
        statements
    })(input)
}

fn parse_prefix_operator(input: Tokens) -> IResult<Tokens, Prefix> {
    let (remaining_tokens, token) = take(1usize)(input)?;
    match token.tok[0] {
        TokenType::Plus => Ok((remaining_tokens, Prefix::PrefixPlus)),
        TokenType::Minus => Ok((remaining_tokens, Prefix::PrefixMinus)),
        TokenType::Not => Ok((remaining_tokens, Prefix::Not)),
        _ => Err(nom::Err::Error(nom::error::Error {
            input,
            code: nom::error::ErrorKind::Fail,
        })),
    }
}
