use std::iter::Peekable;
use std::mem::discriminant;
use std::ops::Range;
use std::vec::IntoIter;
use crate::wargc_driver::File;
use crate::wargc_ast::lexer::{Token, TokenKind};
use crate::wargc_errors::error::Error;
use crate::wargc_errors::parse::{ParseError, ParseErrorKind};
use crate::wargc_util::{expect_token, Position};

#[derive(Debug)]
pub enum Node {
    Expression(Expr),
    Block(BlockStmt),
    For(ForStmt),
    Function(FuncDecl),
    If(IfStmt),
    Import(ImportStmt),
    Export(ExportStmt),
    Return(ReturnStmt),
    Variable(VarDecl),
    StaticVariable(StaticVarDecl),
    Structure(StructDecl),
    Interface(InterfaceDecl),
}

impl Node {
    pub fn parse(file: &File, iter: &mut Peekable<IntoIter<Token>>, allowed_token_kinds: Option<Vec<TokenKind>>) -> Self {
        if let Some(token) = iter.peek() {
            if let Some(allowed_token_kinds) = allowed_token_kinds {
                let is_allowed = allowed_token_kinds
                    .iter()
                    .any(|kind| discriminant(&token.kind) == discriminant(kind));

                if !is_allowed {
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

            match token.kind {
                TokenKind::Func => {
                    Node::Function(
                        FuncDecl::parse(file, iter)
                    )
                },

                TokenKind::Const |
                TokenKind::Let => {
                    Node::Variable(
                        VarDecl::parse(file, iter, false)
                    )
                }

                TokenKind::If => {
                    Node::If(
                        IfStmt::parse(file, iter)
                    )
                }

                TokenKind::Import => {
                    Node::Import(
                        ImportStmt::parse(file, iter)
                    )
                }

                TokenKind::Export => {
                    Node::Export(
                        ExportStmt::parse(file, iter)
                    )
                }

                TokenKind::Return => {
                    Node::Return(
                        ReturnStmt::parse(file, iter)
                    )
                }

                TokenKind::For => {
                    Node::For(
                        ForStmt::parse(file, iter)
                    )
                }

                TokenKind::LeftBrace => {
                    Node::Block(
                        BlockStmt::parse(file, iter, false)
                    )
                }

                TokenKind::Static => {
                    Node::StaticVariable(
                        StaticVarDecl::parse(file, iter)
                    )
                }

                TokenKind::Struct => {
                    Node::Structure(
                        StructDecl::parse(file, iter)
                    )
                }
                
                TokenKind::Interface => {
                    Node::Interface(
                        InterfaceDecl::parse(file, iter)
                    )
                }

                _ => {
                    let expr = Expr::parse(file, iter, 1, None);
                    expect_token(file, iter, TokenKind::Semicolon, ";");

                    Node::Expression(expr)
                }
            }
        }

        else {
            unreachable!();
        }
    }
}

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
        index: Option<Box<Expr>>,
    },

    Property {
        base: Box<Expr>,
        prop_name: Box<Expr>
    },

    ConstructorCall {
        struct_type: Box<Expr>,
        args: Vec<Expr>,
    },
    
    Typeof {
        expr: Box<Expr>,
    }
}

#[derive(Debug)]
pub struct BlockStmt {
    pub body: Vec<Node>
}

#[derive(Debug)]
pub struct ForStmt {
    pub var: VarDecl,
    pub range: Expr,
    pub body: BlockStmt,
}

#[derive(Debug)]
pub struct FuncDecl {
    pub name: Token,
    pub return_type: Option<Expr>,
    pub args: Vec<FuncParamDecl>,
    pub body: BlockStmt,
}

#[derive(Debug)]
pub struct FuncParamDecl {
    pub name: Token,
    pub data_type: Expr,
}

#[derive(Debug)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_body: BlockStmt,
    pub else_body: Option<BlockStmt>,
}

#[derive(Debug)]
pub struct ImportStmt {
    pub members: Vec<Token>,
    pub module: Vec<Token>
}

#[derive(Debug)]
pub struct ExportStmt {
    pub item: Box<Node>,
}

#[derive(Debug)]
pub struct ReturnStmt {
    pub value: Expr
}

#[derive(Debug)]
pub struct VarDecl {
    pub name: Token,
    pub data_type: Option<Expr>,
    pub value: Option<Expr>,
    pub mutable: bool
}

#[derive(Debug)]
pub struct StaticVarDecl {
    pub name: Token,
    pub data_type: Option<Expr>,
    pub value: Expr,
}

#[derive(Debug)]
pub struct StructDecl {
    pub name: Token,
    pub fields: Vec<StructFieldDecl>,
    pub methods: Vec<FuncDecl>,
    pub constructors: Vec<ConstructorDecl>,
    pub interfaces: Vec<Token>
}

#[derive(Debug)]
pub struct StructFieldDecl {
    pub name: Token,
    pub data_type: Expr,
}

#[derive(Debug)]
pub struct ConstructorDecl {
    pub args: Vec<FuncParamDecl>,
    pub body: BlockStmt,
}

#[derive(Debug)]
pub struct InterfaceDecl {
    pub name: Token,
    pub methods: Vec<FuncSignatureDecl>,
    pub fields: Vec<StructFieldDecl>,
}

#[derive(Debug)]
pub struct FuncSignatureDecl {
    pub name: Token,
    pub return_type: Expr,
    pub args: Vec<FuncParamDecl>,
}