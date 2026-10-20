use std::path::PathBuf;
use crate::wargc_parse::lexer::Token;
use crate::wargc_parse::parser::Expr;

pub struct File {
    pub path: PathBuf,
    pub source: String,

}