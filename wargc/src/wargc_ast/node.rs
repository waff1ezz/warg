use std::iter::Peekable;
use std::vec::IntoIter;
use crate::wargc_driver::File;
use crate::wargc_ast::lexer::{Token, TokenKind};
use crate::wargc_util::expect_token;

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
    Variable(VarDecl)
}

impl Node {
    pub(crate) fn parse(file: &File, iter: &mut Peekable<IntoIter<Token>>) -> Self {
        if let Some(token) = iter.peek() {
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
    pub args: Vec<FuncParam>,
    pub body: BlockStmt,
}

#[derive(Debug)]
pub struct FuncParam {
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