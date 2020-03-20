use crate::parser::lexer::Lexer;

mod parser;

fn main() {
    let mut lexer = Lexer::new("$a = 1 + 2;");

    println!("{:?}", lexer.next_token());
    println!("{:?}", lexer.next_token());
    println!("{:?}", lexer.next_token());
    println!("{:?}", lexer.next_token());
    println!("{:?}", lexer.next_token());
    println!("{:?}", lexer.next_token());
    println!("{:?}", lexer.next_token());
    println!("{:?}", lexer.next_token());
    println!("{:?}", lexer.next_token());
    println!("{:?}", lexer.next_token());
    println!("{:?}", lexer.next_token());
}
