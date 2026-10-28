use std::iter::Peekable;
use std::vec::IntoIter;
use crate::wargc_ast::lexer::{Token, TokenKind};
use crate::wargc_ast::node::{BlockStmt, Expr, FuncParamDecl, FuncSignatureDecl, InterfaceDecl, StructFieldDecl};
use crate::wargc_driver::File;
use crate::wargc_errors::error::Error;
use crate::wargc_errors::parse::{ParseError, ParseErrorKind};
use crate::wargc_util::expect_token;

impl InterfaceDecl {
    pub fn parse(file: &File, iter: &mut Peekable<IntoIter<Token>>) -> InterfaceDecl {
        expect_token(file, iter, TokenKind::Interface, "interface");

        let name: Token;
        let mut fields: Vec<StructFieldDecl> = vec![];
        let mut methods: Vec<FuncSignatureDecl> = vec![];

        name = expect_token(file, iter, TokenKind::Identifier, "identifier");

        expect_token(file, iter, TokenKind::LeftBrace, "{");

        while let Some(token) = iter.peek() && !matches!(token.kind, TokenKind::RightBrace) {
            match token.kind {
                TokenKind::Func => {
                    expect_token(file, iter, TokenKind::Func, "func");

                    let func_name: Token;
                    let return_type: Expr;
                    let mut args: Vec<FuncParamDecl> = vec![];

                    func_name = expect_token(file, iter, TokenKind::Identifier, "identifier");
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
                    expect_token(file, iter, TokenKind::Colon, ":");

                    return_type =  Expr::parse(file, iter, 1, None);
                    expect_token(file, iter, TokenKind::Semicolon, ";");

                    methods.push(FuncSignatureDecl {
                        name: func_name,
                        return_type,
                        args,
                    });
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

        InterfaceDecl {
            name,
            fields,
            methods,
        }
    }
}