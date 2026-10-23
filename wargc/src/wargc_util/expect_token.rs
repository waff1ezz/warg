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
                throw_expected_token(file, err_msg);
            }
        }

        None => {
            throw_expected_token(file, err_msg);
        }
    }

    fn throw_expected_token(file: &File, msg: &str) -> ! {
        let pos = file.eof_pos();
        let err = ParseError {
            src: &file.source,
            file_path: file.path.clone(),
            line: pos.line,
            char: pos.char,
            length: 1,
            kind: ParseErrorKind::ExpectedToken(msg),
        };

        err.throw();
    }
}
