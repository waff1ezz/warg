use crate::wargc_ast::lexer::{Token, TokenKind};
use crate::wargc_ast::node::Node;
use crate::wargc_driver::File;

impl File {
    pub fn make_ast(&self, tokens: Vec<Token>) -> Vec<Node> {
        let mut nodes: Vec<Node> = vec![];
        let mut iter = tokens
            .into_iter()
            .peekable();
        
        while let Some(_) = iter.peek() {
            nodes.push(
                Node::parse(&self, &mut iter, Some(vec![
                    TokenKind::Func,
                    TokenKind::Struct,
                    TokenKind::Static,
                    TokenKind::Interface,
                    TokenKind::Import,
                    TokenKind::Export,
                ]))
            );
        }
        
        nodes
    }
}