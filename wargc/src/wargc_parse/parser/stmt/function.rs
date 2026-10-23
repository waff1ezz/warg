use std::iter::Peekable;
use std::vec::IntoIter;
use crate::wargc_driver::File;
use crate::wargc_parse::ast::ASTNode;
use crate::wargc_parse::lexer::{Token, TokenKind};
use crate::wargc_util::expect_token;

pub struct Function {
    name: Token,
    ret_type: Option<Token>,
    args: Vec<FunctionArgument>,
    body: Vec<Box<ASTNode>>,
}

pub struct FunctionArgument {
    name: Token,
    arg_type: Token,
}

impl Function {
    pub fn parse(file: &File, iter: &mut Peekable<IntoIter<Token>>) -> Function {
        iter.next();

        let name: Token;
        let mut ret_type: Option<Token> = None;
        let mut args: Vec<FunctionArgument> = vec![];
        let mut body: Vec<Box<ASTNode>> = vec![];

        name = expect_token(file, iter, TokenKind::Fn, "function name");
        expect_token(file, iter, TokenKind::LeftParentheses, "(");

        while let Some(_) = iter.next() {
            let arg_name = expect_token(file, iter, TokenKind::Identifier, "argument name");
            expect_token(file, iter, TokenKind::Colon, ":");
            let arg_type = expect_token(file, iter, TokenKind::Identifier, "argument type");

            args.push(FunctionArgument {
                name: arg_name,
                arg_type
            });

            if let Some(next_token) = iter.peek() && matches!(next_token.kind, TokenKind::RightParentheses) {
                break;
            } else {
                expect_token(file, iter, TokenKind::Comma, ",");
            }
        }

        expect_token(file, iter, TokenKind::RightParentheses, ")");

        if let Some(next_token) = iter.peek() && matches!(next_token.kind, TokenKind::Colon) {
            expect_token(file, iter, TokenKind::Colon, ":");
            ret_type = Some(expect_token(file, iter, TokenKind::Identifier, "function return type"));
        }

        expect_token(file, iter, TokenKind::RightParentheses, "{");

        while let Some(token) = iter.peek() && !matches!(token.kind, TokenKind::RightParentheses) {
            body.push(
                Box::from(ASTNode::parse(file, iter))
            );
        }

        expect_token(file, iter, TokenKind::RightParentheses, "}");

        Function {
            name,
            ret_type,
            args,
            body,
        }
    }
}