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
        let token = expect_token(file, iter, TokenKind::Export, "export");

        let item = Node::parse(file, iter);
        match item {
            Node::Function(_) |
            Node::StaticVariable(_) => {
                ExportStmt {
                    item: Box::from(item)
                }
            }

            _ => {
                let err = ParseError {
                    src: &file.source,
                    file_path: file.path.clone(),
                    line: token.pos.line,
                    char: token.pos.char,
                    length: token.original_slice(&file.source).chars().count(),
                    kind: ParseErrorKind::ExpectedToken(""), // todo
                };

                err.throw();
            }
        }
    }
}