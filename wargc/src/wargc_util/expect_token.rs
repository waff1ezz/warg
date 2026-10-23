use std::iter::Peekable;
use std::vec::IntoIter;
use crate::wargc_driver::File;
use crate::wargc_errors::error::Error;
use crate::wargc_errors::parse::{ParseError, ParseErrorKind};
use crate::wargc_parse::lexer::{Token, TokenKind};

pub fn expect_token(file: &File, iter: &mut Peekable<IntoIter<Token>>, expected: TokenKind, err_msg: &str) -> Token {
    match iter.next() {
        Some(token) => {
            if std::mem::discriminant(&token.kind) == std::mem::discriminant(&expected) {
                return token;
            }

            else {
                throw_expected_token(file, Some(token), err_msg);
            }
        }

        None => {
            throw_expected_token(file, None, err_msg);
        }
    }

    fn throw_expected_token(file: &File, token: Option<Token>, msg: &str) -> ! {
        let pos = match &token {
            Some(t) => t.pos.clone(),
            None => file.eof_pos()
        };

        let len = match token {
            Some(t) => t.original_slice(&file.source).chars().count(),
            None => 1
        };

        let err = ParseError {
            src: &file.source,
            file_path: file.path.clone(),
            line: pos.line,
            char: pos.char,
            length: len,
            kind: ParseErrorKind::ExpectedToken(msg),
        };

        err.throw();
    }
}
