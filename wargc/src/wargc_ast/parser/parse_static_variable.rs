use std::iter::Peekable;
use std::vec::IntoIter;
use crate::wargc_ast::lexer::{Token, TokenKind};
use crate::wargc_ast::node::{Expr, StaticVarDecl};
use crate::wargc_driver::File;
use crate::wargc_util::expect_token;

impl StaticVarDecl {
    pub fn parse(file: &File, iter: &mut Peekable<IntoIter<Token>>) -> StaticVarDecl {
        expect_token(file, iter, TokenKind::Static, "static");

        let name: Token;
        let data_type: Option<Expr>;
        let value: Expr;

        name = expect_token(file, iter, TokenKind::Identifier, "identifier");

        if let Some(next_token) = iter.peek() && matches!(next_token.kind, TokenKind::Colon) {
            expect_token(file, iter, TokenKind::Colon, ":");
            data_type = Some(
                // min_pr set to 2 so we don't parse '='
                Expr::parse(file, iter, 2, None)
            );
        } else {
            data_type = None;
        }

        expect_token(file, iter, TokenKind::Equals, "=");
        value =  Expr::parse(file, iter, 1, None);

        expect_token(file, iter, TokenKind::Semicolon, ";");

        StaticVarDecl {
            name,
            data_type,
            value
        }
    }
}