use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::fs;
use serde_json;
use colored::Colorize;
use crate::wargc_ast::node::Node;
use crate::wargc_driver::Flags;
use crate::wargc_driver::File;
use crate::wargc_errors::driver::DriverError;
use crate::wargc_errors::error::Error;

#[derive(Debug)]
pub struct Project {
    pub dir: PathBuf,
    pub name: String,
    pub version: String,
    pub entry_point: PathBuf,
    pub output_dir: PathBuf,
    pub modules: Vec<File>,
    pub visited: HashSet<PathBuf>,
}

pub const CONFIG_FILE_NAME: &str = "config.wg.json";

impl Project {
    pub fn build(&mut self, _flags: Vec<Flags>) -> Result<(), DriverError> {
        fn verbose_notice(msg: &str) {
            println!("{}: {msg}", "notice".bright_blue().bold());
        }

        let mut entry_file = File {
            path: self.entry_point.clone(),
            source: fs::read_to_string(PathBuf::from(&self.dir)
                .join(&self.entry_point))
                .map_err(|err| DriverError::IO(self.entry_point.to_string_lossy().into_owned(), err))?,
            ast: None
        };

        if self.visited.insert(self.entry_point.clone()) {
            self.parse_file(&mut entry_file);
            self.modules.push(entry_file);
        }

        for module in &self.modules {
            module.resolve_symbols();
        }

        dbg!(&self.modules);
        Ok(())
    }

    fn parse_file(&mut self, file: &mut File) {
        let tokens = file.generate_tokens();
        file.ast = Some(file.make_ast(tokens));

        let imports = file.ast.as_ref().unwrap()
            .iter()
            .filter(|node| matches!(&***node, Node::Import(..)))
            .collect::<Vec<_>>();

        for import in imports {
            if let Node::Import(stmt) = &**import {
                let mut file_path = PathBuf::from(&self.dir);

                for mod_part in &stmt.module {
                    file_path.push(
                        mod_part.original_slice(&file.source).to_string()
                    );
                }

                file_path.set_extension("wg");

                let mut file = File {
                    path: file_path.clone(),
                    source: match fs::read_to_string(&file_path) {
                        Ok(source) => source,
                        Err(err) => {
                            let err = DriverError::IO(file_path.to_string_lossy().into_owned(), err);
                            err.throw();
                        }
                    },
                    ast: None
                };

                if self.visited.insert(file_path.clone()) {
                    self.parse_file(&mut file);
                    self.modules.push(file);
                }
            }
        }


    }

    pub fn resolve() -> Result<Project, DriverError> {
        let args: Vec<String> = std::env::args()
            .collect::<Vec<String>>();

        let config_path = match args.get(1) {
            Some(dir_path) => {
                if dir_path.starts_with("--") {
                    PathBuf::from(CONFIG_FILE_NAME)
                } else {
                    Path::new(&dir_path).join(CONFIG_FILE_NAME)
                }
            },
            None => PathBuf::from(CONFIG_FILE_NAME)
        };

        let config_unserialized = fs::read_to_string(&config_path)
            .map_err(|err| DriverError::IO(
                String::from(CONFIG_FILE_NAME),
                err
            ))?;

        let config: serde_json::Value = serde_json::from_str(&config_unserialized)
            .map_err(|err| DriverError::ConfigDeserialization(err))?;

        let expected_props_names = vec!("name", "version", "entry_point", "output_dir");
        let mut config_props = vec![];

        for prop_name in expected_props_names {
            config_props.push(
                config.get(prop_name)
                    .ok_or(DriverError::ConfigPropertyMissing (String::from(prop_name)) )?
                    .as_str()
                    .ok_or(DriverError::ConfigPropertyInvalid (String::from(prop_name) ))?
            );
        }

        Ok(Project {
            dir: config_path.parent().unwrap_or(&*std::env::current_dir().unwrap()).to_path_buf(),
            name: String::from(config_props[0]),
            version: String::from(config_props[1]),
            entry_point: PathBuf::from(config_props[2]),
            output_dir: PathBuf::from(config_props[3]),
            modules: vec![],
            visited: HashSet::new(),
        })
    }
}