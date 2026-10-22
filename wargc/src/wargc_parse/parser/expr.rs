use std::iter::Peekable;
use std::vec::IntoIter;
use crate::wargc_driver::File;
use crate::wargc_errors::error::Error;
use crate::wargc_parse::lexer::{Token, TokenKind};
use crate::wargc_errors::parse::{ParseError, ParseErrorKind};
use crate::wargc_util::Position;

// todo!(check line validity)

#[derive(Debug)]
pub enum Expr {
    Identifier(Token),
    NumberLiteral(Token),
    StringLiteral(Token),

    Binary {
        left: Box<Expr>,
        right: Box<Expr>,
        operator: Token
    },

    Unary {
        operand: Box<Expr>,
        operator: Token
    }
}

enum OpType<'a> {
    Prefix(&'a TokenKind),
    Postfix(&'a TokenKind),
    Infix(&'a TokenKind)
}

enum ParserPosition {
    LeftOperand,
    RightOperand,
    InfixOperator,
}

impl Expr {
    pub fn parse(file: &File, iter: &mut Peekable<IntoIter<Token>>, min_pr: usize) -> Expr {
        let mut left: Expr = Self::parse_left_operand(file, iter);

        while let Some(token) = iter.peek() && !matches!(token.kind, TokenKind::Semicolon) {
            if matches!(token.kind, TokenKind::RightParentheses) {
                return left;
            }

            let op_pr = Self::precedence(OpType::Infix(&token.kind));

            if op_pr >= min_pr {
                // unwrap because we're sure it's Some (otherwise the program wouldn't enter this iteration)
                let owned_token = iter.next().unwrap();

                left = Expr::Binary {
                    left: Box::from(left),
                    right: Box::from(Self::parse(file, iter, op_pr + 1)),
                    operator: owned_token
                }
            }

            else {
                return left;
            }
        }

        left
    }

    fn parse_left_operand(file: &File, iter: &mut Peekable<IntoIter<Token>>) -> Expr {
        if let None = iter.peek() {
            let pos = file.eof_pos();

            let err = ParseError {
                src: &file.source,
                file_path: file.path.clone(),
                line: pos.line,
                char: pos.char,
                length: 1,
                kind: ParseErrorKind::ExpectedToken(")"),
            };

            err.throw();
        }

        let token = iter.next().unwrap();

        match token.kind {
            TokenKind::Number => {
                Expr::NumberLiteral(token)
            }

            TokenKind::String(..) => {
                Expr::StringLiteral(token)
            }

            TokenKind::Identifier => {
                Expr::Identifier(token)
            }

            TokenKind::Minus => {
                let pr = Self::precedence(OpType::Prefix(&token.kind));

                Expr::Unary {
                    operand: Box::from(Self::parse(file, iter, pr)),
                    operator: token
                }
            }

            TokenKind::LeftParentheses => {
                let expr = Self::parse(file, iter, 1);

                if let Some(t) = iter.peek() && matches!(t.kind, TokenKind::RightParentheses) {
                    iter.next();
                    expr
                } else {
                    let pos: Position = match iter.peek() {
                        Some(_) => {
                            token.pos
                        },

                        None => file.eof_pos()
                    };

                    let err = ParseError {
                        src: &file.source,
                        file_path: file.path.clone(),
                        line: pos.line,
                        char: pos.char,
                        length: 1,
                        kind: ParseErrorKind::ExpectedToken(")"),
                    };

                    err.throw();
                }
            }

            _ => {
                let err = ParseError {
                    src: &file.source,
                    file_path: file.path.clone(),
                    line: token.pos.line,
                    char: token.pos.char,
                    length: token.original_slice(&file.source).chars().count(),
                    kind: ParseErrorKind::UnexpectedToken(&token),
                };

                err.throw();
            }
        }
    }

    fn precedence(op_type: OpType) -> usize {
        match op_type {
            OpType::Prefix(op) => match op {
                TokenKind::Minus => 10,
                _ => unreachable!(),
            }

            OpType::Infix(op) => match op {
                TokenKind::Equals => 1,
                TokenKind::DoubleEquals => 2,
                TokenKind::NotEquals => 2,
                TokenKind::LessThan => 2,
                TokenKind::GreaterThan => 2,
                TokenKind::LessThanEquals => 2,
                TokenKind::GreaterThanEquals => 2,
                TokenKind::Plus => 3,
                TokenKind::Minus => 3,
                TokenKind::Star => 4,
                TokenKind::Slash => 4,
                _ => unreachable!(),
            }

            OpType::Postfix(op) => match op {
                TokenKind::LeftParentheses => 15,
                _ => unreachable!()
            }
        }
    }
}