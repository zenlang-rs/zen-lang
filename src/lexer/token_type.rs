#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    // Special tokens
    Illegal,
    Eof,

    // Identifiers and literals
    Identifier(String),
    Number(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
    Comment(String),

    // Operators
    Assign,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,

    // Relational Operators
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    Equal,
    NotEqual,

    // Logical Operators
    Not,
    LogicalAnd,
    LogicalOr,

    // Keywords
    StartProgram,
    EndProgram,
    If,
    Then,
    ElseIf,
    Else,
    EndIf,
    While,
    Do,
    EndWhile,
    Print,
    Input,

    // Punctuation
    LeftParen,
    RightParen,
}
