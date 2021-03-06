pub mod tokens;

use std::iter::Peekable;
use std::str::Chars;
use crate::parser::lexer::tokens::Token;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &str) -> Lexer {
        Lexer {
            input: code.chars().peekable(),
        }
    }

    fn next_char(&mut self) -> Option<char> {
        self.input.next()
    }

    fn is_letter(&self, ch: char) -> bool {
        ch.is_alphanumeric() || ch == '_'
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn peek_is_letter(&mut self) -> bool {
        match self.peek_char() {
            Some(&ch) => self.is_letter(ch),
            None => false,
        }
    }

    fn read_number(&mut self, first_number: char) -> String {
        let mut number: String = String::new();
        number.push(first_number);

        while let Some(&ch) = self.peek_char() {
            if ch.is_numeric() == false {
                break;
            }

            number.push(self.next_char().unwrap());
        }

        number
    }

    pub fn next_token(&mut self) -> Token {
        match self.next_char() {
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('*') => Token::Asterisk,
            Some('/') => Token::Slash,
            Some('%') => Token::Percentage,

            Some('$') => {
                // Started with $, might be a variable...

                // If next isn't a letter, this is an Illegal token
                if self.peek_is_letter() == false {
                    return Token::Illegal;
                }

                let mut name = String::new();
                while self.peek_is_letter() {
                    name.push(self.next_char().unwrap());
                }

                Token::Variable(name)
            }

            Some(' ') => {
                Token::Whitespace
            }

            Some('=') => {
                // If next token is not '=', it is an assignment
                match self.peek_char() {
                    Some(&ch) => {
                        if ch == '=' {
                            // @todo implement comparison token
                            return Token::Illegal;
                        }

                        Token::Assignment
                    }
                    _ => Token::Illegal
                }
            }

            Some(';') => Token::Semicolon,

            Some(ch @ _) => {
                if ch.is_numeric() {
                    Token::LNumber(self.read_number(ch))
                } else {
                    Token::Illegal
                }
            }

            None => {
                Token::EndOfFile
            }
        }
    }
}

#[cfg(test)]
mod test;
