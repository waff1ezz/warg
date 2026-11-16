use crate::wargc_ast::ast::Ast;
use crate::wargc_ast::node::{Expr, Node, SymbolId};
use crate::wargc_driver::{File, Project};
use crate::wargc_errors::fatal::throw_fatal;

impl Project {
    pub fn resolve_symbols(&self) {
        for module in &self.modules {
        }
    }
}

impl File {
    pub fn resolve_symbols(&self) {
        let ast = self.ast
            .as_ref()
            .unwrap_or_else(|| throw_fatal("an attempt to resolve symbols of an unready file"));

        for node in &ast.nodes {
            match node {
                Node::Expression(expr) => {
                    if let Expr::Identifier { token, decl } = expr {
                        
                    }
                }
                Node::Block(_) => {}
                Node::For(_) => {}
                Node::Function(_) => {}
                Node::If(_) => {}
                Node::Export(_) => {}
                Node::Return(_) => {}
                Node::Import(_) => {}
                Node::Variable(_) => {}
                Node::StaticVariable(_) => {}
                Node::Structure(_) => {}
                Node::Interface(_) => {}
                Node::FunctionParam(_) => {}
            }
        }

    }

    fn find_symbol_by_name(ast: &Ast, name: &str) {

    }
}