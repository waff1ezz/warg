use std::iter::Peekable;
use std::vec::IntoIter;
use crate::wargc_driver::File;
use crate::wargc_parse::lexer::Token;
use crate::wargc_parse::parser::Expr;

impl File {
    pub fn gen_ast(&self, tokens: Vec<Token>) {
        let mut iter = tokens
            .into_iter()
            .peekable();

        while let Some(token) = iter.peek() {

        }
    }
}