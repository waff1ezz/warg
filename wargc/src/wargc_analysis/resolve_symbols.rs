use std::slice::Iter;
use crate::wargc_ast::ast::Ast;
use crate::wargc_ast::lexer::Token;
use crate::wargc_ast::node::{BlockStmt, Expr, Node, SymbolId};
use crate::wargc_driver::{File, Project};
use crate::wargc_errors::fatal::throw_fatal;

impl Project {
    pub fn resolve_symbols(&self) {
        for module in &self.modules {
        }
    }
}

impl File {
    pub fn resolve_symbols(&mut self) {
        if let Some(ast) = &self.ast {
            let symbols = self.resolve_scope_symbols(
                ast,
                ast.nodes.iter(),
                &vec![]
            );

            dbg!(symbols);
        } 
        
        else {
            throw_fatal("an attempt to resolve symbols of an unready file")
        }
    }

    fn resolve_scope_symbols(&self, ast: &Ast, mut nodes_iter: Iter<Node>, parent_scope_symbols: &Vec<&str>) -> Vec<&str> {
        let mut resolved_symbols: Vec<&str> = vec![];

        while let Some(node) = nodes_iter.next() {
            match node {
                Node::Expression(expr) => {
                    if let Expr::Identifier { token, decl } = &expr {}
                }

                Node::Import(import_stmt) => {
                    for member in &import_stmt.members {

                    }
                }

                Node::Function(func_stmt) => {
                    resolved_symbols.push(
                        func_stmt.name.original_slice(&self.source)
                    );

                    for func_arg in &func_stmt.args {
                        resolved_symbols.push(
                            func_arg.name.original_slice(&self.source)
                        );
                    }

                    resolved_symbols.append(
                        &mut self.resolve_scope_symbols(
                            ast,
                            func_stmt.body.members.iter(),
                            &resolved_symbols
                        )
                    );
                }

                Node::Block(_) => {}
                Node::For(_) => {}
                Node::If(_) => {}
                Node::Export(_) => {}
                Node::Return(_) => {}
                Node::Variable(_) => {}
                Node::StaticVariable(_) => {}
                Node::Structure(_) => {}
                Node::Interface(_) => {}
                Node::FunctionParam(_) => {}
            }
        }

        resolved_symbols
    }
}