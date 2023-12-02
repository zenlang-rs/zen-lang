use zen::evaluator;
use zen::lexer::lexer_util::Lexer;
use zen::lexer::tokens::Tokens;
use zen::parser::ast::Program;
use zen::parser::parser_util::Parser;

fn assert_input_with_program(input: &[u8]) -> Program {
    let (_, r) = Lexer::lex_tokens(input).unwrap();
    let tokens = Tokens::new(&r);
    let (_, result) = Parser::parse_tokens(tokens).unwrap();
    result
}
#[test]
fn test_evaluator_works() {
    let input = "PARAMPARA PRATISHTA ANUSHASHAN A BOLE TOH 10
        @ B BOLE TOH 20\n B BOLE TOH 20\nPRINT BASANTI PRINT A + B * A\n KHATAM TATA BYE BYE"
        .as_bytes();

    println!(
        "{:#?}",
        evaluator::interpreter::Interpreter::new("", false)
            .run_code(assert_input_with_program(input))
    );
}

#[test]
fn test_if_evaluator_works() {
    let input = "PARAMPARA PRATISHTA ANUSHASHAN
        A BOLE TOH 10
        A BOLE TOH INPUT LE LE RE BABA
        A BOLE TOH INPUT LE LE RE BABA
        A BOLE TOH INPUT LE LE RE BABA
        JAB TAK HAI JAAN A < 20 && A > 0 TAB TAK
            PRINT BASANTI PRINT A
            A BOLE TOH A - 1
        JAHAN
        KHATAM TATA BYE BYE"
        .as_bytes();

    println!(
        "{:#?}",
        evaluator::interpreter::Interpreter::new("", false)
            .run_code(assert_input_with_program(input))
    );
}

#[test]
fn test_evaluator_arithmetic_operations() {
    let input = "PARAMPARA PRATISHTA ANUSHASHAN A BOLE TOH 10 + 5 - 3 * 2 / 1 % 2 PRINT BASANTI PRINT A KHATAM TATA BYE BYE".as_bytes();
    let expected_output = 10.0 + 5.0 - 3.0 * 2.0 / 1.0 % 2.0;
    assert_eq!(
        evaluator::interpreter::Interpreter::new("", false)
            .run_code(assert_input_with_program(input))
            .unwrap()
            .trim_end(),
        expected_output.to_string()
    );
}
#[test]
fn test_evaluator_arithmetic_operations1() {
    let input = "PARAMPARA PRATISHTA ANUSHASHAN A BOLE TOH 100-234/1*4-9%2 PRINT BASANTI PRINT A KHATAM TATA BYE BYE".as_bytes();
    let expected_output = 100.0 - 234.0 / 1.0 * 4.0 - 9.0 % 2.0;
    assert_eq!(
        evaluator::interpreter::Interpreter::new("", false)
            .run_code(assert_input_with_program(input))
            .unwrap()
            .trim_end(),
        expected_output.to_string()
    );
}

#[test]
fn test_evaluator_arithmetic_operations12() {
    let input = "PARAMPARA PRATISHTA ANUSHASHAN A BOLE TOH 3-4/2*3 PRINT BASANTI PRINT A KHATAM TATA BYE BYE".as_bytes();
    let expected_output = 3.0 - 4.0 / 2.0 * 3.0;
    assert_eq!(
        evaluator::interpreter::Interpreter::new("", false)
            .run_code(assert_input_with_program(input))
            .unwrap()
            .trim_end(),
        expected_output.to_string()
    );
}

#[test]
fn test_evaluator_complex_expression() {
    let input = "PARAMPARA PRATISHTA ANUSHASHAN A BOLE TOH 3.0 - 4.0 / 2.0 * 3.0 % 2.0 PRINT BASANTI PRINT A KHATAM TATA BYE BYE".as_bytes();
    let expected_output = 3.0 - 4.0 / 2.0 * 3.0 % 2.0;
    assert_eq!(
        evaluator::interpreter::Interpreter::new("", false)
            .run_code(assert_input_with_program(input))
            .unwrap()
            .trim_end(),
        expected_output.to_string()
    );
}

#[test]
fn test_evaluator_complex_arithmetic() {
    let input = "PARAMPARA PRATISHTA ANUSHASHAN A BOLE TOH 100-234/1*4-9%2 PRINT BASANTI PRINT A KHATAM TATA BYE BYE".as_bytes();
    let expected_output = 100.0 - 234.0 / 1.0 * 4.0 - 9.0 % 2.0;
    assert_eq!(
        evaluator::interpreter::Interpreter::new("", false)
            .run_code(assert_input_with_program(input))
            .unwrap()
            .trim_end(),
        expected_output.to_string()
    );
}

#[test]
fn test_evaluator_while_loop() {
    let input = "PARAMPARA PRATISHTA ANUSHASHAN A BOLE TOH 10 JAB TAK HAI JAAN A > 0 TAB TAK PRINT BASANTI PRINT A A BOLE TOH A - 1 JAHAN KHATAM TATA BYE BYE".as_bytes();
    let expected_output = "10\n9\n8\n7\n6\n5\n4\n3\n2\n1";
    assert_eq!(
        evaluator::interpreter::Interpreter::new("", false)
            .run_code(assert_input_with_program(input))
            .unwrap()
            .trim_end(),
        expected_output
    );
}
#[test]
fn test_evaluator_if_elif_else() {
    let input = "PARAMPARA PRATISHTA ANUSHASHAN A BOLE TOH 10 AGAR A > 15 TAB A BOLE TOH A+10 WARNA AGAR A < 5 TAB A BOLE TOH A-10 NHI TOH A BOLE TOH A*10 BAS ITNA HI PRINT BASANTI PRINT A KHATAM TATA BYE BYE".as_bytes();
    let expected_output = (10.0 * 10.0).to_string();
    assert_eq!(
        evaluator::interpreter::Interpreter::new("", false)
            .run_code(assert_input_with_program(input))
            .unwrap()
            .trim_end(),
        expected_output
    );
}
#[test]
fn test_evaluator_while_if() {
    let input = "PARAMPARA PRATISHTA ANUSHASHAN A BOLE TOH 10 JAB TAK HAI JAAN A > 0 TAB TAK AGAR A % 2 == 0 TAB PRINT BASANTI PRINT A\n BAS ITNA HI A BOLE TOH A - 1 JAHAN KHATAM TATA BYE BYE".as_bytes();
    let expected_output = "10\n8\n6\n4\n2";
    assert_eq!(
        evaluator::interpreter::Interpreter::new("", false)
            .run_code(assert_input_with_program(input))
            .unwrap()
            .trim_end(),
        expected_output
    );
}

#[test]
fn test_evaluator_input_arithmetic() {
    let input = "PARAMPARA PRATISHTA ANUSHASHAN A BOLE TOH INPUT LE LE RE BABA A BOLE TOH A + 10 PRINT BASANTI PRINT A KHATAM TATA BYE BYE".as_bytes();
    // Assuming the input provided is 20
    let expected_output = (20.0 + 10.0).to_string();
    assert_eq!(
        evaluator::interpreter::Interpreter::new("20\n", false)
            .run_code(assert_input_with_program(input))
            .unwrap()
            .trim_end(),
        expected_output
    );
}
#[test]
fn test_evaluator_input_while_loop() {
    let input = "PARAMPARA PRATISHTA ANUSHASHAN A BOLE TOH INPUT LE LE RE BABA JAB TAK HAI JAAN A > 0 TAB TAK PRINT BASANTI PRINT A A BOLE TOH A - 1 JAHAN KHATAM TATA BYE BYE".as_bytes();
    // Assuming the input provided is 5
    let expected_output = "5\n4\n3\n2\n1";
    assert_eq!(
        evaluator::interpreter::Interpreter::new("5\n", false)
            .run_code(assert_input_with_program(input))
            .unwrap()
            .trim_end(),
        expected_output
    );
}

#[test]
fn test_evaluator_multiple_inputs() {
    let input = "PARAMPARA PRATISHTA ANUSHASHAN A BOLE TOH INPUT LE LE RE BABA B BOLE TOH INPUT LE LE RE BABA A BOLE TOH A + B PRINT BASANTI PRINT A KHATAM TATA BYE BYE".as_bytes();
    // Assuming the inputs provided are 20 and 30
    let expected_output = (20.0 + 30.0).to_string();
    assert_eq!(
        evaluator::interpreter::Interpreter::new("20\n30\n", false)
            .run_code(assert_input_with_program(input))
            .unwrap()
            .trim_end(),
        expected_output
    );
}
#[test]
fn test_evaluator_input_inside_if() {
    let input = "PARAMPARA PRATISHTA ANUSHASHAN A BOLE TOH 10 AGAR A > 5 TAB A BOLE TOH INPUT LE LE RE BABA BAS ITNA HI PRINT BASANTI PRINT A%6 KHATAM TATA BYE BYE".as_bytes();
    // Assuming the input provided is 20
    let expected_output = "2";
    assert_eq!(
        evaluator::interpreter::Interpreter::new("20\n", false)
            .run_code(assert_input_with_program(input))
            .unwrap()
            .trim_end(),
        expected_output
    );
}
#[test]
fn test_evaluator_input_inside_while() {
    let input = "PARAMPARA PRATISHTA ANUSHASHAN A BOLE TOH 3 JAB TAK HAI JAAN A > 0 TAB TAK B BOLE TOH INPUT LE LE RE BABA PRINT BASANTI PRINT B*2 -1\n A BOLE TOH A - 1 JAHAN KHATAM TATA BYE BYE".as_bytes();
    // Assuming the inputs provided are 10, 20, and 30
    let expected_output = "19\n39\n59";
    assert_eq!(
        evaluator::interpreter::Interpreter::new("10\n20\n30\n", false)
            .run_code(assert_input_with_program(input))
            .unwrap()
            .trim_end(),
        expected_output
    );
}
