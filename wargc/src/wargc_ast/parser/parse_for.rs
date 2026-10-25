use crate::wargc_driver::File;
use crate::wargc_ast::node::{BlockStmt, Expr, ForStmt, VarDecl};
use crate::wargc_util::expect_token;
use std::iter::Peekable;
use std::vec::IntoIter;
use crate::wargc_ast::lexer::{Token, TokenKind};

impl ForStmt {
    pub fn parse(file: &File, iter: &mut Peekable<IntoIter<Token>>) -> ForStmt {
        let var: VarDecl;
        let range: Expr;
        let body: BlockStmt;

        expect_token(file, iter, TokenKind::For, "for");
        expect_token(file, iter, TokenKind::LeftParentheses, "(");

        var = VarDecl::parse(file, iter, true);

        expect_token(file, iter, TokenKind::In, "in");

        range = Expr::parse(file, iter, 1, None);

        expect_token(file, iter, TokenKind::RightParentheses, ")");

        body = BlockStmt::parse(file, iter, false);

        ForStmt {
            var,
            range,
            body
        }
    }
}