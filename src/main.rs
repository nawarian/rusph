use crate::parser::lexer::Lexer;

mod parser;
mod interpreter;

fn main() {
}

#[cfg(test)]
mod test {
    use crate::interpreter::Interpreter;

    #[test]
    pub fn echo_hello_world() {
        assert_eq!(
            String::from("Hello world"),
            Interpreter::interpret("<?php echo 'Hello world';")
        );
    }
}
