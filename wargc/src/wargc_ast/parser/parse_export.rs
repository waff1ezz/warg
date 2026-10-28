use std::iter::Peekable;
use std::vec::IntoIter;
use crate::wargc_ast::lexer::{Token, TokenKind};
use crate::wargc_ast::node::{ExportStmt, Node};
use crate::wargc_driver::File;
use crate::wargc_util::expect_token;

impl ExportStmt {
    pub fn parse(file: &File, iter: &mut Peekable<IntoIter<Token>>) -> ExportStmt {
        expect_token(file, iter, TokenKind::Export, "export");

        let item = Node::parse(file, iter, Some(vec![
            TokenKind::Func,
            TokenKind::Static,
            TokenKind::Interface,
            TokenKind::Struct
        ]));

        ExportStmt {
            item: Box::from(item)
        }
    }
}