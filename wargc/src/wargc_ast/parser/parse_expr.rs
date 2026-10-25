use std::iter::Peekable;
use std::vec::IntoIter;
use crate::wargc_driver::File;
use crate::wargc_errors::error::Error;
use crate::wargc_ast::lexer::{Token, TokenKind};
use crate::wargc_ast::node::Expr;
use crate::wargc_errors::parse::{ParseError, ParseErrorKind};
use crate::wargc_util::Position;

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
                TokenKind::LeftBrace => {
                    return left;
                }

                TokenKind::LeftParentheses => {
                    left = Self::parse_fn_call(file, iter, left);
                    return Self::parse(file, iter, min_pr, Some(left));
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

        while let Some(token) = iter.peek() {
            match token.kind {
                TokenKind::RightParentheses |
                TokenKind::RightBracket |
                TokenKind::Semicolon |
                TokenKind::Comma => {
                    return left;
                }

                _ => {}
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

        iter.next(); // function name

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

        let index = if let Some(token) = iter.peek() && !matches!(token.kind, TokenKind::RightBracket) {
            Some(Self::parse(file, iter, 1, None))
        } else {
            None
        };

        match iter.next() {
            Some(trailing_token) => {
                match trailing_token.kind {
                    TokenKind::RightBracket => {
                        Expr::Index {
                            base: Box::new(base),
                            index: match index {
                                Some(i) => Some(Box::new(i)),
                                None => None,
                            },
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
                TokenKind::Tilde => 10,
                _ => unreachable!(),
            }

            OpType::Infix(op) => match op.kind {
                TokenKind::Equals => 1,
                TokenKind::PlusEquals => 1,
                TokenKind::MinusEquals => 1,
                TokenKind::StarEquals => 1,
                TokenKind::SlashEquals => 1,
                TokenKind::CaretEquals => 1,
                TokenKind::AmpersandEquals => 1,
                TokenKind::PipeEquals => 1,
                TokenKind::TildeEquals => 1,
                TokenKind::PercentEquals=> 1,
                TokenKind::DoublePipe => 2,
                TokenKind::DoubleAmpersand => 3,
                TokenKind::DoubleEquals => 4,
                TokenKind::NotEquals => 4,
                TokenKind::LessThan => 4,
                TokenKind::GreaterThan => 4,
                TokenKind::LessThanEquals => 4,
                TokenKind::GreaterThanEquals => 4,
                TokenKind::Pipe => 5,
                TokenKind::Caret => 6,
                TokenKind::Ampersand => 7,
                TokenKind::Plus => 8,
                TokenKind::Minus => 8,
                TokenKind::Star => 9,
                TokenKind::Slash => 9,
                TokenKind::Percent => 9,
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