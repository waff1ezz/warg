use std::collections::HashMap;
use std::env::var;
use crate::wargc_ast::node::{Node, SymbolId};

#[derive(Debug)]
pub struct Ast {
    pub nodes: Vec<Node>,
    pub symbol_table: HashMap<String, SymbolId>,
}

impl Ast {
    pub fn find_symbol(nodes: &Vec<Node>, symbol_id: SymbolId) -> Option<&Node> {
        for node in nodes {
            match node {
                Node::Block(block_stmt) => {
                    return Self::find_symbol(&block_stmt.members, symbol_id);
                }

                Node::For(for_stmt) => {
                    if for_stmt.var.symbol_id == symbol_id {
                        return Some(node);
                    }

                    return Self::find_symbol(&for_stmt.body.members, symbol_id);
                }

                Node::Function(func_decl) => {
                    if func_decl.symbol_id == symbol_id {
                        return Some(node);
                    }

                    for func_param in &func_decl.args {
                        if func_param.symbol_id == symbol_id {
                            return Some(node);
                        }
                    }

                    return Self::find_symbol(&func_decl.body.members, symbol_id);
                }

                Node::Variable(var_decl) => {
                    if var_decl.symbol_id == symbol_id {
                        return Some(node);
                    }
                }

                Node::StaticVariable(var_decl) => {
                    if var_decl.symbol_id == symbol_id {
                        return Some(node);
                    }
                }

                Node::Structure(struct_decl) => {
                    if struct_decl.symbol_id == symbol_id {
                        return Some(node);
                    }
                }

                Node::Interface(interface_decl) => {
                    if interface_decl.symbol_id == symbol_id {
                        return Some(node);
                    }
                }

                _ => return None
            }
        }

        None
    }
}