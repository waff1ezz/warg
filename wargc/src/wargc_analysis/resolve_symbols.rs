use std::path::PathBuf;
use std::slice::IterMut;
use crate::wargc_ast::lexer::Token;
use crate::wargc_ast::make::BUILT_IN_SYMBOLS;
use crate::wargc_ast::node::{Expr, Node, StructDecl, SymbolId};
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

    fn check_name_usage(token: &Token, resolved_symbols: &Vec<(&str, SymbolId)>, source: &str, path: &PathBuf) {
        let symbol_name = token.original_slice(source);

        let is_used = resolved_symbols
            .iter()
            .rev()
            .find(|symbol| { symbol.0 == symbol_name })
            .is_some();

        if is_used {
            let err = SemanticError {
                src: source,
                file_path: path.clone(),
                line: token.pos.line,
                char: token.pos.char,
                length: symbol_name.chars().count(),
                kind: SemanticErrorKind::ForbiddenSymbolRedeclaration(symbol_name),
            };

            err.throw();
        }
    }

    fn resolve_expr_symbols(path: &PathBuf, source: &str, resolved_symbols: &mut Vec<(&str, SymbolId)>, expr: &mut Expr, ctx: Option<&StructDecl>) {
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
                    if !BUILT_IN_SYMBOLS.contains(&symbol_name) {
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
                }
            },

            Expr::Binary { left, right, .. } => {
                Self::resolve_expr_symbols(
                    path,
                    source,
                    resolved_symbols,
                    left,
                    ctx
                );

                Self::resolve_expr_symbols(
                    path,
                    source,
                    resolved_symbols,
                    right,
                    ctx
                );
            }

            Expr::Unary { operand, .. } => {
                Self::resolve_expr_symbols(
                    path,
                    source,
                    resolved_symbols,
                    operand,
                    ctx
                );
            }

            Expr::FunctionCall { callee, args } => {
                Self::resolve_expr_symbols(
                    path,
                    source,
                    resolved_symbols,
                    callee,
                    ctx
                );

                for func_param in args {
                    Self::resolve_expr_symbols(
                        path,
                        source,
                        resolved_symbols,
                        func_param,
                        ctx
                    );
                }
            }

            Expr::Index { base, index } => {
                Self::resolve_expr_symbols(
                    path,
                    source,
                    resolved_symbols,
                    base,
                    ctx
                );

                if let Some(index) = index {
                    Self::resolve_expr_symbols(
                        path,
                        source,
                        resolved_symbols,
                        index,
                        ctx
                    );
                }
            }

            Expr::Property { base, prop_name } => {
                Self::resolve_expr_symbols(
                    path,
                    source,
                    resolved_symbols,
                    base,
                    ctx
                );

                Self::resolve_expr_symbols(
                    path,
                    source,
                    resolved_symbols,
                    prop_name,
                    ctx
                );
            }

            Expr::ConstructorCall { struct_type, args } => {
                Self::resolve_expr_symbols(
                    path,
                    source,
                    resolved_symbols,
                    struct_type,
                    ctx
                );

                for constructor_args in args {
                    Self::resolve_expr_symbols(
                        path,
                        source,
                        resolved_symbols,
                        constructor_args,
                        ctx
                    );
                }
            }

            Expr::Typeof { expr } => {
                Self::resolve_expr_symbols(
                    path,
                    source,
                    resolved_symbols,
                    expr,
                    ctx
                );
            }

            _ => {}
        }
    }

    fn resolve_scope_symbols<'a>(
        path: &PathBuf,
        source: &'a str,
        mut nodes_iter: IterMut<Node>,
        resolved_symbols: &mut Vec<(&'a str, SymbolId)>,
        global_scope: bool
    ) {
        // this doesn't refer to child scopes
        let mut scope_symbols_counter = 0;

        while let Some(node) = nodes_iter.next() {
            match node {
                Node::Expression(expr) => {
                    Self::resolve_expr_symbols(
                        path,
                        source,
                        resolved_symbols,
                        expr,
                        None
                    );
                }

                Node::Function(func_stmt) => {
                    if global_scope {
                        Self::check_name_usage(&func_stmt.name, &resolved_symbols, source, path);
                    }

                    // if global_scope && func_stmt.name.original_slice(source) == "main" && resolved_symbols.iter().any(|s| s.0 == "main") {
                    //     let err = SemanticError {
                    //         src: source,
                    //         file_path: path.clone(),
                    //         line: func_stmt.name.pos.line,
                    //         char: func_stmt.name.pos.char,
                    //         length: 4,
                    //         kind: SemanticErrorKind::ForbiddenSymbolRedeclaration("main"),
                    //     };
                    //
                    //     err.throw();
                    // }

                    resolved_symbols.push((
                        func_stmt.name.original_slice(source),
                        func_stmt.symbol_id
                    ));

                    if global_scope {
                        continue;
                    }

                    scope_symbols_counter += 1;

                    if let Some(return_type) = &mut func_stmt.return_type {
                        Self::resolve_expr_symbols(
                            path,
                            source,
                            resolved_symbols,
                            return_type,
                            None
                        );
                    }

                    /*  all functions are in one scope so folding doesn't occur after function declaration,
                     *  but it's params are being added to resolved_symbols that's why there's an additional counter to remove them.
                     *  it doesn't collide with variables declared inside the function because they are removed by a new resolved_scope_symbols() call
                     */

                    let mut func_params_counter = 0;

                    for func_param in &mut func_stmt.args {
                        dbg!(&resolved_symbols);
                        Self::check_name_usage(&func_param.name, &resolved_symbols, source, path);

                        resolved_symbols.push((
                            func_param.name.original_slice(source),
                            func_param.symbol_id
                        ));

                        Self::resolve_expr_symbols(
                            path,
                            source,
                            resolved_symbols,
                            &mut func_param.data_type,
                            None
                        );

                        func_params_counter += 1;
                        scope_symbols_counter += 1;
                    }


                    Self::resolve_scope_symbols(
                        path,
                        source,
                        func_stmt.body.members.iter_mut(),
                        resolved_symbols,
                        false
                    );

                    resolved_symbols.truncate(resolved_symbols.len() - func_params_counter);
                }

                Node::Variable(var_decl) => {
                    Self::check_name_usage(&var_decl.name, &resolved_symbols, source, path);

                    resolved_symbols.push((
                        var_decl.name.original_slice(source),
                        var_decl.symbol_id
                    ));

                    if let Some(data_type) = &mut var_decl.data_type {
                        Self::resolve_expr_symbols(
                            path,
                            source,
                            resolved_symbols,
                            data_type,
                            None
                        );
                    }

                    scope_symbols_counter += 1;
                }

                Node::Structure(struct_decl) => {
                    if global_scope {
                        Self::check_name_usage(&struct_decl.name, &resolved_symbols, source, path);
                    }

                    resolved_symbols.push((
                        struct_decl.name.original_slice(source),
                        struct_decl.symbol_id
                    ));

                    if global_scope {
                        continue;
                    }

                    scope_symbols_counter += 1;

                    for interface in &mut struct_decl.interfaces {
                        Self::resolve_expr_symbols(
                            path,
                            source,
                            resolved_symbols,
                            interface,
                            None
                        );
                    }

                    resolved_symbols.push((
                        "this",
                        struct_decl.symbol_id
                    ));
                }

                Node::Block(block_stmt) => {
                    Self::resolve_scope_symbols(
                        path,
                        source,
                        block_stmt.members.iter_mut(),
                        resolved_symbols,
                        false
                    );
                }

                Node::For(for_stmt) => {
                    Self::check_name_usage(&for_stmt.var.name, &resolved_symbols, source, path);

                    resolved_symbols.push((
                        for_stmt.var.name.original_slice(source),
                        for_stmt.var.symbol_id
                    ));

                    Self::resolve_scope_symbols(
                        path,
                        source,
                        for_stmt.body.members.iter_mut(),
                        resolved_symbols,
                        false
                    );

                    // this so we remove this additional for-variable
                    resolved_symbols.truncate(resolved_symbols.len() - 1);
                }

                Node::StaticVariable(var_decl) => {
                    if global_scope {
                        Self::check_name_usage(&var_decl.name, &resolved_symbols, source, path);
                    }

                    resolved_symbols.push((
                        var_decl.name.original_slice(source),
                        var_decl.symbol_id
                    ));

                    if global_scope {
                        continue;
                    }

                    scope_symbols_counter += 1;
                }

                Node::Interface(interface_decl) => {
                    if global_scope {
                        Self::check_name_usage(&interface_decl.name, &resolved_symbols, source, path);
                    }

                    resolved_symbols.push((
                        interface_decl.name.original_slice(source),
                        interface_decl.symbol_id
                    ));

                    if global_scope {
                        continue;
                    }

                    scope_symbols_counter += 1;
                }

                Node::If(if_stmt) => {
                    Self::resolve_expr_symbols(
                        path,
                        source,
                        resolved_symbols,
                        &mut if_stmt.condition,
                        None
                    );

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
                    Self::resolve_expr_symbols(
                        path,
                        source,
                        resolved_symbols,
                        &mut return_stmt.value,
                        None
                    );
                }

                Node::Import(_) => todo!()
            }
        }

        resolved_symbols.truncate(resolved_symbols.len() - scope_symbols_counter);
    }
}