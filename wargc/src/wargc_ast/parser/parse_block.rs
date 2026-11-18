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
            let node = Node::parse(file, iter, Some(vec![
                TokenKind::Number,
                TokenKind::Identifier,
                TokenKind::String(String::new()),
                TokenKind::Let,
                TokenKind::Const,
                TokenKind::If,
                TokenKind::Else,
                TokenKind::For,
                TokenKind::While,
                TokenKind::Return,
                TokenKind::New,
                TokenKind::Typeof,
                TokenKind::Plus,
                TokenKind::Minus,
                TokenKind::Star,
                TokenKind::Slash,
                TokenKind::Equals,
                TokenKind::DoubleEquals,
                TokenKind::NotEquals,
                TokenKind::LessThan,
                TokenKind::GreaterThan,
                TokenKind::LessThanEquals,
                TokenKind::GreaterThanEquals,
                TokenKind::LeftParentheses,
                TokenKind::RightParentheses,
                TokenKind::LeftBracket,
                TokenKind::RightBracket,
                TokenKind::LeftBrace,
                TokenKind::RightBrace,
                TokenKind::Semicolon,
                TokenKind::Comma,
                TokenKind::Colon,
                TokenKind::Bang,
                TokenKind::Dot,
                TokenKind::Pipe,
                TokenKind::Ampersand,
                TokenKind::Percent,
                TokenKind::DoubleAmpersand,
                TokenKind::DoublePipe,
                TokenKind::DoubleGreaterThan,
                TokenKind::DoubleLessThan,
                TokenKind::Caret,
                TokenKind::Tilde,
                TokenKind::PlusEquals,
                TokenKind::MinusEquals,
                TokenKind::StarEquals,
                TokenKind::SlashEquals,
                TokenKind::AmpersandEquals,
                TokenKind::PipeEquals,
                TokenKind::PercentEquals,
                TokenKind::CaretEquals,
                TokenKind::TildeEquals,
            ]));

            body.push(node);

            // we take only one node
            if braceless {
                break;
            }
        }

        if !braceless {
            expect_token(file, iter, TokenKind::RightBrace, "}");
        }

        BlockStmt {
            members: body
        }
    }
}