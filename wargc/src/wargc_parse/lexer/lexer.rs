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
            line: 0,
            char: 0,
        };

        while let Some((char_byte_idx, this_char)) = state.iter.next() {
            match this_char {
                c if c.is_alphabetic() || c == '_' || c == '$' => Self::parse_keyword_or_identifier(
                    &self.source, &mut tokens, &mut state, char_byte_idx, this_char
                ),

                c if c.is_ascii_digit() => Self::parse_number_literal(
                    &self.source, &mut tokens, self.path.clone(), &mut state, char_byte_idx
                ),

                '\"' => Self::parse_string_literal(
                    &self.source, &mut tokens, self.path.clone(), &mut state, char_byte_idx
                ),

                c if c.is_ascii_punctuation() => Self::parse_operator(
                    &self.source, &mut tokens, &mut state, char_byte_idx
                ),

                '\n' => {
                    state.char = 0;
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
                    line: t.pos.line,
                    char: t.pos.char,
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
        first_char_byte_idx: usize,
        last_char_byte_idx: usize,
        first_char_pos: usize,
        str: Option<String>
    ) {
        let span = first_char_byte_idx..last_char_byte_idx;
        let token_chars = &source[first_char_byte_idx..last_char_byte_idx];
        let token = match state.read_mode {
            ReadMode::ReadIdentifierOrKeyword => match token_chars {
                "fn" => Token::new(TokenKind::Fn, state.line, first_char_pos, span),
                "let" => Token::new(TokenKind::Let, state.line, first_char_pos, span),
                "const" => Token::new(TokenKind::Const, state.line, first_char_pos, span),
                "true" => Token::new(TokenKind::True, state.line, first_char_pos, span),
                "false" => Token::new(TokenKind::False, state.line, first_char_pos, span),
                _ => Token::new(TokenKind::Identifier, state.line, first_char_pos, span),
            }

            ReadMode::ReadOperator => match token_chars {
                "," => Token::new(TokenKind::Comma, state.line, first_char_pos, span),
                "." => Token::new(TokenKind::Dot, state.line, first_char_pos, span),
                "+" => Token::new(TokenKind::Plus, state.line, first_char_pos, span),
                "-" => Token::new(TokenKind::Minus, state.line, first_char_pos, span),
                "*" => Token::new(TokenKind::Star, state.line, first_char_pos, span),
                "/" => Token::new(TokenKind::Slash, state.line, first_char_pos, span),
                "=" => Token::new(TokenKind::Equals, state.line, first_char_pos, span),
                "==" => Token::new(TokenKind::DoubleEquals, state.line, first_char_pos, span),
                "!=" => Token::new(TokenKind::NotEquals, state.line, first_char_pos, span),
                "<" => Token::new(TokenKind::LessThan, state.line, first_char_pos, span),
                ">" => Token::new(TokenKind::GreaterThan, state.line, first_char_pos, span),
                "<=" => Token::new(TokenKind::LessThanEquals, state.line, first_char_pos, span),
                ">=" => Token::new(TokenKind::GreaterThanEquals, state.line, first_char_pos, span),
                "(" => Token::new(TokenKind::LeftParentheses, state.line, first_char_pos, span),
                ")" => Token::new(TokenKind::RightParentheses, state.line, first_char_pos, span),
                "[" => Token::new(TokenKind::LeftBracket, state.line, first_char_pos, span),
                "]" => Token::new(TokenKind::RightBracket, state.line, first_char_pos, span),
                "{" => Token::new(TokenKind::LeftBrace, state.line, first_char_pos, span),
                "}" => Token::new(TokenKind::RightBrace, state.line, first_char_pos, span),
                ";" => Token::new(TokenKind::Semicolon, state.line, first_char_pos, span),
                ":" => Token::new(TokenKind::Colon, state.line, first_char_pos, span),
                "!" => Token::new(TokenKind::Bang, state.line, first_char_pos, span),
                "&" => Token::new(TokenKind::Ampersand, state.line, first_char_pos, span),
                "&&" => Token::new(TokenKind::DoubleAmpersand, state.line, first_char_pos, span),
                "||" => Token::new(TokenKind::DoublePipe, state.line, first_char_pos, span),
                _ => Token::new(TokenKind::UnknownToken, state.line, first_char_pos, span)
            }

            ReadMode::ReadNumberLiteral => {
                Token::new(TokenKind::Number, state.line, first_char_pos, span)
            }

            ReadMode::ReadStringLiteral => {
                let length = token_chars.chars().count();
                match str {
                    Some(str) => Token::new(TokenKind::String(str), state.line, first_char_pos - length, span),
                    None => throw_fatal("an attempt to push string literal without passing 'str' argument")
                }
            }

            ReadMode::Unset => {
                Token::new(TokenKind::UnknownToken, state.line, first_char_pos, span)
            }
        };

        tokens.push(token);
    }

    fn parse_keyword_or_identifier(
        source: &str,
        tokens: &mut Vec<Token>,
        state: &mut ReadingState,
        first_char_byte_idx: usize,
        first_char: char
    ) {
        let mut last_char_byte_idx = first_char_byte_idx + first_char.len_utf8();
        let first_char_pos = state.char;

        state.read_mode = ReadMode::ReadIdentifierOrKeyword;
        state.char += 1;

        while let Some(&(_, char)) = state.iter.peek() {
            if !char.is_alphanumeric() && char != '_' && char != '$' {
                break;
            }

            state.char += 1;
            last_char_byte_idx += state.iter
                .next()
                .unwrap()
                .1.len_utf8();
        }

        Self::push_token(
            source,
            tokens,
            state,
            first_char_byte_idx,
            last_char_byte_idx,
            first_char_pos,
            None
        );
    }

    fn parse_string_literal(
        source: &str,
        tokens: &mut Vec<Token>,
        file_path: PathBuf,
        state: &mut ReadingState,
        first_char_byte_idx: usize,
    ) {
        // we don't use first_char because it's always " with byte length 1

        let mut last_char_byte_idx = first_char_byte_idx + 1;
        let first_char_pos = state.char;

        state.read_mode = ReadMode::ReadStringLiteral;
        state.char += 1;

        let mut last_consumed = '"';
        while let Some((_, char)) = state.iter.next() {
            if char == '"' && last_consumed != '\\' {
                break;
            }

            last_char_byte_idx += char.len_utf8();
            state.char += 1;

            last_consumed = char;
        }

        let str_slice = source[(first_char_byte_idx + 1)..last_char_byte_idx]
            .to_string();

        let mut str_iter = str_slice
            .chars()
            .enumerate()
            .peekable();

        let mut escaped_str = String::new();
        while let Some((_, char)) = str_iter.next() {
            if char == '\\' {
                let next_char = str_iter.peek().map(|(_, char)| char);
                match next_char {
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
                            kind: ParseErrorKind::UnknownCharacterEscape(
                                *next_char.unwrap_or(&' ')
                            ),
                        };

                        err.throw();
                    }
                }

                str_iter.next().unwrap();
                continue;
            }

            escaped_str.push(char);
        }

        Self::push_token(
            source,
            tokens,
            state,
            first_char_byte_idx,
            last_char_byte_idx,
            first_char_pos,
            Some(escaped_str)
        );
    }

    fn parse_number_literal(
        source: &str,
        tokens: &mut Vec<Token>,
        file_path: PathBuf,
        state: &mut ReadingState,
        first_char_byte_idx: usize,
    ) {
        // we don't use first_char because numbers in utf are always 1 byte so we don't need a variable to check the length

        let mut last_char_byte_idx = first_char_byte_idx + 1;
        let mut dots = 0;
        let first_char_pos = state.char;

        state.read_mode = ReadMode::ReadNumberLiteral;
        state.char += 1;

        while let Some(&(idx, next)) = state.iter.peek() {
            let this = source.chars().nth(idx - 1).unwrap();

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

            state.char += 1;
            last_char_byte_idx += 1;
            state.iter.next().unwrap();
        }

        Self::push_token(
            source,
            tokens,
            state,
            first_char_byte_idx,
            last_char_byte_idx,
            first_char_pos,
            None
        );
    }

    fn parse_operator(
        source: &str,
        tokens: &mut Vec<Token>,
        state: &mut ReadingState,
        first_char_byte_idx: usize
    ) {
        let mut last_char_byte_idx = first_char_byte_idx + 1;
        let first_char_pos = state.char;

        state.read_mode = ReadMode::ReadOperator;
        state.char += 1;

        let double_char_operators = [
            "==",
            "!=",
            "<=",
            ">=",
            "=>",
            "&&",
            "||"
        ];

        while let Some(&(_, _next)) = state.iter.peek() {
            if !double_char_operators.contains(&&source[first_char_byte_idx..last_char_byte_idx + 1]) {
                break;
            }

            state.char += 1;
            last_char_byte_idx += 1;
            state.iter.next().unwrap();
        }

        Self::push_token(
            source,
            tokens,
            state,
            first_char_byte_idx,
            last_char_byte_idx,
            first_char_pos,
            None
        );
    }
}