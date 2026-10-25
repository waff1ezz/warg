use crate::wargc_driver::File;
use crate::wargc_errors::error::Error;
use crate::wargc_errors::parse::{ParseError, ParseErrorKind};
use crate::wargc_util::expect_token;
use std::iter::Peekable;
use std::vec::IntoIter;
use crate::wargc_ast::lexer::{Token, TokenKind};
use crate::wargc_ast::node::{BlockStmt, Expr, IfStmt};

impl IfStmt {
    pub fn parse(file: &File, iter: &mut Peekable<IntoIter<Token>>) -> IfStmt {
        expect_token(file, iter, TokenKind::If, "if");

        let condition: Expr;
        let then_body: BlockStmt;
        let mut else_body: Option<BlockStmt> = None;

        expect_token(file, iter, TokenKind::LeftParentheses, "(");

        condition = Expr::parse(file, iter, 1, None);

        expect_token(file, iter, TokenKind::RightParentheses, ")");

        then_body = BlockStmt::parse(file, iter, false);

        if let Some(token) = iter.peek() && matches!(token.kind, TokenKind::Else) {
            expect_token(file, iter, TokenKind::Else, "else");

            match iter.peek() {
                Some(next_token) if matches!(next_token.kind, TokenKind::If) => {
                    else_body = Some(
                        BlockStmt::parse(file, iter, true)
                    );
                }

                Some(next_token) if matches!(next_token.kind, TokenKind::LeftBrace) => {
                    else_body = Some(
                        BlockStmt::parse(file, iter, false)
                    );
                }

                _ => {
                    let next_token = iter.peek();

                    let pos = match next_token {
                        Some(t) => &t.pos,
                        None => &file.eof_pos()
                    };

                    let len = match next_token {
                        Some(t) => t.original_slice(&file.source).chars().count(),
                        None => 1,
                    };

                    let err = ParseError {
                        src: &file.source,
                        file_path: file.path.clone(),
                        line: pos.line,
                        char: pos.char,
                        length: len,
                        kind: ParseErrorKind::ExpectedToken("if' of 'block statement"),
                    };

                    err.throw();
                }
            }
        }

        IfStmt {
            condition,
            then_body,
            else_body
        }
    }
}