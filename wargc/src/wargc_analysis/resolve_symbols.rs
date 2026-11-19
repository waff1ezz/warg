use std::path::PathBuf;
use std::slice::IterMut;
use crate::wargc_ast::node::{Expr, Node, SymbolId};
use crate::wargc_driver::File;
use crate::wargc_errors::error::Error;
use crate::wargc_errors::fatal::throw_fatal;
use crate::wargc_errors::semantic::{SemanticError, SemanticErrorKind};

impl File {
    pub fn resolve_symbols(&mut self) {
        let mut resolved_symbols: Vec<(&str, SymbolId)> = vec![];

        if let Some(ast) = &mut self.ast {
            // this one to capture global symbols
            Self::resolve_scope_symbols(
                &self.path,
                &self.source,
                ast.nodes.iter_mut(),
                &mut resolved_symbols,
                true
            );

            Self::resolve_scope_symbols(
                &self.path,
                &self.source,
                ast.nodes.iter_mut(),
                &mut resolved_symbols,
                false
            );
        } else {
            throw_fatal("an attempt to resolve symbols of an unready file")
        }
    }

    fn resolve_scope_symbols<'a>(
        path: &PathBuf,
        source: &'a str,
        mut nodes_iter: IterMut<Node>,
        resolved_symbols: &mut Vec<(&'a str, SymbolId)>,
        global_scope: bool
    ) {
        fn dive_into_expr(path: &PathBuf, source: &str, resolved_symbols: &mut Vec<(&str, SymbolId)>, expr: &mut Expr) {
            match expr {
                Expr::Identifier { token, decl } => {
                    let symbol_name = token.original_slice(source);
                    let maybe_symbol_decl = resolved_symbols
                        .iter()
                        .rev()
                        .find(|&&symbol| { symbol.0 == symbol_name });

                    if let Some(symbol_decl) = maybe_symbol_decl {
                        *decl = Some(symbol_decl.1);
                    } else {
                        let err = SemanticError {
                            src: source,
                            file_path: path.clone(),
                            line: token.pos.line,
                            char: token.pos.char,
                            length: token.original_slice(source).chars().count(),
                            kind: SemanticErrorKind::UnresolvedSymbol(token.original_slice(source)),
                        };

                        err.throw();
                    }
                },

                Expr::Binary { left, right, .. } => {
                    dive_into_expr(path, source, resolved_symbols, left);
                    dive_into_expr(path, source, resolved_symbols, right);
                }

                Expr::Unary { operand, .. } => {
                    dive_into_expr(path, source, resolved_symbols, operand);
                }

                Expr::FunctionCall { callee, args } => {
                    dive_into_expr(path, source, resolved_symbols, callee);

                    for func_param in args {
                        dive_into_expr(path, source, resolved_symbols, func_param);
                    }
                }

                Expr::Index { base, index } => {
                    dive_into_expr(path, source, resolved_symbols, base);

                    if let Some(index) = index {
                        dive_into_expr(path, source, resolved_symbols, index);
                    }
                }

                Expr::Property { base, prop_name } => {
                    dive_into_expr(path, source, resolved_symbols, base);
                    dive_into_expr(path, source, resolved_symbols, prop_name);
                }

                Expr::ConstructorCall { struct_type, args } => {
                    dive_into_expr(path, source, resolved_symbols, struct_type);

                    for constructor_args in args {
                        dive_into_expr(path, source, resolved_symbols, constructor_args);
                    }
                }

                Expr::Typeof { expr } => {
                    dive_into_expr(path, source, resolved_symbols, expr);
                }

                _ => {}
            }
        }

        // this doesn't refer to child scopes
        let mut scope_symbols_counter = 0;

        while let Some(node) = nodes_iter.next() {
            match node {
                Node::Expression(expr) => {
                    dive_into_expr(path, source, resolved_symbols, expr);
                }

                Node::Function(func_stmt) => {
                    resolved_symbols.push((
                        func_stmt.name.original_slice(source),
                        func_stmt.symbol_id
                    ));

                    if !global_scope {
                        scope_symbols_counter += 1;
                    }

                    for func_param in &func_stmt.args {
                        resolved_symbols.push((
                            func_param.name.original_slice(source),
                            func_param.symbol_id
                        ));

                        if !global_scope {
                            scope_symbols_counter += 1;
                        }
                    }

                    if !global_scope {
                        Self::resolve_scope_symbols(
                            path,
                            source,
                            func_stmt.body.members.iter_mut(),
                            resolved_symbols,
                            false
                        );
                    }
                }

                Node::Variable(var_decl) => {
                    resolved_symbols.push((
                        var_decl.name.original_slice(source),
                        var_decl.symbol_id
                    ));

                    if !global_scope {
                        scope_symbols_counter += 1;
                    }
                }

                Node::Structure(struct_decl) => {
                    resolved_symbols.push((
                        struct_decl.name.original_slice(source),
                        struct_decl.symbol_id
                    ));

                    if !global_scope {
                        scope_symbols_counter += 1;
                    }
                }

                Node::Block(block_stmt) => {
                    if !global_scope {
                        Self::resolve_scope_symbols(
                            path,
                            source,
                            block_stmt.members.iter_mut(),
                            resolved_symbols,
                            false
                        );
                    }
                }

                Node::For(for_stmt) => {
                    resolved_symbols.push((
                        for_stmt.var.name.original_slice(source),
                        for_stmt.var.symbol_id
                    ));

                    if !global_scope {
                        Self::resolve_scope_symbols(
                            path,
                            source,
                            for_stmt.body.members.iter_mut(),
                            resolved_symbols,
                            false
                        );
                    }

                    resolved_symbols.truncate(resolved_symbols.len() - 1);
                }

                Node::StaticVariable(var_decl) => {
                    resolved_symbols.push((
                        var_decl.name.original_slice(source),
                        var_decl.symbol_id
                    ));

                    if !global_scope {
                        scope_symbols_counter += 1;
                    }
                }

                Node::Interface(interface_decl) => {
                    resolved_symbols.push((
                        interface_decl.name.original_slice(source),
                        interface_decl.symbol_id
                    ));

                    if !global_scope {
                        scope_symbols_counter += 1;
                    }
                }

                Node::If(if_stmt) => {
                    dive_into_expr(path, source, resolved_symbols, &mut if_stmt.condition);

                    Self::resolve_scope_symbols(
                        path,
                        source,
                        if_stmt.then_body.members.iter_mut(),
                        resolved_symbols,
                        false
                    );

                    if if_stmt.else_body.is_some() {
                        Self::resolve_scope_symbols(
                            path,
                            source,
                            if_stmt.else_body.as_mut().unwrap().members.iter_mut(),
                            resolved_symbols,
                            false
                        );
                    }
                }

                Node::Return(return_stmt) => {
                    dive_into_expr(path, source, resolved_symbols, &mut return_stmt.value);
                }

                Node::Import(_) => todo!()
            }
        }

        resolved_symbols.truncate(resolved_symbols.len() - scope_symbols_counter);
    }
}