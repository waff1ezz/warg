use crate::wargc_ast::lexer::Token;
use crate::wargc_ast::make::BUILT_IN_SYMBOLS;
use crate::wargc_driver::File;
use crate::wargc_errors::error::Error;
use crate::wargc_errors::parse::{ParseError, ParseErrorKind};

pub fn verify_symbol_name(file: &File, name: &Token) {
    if BUILT_IN_SYMBOLS.contains(&name.original_slice(&file.source)) {
        let err = ParseError {
            src: &file.source,
            file_path: file.path.clone(),
            line: name.pos.line,
            char: name.pos.char,
            length: name.original_slice(&file.source).chars().count(),
            kind: ParseErrorKind::ForbiddenName(name.original_slice(&file.source)),
        };

        err.throw();
    }
}