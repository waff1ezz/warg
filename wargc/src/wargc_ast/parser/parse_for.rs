use crate::wargc_driver::File;
use crate::wargc_ast::node::{BlockStmt, Expr, ForStmt, VarDecl};
use crate::wargc_util::expect_token;
use std::iter::Peekable;
use std::vec::IntoIter;
use crate::wargc_ast::lexer::{Token, TokenKind};
use crate::wargc_errors::error::Error;
use crate::wargc_errors::parse::{ParseError, ParseErrorKind};

impl ForStmt {
    pub fn parse(file: &File, iter: &mut Peekable<IntoIter<Token>>) -> ForStmt {
        let var: VarDecl;
        let range: Expr;
        let body: BlockStmt;

        expect_token(file, iter, TokenKind::For, "for");
        let paren = expect_token(file, iter, TokenKind::LeftParentheses, "(");

        match iter.peek() {
            None => {
                let err = ParseError {
                    src: &file.source,
                    file_path: file.path.clone(),
                    line: paren.pos.line,
                    char: paren.pos.char + 1,
                    length: 1,
                    kind: ParseErrorKind::ExpectedToken("let"),
                };

                err.throw();
            }

            Some(t) => {
                if !matches!(t.kind, TokenKind::Let) {
                    let err = ParseError {
                        src: &file.source,
                        file_path: file.path.clone(),
                        line: t.pos.line,
                        char: t.pos.char,
                        length: t.original_slice(&file.source).chars().count(),
                        kind: ParseErrorKind::ExpectedToken("let"),
                    };

                    err.throw();
                }
            }
        }

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