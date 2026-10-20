use std::cmp::PartialEq;
use crate::wargc_errors::driver::DriverError;

#[derive(PartialEq, Debug)]
pub enum Flags {
    RunVerbose,
    RunCheck,
    GetVersion,
    DumpTokens,
    DumpAst,
    DumpBc,
}

impl Flags {
    pub fn from_args() -> Result<Vec<Flags>, DriverError> {
        let mut flags: Vec<Flags> = vec![];
        let args: Vec<String> = std::env::args()
            .filter(|a| a.starts_with("--"))
            .collect();

        let mut push_flag = |name: &str, enum_ident: Flags| {
            let has_duplicates = flags
                .iter()
                .filter(|f| **f == enum_ident)
                .collect::<Vec<&Flags>>()
                .len() > 0;

            if has_duplicates {
                return Err(DriverError::DuplicatedFlag(String::from(name)));
            }

            flags.push(enum_ident);

            if flags.len() > 1 && !flags.contains(&&Flags::RunVerbose) {
                return Err(DriverError::ContradictoryFlags(String::from(name)));
            }

            Ok(())
        };

        for arg in args {
            match arg.as_str() {
                "--verbose" => push_flag(arg.as_str(), Flags::RunVerbose)?,
                "--check" => push_flag(arg.as_str(), Flags::RunCheck)?,
                "--version" => push_flag(arg.as_str(), Flags::GetVersion)?,
                "--dump-tokens" => push_flag(arg.as_str(), Flags::DumpTokens)?,
                "--dump-ast" => push_flag(arg.as_str(), Flags::DumpAst)?,
                "--dump-bc" => push_flag(arg.as_str(), Flags::DumpBc)?,
                _ => return Err(DriverError::InvalidFlag(String::from(&*arg)))
            }
        }

        Ok(flags)
    }
}