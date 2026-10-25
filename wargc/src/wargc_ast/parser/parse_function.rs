use crate::wargc_driver::File;
use crate::wargc_util::expect_token;
use std::iter::Peekable;
use std::vec::IntoIter;
use crate::wargc_ast::lexer::{Token, TokenKind};
use crate::wargc_ast::node::{BlockStmt, Expr, FuncDecl, FuncParam};

impl FuncDecl {
    pub fn parse(file: &File, iter: &mut Peekable<IntoIter<Token>>) -> FuncDecl {
        expect_token(file, iter, TokenKind::Func, "func");

        let name: Token;
        let mut return_type: Option<Expr> = None;
        let mut args: Vec<FuncParam> = vec![];
        let body: BlockStmt;

        name = expect_token(file, iter, TokenKind::Identifier, "identifier");
        expect_token(file, iter, TokenKind::LeftParentheses, "(");

        while let Some(token) = iter.peek() && !matches!(token.kind, TokenKind::RightParentheses) {
            let arg_name = expect_token(file, iter, TokenKind::Identifier, "identifier");
            expect_token(file, iter, TokenKind::Colon, ":");
            let data_type = Expr::parse(file, iter, 1, None);

            args.push(FuncParam {
                name: arg_name,
                data_type
            });

            if let Some(next_token) = iter.peek() && !matches!(next_token.kind, TokenKind::RightParentheses) {
                expect_token(file, iter, TokenKind::Comma, ",");
            }
        }

        expect_token(file, iter, TokenKind::RightParentheses, ")");

        if let Some(next_token) = iter.peek() && matches!(next_token.kind, TokenKind::Colon) {
            expect_token(file, iter, TokenKind::Colon, ":");
            return_type = Some(
                Expr::parse(file, iter, 1, None)
            );
        }

        body = BlockStmt::parse(file, iter, false);

        FuncDecl {
            name,
            return_type,
            args,
            body,
        }
    }
}