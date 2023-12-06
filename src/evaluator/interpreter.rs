use crate::evaluator::interpreter::InterpreterErrorType::{
    DeadlyError, DivisionByZero, EmptyCustomInputStack, IncompatibleDataType, InvalidInputError,
    MissingStartSymbol, SyntaxError, UndefinedVariable, UnknownParserError,
};
use crate::parser::ast::{Expression, Ident, Infix, Literal, Prefix, Program, Statement};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;

pub struct Interpreter {
    output: String,
    variable_stack: HashMap<String, f64>,
    input: String,
    is_on_console: bool,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum InterpreterErrorType {
    SyntaxError,
    MissingStartSymbol,
    UndefinedVariable,
    DivisionByZero,
    IncompatibleDataType,
    DeadlyError,
    InvalidInputError,
    EmptyCustomInputStack,
    UnknownParserError,
}

lazy_static! {
    static ref ERROR_MESSAGES: HashMap<InterpreterErrorType, &'static str> = {
        let mut m = HashMap::new();
        m.insert(
            SyntaxError,
            "Dost, your code is like a puzzle I can't solve. 'Mere pass bhi syntax hai!'",
        );
        m.insert(MissingStartSymbol, "'PARAMPARA PRATISHTA ANUSHASHAN' missing! 'Rishte mein toh hum tumhare baap lagte hain, naam hai start symbol.'");
        m.insert(UndefinedVariable, "Undefined variable");
        m.insert(DivisionByZero, "Division by zero? 'Ye zero hai, ye divide karne ka sign hai, aur ye divide by zero ka darr.'");
        m.insert(
            IncompatibleDataType,
            "Incompatible data types! 'Ek chutki datatype ki keemat, tum kya jaano Ramesh babu.'",
        );
        m.insert(DeadlyError, "Oh no! Something went really wrong. 'Babuji ne kaha ERROR chhod do, sab ne kaha ERROR chhod do, lekin kisi ne yeh nahi bataya ki error kaise chhodte hain.'");
        m.insert(InvalidInputError, "Expected numeral types as input only! 'Number chahiye, number! Number k alawa kuch nhi.'");
        m.insert(
            EmptyCustomInputStack,
            "Empty input stack! 'Uncle Ji, Uncle Ji, thoda data deejiye.'",
        );
        m.insert(UnknownParserError, "Parsing error due to invalid syntax ! 'Mogambo dukhi hua... kyunki kuch toh gadbad hai ??'");
        m
    };
}
#[derive(Debug)]
pub struct InterpreterError {
    pub msg: String,
    pub error_type: InterpreterErrorType,
}

impl fmt::Display for InterpreterErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

impl InterpreterError {
    pub fn new(err_type: InterpreterErrorType) -> Self {
        let msg = ERROR_MESSAGES.get(&err_type).unwrap_or(&"Unknown Error");
        InterpreterError {
            msg: msg.to_string(),
            error_type: err_type,
        }
    }
    pub fn new_from_custom_error(msg: &str, err_type: InterpreterErrorType) -> Self {
        InterpreterError {
            msg: msg.to_string(),
            error_type: err_type,
        }
    }
    pub fn new_from_append_error(msg_append: &str, err_type: InterpreterErrorType) -> Self {
        let msg = ERROR_MESSAGES.get(&err_type).unwrap_or(&"Unknown Error");
        InterpreterError {
            msg: msg.to_string() + msg_append,
            error_type: err_type,
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new("", false)
    }
}

impl Interpreter {
    pub fn new(input: &str, is_on_console: bool) -> Self {
        Self {
            output: "".to_string(),
            variable_stack: Default::default(),
            input: input.to_string(),
            is_on_console,
        }
    }
    pub fn run_code(&mut self, program_ast: Program) -> Result<String, InterpreterError> {
        let mut program_ast_iter = program_ast.statements.iter();

        if program_ast_iter.next() != Some(&Statement::ProgramStart) {
            return Err(InterpreterError::new(MissingStartSymbol));
        }

        for statement in program_ast_iter {
            match statement {
                Statement::ProgramStart => {
                    return Err(InterpreterError::new_from_custom_error(
                        "Only one 'PARAMPARA PRATISHTA ANUSHASHAN' allowed! 'Ek hi baar bolna kaafi hai.'",
                        SyntaxError,
                    ));
                }
                Statement::ProgramEnd => {
                    break;
                }
                Statement::Let { name, value } => {
                    self.set_value_in_stack(name, value)?;
                }
                Statement::If {
                    condition,
                    consequence,
                    alternative,
                } => {
                    let condition_truth_val = self.evaluate_expression(condition)?;
                    if condition_truth_val == Literal::BoolLiteral(true) {
                        let mut consequence = consequence.clone();
                        consequence.insert(0, Statement::ProgramStart);
                        consequence.push(Statement::ProgramEnd);

                        self.run_code(Program {
                            statements: consequence,
                        })?;
                    } else if let Some(statements) = alternative {
                        let mut consequence = statements.clone();
                        consequence.insert(0, Statement::ProgramStart);
                        consequence.push(Statement::ProgramEnd);

                        self.run_code(Program {
                            statements: consequence,
                        })?;
                    }
                }
                Statement::While { condition, body } => loop {
                    let condition_expr = self.evaluate_expression(condition)?;
                    if condition_expr != Literal::BoolLiteral(true) {
                        break;
                    }

                    let mut body = body.clone();
                    body.insert(0, Statement::ProgramStart);
                    body.push(Statement::ProgramEnd);

                    self.run_code(Program { statements: body })?;
                },
                Statement::Print(expr) => {
                    let value = self.evaluate_expression(expr)?;
                    if self.is_on_console {
                        match value {
                            Literal::Number(num) => {
                                println!("{}", num);
                            }
                            Literal::BoolLiteral(bool) => {
                                println!("{}", bool);
                            }
                            Literal::StringLiteral(str) => {
                                println!("{}", str);
                            }
                        }
                    } else {
                        match value {
                            Literal::Number(num) => {
                                self.output.push_str(&num.to_string());
                            }
                            Literal::BoolLiteral(bool) => {
                                self.output.push_str(&bool.to_string());
                            }
                            Literal::StringLiteral(str) => {
                                self.output.push_str(&str);
                            }
                        }
                        self.output.push('\n');
                    }
                }
                Statement::Expression(expr) => {
                    self.evaluate_expression(expr)?;
                }
            }
        }

        Ok(self.output.clone())
    }

    fn evaluate_expression(
        &mut self,
        expression: &Expression,
    ) -> Result<Literal, InterpreterError> {
        match expression {
            Expression::IdentifierExpr(ident) => self.get_value_of(ident),
            Expression::LiteralExpr(literal) => Ok(literal.clone()),
            Expression::PrefixExpr { .. } => self.evaluate_prefix_expression(expression),
            Expression::InfixExpr { .. } => self.evaluate_infix_expression(expression),
            Expression::Input => self.take_input_from_stdin(),
        }
    }

    fn take_input_from_stdin(&mut self) -> Result<Literal, InterpreterError> {
        // TODO: Implement taking input from user, with possible account for string based input!
        let mut value = String::new();
        if self.input.is_empty() && self.is_on_console {
            std::io::stdin()
                .read_line(&mut value)
                .expect("Failed to read line");

            let value: f64 = match value.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    return Err(InterpreterError::new(
                        InterpreterErrorType::InvalidInputError,
                    ))
                }
            };

            Ok(Literal::Number(value))
        } else {
            let mut value = self.input.split_whitespace().collect::<Vec<&str>>();

            if value.is_empty() {
                return Err(InterpreterError::new(
                    InterpreterErrorType::EmptyCustomInputStack,
                ));
            }

            let num_value: f64 = match value[0].trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    return Err(InterpreterError::new(
                        InterpreterErrorType::InvalidInputError,
                    ))
                }
            };
            value.remove(0);
            self.input = value.join(" ");

            Ok(Literal::Number(num_value))
        }
    }

    fn get_value_of(&self, ident: &Ident) -> Result<Literal, InterpreterError> {
        match self.variable_stack.get(&ident.0) {
            Some(t) => Ok(Literal::Number(*t)),
            None => Err(InterpreterError::new_from_custom_error(
                &format!(
                    "Undefined variable! 'Tumhara value kya hai, {}? , Batao bhi Basanti !'",
                    ident.0
                ),
                UndefinedVariable,
            )),
        }
    }

    fn set_value_in_stack(
        &mut self,
        ident: &Ident,
        value: &Expression,
    ) -> Result<(), InterpreterError> {
        let num_val = self.evaluate_expression(value)?;
        // Redefinition of variables is allowed
        if let Literal::Number(num) = num_val {
            self.variable_stack.insert(ident.0.clone(), num);
            return Ok(());
        }
        Err(InterpreterError::new(IncompatibleDataType))
    }

    fn evaluate_prefix_expression(
        &mut self,
        expression: &Expression,
    ) -> Result<Literal, InterpreterError> {
        if let Expression::PrefixExpr { operator, right } = expression {
            let right = self.evaluate_expression(right)?;

            return match operator {
                Prefix::PrefixPlus => {
                    if let Literal::Number(num_right) = right {
                        return Ok(Literal::Number(num_right));
                    }

                    Err(InterpreterError::new_from_append_error(
                        "Only numeral types allowed with unary addition!",
                        IncompatibleDataType,
                    ))
                }
                Prefix::PrefixMinus => {
                    if let Literal::Number(num_right) = right {
                        return Ok(Literal::Number(-1_f64 * num_right));
                    }

                    Err(InterpreterError::new_from_append_error(
                        "Only numeral types allowed with unary negation!",
                        IncompatibleDataType,
                    ))
                }
                Prefix::Not => {
                    if let Literal::BoolLiteral(bool_right) = right {
                        return Ok(Literal::BoolLiteral(!bool_right));
                    }

                    Err(InterpreterError::new_from_custom_error(
                        "Only boolean types allowed with not! 'Yeh Kya Ho gya hai duniyan ko ??'",
                        IncompatibleDataType,
                    ))
                }
            };
        }
        Err(InterpreterError::new(DeadlyError))
    }
    fn evaluate_infix_expression(
        &mut self,
        expression: &Expression,
    ) -> Result<Literal, InterpreterError> {
        if let Expression::InfixExpr {
            left,
            operator,
            right,
        } = expression
        {
            let left = self.evaluate_expression(left)?;
            let right = self.evaluate_expression(right)?;

            return match operator {
                Infix::Plus => {
                    if let Literal::Number(num_left) = left {
                        if let Literal::Number(num_right) = right {
                            return Ok(Literal::Number(num_left + num_right));
                        }
                    }
                    Err(InterpreterError::new_from_append_error(
                        "Only numeral types allowed with addition! ",
                        IncompatibleDataType,
                    ))
                }
                Infix::Minus => {
                    if let Literal::Number(num_left) = left {
                        if let Literal::Number(num_right) = right {
                            return Ok(Literal::Number(num_left - num_right));
                        }
                    }
                    Err(InterpreterError::new_from_append_error(
                        "Only numeral types allowed with subtraction! 'Number chahiye, number!",
                        IncompatibleDataType,
                    ))
                }
                Infix::Multiply => {
                    if let Literal::Number(num_left) = left {
                        if let Literal::Number(num_right) = right {
                            return Ok(Literal::Number(num_left * num_right));
                        }
                    }
                    Err(InterpreterError::new_from_append_error(
                        "Only numeral types allowed with multiplication!",
                        IncompatibleDataType,
                    ))
                }
                Infix::Divide => {
                    if let Literal::Number(num_left) = left {
                        if let Literal::Number(num_right) = right {
                            if num_right == 0_f64 {
                                return Err(InterpreterError::new(DivisionByZero));
                            }
                            return Ok(Literal::Number(num_left / num_right));
                        }
                    }
                    Err(InterpreterError::new_from_append_error(
                        "Only numeral types allowed with division!",
                        IncompatibleDataType,
                    ))
                }
                Infix::Modulo => {
                    if let Literal::Number(num_left) = left {
                        if let Literal::Number(num_right) = right {
                            return Ok(Literal::Number(num_left % num_right));
                        }
                    }
                    Err(InterpreterError::new_from_append_error(
                        "Only numeral types allowed with modulus! ",
                        IncompatibleDataType,
                    ))
                }

                Infix::Equal => {
                    match left {
                        Literal::Number(num_left) => {
                            if let Literal::Number(num_right) = right {
                                return Ok(Literal::BoolLiteral(num_left == num_right));
                            }
                        }
                        Literal::BoolLiteral(bool_left) => {
                            if let Literal::BoolLiteral(bool_right) = right {
                                return Ok(Literal::BoolLiteral(bool_left == bool_right));
                            }
                        }
                        Literal::StringLiteral(str_left) => {
                            if let Literal::StringLiteral(str_right) = right {
                                return Ok(Literal::BoolLiteral(str_left == str_right));
                            }
                        }
                    }

                    Err(InterpreterError::new_from_custom_error(
                        "Comparison with only homogeneous data types is allowed! 'Ek chutki datatype ki keemat, tum kya jaano Ramesh babu.'",
                        IncompatibleDataType,
                    ))
                }
                Infix::NotEqual => {
                    match left {
                        Literal::Number(num_left) => {
                            if let Literal::Number(num_right) = right {
                                return Ok(Literal::BoolLiteral(num_left != num_right));
                            }
                        }
                        Literal::BoolLiteral(bool_left) => {
                            if let Literal::BoolLiteral(bool_right) = right {
                                return Ok(Literal::BoolLiteral(bool_left != bool_right));
                            }
                        }
                        Literal::StringLiteral(str_left) => {
                            if let Literal::StringLiteral(str_right) = right {
                                return Ok(Literal::BoolLiteral(str_left != str_right));
                            }
                        }
                    }

                    Err(InterpreterError::new_from_append_error(
                        "Comparison with only homogeneous data types is allowed!",
                        IncompatibleDataType,
                    ))
                }
                Infix::GreaterThan => {
                    if let Literal::Number(num_left) = left {
                        if let Literal::Number(num_right) = right {
                            return Ok(Literal::BoolLiteral(num_left > num_right));
                        }
                    }

                    Err(InterpreterError::new_from_append_error(
                        "Comparison with only numeral data types is allowed!",
                        IncompatibleDataType,
                    ))
                }
                Infix::GreaterThanEqual => {
                    if let Literal::Number(num_left) = left {
                        if let Literal::Number(num_right) = right {
                            return Ok(Literal::BoolLiteral(num_left >= num_right));
                        }
                    }

                    Err(InterpreterError::new_from_append_error(
                        "Comparison with only numeral data types is allowed! ",
                        IncompatibleDataType,
                    ))
                }
                Infix::LessThan => {
                    if let Literal::Number(num_left) = left {
                        if let Literal::Number(num_right) = right {
                            return Ok(Literal::BoolLiteral(num_left < num_right));
                        }
                    }

                    Err(InterpreterError::new_from_append_error(
                        "Comparison with only numeral data types is allowed! ",
                        IncompatibleDataType,
                    ))
                }
                Infix::LessThanEqual => {
                    if let Literal::Number(num_left) = left {
                        if let Literal::Number(num_right) = right {
                            return Ok(Literal::BoolLiteral(num_left <= num_right));
                        }
                    }

                    Err(InterpreterError::new_from_append_error(
                        "Comparison with only numeral data types is allowed! ",
                        IncompatibleDataType,
                    ))
                }

                Infix::LogicalAnd => {
                    if let Literal::BoolLiteral(bool_left) = left {
                        if let Literal::BoolLiteral(bool_right) = right {
                            return Ok(Literal::BoolLiteral(bool_left && bool_right));
                        }
                    }

                    Err(InterpreterError::new_from_append_error(
                        "Comparison with only boolean data types is allowed! ",
                        IncompatibleDataType,
                    ))
                }
                Infix::LogicalOr => {
                    if let Literal::BoolLiteral(bool_left) = left {
                        if let Literal::BoolLiteral(bool_right) = right {
                            return Ok(Literal::BoolLiteral(bool_left || bool_right));
                        }
                    }

                    Err(InterpreterError::new_from_append_error(
                        "Comparison with only boolean data types is allowed!",
                        IncompatibleDataType,
                    ))
                }
            };
        }
        Err(InterpreterError::new(DeadlyError))
    }
}
