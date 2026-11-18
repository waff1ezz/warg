use std::collections::HashMap;
use std::sync::Mutex;
use crate::wargc_ast::ast::Ast;
use crate::wargc_ast::lexer::{Token, TokenKind};
use crate::wargc_ast::node::{Node, SymbolId};
use crate::wargc_driver::File;

pub static SYMBOL_COUNTER: Mutex<usize> = Mutex::new(0);

impl File {
    pub fn make_ast(&self, tokens: Vec<Token>) -> Ast {
        let mut nodes: Vec<Node> = vec![];
        let mut iter = tokens
            .into_iter()
            .peekable();

        while let Some(_) = iter.peek() {
            nodes.push(
                Node::parse(&self, &mut iter, false, Some(vec![
                    TokenKind::Func,
                    TokenKind::Struct,
                    TokenKind::Static,
                    TokenKind::Interface,
                    TokenKind::Import,
                    TokenKind::Export,
                ]))
            );
        }

        Ast {
            nodes,
            symbol_table: HashMap::new(),
        }
    }
}