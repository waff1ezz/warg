use std::collections::HashMap;
use crate::wargc_ast::node::{Node, SymbolId};

#[derive(Debug)]
pub struct Ast {
    pub nodes: Vec<Node>,
    pub symbol_table: HashMap<String, SymbolId>,
}