#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    EndOfFile,

    // Symbols
    Variable(String),
    LNumber(String),

    // Operations
    Assignment,

    // Operators
    Plus,
    Minus,
    Slash,
    Asterisk,
    Percentage,

    // Delimiters
    Semicolon,
    Whitespace,
}
