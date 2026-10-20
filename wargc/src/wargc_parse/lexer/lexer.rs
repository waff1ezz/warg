use std::iter::Peekable;
use std::path::PathBuf;
use std::str::CharIndices;
use crate::wargc_parse::lexer::{Token, TokenKind};
use crate::wargc_driver::File;
use crate::wargc_errors::error::Error;
use crate::wargc_errors::fatal::throw_fatal;
use crate::wargc_errors::parse::{ParseError, ParseErrorKind};

#[derive(Clone, Copy)]
enum ReadMode {
    Unset,
    ReadIdentifierOrKeyword,
    ReadNumberLiteral,
    ReadStringLiteral,
    ReadOperator,
}

struct ReadingState<'a> {
    iter: Peekable<CharIndices<'a>>,
    read_mode: ReadMode,
    line: usize,
    char: usize
}

impl File {
    pub fn generate_tokens<'a>(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        let mut state = ReadingState {
            iter: self.source.char_indices().peekable(),
            read_mode: ReadMode::Unset,
            line: 1,
            char: 1,
        };

        while let Some((idx, this_char)) = state.iter.next() {
            match this_char {
                c if c.is_alphabetic() => Self::parse_keyword_or_identifier(
                    &self.source, &mut tokens, &mut state, idx
                ),

                c if c.is_ascii_digit() => Self::parse_number_literal(
                    &self.source, &mut tokens, self.path.clone(), &mut state, idx
                ),

                '\"' => Self::parse_string_literal(
                    &self.source, &mut tokens, self.path.clone(), &mut state, idx
                ),

                c if c.is_ascii_punctuation() => Self::parse_operator(
                    &self.source, &mut tokens, &mut state, idx
                ),

                '\n' => {
                    state.char = 1;
                    state.line += 1;
                }

                ' ' => state.char += 1,

                _ => {}
            }
        }

        let unknown_token = tokens
            .iter()
            .find(|t| matches!(t.kind, TokenKind::UnknownToken))
            .map(|t| {
                ParseError {
                    src: &self.source,
                    file_path: self.path.clone(),
                    line: t.line,
                    char: t.char,
                    length: t.original_slice(&self.source).chars().count(),
                    kind: ParseErrorKind::UnknownToken(t),
                }
            });

        if let Some(unknown_token) = unknown_token {
            unknown_token.throw();
        }

        tokens
    }

    fn push_token(
        source: &str,
        tokens: &mut Vec<Token>,
        state: &mut ReadingState,
        buf_start: usize,
        buf_end: usize,
        str: Option<String>
    ) {
        let span = buf_start..buf_end;
        let token_chars = &source[buf_start..buf_end];
        let token = match state.read_mode {
            ReadMode::ReadIdentifierOrKeyword => match token_chars {
                "true" => Token::new(TokenKind::True, state.line, state.char, span),
                "false" => Token::new(TokenKind::False, state.line, state.char, span),
                _ => Token::new(TokenKind::Identifier, state.line, state.char, span),
            }

            ReadMode::ReadOperator => match token_chars {
                "+" => Token::new(TokenKind::Plus, state.line, state.char, span),
                "-" => Token::new(TokenKind::Minus, state.line, state.char, span),
                "*" => Token::new(TokenKind::Star, state.line, state.char, span),
                "/" => Token::new(TokenKind::Slash, state.line, state.char, span),
                "=" => Token::new(TokenKind::Equals, state.line, state.char, span),
                "==" => Token::new(TokenKind::DoubleEquals, state.line, state.char, span),
                "!=" => Token::new(TokenKind::NotEquals, state.line, state.char, span),
                "<" => Token::new(TokenKind::LessThan, state.line, state.char, span),
                ">" => Token::new(TokenKind::GreaterThan, state.line, state.char, span),
                "<=" => Token::new(TokenKind::LessThanEquals, state.line, state.char, span),
                ">=" => Token::new(TokenKind::GreaterThanEquals, state.line, state.char, span),
                "(" => Token::new(TokenKind::LeftParentheses, state.line, state.char, span),
                ")" => Token::new(TokenKind::RightParentheses, state.line, state.char, span),
                ";" => Token::new(TokenKind::Semicolon, state.line, state.char, span),
                _ => Token::new(TokenKind::UnknownToken, state.line, state.char, span)
            }

            ReadMode::ReadNumberLiteral => {
                Token::new(TokenKind::Number, state.line, state.char, span)
            }

            ReadMode::ReadStringLiteral => {
                let length = token_chars.chars().count();
                match str {
                    Some(str) => Token::new(TokenKind::String(str), state.line, state.char - length, span),
                    None => throw_fatal("an attempt to push string literal without passing 'str' argument")
                }
            }

            ReadMode::Unset => {
                Token::new(TokenKind::UnknownToken, state.line, state.char, span)
            }
        };

        tokens.push(token);
    }

    fn parse_keyword_or_identifier<'a>(
        source: &str,
        tokens: &mut Vec<Token>,
        state: &mut ReadingState,
        buf_start: usize
    ) {
        let mut buf_end = buf_start + 1;
        state.read_mode = ReadMode::ReadIdentifierOrKeyword;

        while let Some(&(_, next)) = state.iter.peek() {
            state.char += 1;

            if !next.is_alphanumeric() {
                break;
            }

            buf_end += 1;
            state.iter.next();
        }

        Self::push_token(source, tokens, state, buf_start, buf_end, None);
    }

    fn parse_string_literal<'a>(
        source: &str,
        tokens: &mut Vec<Token>,
        file_path: PathBuf,
        state: &mut ReadingState,
        buf_start: usize
    ) {
        let mut buf_end = buf_start + 1;
        state.read_mode = ReadMode::ReadStringLiteral;

        while let Some(&(_, next)) = state.iter.peek() {
            state.char += 1;

            if next == '"' && source.chars().nth(buf_end - 1).unwrap() != '\\' {
                state.iter.next();
                break;
            }

            buf_end += 1;
            state.iter.next();
        }

        let str_slice = source[buf_start + 1..buf_end].to_string();
        let mut str_iter = str_slice
            .chars()
            .enumerate()
            .peekable();

        let mut escaped_str = String::new();
        while let Some((_idx, char)) = str_iter.next() {
            if char == '\\' {
                let next = str_iter.peek().map(|(_, char)| char);
                match next {
                    Some('n') => escaped_str.push('\n'),
                    Some('r') => escaped_str.push('\r'),
                    Some('t') => escaped_str.push('\t'),
                    Some('\\') => escaped_str.push('\\'),
                    Some('"') => escaped_str.push('"'),
                    _ => {
                        let err = ParseError {
                            src: &source,
                            file_path: file_path.clone(),
                            line: state.line,
                            char: state.char,
                            length: 2,
                            kind: ParseErrorKind::UnknownCharacterEscape(*next.unwrap_or(&' ')),
                        };

                        err.throw();
                    }
                }

                str_iter.next();
                continue;
            }

            escaped_str.push(char);
        }

        Self::push_token(source, tokens, state, buf_start, buf_end, Some(escaped_str));
    }

    fn parse_number_literal<'a>(
        source: &str,
        tokens: &mut Vec<Token>,
        file_path: PathBuf,
        state: &mut ReadingState,
        buf_start: usize
    ) {
        let mut buf_end = buf_start + 1;
        let mut dots = 0;

        state.read_mode = ReadMode::ReadNumberLiteral;

        while let Some(&(idx, next)) = state.iter.peek() {
            let this = source.chars().nth(idx - 1).unwrap();
            state.char += 1;

            if this == '.' {
                dots += 1;

                if dots > 1 || !next.is_ascii_digit() {
                    let err = ParseError {
                        src: source,
                        file_path: file_path.clone(),
                        line: state.line,
                        char: state.char,
                        length: 1,
                        kind: ParseErrorKind::NumberLiteralNotParsable,
                    };

                    err.throw();
                }
            }

            if !next.is_ascii_digit() && next != '.' {
                break;
            }

            buf_end += 1;
            state.iter.next();
        }

        Self::push_token(source, tokens, state, buf_start, buf_end, None);
    }

    fn parse_operator<'a>(
        source: &str,
        tokens: &mut Vec<Token>,
        state: &mut ReadingState,
        buf_start: usize
    ) {
        let mut buf_end = buf_start + 1;
        state.read_mode = ReadMode::ReadOperator;

        let double_char_operators = [
            "==",
            "!=",
            "<=",
            ">=",
            "=>"
        ];

        while let Some(&(_, _next)) = state.iter.peek() {
            state.char += 1;

            if !double_char_operators.contains(&&source[buf_start..buf_end + 1]) {
                break;
            }

            buf_end += 1;
            state.iter.next();
        }

        Self::push_token(source, tokens, state, buf_start, buf_end, None);
    }
}