use crate::wargc_parse::parser::Expr;

#[derive(Debug)]
pub enum ASTNode {
    Expr(Expr)
}