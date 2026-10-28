use std::iter::Peekable;
use std::vec::IntoIter;
use crate::wargc_ast::lexer::{Token, TokenKind};
use crate::wargc_ast::node::{FuncDecl, ConstructorDecl, StructDecl, StructFieldDecl, Expr, FuncParamDecl, BlockStmt};
use crate::wargc_driver::File;
use crate::wargc_errors::error::Error;
use crate::wargc_errors::parse::{ParseError, ParseErrorKind};
use crate::wargc_util::expect_token;

impl StructDecl {
    pub fn parse(file: &File, iter: &mut Peekable<IntoIter<Token>>) -> StructDecl {
        expect_token(file, iter, TokenKind::Struct, "struct");

        let name: Token;
        let mut fields: Vec<StructFieldDecl> = vec![];
        let mut methods: Vec<FuncDecl> = vec![];
        let mut constructors: Vec<ConstructorDecl> = vec![];
        let mut interfaces: Vec<Token> = vec![];

        name = expect_token(file, iter, TokenKind::Identifier, "identifier");

        if let Some(next_token) = iter.peek() && matches!(next_token.kind, TokenKind::Implements) {
            expect_token(file, iter, TokenKind::Implements, "implements");

            while let Some(token) = iter.peek() && !matches!(token.kind, TokenKind::LeftBrace) {
                interfaces.push(
                    expect_token(file, iter, TokenKind::Identifier, "identifier")
                );

                if let Some(next_token) = iter.peek() && !matches!(next_token.kind, TokenKind::LeftBrace) {
                    expect_token(file, iter, TokenKind::Comma, ",");
                }
            }
        }

        expect_token(file, iter, TokenKind::LeftBrace, "{");

        while let Some(token) = iter.peek() && !matches!(token.kind, TokenKind::RightBrace) {
            match token.kind {
                TokenKind::Func => {
                    methods.push(
                        FuncDecl::parse(file, iter)
                    )
                }

                TokenKind::Identifier => {
                    let field_name: Token;
                    let field_type: Expr;

                    field_name = expect_token(file, iter, TokenKind::Identifier, "identifier");
                    expect_token(file, iter, TokenKind::Colon, ":");
                    field_type = Expr::parse(file, iter, 1, None);
                    expect_token(file, iter, TokenKind::Semicolon, ";");

                    fields.push(StructFieldDecl {
                        name: field_name,
                        data_type: field_type,
                    });
                }

                TokenKind::New => {
                    let mut args: Vec<FuncParamDecl> = vec![];
                    let body: BlockStmt;

                    expect_token(file, iter, TokenKind::New, "new");
                    expect_token(file, iter, TokenKind::LeftParentheses, "(");

                    while let Some(token) = iter.peek() && !matches!(token.kind, TokenKind::RightParentheses) {
                        let arg_name = expect_token(file, iter, TokenKind::Identifier, "identifier");
                        expect_token(file, iter, TokenKind::Colon, ":");
                        let data_type = Expr::parse(file, iter, 1, None);

                        args.push(FuncParamDecl {
                            name: arg_name,
                            data_type
                        });

                        if let Some(next_token) = iter.peek() && !matches!(next_token.kind, TokenKind::RightParentheses) {
                            expect_token(file, iter, TokenKind::Comma, ",");
                        }
                    }

                    expect_token(file, iter, TokenKind::RightParentheses, ")");
                    body = BlockStmt::parse(file, iter, false);

                    constructors.push(ConstructorDecl {
                        args,
                        body,
                    });
                }

                _ => {
                    let err = ParseError {
                        src: &file.source,
                        file_path: file.path.clone(),
                        line: token.pos.line,
                        char: token.pos.char,
                        length: token.original_slice(&file.source).chars().count(),
                        kind: ParseErrorKind::UnexpectedToken(token),
                    };

                    err.throw();
                }
            }
        }

        expect_token(file, iter, TokenKind::RightBrace, "}");

        StructDecl {
            name,
            fields,
            methods,
            constructors,
            interfaces,
        }
    }
}