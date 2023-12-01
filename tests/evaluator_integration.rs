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

    println!("{:#?}", evaluator::interpreter::Interpreter::new().run_code(assert_input_with_program(input)));

}

#[test]
fn test_if_evaluator_works() {
    let input = "PARAMPARA PRATISHTA ANUSHASHAN
        A BOLE TOH 10
        @ B BOLE TOH 20
        JAB TAK HAI JAAN A < 20 && A > 0 TAB TAK
            PRINT BASANTI PRINT A
            A BOLE TOH A - 1
        JAHAN
        KHATAM TATA BYE BYE"
        .as_bytes();

    println!("{:#?}", evaluator::interpreter::Interpreter::new().run_code(assert_input_with_program(input)));

}