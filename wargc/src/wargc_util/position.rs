use std::iter::Peekable;
use std::ops::Range;
use std::vec::IntoIter;
use crate::wargc_ast::lexer::Token;
use crate::wargc_driver::File;

#[derive(Debug, Clone)]
pub struct Position {
    pub char: usize,
    pub line: usize
}

pub fn where_am_i(file: &File, iter: &mut Peekable<IntoIter<Token>>) -> Position {
    let pos = match iter.peek() {
        None => Position {
            char: 0,
            line: 0
        },

        Some(t) => t.pos.clone()
    };

    pos
}