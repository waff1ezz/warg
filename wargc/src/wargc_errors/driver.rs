use super::error::Error;

pub enum DriverError {
    Fatal(Box<str>),
    IO (Box<str>, std::io::Error),
    ConfigDeserialization (serde_json::Error),
    ConfigPropertyMissing (Box<str>),
    ConfigPropertyInvalid (Box<str>)

}

impl Error for DriverError {
    fn throw(&self) -> ! {
        match self {
            DriverError::IO(file, err ) => throw_io_error(file, err.kind()),
            DriverError::ConfigDeserialization(err) => throw_config_deserialization_error(err),
            DriverError::ConfigPropertyMissing(prop_name) => throw_config_property_missing(prop_name),
            DriverError::ConfigPropertyInvalid(prop_name) => throw_config_property_invalid(prop_name),
            DriverError::Fatal(cause) => throw_fatal(cause),
        }

        fn throw_config_property_invalid(prop_name: &str) -> ! {
            eprintln!("Failed to resolve project configuration. Invalid property: '{prop_name}'");
            std::process::exit(5);
        }

        fn throw_config_property_missing(prop_name: &str) -> ! {
            eprintln!("Failed to resolve project configuration. Missing required property: '{prop_name}'");
            std::process::exit(5);
        }

        fn throw_config_deserialization_error(err: &serde_json::Error) -> ! {
            if err.is_io() {
                throw_io_error("config.json", err.io_error_kind().unwrap())
            }

            eprintln!("Failed to resolve project configuration. Unable to parse the config file");
            std::process::exit(4);
        }

        fn throw_io_error(file: &str, kind: std::io::ErrorKind) -> ! {
            eprintln!("Failed to read file: '{file}'. io error: {}", kind.to_string());
            std::process::exit(3);
        }

        fn throw_fatal(cause: &str) -> ! {
            eprintln!("Fatal compiler error: {cause}");
            std::process::exit(3);
        }
    }
}

