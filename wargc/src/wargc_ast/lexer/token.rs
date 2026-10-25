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

    Func,
    Let,
    Const,
    If,
    Else,
    For,
    While,
    Import,
    Export,
    From,
    Return,
    Struct,
    Interface,
    Implements,
    In,
    New,

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
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Semicolon,
    Comma,
    Colon,
    Bang,
    Dot,
    Pipe,
    Ampersand,
    Percent,
    DoubleAmpersand,
    DoublePipe,
    DoubleGreaterThan,
    DoubleLessThan,
    Caret,
    Tilde,
    PlusEquals,
    MinusEquals,
    StarEquals,
    SlashEquals,
    AmpersandEquals,
    PipeEquals,
    PercentEquals,
    CaretEquals,
    TildeEquals,

    UnknownToken
}