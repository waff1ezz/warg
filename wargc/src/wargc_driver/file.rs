use std::path::PathBuf;
use crate::wargc_util::Position;
use crate::wargc_ast::ast::Ast;

#[derive(Debug)]
pub struct File {
    pub path: PathBuf,
    pub source: String,
    pub ast: Option<Ast>
}

impl File {
    pub fn eof_pos(&self) -> Position {
        if let Some((line_idx, content)) = self.source.lines().enumerate().last() {
            let char_idx = content.chars().count();

            return Position {
                line: line_idx,
                char: char_idx
            };
        }

        Position {
            line: 1,
            char: 1
        }
    }
}