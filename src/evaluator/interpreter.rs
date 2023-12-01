#![allow(dead_code)]

use crate::evaluator::interpreter::InterpreterErrorType::{DeadlyError, DivisionByZero, IncompatibleDataType, MissingStartSymbol, SyntaxError, UndefinedVariable};
use crate::parser::ast::{Expression, Ident, Infix, Literal, Prefix, Program, Statement};
use std::collections::HashMap;
use std::process::exit;

pub struct Interpreter {
    output: String,
    variable_stack: HashMap<String, f64>,
    input: String
}

#[derive(Debug)]
pub enum InterpreterErrorType {
    SyntaxError,
    MissingStartSymbol,
    UndefinedVariable,
    DivisionByZero,
    IncompatibleDataType,
    DeadlyError
}

#[derive(Debug)]
pub struct InterpreterError {
    msg: String,
    error_type: InterpreterErrorType,
}

impl InterpreterError {
    pub fn new(msg: &str, err_type: InterpreterErrorType) -> Self {
        InterpreterError {
            msg: msg.to_string(),
            error_type: err_type,
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {

    pub fn new() -> Self {
        Self {
            output: "".to_string(),
            variable_stack: Default::default(),
            input: "".to_string(),
        }
    }
    pub fn run_code(&mut self, program_ast: Program) -> Result<String, InterpreterError> {
        let mut program_ast_iter = program_ast.statements.iter();

        if program_ast_iter.next() != Some(&Statement::ProgramStart) {
            return Err(InterpreterError::new("Expected start symbol at start!", MissingStartSymbol));
        }

        for statement in program_ast_iter {
            match statement {
                Statement::ProgramStart => {
                    return Err(InterpreterError::new("Only one start symbol allowed!", SyntaxError));
                }
                Statement::ProgramEnd => {
                    break;
                }
                Statement::Let { name, value } => {
                    self.set_value_in_stack(name, value)?;
                }
                Statement::If { condition, consequence, alternative } => {
                    let condition_truth_val = self.evaluate_expression(condition)?;
                    if condition_truth_val == Literal::BoolLiteral(true) {
                        let mut consequence = consequence.clone();
                        consequence.insert(0, Statement::ProgramStart);
                        consequence.push(Statement::ProgramEnd);

                        self.run_code(Program {
                            statements: consequence,
                        })?;
                    }
                    else if let Some(statements) = alternative {
                        let mut consequence = statements.clone();
                        consequence.insert(0, Statement::ProgramStart);
                        consequence.push(Statement::ProgramEnd);

                        self.run_code(Program {
                            statements: consequence,
                        })?;
                    }
                }
                Statement::While { condition, body} => {
                    loop {
                        let condition_expr = self.evaluate_expression(condition)?;
                        if condition_expr != Literal::BoolLiteral(true) {
                            break;
                        }

                        let mut body = body.clone();
                        body.insert(0, Statement::ProgramStart);
                        body.push(Statement::ProgramEnd);

                        self.run_code(Program {
                            statements: body,
                        })?;
                    }
                }
                Statement::Print(expr) => {
                    let value = self.evaluate_expression(expr)?;
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
                Statement::Expression(expr) => {
                    self.evaluate_expression(expr)?;
                }
            }
        }

        Ok(self.output.clone())
    }

    fn evaluate_expression(&self, expression: &Expression) -> Result<Literal, InterpreterError> {
        match expression {
            Expression::IdentifierExpr(ident) => self.get_value_of(ident),
            Expression::LiteralExpr(literal) => Ok(literal.clone()),
            Expression::PrefixExpr { .. } => self.evaluate_prefix_expression(expression),
            Expression::InfixExpr { .. } => self.evaluate_infix_expression(expression),
            Expression::Input => Ok(Literal::Number(Self::take_input_from_stdin())),
        }
    }

    fn take_input_from_stdin() -> f64 {
        // TODO: Implement taking input from user, with possible account for string based input!
        12_f64
    }

    fn get_value_of(&self, ident: &Ident) -> Result<Literal, InterpreterError> {
        match self.variable_stack.get(&ident.0) {
            Some(t) => Ok(Literal::Number(*t)),
            None => Err(InterpreterError::new(&format!("Can't find definition of {}", ident.0), UndefinedVariable)),
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
        Err(InterpreterError::new(
            "Only Numeral data types can be stored in variables!",
            IncompatibleDataType
        ))
    }

    fn evaluate_prefix_expression(
        &self,
        expression: &Expression,
    ) -> Result<Literal, InterpreterError> {
        if let Expression::PrefixExpr { operator, right } = expression {
            let right = self.evaluate_expression(right)?;

            return match operator {
                Prefix::PrefixPlus => {
                    if let Literal::Number(num_right) = right {
                        return Ok(Literal::Number(num_right));
                    }

                    Err(InterpreterError::new(
                        "Only numeral types allowed with unary addition!",
                        IncompatibleDataType
                    ))
                }
                Prefix::PrefixMinus => {
                    if let Literal::Number(num_right) = right {
                        return Ok(Literal::Number(-1_f64 * num_right));
                    }

                    Err(InterpreterError::new(
                        "Only numeral types allowed with unary negation!",
                        IncompatibleDataType
                    ))
                }
                Prefix::Not => {
                    if let Literal::BoolLiteral(bool_right) = right {
                        return Ok(Literal::BoolLiteral(!bool_right));
                    }

                    Err(InterpreterError::new(
                        "Only boolean types allowed with not!",
                        IncompatibleDataType
                    ))
                }
            };
        }
        Err(InterpreterError::new(
            "Issue parsing the prefix expression!",
            DeadlyError
        ))
    }
    fn evaluate_infix_expression(
        &self,
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
                    Err(InterpreterError::new(
                        "Only numeral types allowed with addition!",
                        IncompatibleDataType
                    ))
                }
                Infix::Minus => {
                    if let Literal::Number(num_left) = left {
                        if let Literal::Number(num_right) = right {
                            return Ok(Literal::Number(num_left - num_right));
                        }
                    }
                    Err(InterpreterError::new(
                        "Only numeral types allowed with subtraction!",
                        IncompatibleDataType
                    ))
                }
                Infix::Multiply => {
                    if let Literal::Number(num_left) = left {
                        if let Literal::Number(num_right) = right {
                            return Ok(Literal::Number(num_left * num_right));
                        }
                    }
                    Err(InterpreterError::new(
                        "Only numeral types allowed with multiplication!",
                        IncompatibleDataType
                    ))
                }
                Infix::Divide => {
                    if let Literal::Number(num_left) = left {
                        if let Literal::Number(num_right) = right {
                            if num_right == 0_f64 {
                                return Err(InterpreterError::new("Can't divide by zero", DivisionByZero));
                            }
                            return Ok(Literal::Number(num_left + num_right));
                        }
                    }
                    Err(InterpreterError::new(
                        "Only numeral types allowed with division!",
                        IncompatibleDataType
                    ))
                }
                Infix::Modulo => {
                    if let Literal::Number(num_left) = left {
                        if let Literal::Number(num_right) = right {
                            return Ok(Literal::Number(num_left % num_right));
                        }
                    }
                    Err(InterpreterError::new(
                        "Only numeral types allowed with modulus!",
                        IncompatibleDataType
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

                    Err(InterpreterError::new(
                        "Comparison with only homogeneous data types is allowed!",
                        IncompatibleDataType
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

                    Err(InterpreterError::new(
                        "Comparison with only homogeneous data types is allowed!",
                        IncompatibleDataType
                    ))
                }
                Infix::GreaterThan => {
                    if let Literal::Number(num_left) = left {
                        if let Literal::Number(num_right) = right {
                            return Ok(Literal::BoolLiteral(num_left > num_right));
                        }
                    }

                    Err(InterpreterError::new(
                        "Comparison with only numeral data types is allowed!",
                        IncompatibleDataType
                    ))
                }
                Infix::GreaterThanEqual => {
                    if let Literal::Number(num_left) = left {
                        if let Literal::Number(num_right) = right {
                            return Ok(Literal::BoolLiteral(num_left >= num_right));
                        }
                    }

                    Err(InterpreterError::new(
                        "Comparison with only numeral data types is allowed!",
                        IncompatibleDataType
                    ))
                }
                Infix::LessThan => {
                    if let Literal::Number(num_left) = left {
                        if let Literal::Number(num_right) = right {
                            return Ok(Literal::BoolLiteral(num_left < num_right));
                        }
                    }

                    Err(InterpreterError::new(
                        "Comparison with only numeral data types is allowed!",
                        IncompatibleDataType
                    ))
                }
                Infix::LessThanEqual => {
                    if let Literal::Number(num_left) = left {
                        if let Literal::Number(num_right) = right {
                            return Ok(Literal::BoolLiteral(num_left <= num_right));
                        }
                    }

                    Err(InterpreterError::new(
                        "Comparison with only numeral data types is allowed!",
                        IncompatibleDataType
                    ))
                }

                Infix::LogicalAnd => {
                    if let Literal::BoolLiteral(bool_left) = left {
                        if let Literal::BoolLiteral(bool_right) = right {
                            return Ok(Literal::BoolLiteral(bool_left && bool_right));
                        }
                    }

                    Err(InterpreterError::new(
                        "Comparison with only boolean data types is allowed!",
                        IncompatibleDataType
                    ))
                }
                Infix::LogicalOr => {
                    if let Literal::BoolLiteral(bool_left) = left {
                        if let Literal::BoolLiteral(bool_right) = right {
                            return Ok(Literal::BoolLiteral(bool_left || bool_right));
                        }
                    }

                    Err(InterpreterError::new(
                        "Comparison with only boolean data types is allowed!",
                        IncompatibleDataType
                    ))
                }
            };
        }
        Err(InterpreterError::new("Issue parsing the infix expression!", DeadlyError))
    }
}
