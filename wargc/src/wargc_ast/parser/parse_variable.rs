use std::iter::Peekable;
use std::vec::IntoIter;
use crate::wargc_ast::lexer::{Token, TokenKind};
use crate::wargc_ast::node::{Expr, SymbolId, VarDecl};
use crate::wargc_driver::File;
use crate::wargc_util::{expect_token, inc_mutex, verify_symbol_name};
use crate::wargc_ast::make::{SYMBOL_COUNTER, BUILT_IN_SYMBOLS};
use crate::wargc_errors::error::Error;
use crate::wargc_errors::parse::{ParseError, ParseErrorKind};

impl VarDecl {
    pub fn parse(file: &File, iter: &mut Peekable<IntoIter<Token>>, omit_semicolon: bool) -> VarDecl {
        let mutable = match iter.next().unwrap().kind {
            TokenKind::Const => false,
            TokenKind::Let => true,
            _ => unreachable!()
        };

        let name: Token;
        let data_type: Option<Expr>;
        let value: Option<Expr>;

        name = expect_token(file, iter, TokenKind::Identifier, "identifier");
        verify_symbol_name(file, &name);

        if let Some(next_token) = iter.peek() && matches!(next_token.kind, TokenKind::Colon) {
            expect_token(file, iter, TokenKind::Colon, ":");
            data_type = Some(
                // min_pr set to 2 so we don't parse '='
                Expr::parse(file, iter, 2, None)
            );
        } else {
            data_type = None;
        }

        if let Some(next_token) = iter.peek() && matches!(next_token.kind, TokenKind::Equals) {
            expect_token(file, iter, TokenKind::Equals, "=");
            value = Some(
                Expr::parse(file, iter, 1, None)
            );
        } else {
            value = None;
        }

        if !omit_semicolon {
            expect_token(file, iter, TokenKind::Semicolon, ";");
        }


        VarDecl {
            name,
            data_type,
            value,
            mutable,
            symbol_id: SymbolId(
                inc_mutex(&SYMBOL_COUNTER)
            )
        }
    }
}