use std::iter::Peekable;
use std::vec::IntoIter;
use crate::wargc_ast::lexer::{Token, TokenKind};
use crate::wargc_ast::node::{ExportStmt, Node};
use crate::wargc_driver::File;
use crate::wargc_errors::error::Error;
use crate::wargc_errors::parse::{ParseError, ParseErrorKind};
use crate::wargc_util::expect_token;

impl ExportStmt {
    pub fn parse(file: &File, iter: &mut Peekable<IntoIter<Token>>) -> ExportStmt {
        expect_token(file, iter, TokenKind::Export, "return");

        let item = Node::parse(file, iter);
        match item {
            Node::Expression(_) => {}
            Node::Block(_) => {}
            Node::For(_) => {}
            Node::Function(_) => {}
            Node::If(_) => {}
            Node::Import(_) => {}
            Node::Export(_) => {}
            Node::Return(_) => {}
            Node::Variable(_) => {}
        }

        ExportStmt {
            item: Box::from(item)
        }
    }
}