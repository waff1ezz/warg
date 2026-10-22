use std::path::PathBuf;
use colored::{ColoredString, Colorize};
use crate::wargc_errors::error::Error;
use crate::wargc_parse::lexer::Token;

pub struct ParseError<'src> {
    pub src: &'src str,
    pub file_path: PathBuf,
    pub line: usize,
    pub char: usize,
    pub length: usize,
    pub kind: ParseErrorKind<'src>,
}

pub enum ParseErrorKind<'src> {
    NumberLiteralNotParsable,
    UnknownCharacterEscape(char),
    UnknownToken(&'src Token),
    UnexpectedToken(&'src Token),
    ExpectedToken(&'src str),
}

impl<'src> Error for ParseError<'src> {
    fn throw(&self) -> ! {
        let err_line_content: String = self.src
            .lines()
            .nth(self.line - 1)
            .unwrap_or("")
            .to_string();

        let underline = " ".repeat(self.char + 6) + "-" + &"^".repeat(self.length) + "-";

        eprintln!("\n{} ~ {:0>2}:{:0>2}", self.file_path.display(), self.line, self.char);
        eprintln!("{:0>5} | {err_line_content}", self.line);
        eprintln!("{}", underline.bright_red());

        let msg = match &self.kind {
            ParseErrorKind::UnknownCharacterEscape(char) => {
                format!("{}: unknown character escape: '{char}'", error_label())
            },

            ParseErrorKind::NumberLiteralNotParsable => {
                format!("{}: number literal not parsable", error_label())
            },

            ParseErrorKind::UnknownToken(token) => {
                format!("{}: unknown token '{}'", error_label(), token.original_slice(self.src))
            },

            ParseErrorKind::UnexpectedToken(token) => {
                format!("{}: unexpected token: '{}'", error_label(), token.original_slice(self.src))
            },

            ParseErrorKind::ExpectedToken(str) => {
                format!("{}: expected token: '{}'", error_label(), str)
            },
        };

        eprintln!("{msg}");
        std::process::exit(16);

        fn error_label() -> ColoredString {
            "error".bold().red()
        }
    }
}

