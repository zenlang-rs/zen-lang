#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    ProgramStart, // PARAMPARA PRATISHTA ANUSHASHAN
    ProgramEnd,   // KHATAM TATA BYE BYE
    Let {
        name: String,
        value: Expression,
    }, // BOLE TOH - Initialization
    If {
        condition: Box<Expression>,
        consequence: Vec<Statement>,
        alternative: Option<Vec<Statement>>,
    }, // AGAR - If Statement, TAB - Then Statement, WARNA AGAR - ElseIF Statement, NHI TOH - Else Statement, BAS ITNA HI - End of IF Statement
    While {
        condition: Box<Expression>,
        body: Vec<Statement>,
    }, // JAB TAK HAI JAAN - While Statement, TAB TAK - Then Statement after While Statement, JAHAN - End of While Statement
    Print(Box<Expression>), // PRINT BASANTI PRINT - Print Statement
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    IdentifierExpr(Ident),
    LiteralExpr(Literal),
    PrefixExpr {
        operator: Prefix, // Prefix operators
        right: Box<Expression>,
    },
    InfixExpr {
        left: Box<Expression>,
        operator: Infix, // Infix operators
        right: Box<Expression>,
    },
    Input, // INPUT LE LE RE BABA - Take Input from Console
}
#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    Number(f64),
    BoolLiteral(bool),
    StringLiteral(String),
}

#[derive(PartialEq, Debug, Eq, Clone)]
pub struct Ident(pub String);
// copy code
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
    Modulo, //
    Equal,
    NotEqual,
    GreaterThanEqual,
    LessThanEqual,
    GreaterThan,
    LessThan,
    LogicalAnd, //
    LogicalOr,  //
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
    PRParen, // for precedence of expr. eval
}
