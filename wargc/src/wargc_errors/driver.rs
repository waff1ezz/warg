use colored::{ColoredString, Colorize};
use crate::wargc_errors::error::Error;
use crate::wargc_driver::CONFIG_FILE_NAME;

pub enum DriverError {
    Fatal (Box<str>),
    IO (Box<str>, std::io::Error),
    ConfigDeserialization (serde_json::Error),
    ConfigPropertyMissing (Box<str>),
    ConfigPropertyInvalid (Box<str>),
    InvalidFlag (Box<str>),
    DuplicatedFlag (Box<str>),
    ContradictoryFlags (Box<str>)
}

impl Error for DriverError {
    fn throw(&self) -> ! {
        match self {
            DriverError::IO(file, err ) => throw_io_error(file, err.kind()),
            DriverError::ConfigDeserialization(err) => throw_config_deserialization_error(err),
            DriverError::ConfigPropertyMissing(prop_name) => throw_config_property_missing(prop_name),
            DriverError::ConfigPropertyInvalid(prop_name) => throw_config_property_invalid(prop_name),
            DriverError::InvalidFlag(flag) => throw_invalid_flag(flag),
            DriverError::DuplicatedFlag(flag) => throw_duplicated_flag(flag),
            DriverError::ContradictoryFlags(flag) => throw_contradictory_flags(flag),
            DriverError::Fatal(cause) => throw_fatal(cause),
        }

        fn throw_duplicated_flag(flag_name: &str) -> ! {
            eprintln!("{}: duplicated flag '{flag_name}'", error_label());
            std::process::exit(6);
        }
        
        fn throw_invalid_flag(flag_name: &str) -> ! {
            eprintln!("{}: invalid compilation flag '{flag_name}'", error_label());
            std::process::exit(6);
        }

        fn throw_contradictory_flags(flag_name: &str) -> ! {
            eprintln!("{}: flag '{flag_name}' conflicts with previously passed compilation flags", error_label());
            std::process::exit(6);
        }

        fn throw_config_property_invalid(prop_name: &str) -> ! {
            eprintln!("{}: failed to resolve project configuration: invalid property: '{prop_name}'", error_label());
            std::process::exit(5);
        }

        fn throw_config_property_missing(prop_name: &str) -> ! {
            eprintln!("{}: failed to resolve project configuration: missing required property: '{prop_name}'", error_label());
            std::process::exit(5);
        }

        fn throw_config_deserialization_error(err: &serde_json::Error) -> ! {
            if err.is_io() {
                throw_io_error(CONFIG_FILE_NAME, err.io_error_kind().unwrap())
            }

            eprintln!("{}: failed to resolve project configuration: unable to parse the config file", error_label());
            std::process::exit(4);
        }

        fn throw_io_error(file: &str, kind: std::io::ErrorKind) -> ! {
            eprintln!("{}: failed to read file: '{file}'. io: {}", error_label(), kind.to_string());
            std::process::exit(3);
        }

        fn throw_fatal(cause: &str) -> ! {
            eprintln!("{}: Fatal compiler error: {cause}", error_label());
            std::process::exit(3);
        }
        
        fn error_label() -> ColoredString {
            "error".bold().red()
        }
    }
}

