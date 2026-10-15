use std::path::PathBuf;
use std::fs;
use serde_json;
use crate::wargc_errors::driver::DriverError;

pub struct Project {
    name: Box<str>,
    version: Box<str>,
    entry_point: PathBuf
}

impl Project {
    pub fn resolve() -> Result<Project, DriverError> {
        let json = fs::read_to_string("project.json")
            .map_err(|err| DriverError::IO(
                Box::from("project.json"),
                err
            ))?;

        let config: serde_json::Value = serde_json::from_str(&json)
            .map_err(|err| DriverError::ConfigDeserialization(err))?;

        let expected_props_names = vec!("name", "version", "entry_point");
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
            entry_point: PathBuf::from(config_props[2])
        })
    }
}