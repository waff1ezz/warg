use std::iter::Peekable;
use std::vec::IntoIter;
use crate::wargc_ast::lexer::{Token, TokenKind};
use crate::wargc_driver::File;
use crate::wargc_util::expect_token;
use crate::wargc_ast::node::{Node, BlockStmt};

impl BlockStmt {
    pub fn parse(file: &File, iter: &mut Peekable<IntoIter<Token>>, braceless: bool) -> BlockStmt {
        let mut body: Vec<Node> = vec![];
        
        if !braceless {
            expect_token(file, iter, TokenKind::LeftBrace, "{");
        }

        while let Some(token) = iter.peek() && !matches!(token.kind, TokenKind::RightBrace) {
            body.push(
                Node::parse(file, iter, None)
            );

            // we take only one node
            if braceless {
                break;
            }
        }

        if !braceless {
            expect_token(file, iter, TokenKind::RightBrace, "}");
        }

        BlockStmt {
            body
        }
    }
}