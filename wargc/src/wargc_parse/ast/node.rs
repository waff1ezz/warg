use std::iter::Peekable;
use std::vec::IntoIter;
use crate::wargc_driver::File;
use crate::wargc_parse::lexer::{Token, TokenKind};
use crate::wargc_parse::parser::Function;

pub enum ASTNode {
    Empty,
    Function(Function),
}

impl ASTNode {
    pub fn parse(file: &File, iter: &mut Peekable<IntoIter<Token>>) -> Self {
        if let Some(token) = iter.peek() {
            match token.kind {
                TokenKind::Fn => {
                    return ASTNode::Function(
                        Function::parse(file, iter)
                    );
                },

                _ => unreachable!(),
            }
        }
        
        ASTNode::Empty
    }
}