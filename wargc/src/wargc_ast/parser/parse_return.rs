use std::iter::Peekable;
use std::vec::IntoIter;
use crate::wargc_ast::lexer::{Token, TokenKind};
use crate::wargc_ast::node::{Expr, ReturnStmt};
use crate::wargc_driver::File;
use crate::wargc_util::expect_token;

impl ReturnStmt {
    pub fn parse(file: &File, iter: &mut Peekable<IntoIter<Token>>) -> ReturnStmt {
        expect_token(file, iter, TokenKind::Return, "return");
        
        let value = Expr::parse(file, iter, 1, None);
        expect_token(file, iter, TokenKind::Semicolon, ";");
        
        ReturnStmt { value }
    }
}