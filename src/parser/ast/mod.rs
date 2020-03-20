use crate::parser::lexer::tokens::Token;

struct Node {
    name: String,
    children: Vec<Node>,
}

pub struct Ast {
    root: Node,
}

impl Ast {
    pub fn parse_tokens(_tokens: Vec<Token>) -> Ast {
        let root: Node = Node {
            name: String::from(""),
            children: Vec::new(),
        };

        Ast {
            root
        }
    }
}
