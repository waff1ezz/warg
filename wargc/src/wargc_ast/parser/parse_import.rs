use std::iter::Peekable;
use std::vec::IntoIter;
use crate::wargc_ast::lexer::{Token, TokenKind};
use crate::wargc_ast::node::ImportStmt;
use crate::wargc_driver::File;
use crate::wargc_util::expect_token;

impl ImportStmt {
    pub fn parse(file: &File, iter: &mut Peekable<IntoIter<Token>>) -> ImportStmt {
        expect_token(file, iter, TokenKind::Import, "import");

        let mut members: Vec<Token> = vec![];
        let mut module: Vec<Token> = vec![];

        while let Some(_) = iter.peek() {
            members.push(
                expect_token(file, iter, TokenKind::Identifier, "identifier")
            );

            if let Some(next_token) = iter.peek() && !matches!(next_token.kind, TokenKind::Comma) {
                break;
            }

            else {
                expect_token(file, iter, TokenKind::Comma, "comma");
            }
        }

        expect_token(file, iter, TokenKind::From, "from");

        while let Some(_) = iter.peek() {
            module.push(
                expect_token(file, iter, TokenKind::Identifier, "identifier")
            );

            if let Some(next_token) = iter.peek() && !matches!(next_token.kind, TokenKind::Dot) {
                break;
            }

            else {
                expect_token(file, iter, TokenKind::Dot, ".");
            }
        }

        expect_token(file, iter, TokenKind::Semicolon, ";");

        ImportStmt {
            members,
            module
        }
    }
}