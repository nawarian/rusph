use crate::parser::lexer::Lexer;
use crate::parser::lexer::tokens::Token;

#[test]
pub fn lex_assignment() {
    let mut lexer: Lexer = Lexer::new("$a = 1;");

    assert_eq!(Token::Variable("a".into()), lexer.next_token());
    assert_eq!(Token::Whitespace, lexer.next_token());
    assert_eq!(Token::Assignment, lexer.next_token());
    assert_eq!(Token::Whitespace, lexer.next_token());
    assert_eq!(Token::LNumber("1".into()), lexer.next_token());
    assert_eq!(Token::Semicolon, lexer.next_token());
    assert_eq!(Token::EndOfFile, lexer.next_token());
}

#[test]
pub fn lex_operations() {
    let mut lexer: Lexer = Lexer::new("$a = 1 + 2 - 4 * 2 / 5 % 9;");

    assert_eq!(Token::Variable("a".into()), lexer.next_token());
    assert_eq!(Token::Whitespace, lexer.next_token());

    assert_eq!(Token::Assignment, lexer.next_token());
    assert_eq!(Token::Whitespace, lexer.next_token());

    assert_eq!(Token::LNumber("1".into()), lexer.next_token());
    assert_eq!(Token::Whitespace, lexer.next_token());

    assert_eq!(Token::Plus, lexer.next_token());
    assert_eq!(Token::Whitespace, lexer.next_token());

    assert_eq!(Token::LNumber("2".into()), lexer.next_token());
    assert_eq!(Token::Whitespace, lexer.next_token());

    assert_eq!(Token::Minus, lexer.next_token());
    assert_eq!(Token::Whitespace, lexer.next_token());

    assert_eq!(Token::LNumber("4".into()), lexer.next_token());
    assert_eq!(Token::Whitespace, lexer.next_token());

    assert_eq!(Token::Asterisk, lexer.next_token());
    assert_eq!(Token::Whitespace, lexer.next_token());

    assert_eq!(Token::LNumber("2".into()), lexer.next_token());
    assert_eq!(Token::Whitespace, lexer.next_token());

    assert_eq!(Token::Slash, lexer.next_token());
    assert_eq!(Token::Whitespace, lexer.next_token());

    assert_eq!(Token::LNumber("5".into()), lexer.next_token());
    assert_eq!(Token::Whitespace, lexer.next_token());

    assert_eq!(Token::Percentage, lexer.next_token());
    assert_eq!(Token::Whitespace, lexer.next_token());

    assert_eq!(Token::LNumber("9".into()), lexer.next_token());
    assert_eq!(Token::Semicolon, lexer.next_token());

    assert_eq!(Token::EndOfFile, lexer.next_token());
}
