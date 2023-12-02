use zen::lexer::{lexer_util::Lexer, tokens::Tokens};
use zen::parser::ast::*;
use zen::parser::parser_util::Parser;

#[allow(dead_code)]
fn assert_input_with_program(input: &[u8], expected_results: Program) {
    let (_, r) = Lexer::lex_tokens(input).unwrap();
    let tokens = Tokens::new(&r);
    let (_, result) = Parser::parse_tokens(tokens).unwrap();
    assert_eq!(result, expected_results);
}

#[test]
fn test_program_start_and_end() {
    let input = "PARAMPARA PRATISHTA ANUSHASHAN KHATAM TATA BYE BYE".as_bytes();
    let program: Program = Program {
        statements: vec![Statement::ProgramStart, Statement::ProgramEnd],
    };
    assert_input_with_program(input, program);
}

#[test]
fn test_variable_initialization() {
    let input = "A BOLE TOH 10".as_bytes();
    let program: Program = Program {
        statements: vec![Statement::Let {
            name: Ident("A".to_owned()),
            value: Expression::LiteralExpr(Literal::Number(10.0)),
        }],
    };
    assert_input_with_program(input, program);
}
#[test]
fn test_variable_initialization2() {
    let input = "A BOLE TOH 10+5".as_bytes();
    let program: Program = Program {
        statements: vec![Statement::Let {
            name: Ident("A".to_owned()),
            value: Expression::InfixExpr {
                left: Box::new(Expression::LiteralExpr(Literal::Number(10.0))),
                operator: Infix::Plus,
                right: Box::new(Expression::LiteralExpr(Literal::Number(5.0))),
            },
        }],
    };
    assert_input_with_program(input, program);
}
#[test]
fn test_addition_expression() {
    let input = "10 + 20".as_bytes();
    let expected_program: Program = Program {
        statements: vec![Statement::Expression(Expression::InfixExpr {
            left: Box::new(Expression::LiteralExpr(Literal::Number(10.0))),
            operator: Infix::Plus,
            right: Box::new(Expression::LiteralExpr(Literal::Number(20.0))),
        })],
    };
    assert_input_with_program(input, expected_program);
}

#[test]
fn test_input_from_console() {
    let input = "C BOLE TOH INPUT LE LE RE BABA".as_bytes();
    let program: Program = Program {
        statements: vec![Statement::Let {
            name: Ident("C".to_owned()),
            value: Expression::Input,
        }],
    };
    assert_input_with_program(input, program);
}
#[test]
fn test_complex_expression1() {
    let input = "100-234/1*4-9%2".as_bytes();
    let expected_program: Program = Program {
        statements: vec![Statement::Expression(Expression::InfixExpr {
            left: Box::new(Expression::InfixExpr {
                left: Box::new(Expression::LiteralExpr(Literal::Number(100.0))),
                operator: Infix::Minus,
                right: Box::new(Expression::InfixExpr {
                    left: Box::new(Expression::InfixExpr {
                        left: Box::new(Expression::LiteralExpr(Literal::Number(234.0))),
                        operator: Infix::Divide,
                        right: Box::new(Expression::LiteralExpr(Literal::Number(1.0))),
                    }),
                    operator: Infix::Multiply,
                    right: Box::new(Expression::LiteralExpr(Literal::Number(4.0))),
                }),
            }),
            operator: Infix::Minus,
            right: Box::new(Expression::InfixExpr {
                left: Box::new(Expression::LiteralExpr(Literal::Number(9.0))),
                operator: Infix::Modulo,
                right: Box::new(Expression::LiteralExpr(Literal::Number(2.0))),
            }),
        })],
    };
    assert_input_with_program(input, expected_program);
}

#[test]
fn test_print_statement() {
    let input = "PRINT BASANTI PRINT A".as_bytes();
    let program: Program = Program {
        statements: vec![Statement::Print(Box::new(Expression::IdentifierExpr(
            Ident("A".to_string()),
        )))],
    };
    assert_input_with_program(input, program);
}

#[test]
fn test_while_statement() {
    let input = "JAB TAK HAI JAAN A > 3 TAB TAK \n PRINT BASANTI PRINT A \
            PRINT BASANTI PRINT B 
            JAHAN"
        .as_bytes();
    let program: Program = Program {
        statements: vec![Statement::While {
            condition: Box::new(Expression::InfixExpr {
                left: Box::new(Expression::IdentifierExpr(Ident("A".to_string()))),
                operator: Infix::GreaterThan,
                right: Box::new(Expression::LiteralExpr(Literal::Number(3.0))),
            }),
            body: vec![
                Statement::Print(Box::new(Expression::IdentifierExpr(Ident("A".to_string())))),
                Statement::Print(Box::new(Expression::IdentifierExpr(Ident("B".to_string())))),
            ],
        }],
    };
    assert_input_with_program(input, program);
}

#[test]
fn test_expression() {
    let input = "(10-5)*4".as_bytes();
    let expected_program: Program = Program {
        statements: vec![Statement::Expression(Expression::InfixExpr {
            left: Box::new(Expression::InfixExpr {
                left: Box::new(Expression::LiteralExpr(Literal::Number(10.0))),
                operator: Infix::Minus,
                right: Box::new(Expression::LiteralExpr(Literal::Number(5.0))),
            }),
            operator: Infix::Multiply,
            right: Box::new(Expression::LiteralExpr(Literal::Number(4.0))),
        })],
    };
    assert_input_with_program(input, expected_program);
}

#[test]
fn test_complex_expression() {
    let input = "1/2*(2+3%2)-3".as_bytes();
    let expected_program: Program = Program {
        statements: vec![Statement::Expression(Expression::InfixExpr {
            left: Box::new(Expression::InfixExpr {
                left: Box::new(Expression::InfixExpr {
                    left: Box::new(Expression::LiteralExpr(Literal::Number(1.0))),
                    operator: Infix::Divide,
                    right: Box::new(Expression::LiteralExpr(Literal::Number(2.0))),
                }),
                operator: Infix::Multiply,
                right: Box::new(Expression::InfixExpr {
                    left: Box::new(Expression::LiteralExpr(Literal::Number(2.0))),
                    operator: Infix::Plus,
                    right: Box::new(Expression::InfixExpr {
                        left: Box::new(Expression::LiteralExpr(Literal::Number(3.0))),
                        operator: Infix::Modulo,
                        right: Box::new(Expression::LiteralExpr(Literal::Number(2.0))),
                    }),
                }),
            }),
            operator: Infix::Minus,
            right: Box::new(Expression::LiteralExpr(Literal::Number(3.0))),
        })],
    };
    assert_input_with_program(input, expected_program);
}

#[test]
fn test_multiple_parenthesized_expressions() {
    let input = "(1+2)*(3-4)/(5%6)".as_bytes();
    let expected_program: Program = Program {
        statements: vec![Statement::Expression(Expression::InfixExpr {
            left: Box::new(Expression::InfixExpr {
                left: Box::new(Expression::InfixExpr {
                    left: Box::new(Expression::LiteralExpr(Literal::Number(1.0))),
                    operator: Infix::Plus,
                    right: Box::new(Expression::LiteralExpr(Literal::Number(2.0))),
                }),
                operator: Infix::Multiply,
                right: Box::new(Expression::InfixExpr {
                    left: Box::new(Expression::LiteralExpr(Literal::Number(3.0))),
                    operator: Infix::Minus,
                    right: Box::new(Expression::LiteralExpr(Literal::Number(4.0))),
                }),
            }),
            operator: Infix::Divide,
            right: Box::new(Expression::InfixExpr {
                left: Box::new(Expression::LiteralExpr(Literal::Number(5.0))),
                operator: Infix::Modulo,
                right: Box::new(Expression::LiteralExpr(Literal::Number(6.0))),
            }),
        })],
    };
    assert_input_with_program(input, expected_program);
}

#[test]
fn test_nested_parentheses() {
    let input = "((1+2)*3)/(4-(5%6))".as_bytes();
    let expected_program: Program = Program {
        statements: vec![Statement::Expression(Expression::InfixExpr {
            left: Box::new(Expression::InfixExpr {
                left: Box::new(Expression::InfixExpr {
                    left: Box::new(Expression::LiteralExpr(Literal::Number(1.0))),
                    operator: Infix::Plus,
                    right: Box::new(Expression::LiteralExpr(Literal::Number(2.0))),
                }),
                operator: Infix::Multiply,
                right: Box::new(Expression::LiteralExpr(Literal::Number(3.0))),
            }),
            operator: Infix::Divide,
            right: Box::new(Expression::InfixExpr {
                left: Box::new(Expression::LiteralExpr(Literal::Number(4.0))),
                operator: Infix::Minus,
                right: Box::new(Expression::InfixExpr {
                    left: Box::new(Expression::LiteralExpr(Literal::Number(5.0))),
                    operator: Infix::Modulo,
                    right: Box::new(Expression::LiteralExpr(Literal::Number(6.0))),
                }),
            }),
        })],
    };
    assert_input_with_program(input, expected_program);
}

#[test]
fn test_simple_if_statement() {
    let input = "AGAR A > 3 TAB A BOLE TOH A+10 BAS ITNA HI".as_bytes();
    let program: Program = Program {
        statements: vec![Statement::If {
            condition: Box::new(Expression::InfixExpr {
                left: Box::new(Expression::IdentifierExpr(Ident("A".to_string()))),
                operator: Infix::GreaterThan,
                right: Box::new(Expression::LiteralExpr(Literal::Number(3.0))),
            }),
            consequence: vec![Statement::Let {
                name: Ident("A".to_string()),
                value: Expression::InfixExpr {
                    left: Box::new(Expression::IdentifierExpr(Ident("A".to_string()))),
                    operator: Infix::Plus,
                    right: Box::new(Expression::LiteralExpr(Literal::Number(10.0))),
                },
            }],
            alternative: None,
        }],
    };
    assert_input_with_program(input, program);
}

#[test]
fn test_if_else_statement() {
    let input = "AGAR A < 5 TAB A BOLE TOH A-5 NHI TOH A BOLE TOH A+5 BAS ITNA HI".as_bytes();
    let program: Program = Program {
        statements: vec![Statement::If {
            condition: Box::new(Expression::InfixExpr {
                left: Box::new(Expression::IdentifierExpr(Ident("A".to_string()))),
                operator: Infix::LessThan,
                right: Box::new(Expression::LiteralExpr(Literal::Number(5.0))),
            }),
            consequence: vec![Statement::Let {
                name: Ident("A".to_string()),
                value: Expression::InfixExpr {
                    left: Box::new(Expression::IdentifierExpr(Ident("A".to_string()))),
                    operator: Infix::Minus,
                    right: Box::new(Expression::LiteralExpr(Literal::Number(5.0))),
                },
            }],
            alternative: Some(vec![Statement::Let {
                name: Ident("A".to_string()),
                value: Expression::InfixExpr {
                    left: Box::new(Expression::IdentifierExpr(Ident("A".to_string()))),
                    operator: Infix::Plus,
                    right: Box::new(Expression::LiteralExpr(Literal::Number(5.0))),
                },
            }]),
        }],
    };
    assert_input_with_program(input, program);
}

#[test]
fn test_if_elif_else_statement() {
    let input = "AGAR A > 3 TAB A BOLE TOH A+10 WARNA AGAR A < 3 TAB A BOLE TOH A-10 NHI TOH A BOLE TOH A*10 BAS ITNA HI".as_bytes();
    let program: Program = Program {
        statements: vec![Statement::If {
            condition: Box::new(Expression::InfixExpr {
                left: Box::new(Expression::IdentifierExpr(Ident("A".to_string()))),
                operator: Infix::GreaterThan,
                right: Box::new(Expression::LiteralExpr(Literal::Number(3.0))),
            }),
            consequence: vec![Statement::Let {
                name: Ident("A".to_string()),
                value: Expression::InfixExpr {
                    left: Box::new(Expression::IdentifierExpr(Ident("A".to_string()))),
                    operator: Infix::Plus,
                    right: Box::new(Expression::LiteralExpr(Literal::Number(10.0))),
                },
            }],
            alternative: Some(vec![Statement::If {
                condition: Box::new(Expression::InfixExpr {
                    left: Box::new(Expression::IdentifierExpr(Ident("A".to_string()))),
                    operator: Infix::LessThan,
                    right: Box::new(Expression::LiteralExpr(Literal::Number(3.0))),
                }),
                consequence: vec![Statement::Let {
                    name: Ident("A".to_string()),
                    value: Expression::InfixExpr {
                        left: Box::new(Expression::IdentifierExpr(Ident("A".to_string()))),
                        operator: Infix::Minus,
                        right: Box::new(Expression::LiteralExpr(Literal::Number(10.0))),
                    },
                }],
                alternative: Some(vec![Statement::Let {
                    name: Ident("A".to_string()),
                    value: Expression::InfixExpr {
                        left: Box::new(Expression::IdentifierExpr(Ident("A".to_string()))),
                        operator: Infix::Multiply,
                        right: Box::new(Expression::LiteralExpr(Literal::Number(10.0))),
                    },
                }]),
            }]),
        }],
    };
    assert_input_with_program(input, program);
}

#[test]
fn test_initialization_and_if_statement() {
    let input = "PARAMPARA PRATISHTA ANUSHASHAN
    A BOLE TOH 10
    AGAR A > 5 TAB
    A BOLE TOH A+5
    BAS ITNA HI
    KHATAM TATA BYE BYE"
        .as_bytes();

    let program: Program = Program {
        statements: vec![
            Statement::ProgramStart,
            Statement::Let {
                name: Ident("A".to_string()),
                value: Expression::LiteralExpr(Literal::Number(10.0)),
            },
            Statement::If {
                condition: Box::new(Expression::InfixExpr {
                    left: Box::new(Expression::IdentifierExpr(Ident("A".to_string()))),
                    operator: Infix::GreaterThan,
                    right: Box::new(Expression::LiteralExpr(Literal::Number(5.0))),
                }),
                consequence: vec![Statement::Let {
                    name: Ident("A".to_string()),
                    value: Expression::InfixExpr {
                        left: Box::new(Expression::IdentifierExpr(Ident("A".to_string()))),
                        operator: Infix::Plus,
                        right: Box::new(Expression::LiteralExpr(Literal::Number(5.0))),
                    },
                }],
                alternative: None,
            },
            Statement::ProgramEnd,
        ],
    };
    assert_input_with_program(input, program);
}

#[test]
fn test_two_initializations() {
    let input = "PARAMPARA PRATISHTA ANUSHASHAN A BOLE TOH 10 
        @ B BOLE TOH 20\n B BOLE TOH 20 KHATAM TATA BYE BYE"
        .as_bytes();

    let program: Program = Program {
        statements: vec![
            Statement::ProgramStart,
            Statement::Let {
                name: Ident("A".to_string()),
                value: Expression::LiteralExpr(Literal::Number(10.0)),
            },
            Statement::Let {
                name: Ident("B".to_string()),
                value: Expression::LiteralExpr(Literal::Number(20.0)),
            },
            Statement::ProgramEnd,
        ],
    };
    assert_input_with_program(input, program);
}

#[test]
fn test_initialization_and_while_statement() {
    let input = "PARAMPARA PRATISHTA ANUSHASHAN
    A BOLE TOH 10
    JAB TAK HAI JAAN A > 0 TAB TAK \n
    A BOLE TOH A-1
    JAHAN
    KHATAM TATA BYE BYE"
        .as_bytes();

    let program: Program = Program {
        statements: vec![
            Statement::ProgramStart,
            Statement::Let {
                name: Ident("A".to_string()),
                value: Expression::LiteralExpr(Literal::Number(10.0)),
            },
            Statement::While {
                condition: Box::new(Expression::InfixExpr {
                    left: Box::new(Expression::IdentifierExpr(Ident("A".to_string()))),
                    operator: Infix::GreaterThan,
                    right: Box::new(Expression::LiteralExpr(Literal::Number(0.0))),
                }),
                body: vec![Statement::Let {
                    name: Ident("A".to_string()),
                    value: Expression::InfixExpr {
                        left: Box::new(Expression::IdentifierExpr(Ident("A".to_string()))),
                        operator: Infix::Minus,
                        right: Box::new(Expression::LiteralExpr(Literal::Number(1.0))),
                    },
                }],
            },
            Statement::ProgramEnd,
        ],
    };
    assert_input_with_program(input, program);
}

#[test]
fn test_initialization_by_input_and_if_statement() {
    let input = "PARAMPARA PRATISHTA ANUSHASHAN
    A BOLE TOH INPUT LE LE RE BABA
    AGAR A > 5 TAB\n
     A BOLE TOH A+5
    BAS ITNA HI
    KHATAM TATA BYE BYE"
        .as_bytes();

    let program: Program = Program {
        statements: vec![
            Statement::ProgramStart,
            Statement::Let {
                name: Ident("A".to_string()),
                value: Expression::Input,
            },
            Statement::If {
                condition: Box::new(Expression::InfixExpr {
                    left: Box::new(Expression::IdentifierExpr(Ident("A".to_string()))),
                    operator: Infix::GreaterThan,
                    right: Box::new(Expression::LiteralExpr(Literal::Number(5.0))),
                }),
                consequence: vec![Statement::Let {
                    name: Ident("A".to_string()),
                    value: Expression::InfixExpr {
                        left: Box::new(Expression::IdentifierExpr(Ident("A".to_string()))),
                        operator: Infix::Plus,
                        right: Box::new(Expression::LiteralExpr(Literal::Number(5.0))),
                    },
                }],
                alternative: None,
            },
            Statement::ProgramEnd,
        ],
    };
    assert_input_with_program(input, program);
}

#[test]
fn test_if_statement_complex_logical_condition() {
    let input = "AGAR !(A > B) TAB A BOLE TOH A+10 BAS ITNA HI".as_bytes();
    let program: Program = Program {
        statements: vec![Statement::If {
            condition: Box::new(Expression::PrefixExpr {
                operator: Prefix::Not,
                right: Box::new(Expression::InfixExpr {
                    left: Box::new(Expression::IdentifierExpr(Ident("A".to_string()))),
                    operator: Infix::GreaterThan,
                    right: Box::new(Expression::IdentifierExpr(Ident("B".to_string()))),
                }),
            }),
            consequence: vec![Statement::Let {
                name: Ident("A".to_string()),
                value: Expression::InfixExpr {
                    left: Box::new(Expression::IdentifierExpr(Ident("A".to_string()))),
                    operator: Infix::Plus,
                    right: Box::new(Expression::LiteralExpr(Literal::Number(10.0))),
                },
            }],
            alternative: None,
        }],
    };
    assert_input_with_program(input, program);
}

#[test]
fn test_if_statement_complex_boolean_condition() {
    let input = "AGAR A > B && B <= C TAB A BOLE TOH A+10 BAS ITNA HI".as_bytes();
    let program: Program = Program {
        statements: vec![Statement::If {
            condition: Box::new(Expression::InfixExpr {
                left: Box::new(Expression::InfixExpr {
                    left: Box::new(Expression::IdentifierExpr(Ident("A".to_string()))),
                    operator: Infix::GreaterThan,
                    right: Box::new(Expression::IdentifierExpr(Ident("B".to_string()))),
                }),
                operator: Infix::LogicalAnd,
                right: Box::new(Expression::InfixExpr {
                    left: Box::new(Expression::IdentifierExpr(Ident("B".to_string()))),
                    operator: Infix::LessThanEqual,
                    right: Box::new(Expression::IdentifierExpr(Ident("C".to_string()))),
                }),
            }),
            consequence: vec![Statement::Let {
                name: Ident("A".to_string()),
                value: Expression::InfixExpr {
                    left: Box::new(Expression::IdentifierExpr(Ident("A".to_string()))),
                    operator: Infix::Plus,
                    right: Box::new(Expression::LiteralExpr(Literal::Number(10.0))),
                },
            }],
            alternative: None,
        }],
    };
    assert_input_with_program(input, program);
}

#[test]
fn test_if_statement_complex_arithmetic_condition() {
    let input = "AGAR A + B * 2 > 10 TAB A BOLE TOH A+10 BAS ITNA HI".as_bytes();
    let program: Program = Program {
        statements: vec![Statement::If {
            condition: Box::new(Expression::InfixExpr {
                left: Box::new(Expression::InfixExpr {
                    left: Box::new(Expression::IdentifierExpr(Ident("A".to_string()))),
                    operator: Infix::Plus,
                    right: Box::new(Expression::InfixExpr {
                        left: Box::new(Expression::IdentifierExpr(Ident("B".to_string()))),
                        operator: Infix::Multiply,
                        right: Box::new(Expression::LiteralExpr(Literal::Number(2.0))),
                    }),
                }),
                operator: Infix::GreaterThan,
                right: Box::new(Expression::LiteralExpr(Literal::Number(10.0))),
            }),
            consequence: vec![Statement::Let {
                name: Ident("A".to_string()),
                value: Expression::InfixExpr {
                    left: Box::new(Expression::IdentifierExpr(Ident("A".to_string()))),
                    operator: Infix::Plus,
                    right: Box::new(Expression::LiteralExpr(Literal::Number(10.0))),
                },
            }],
            alternative: None,
        }],
    };
    assert_input_with_program(input, program);
}

#[test]
fn test_complex_program() {
    let input = "PARAMPARA PRATISHTA ANUSHASHAN
    A BOLE TOH 10
    C BOLE TOH INPUT LE LE RE BABA
    AGAR A > 3 TAB
    A BOLE TOH A+10
    WARNA AGAR B >= 3 TAB
    B BOLE TOH B+A
    PRINT BASANTI PRINT B
    NHI TOH
    PRINT BASANTI PRINT A*10+B
    PRINT BASANTI PRINT B
    BAS ITNA HI

    JAB TAK HAI JAAN A > 3 TAB TAK
    PRINT BASANTI PRINT A
    JAHAN

    PRINT BASANTI PRINT A

    KHATAM TATA BYE BYE"
        .as_bytes();
    let program: Program = Program {
        statements: vec![
            Statement::ProgramStart,
            Statement::Let {
                name: Ident("A".to_string()),
                value: Expression::LiteralExpr(Literal::Number(10.0)),
            },
            Statement::Let {
                name: Ident("C".to_string()),
                value: Expression::Input,
            },
            Statement::If {
                condition: Box::new(Expression::InfixExpr {
                    left: Box::new(Expression::IdentifierExpr(Ident("A".to_string()))),
                    operator: Infix::GreaterThan,
                    right: Box::new(Expression::LiteralExpr(Literal::Number(3.0))),
                }),
                consequence: vec![Statement::Let {
                    name: Ident("A".to_string()),
                    value: Expression::InfixExpr {
                        left: Box::new(Expression::IdentifierExpr(Ident("A".to_string()))),
                        operator: Infix::Plus,
                        right: Box::new(Expression::LiteralExpr(Literal::Number(10.0))),
                    },
                }],
                alternative: Some(vec![Statement::If {
                    condition: Box::new(Expression::InfixExpr {
                        left: Box::new(Expression::IdentifierExpr(Ident("B".to_string()))),
                        operator: Infix::GreaterThanEqual,
                        right: Box::new(Expression::LiteralExpr(Literal::Number(3.0))),
                    }),
                    consequence: vec![
                        Statement::Let {
                            name: Ident("B".to_string()),
                            value: Expression::InfixExpr {
                                left: Box::new(Expression::IdentifierExpr(Ident("B".to_string()))),
                                operator: Infix::Plus,
                                right: Box::new(Expression::IdentifierExpr(Ident("A".to_string()))),
                            },
                        },
                        Statement::Print(Box::new(Expression::IdentifierExpr(Ident(
                            "B".to_string(),
                        )))),
                    ],
                    alternative: Some(vec![
                        Statement::Print(Box::new(Expression::InfixExpr {
                            left: Box::new(Expression::InfixExpr {
                                left: Box::new(Expression::IdentifierExpr(Ident("A".to_string()))),
                                operator: Infix::Multiply,
                                right: Box::new(Expression::LiteralExpr(Literal::Number(10.0))),
                            }),
                            operator: Infix::Plus,
                            right: Box::new(Expression::IdentifierExpr(Ident("B".to_string()))),
                        })),
                        Statement::Print(Box::new(Expression::IdentifierExpr(Ident(
                            "B".to_string(),
                        )))),
                    ]),
                }]),
            },
            Statement::While {
                condition: Box::new(Expression::InfixExpr {
                    left: Box::new(Expression::IdentifierExpr(Ident("A".to_string()))),
                    operator: Infix::GreaterThan,
                    right: Box::new(Expression::LiteralExpr(Literal::Number(3.0))),
                }),
                body: vec![Statement::Print(Box::new(Expression::IdentifierExpr(
                    Ident("A".to_string()),
                )))],
            },
            Statement::Print(Box::new(Expression::IdentifierExpr(Ident("A".to_string())))),
            Statement::ProgramEnd,
        ],
    };
    assert_input_with_program(input, program);
}

// #[test]
// fn test_complex_expression12() {
//     let input = "PARAMPARA PRATISHTA ANUSHASHAN A BOLE TOH 10 + 5 - 3 * 2 / 1 % 2 PRINT BASANTI PRINT A KHATAM TATA BYE BYE".as_bytes();
//     let program: Program = Program {
//         statements: vec![
//             Statement::ProgramStart,
//             Statement::Let {
//                 name: Ident("A".to_string()),
//                 value: Expression::InfixExpr {
//                     left: Box::new(Expression::InfixExpr {
//                         left: Box::new(Expression::InfixExpr {
//                             left: Box::new(Expression::InfixExpr {
//                                 left: Box::new(Expression::InfixExpr {
//                                     left: Box::new(Expression::LiteralExpr(Literal::Number(10.0))),
//                                     operator: Infix::Plus,
//                                     right: Box::new(Expression::LiteralExpr(Literal::Number(5.0))),
//                                 }),
//                                 operator: Infix::Minus,
//                                 right: Box::new(Expression::LiteralExpr(Literal::Number(3.0))),
//                             }),
//                             operator: Infix::Multiply,
//                             right: Box::new(Expression::LiteralExpr(Literal::Number(2.0))),
//                         }),
//                         operator: Infix::Divide,
//                         right: Box::new(Expression::LiteralExpr(Literal::Number(1.0))),
//                     }),
//                     operator: Infix::Modulo,
//                     right: Box::new(Expression::LiteralExpr(Literal::Number(2.0))),
//                 },
//             },
//             Statement::Print(Box::new(Expression::IdentifierExpr(Ident("A".to_string())))),
//             Statement::ProgramEnd,
//         ],
//     };
//     assert_input_with_program(input, program);
// }

#[test]
fn test_complex_expression_print() {
    let input = "PRINT BASANTI PRINT 3-4/2*3".as_bytes();
    let program: Program = Program {
        statements: vec![Statement::Print(Box::new(Expression::InfixExpr {
            left: Box::new(Expression::LiteralExpr(Literal::Number(3.0))),
            operator: Infix::Minus,
            right: Box::new(Expression::InfixExpr {
                left: Box::new(Expression::InfixExpr {
                    left: Box::new(Expression::LiteralExpr(Literal::Number(4.0))),
                    operator: Infix::Divide,
                    right: Box::new(Expression::LiteralExpr(Literal::Number(2.0))),
                }),
                operator: Infix::Multiply,
                right: Box::new(Expression::LiteralExpr(Literal::Number(3.0))),
            }),
        }))],
    };
    assert_input_with_program(input, program);
}

#[test]
fn test_complex_expression_123() {
    let input = "PRINT BASANTI PRINT 3.0 - 4.0 / 2.0 * 3.0 % 2.0".as_bytes();
    let program: Program = Program {
        statements: vec![Statement::Print(Box::new(Expression::InfixExpr {
            left: Box::new(Expression::LiteralExpr(Literal::Number(3.0))),
            operator: Infix::Minus,
            right: Box::new(Expression::InfixExpr {
                left: Box::new(Expression::InfixExpr {
                    left: Box::new(Expression::InfixExpr {
                        left: Box::new(Expression::LiteralExpr(Literal::Number(4.0))),
                        operator: Infix::Divide,
                        right: Box::new(Expression::LiteralExpr(Literal::Number(2.0))),
                    }),
                    operator: Infix::Multiply,
                    right: Box::new(Expression::LiteralExpr(Literal::Number(3.0))),
                }),
                operator: Infix::Modulo,
                right: Box::new(Expression::LiteralExpr(Literal::Number(2.0))),
            }),
        }))],
    };
    assert_input_with_program(input, program);
}
