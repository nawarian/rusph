use crate::parser::lexer::tokens::Token;

struct Node {
    name: String,
    children: Vec<Node>,
}

pub struct Ast {
    root: Node,
}

impl Ast {
    pub fn parse_tokens(tokens: Vec<Token>) -> Ast {
        // @todo -> implement token parsing
    }
}
