#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    ProgramStart,
    ProgramEnd,
    Let {
        name: Ident,
        value: Expression,
    },
    If {
        condition: Box<Expression>,
        consequence: Vec<Statement>,
        alternative: Option<Vec<Statement>>,
    },
    While {
        condition: Box<Expression>,
        body: Vec<Statement>,
    },
    Print(Box<Expression>),
    Expression(Expression),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    IdentifierExpr(Ident),
    LiteralExpr(Literal),
    PrefixExpr {
        operator: Prefix,
        right: Box<Expression>,
    },
    InfixExpr {
        left: Box<Expression>,
        operator: Infix,
        right: Box<Expression>,
    },
    Input,
}
#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    Number(f64),
    BoolLiteral(bool),
    StringLiteral(String),
}

#[derive(PartialEq, Debug, Eq, Clone)]
pub struct Ident(pub String);

#[derive(PartialEq, Debug, Clone)]
pub enum Prefix {
    PrefixPlus,
    PrefixMinus,
    Not,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Infix {
    Plus,
    Minus,
    Divide,
    Multiply,
    Modulo,
    Equal,
    NotEqual,
    GreaterThanEqual,
    LessThanEqual,
    GreaterThan,
    LessThan,
    LogicalAnd,
    LogicalOr,
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Precedence {
    PLowest,
    PLogicalOr,
    PLogicalAnd,
    PEquals,
    PLessGreater,
    PSum,
    PProduct,
    PLParen,
    PRParen,
}
