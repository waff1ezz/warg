use std::iter::{Enumerate, Peekable};
use std::ops::Range;
use crate::wargc_util::Position;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: Position,
    pub span: Range<usize>,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, char: usize, span: Range<usize>) -> Self {
        Token {
            pos: Position { line, char },
            kind,
            span
        }
    }

    pub fn original_slice<'src>(&self, source: &'src str) -> &'src str {
        &source[self.span.clone()]
    }
}

#[derive(Debug)]
pub enum TokenKind {
    Number,
    Identifier,
    String(String),

    True,
    False,

    Plus,
    Minus,
    Star,
    Slash,
    Equals,
    DoubleEquals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanEquals,
    GreaterThanEquals,
    LeftParentheses,
    RightParentheses,
    Semicolon,

    UnknownToken
}