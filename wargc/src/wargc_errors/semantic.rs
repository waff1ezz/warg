use std::path::PathBuf;
use colored::{ColoredString, Colorize};
use crate::wargc_ast::lexer::Token;
use crate::wargc_errors::error::Error;

pub struct SemanticError<'src> {
    pub src: &'src str,
    pub file_path: PathBuf,
    pub line: usize,
    pub char: usize,
    pub length: usize,
    pub kind: SemanticErrorKind<'src>,
}

pub enum SemanticErrorKind<'src> {
    UnresolvedSymbol(&'src str),
    ForbiddenSymbolRedeclaration(&'src str),
}

impl<'src> Error for SemanticError<'src> {
    fn throw(&self) -> ! {
        let err_line_content: String = self.src
            .lines()
            .nth(self.line)
            .unwrap_or("")
            .to_string();

        let underline = " ".repeat(self.char + 7) + "-" + &"^".repeat(self.length) + "-";

        eprintln!("\n{} ~ {:0>2}:{:0>2}", self.file_path.display(), self.line + 1, self.char + 1);
        eprintln!("{:0>5} | {err_line_content}", self.line + 1);
        eprintln!("{}", underline.bright_red());

        let msg = match &self.kind {
            SemanticErrorKind::UnresolvedSymbol(name) => {
                format!("{}: unresolved symbol: '{name}'", error_label())
            }

            SemanticErrorKind::ForbiddenSymbolRedeclaration(name) => {
                format!("{}: forbidden symbol redeclaration: '{name}'", error_label())
            }
        };

        eprintln!("{msg}");
        std::process::exit(16);

        fn error_label() -> ColoredString {
            "error".bold().red()
        }
    }
}

