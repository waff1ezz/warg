use std::iter::Peekable;
use std::vec::IntoIter;
use crate::wargc_driver::File;
use crate::wargc_errors::error::Error;
use crate::wargc_parse::lexer::{Token, TokenKind};
use crate::wargc_errors::parse::{ParseError, ParseErrorKind};
use crate::wargc_util::Position;

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
    },

    FunctionCall {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },

    Index {
        base: Box<Expr>,
        index: Box<Expr>,
    },

    Property {
        base: Box<Expr>,
        prop_name: Box<Expr>
    }
}

enum OpType<'a> {
    Prefix(&'a Token),
    Infix(&'a Token)
}

impl Expr {
    pub fn parse(file: &File, iter: &mut Peekable<IntoIter<Token>>, min_pr: usize, left: Option<Expr>) -> Expr {
        /*
        *   Explanation for future me:
        *
        *   We unformtunately can't solve this in a normal way (put postfix parsing inside the loop)
        *   because then rust screams about borrowing a mutable reference while another is still in use
        *   (token is referenced, not owned, because we use 'peek')
        *   We can't use 'next' to claim the ownership because that basically breaks the whole parser and would make it
        *   irrationally more complicated
        */

        let mut left: Expr = match left {
            Some(expr) => expr,
            None => Self::parse_left_operand(file, iter)
        };

        if let Some(token) = iter.peek() {
            match token.kind {
                TokenKind::LeftParentheses => {
                    left = Self::parse_fn_call(file, iter, left);
                    return Self::parse(file, iter, min_pr, Some(left))
                }

                TokenKind::LeftBracket => {
                    left = Self::parse_index(file, iter, left);
                    return Self::parse(file, iter, min_pr, Some(left))
                }

                TokenKind::Dot => {
                    left = Self::parse_property_access(file, iter, left);
                    return Self::parse(file, iter, min_pr, Some(left))
                }

                _ => {}
            };
        }

        while let Some(token) = iter.peek() && !matches!(token.kind, TokenKind::Semicolon | TokenKind::Comma) {
            if matches!(token.kind, TokenKind::RightParentheses | TokenKind::RightBracket) {
                return left;
            }

            let op_pr = Self::precedence(OpType::Infix(&token), file);

            if op_pr >= min_pr {
                // unwrap because we're sure it's Some (otherwise the program wouldn't enter this iteration)
                let owned_token = iter.next().unwrap();

                left = Expr::Binary {
                    left: Box::from(left),
                    right: Box::from(Self::parse(file, iter, op_pr + 1, None)),
                    operator: owned_token
                }
            }

            else {
                return left;
            }
        }

        left
    }

    fn parse_fn_call(file: &File, iter: &mut Peekable<IntoIter<Token>>, callee: Expr) -> Expr {
        let mut args: Vec<Expr> = vec![];

        iter.next();

        loop {
            if let Some(next) = iter.peek() && matches!(next.kind, TokenKind::RightParentheses) {
                iter.next();
                break;
            }

            args.push(
                Self::parse(file, iter, 1, None)
            );

            match iter.next() {
                Some(trailing_token) => {
                    match trailing_token.kind {
                        TokenKind::RightParentheses => {
                            return Expr::FunctionCall {
                                callee: Box::new(callee),
                                args,
                            }
                        }

                        TokenKind::Comma => {}

                        _ => {
                            let err = ParseError {
                                src: &file.source,
                                file_path: file.path.clone(),
                                line: trailing_token.pos.line,
                                char: trailing_token.pos.char,
                                length: trailing_token.original_slice(&file.source).chars().count(),
                                kind: ParseErrorKind::UnexpectedToken(&trailing_token),
                            };

                            err.throw();
                        }
                    }
                }

                None => {
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
            }
        }

        Expr::FunctionCall {
            callee: Box::new(callee),
            args,
        }
    }

    fn parse_property_access(file: &File, iter: &mut Peekable<IntoIter<Token>>, base: Expr) -> Expr {
        let dot = iter.next();

        Expr::Property {
            base: Box::new(base),
            prop_name: Box::new(
                Self::parse(file, iter, Self::precedence(OpType::Infix(&dot.unwrap()), file), None)
            )
        }
    }

    fn parse_index(file: &File, iter: &mut Peekable<IntoIter<Token>>, base: Expr) -> Expr {
        iter.next();

        let index = Self::parse(file, iter, 1, None);

        match iter.next() {
            Some(trailing_token) => {
                match trailing_token.kind {
                    TokenKind::RightBracket => {
                        Expr::Index {
                            base: Box::new(base),
                            index: Box::new(index),
                        }
                    }

                    _ => {
                        let err = ParseError {
                            src: &file.source,
                            file_path: file.path.clone(),
                            line: trailing_token.pos.line,
                            char: trailing_token.pos.char,
                            length: trailing_token.original_slice(&file.source).chars().count(),
                            kind: ParseErrorKind::UnexpectedToken(&trailing_token),
                        };

                        err.throw();
                    }
                }
            }

            None => {
                let pos = file.eof_pos();
                let err = ParseError {
                    src: &file.source,
                    file_path: file.path.clone(),
                    line: pos.line,
                    char: pos.char,
                    length: 1,
                    kind: ParseErrorKind::ExpectedToken("]"),
                };

                err.throw();
            }
        }
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
                kind: ParseErrorKind::IncompleteExpression,
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

            TokenKind::Minus |
            TokenKind::Bang |
            TokenKind::Ampersand |
            TokenKind::Star => {
                let pr = Self::precedence(OpType::Prefix(&token), file);

                Expr::Unary {
                    operand: Box::from(Self::parse(file, iter, pr, None)),
                    operator: token
                }
            }

            TokenKind::LeftParentheses => {
                let expr = Self::parse(file, iter, 1, None);

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

    fn precedence(operator: OpType, file: &File) -> usize {
        match operator {
            OpType::Prefix(op) => match op.kind {
                TokenKind::Minus => 10,
                TokenKind::Bang => 10,
                TokenKind::Ampersand => 10,
                TokenKind::Star => 10,
                _ => unreachable!(),
            }

            OpType::Infix(op) => match op.kind {
                TokenKind::DoubleAmpersand => 1,
                TokenKind::DoublePipe => 1,
                TokenKind::Equals => 2,
                TokenKind::DoubleEquals => 3,
                TokenKind::NotEquals => 3,
                TokenKind::LessThan => 3,
                TokenKind::GreaterThan => 3,
                TokenKind::LessThanEquals => 3,
                TokenKind::GreaterThanEquals => 3,
                TokenKind::Plus => 4,
                TokenKind::Minus => 4,
                TokenKind::Star => 5,
                TokenKind::Slash => 5,
                TokenKind::Dot => 10,

                TokenKind::Identifier |
                TokenKind::Number |
                TokenKind::String(_) => {
                    let err = ParseError {
                        src: &file.source,
                        file_path: file.path.clone(),
                        line: op.pos.line,
                        char: op.pos.char,
                        length: op.original_slice(&file.source).chars().count(),
                        kind: ParseErrorKind::ExpectedOperator,
                    };

                    err.throw();
                }

                _ => {
                    let err = ParseError {
                        src: &file.source,
                        file_path: file.path.clone(),
                        line: op.pos.line,
                        char: op.pos.char,
                        length: op.original_slice(&file.source).chars().count(),
                        kind: ParseErrorKind::UnexpectedToken(&op),
                    };

                    err.throw();
                }
            }
        }
    }
}