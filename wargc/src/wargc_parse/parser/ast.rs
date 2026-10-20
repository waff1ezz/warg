use std::iter::Peekable;
use std::vec::IntoIter;
use crate::wargc_driver::File;
use crate::wargc_parse::lexer::Token;
use crate::wargc_parse::parser::Expr;

impl File {
    pub fn ast(&self, tokens: Vec<Token>) {
        let mut iter = tokens
            .into_iter()
            .peekable();

        let expr = Expr::parse(self, &mut iter, 1);

        dbg!(expr);
    }
}