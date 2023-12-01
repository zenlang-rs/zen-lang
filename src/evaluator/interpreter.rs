#![allow(dead_code)]

use crate::evaluator::interpreter::InterpreterErrorType::SyntaxError;
use crate::parser::ast::{Expression, Ident, Infix, Literal, Prefix, Program, Statement};
use std::collections::HashMap;

pub struct Interpreter {
    output: String,
    iterator: usize,
    variable_stack: HashMap<String, f64>,
}

#[derive(Debug)]

enum InterpreterErrorType {
    SyntaxError,
}

#[derive(Debug)]
pub struct InterpreterError {
    msg: String,
    error_type: InterpreterErrorType,
}

impl InterpreterError {
    pub fn new(msg: &str) -> Self {
        InterpreterError {
            msg: msg.to_string(),
            error_type: SyntaxError,
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
            iterator: 0,
            variable_stack: Default::default(),
        }
    }
    pub fn run_code(&mut self, program_ast: Program) -> Result<(), InterpreterError> {
        let mut program_ast_iter = program_ast.statements.iter();

        if program_ast_iter.next() != Some(&Statement::ProgramStart) {
            return Err(InterpreterError::new("Expected start symbol at start!"));
        }

        for statement in program_ast_iter {
            match statement {
                Statement::ProgramStart => {
                    return Err(InterpreterError::new("Only one start symbol allowed!"));
                }
                Statement::ProgramEnd => {
                    break;
                }
                Statement::Let { name, value } => {
                    self.set_value_in_stack(name, value)
                        .expect("TODO: panic message");
                }
                Statement::If { .. } => {}
                Statement::While { .. } => {}
                Statement::Print(expr) => {
                    println!("[PRINTING] {:#?}", self.evaluate_expression(expr))
                }
                Statement::Expression(expr) => {
                    println!("[EVALUATING] {:#?}", self.evaluate_expression(expr));
                }
            }
        }
        Ok(())
    }

    fn evaluate_expression(&self, expression: &Expression) -> Literal {
        match expression {
            Expression::IdentifierExpr(ident) => self.get_value_of(ident).unwrap(),
            Expression::LiteralExpr(literal) => literal.clone(),
            Expression::PrefixExpr { .. } => self.evaluate_prefix_expression(expression).unwrap(),
            Expression::InfixExpr { .. } => self.evaluate_infix_expression(expression).unwrap(),
            Expression::Input => Literal::Number(Self::take_input_from_stdin()),
        }
    }

    fn take_input_from_stdin() -> f64 {
        // TODO: Implement taking input from user, with possible account for string based input!
        12_f64
    }

    fn get_value_of(&self, ident: &Ident) -> Result<Literal, InterpreterError> {
        match self.variable_stack.get(&ident.0) {
            Some(t) => Ok(Literal::Number(*t)),
            None => Err(InterpreterError::new("Can't find definition of {ident.0}")),
        }
    }

    fn set_value_in_stack(
        &mut self,
        ident: &Ident,
        value: &Expression,
    ) -> Result<(), InterpreterError> {
        let num_val = self.evaluate_expression(value);
        // Redefinition of variables is allowed
        if let Literal::Number(num) = num_val {
            self.variable_stack.insert(ident.0.clone(), num);
            return Ok(());
        }
        Err(InterpreterError::new(
            "Only Numeral data types can be stored in variables!",
        ))
    }

    fn evaluate_prefix_expression(
        &self,
        expression: &Expression,
    ) -> Result<Literal, InterpreterError> {
        if let Expression::PrefixExpr { operator, right } = expression {
            let right = self.evaluate_expression(right);

            return match operator {
                Prefix::PrefixPlus => {
                    if let Literal::Number(num_right) = right {
                        return Ok(Literal::Number(num_right));
                    }

                    Err(InterpreterError::new(
                        "Only numeral types allowed with unary addition!",
                    ))
                }
                Prefix::PrefixMinus => {
                    if let Literal::Number(num_right) = right {
                        return Ok(Literal::Number(-1_f64 * num_right));
                    }

                    Err(InterpreterError::new(
                        "Only numeral types allowed with unary negation!",
                    ))
                }
                Prefix::Not => {
                    if let Literal::BoolLiteral(bool_right) = right {
                        return Ok(Literal::BoolLiteral(!bool_right));
                    }

                    Err(InterpreterError::new(
                        "Only boolean types allowed with not!",
                    ))
                }
            };
        }
        Err(InterpreterError::new(
            "Issue parsing the prefix expression!",
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
            let left = self.evaluate_expression(left);
            let right = self.evaluate_expression(right);

            return match operator {
                Infix::Plus => {
                    if let Literal::Number(num_left) = left {
                        if let Literal::Number(num_right) = right {
                            return Ok(Literal::Number(num_left + num_right));
                        }
                    }
                    Err(InterpreterError::new(
                        "Only numeral types allowed with addition!",
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
                    ))
                }
                Infix::Divide => {
                    if let Literal::Number(num_left) = left {
                        if let Literal::Number(num_right) = right {
                            if num_right == 0_f64 {
                                return Err(InterpreterError::new("Can't divide by zero"));
                            }
                            return Ok(Literal::Number(num_left + num_right));
                        }
                    }
                    Err(InterpreterError::new(
                        "Only numeral types allowed with division!",
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
                        "Comparison with only homogeneous data types allowed!",
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
                        "Comparison with only homogeneous data types allowed!",
                    ))
                }
                Infix::GreaterThan => {
                    if let Literal::Number(num_left) = left {
                        if let Literal::Number(num_right) = right {
                            return Ok(Literal::BoolLiteral(num_left > num_right));
                        }
                    }

                    Err(InterpreterError::new(
                        "Comparison with only numeral data types allowed!",
                    ))
                }
                Infix::GreaterThanEqual => {
                    if let Literal::Number(num_left) = left {
                        if let Literal::Number(num_right) = right {
                            return Ok(Literal::BoolLiteral(num_left >= num_right));
                        }
                    }

                    Err(InterpreterError::new(
                        "Comparison with only numeral data types allowed!",
                    ))
                }
                Infix::LessThan => {
                    if let Literal::Number(num_left) = left {
                        if let Literal::Number(num_right) = right {
                            return Ok(Literal::BoolLiteral(num_left < num_right));
                        }
                    }

                    Err(InterpreterError::new(
                        "Comparison with only numeral data types allowed!",
                    ))
                }
                Infix::LessThanEqual => {
                    if let Literal::Number(num_left) = left {
                        if let Literal::Number(num_right) = right {
                            return Ok(Literal::BoolLiteral(num_left <= num_right));
                        }
                    }

                    Err(InterpreterError::new(
                        "Comparison with only numeral data types allowed!",
                    ))
                }

                Infix::LogicalAnd => {
                    if let Literal::BoolLiteral(bool_left) = left {
                        if let Literal::BoolLiteral(bool_right) = right {
                            return Ok(Literal::BoolLiteral(bool_left && bool_right));
                        }
                    }

                    Err(InterpreterError::new(
                        "Comparison with only boolean data types allowed!",
                    ))
                }
                Infix::LogicalOr => {
                    if let Literal::BoolLiteral(bool_left) = left {
                        if let Literal::BoolLiteral(bool_right) = right {
                            return Ok(Literal::BoolLiteral(bool_left || bool_right));
                        }
                    }

                    Err(InterpreterError::new(
                        "Comparison with only boolean data types allowed!",
                    ))
                }
            };
        }
        Err(InterpreterError::new("Issue parsing the infix expression!"))
    }
}
