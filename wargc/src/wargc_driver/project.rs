use std::path::{Path, PathBuf};
use std::fs;
use serde_json;
use colored::Colorize;
use crate::wargc_driver::Flags;
use crate::wargc_errors::driver::DriverError;

#[derive(Debug)]
pub struct Project {
    pub name: Box<str>,
    pub version: Box<str>,
    pub entry_point: PathBuf,
    pub output_dir: PathBuf,
}

pub const CONFIG_FILE_NAME: &str = "config.wg.json";

impl Project {
    pub fn build(&self, flags: Vec<Flags>) {
        fn verbose_notice(msg: &str) {
            println!("{}: {msg}", "notice".bright_blue().bold());
        }
    }

    pub fn resolve() -> Result<Project, DriverError> {
        let args: Vec<String> = std::env::args()
            .collect::<Vec<String>>();

        let config_path = match args.get(1) {
            Some(dir) => Path::new(&dir).join(CONFIG_FILE_NAME),
            None => PathBuf::from(CONFIG_FILE_NAME)
        };

        let config_unserialized = fs::read_to_string(config_path)
            .map_err(|err| DriverError::IO(
                Box::from(CONFIG_FILE_NAME),
                err
            ))?;

        let config: serde_json::Value = serde_json::from_str(&config_unserialized)
            .map_err(|err| DriverError::ConfigDeserialization(err))?;

        let expected_props_names = vec!("name", "version", "entry_point", "output_dir");
        let mut config_props = vec![];

        for prop_name in expected_props_names {
            config_props.push(
                config.get(prop_name)
                    .ok_or(DriverError::ConfigPropertyMissing (Box::from(prop_name)) )?
                    .as_str()
                    .ok_or(DriverError::ConfigPropertyInvalid (Box::from(prop_name) ))?
            );
        }

        Ok(Project {
            name: Box::from(config_props[0]),
            version: Box::from(config_props[1]),
            entry_point: PathBuf::from(config_props[2]),
            output_dir: PathBuf::from(config_props[3])
        })
    }
}