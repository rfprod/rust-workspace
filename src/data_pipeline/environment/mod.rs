//! Environment loader module for the data pipeline.

use colored::Colorize;
use std::{
    collections::HashMap,
    env::{self},
    fs::{self},
};

/// The entry point of the program.
pub fn main(relative_env_path: Option<String>) {
    DataPipelineEnvironment::new(relative_env_path);
}

struct DataPipelineEnvironment {
    relative_env_path: Option<String>,
}

impl DataPipelineEnvironment {
    /// Program constructor.
    fn new(relative_env_path: Option<String>) -> DataPipelineEnvironment {
        let mut program = DataPipelineEnvironment { relative_env_path };
        program.init();
        program
    }

    /// Initializes the program.
    fn init(&mut self) {
        println!("\n{}", "DataPipelineEnvironment initialized.".blue().bold());

        self.load_env_vars();
    }

    /// Load environment variables read from the .env file.
    fn load_env_vars(&self) -> HashMap<String, String> {
        let result = self.read_env();
        let config = match result {
            Ok(env) => {
                let iter = env.iter();
                for record in iter {
                    let key = record.0;
                    let value = record.1;
                    env::set_var(key, value);
                    println!("key: {key}\nvalue: {value}");
                }
                env
            }
            Err(error) => {
                println!(
                    "\n{}: {:?}",
                    "There was an error reading the .env file".red(),
                    error
                );
                HashMap::from([])
            }
        };
        config
    }

    /// Read variables from the .env file.
    fn read_env(&self) -> std::io::Result<HashMap<String, String>> {
        let cwd = env::current_dir()?;

        println!("The current directory is {}", cwd.display());

        let default_relative_env_path = "/.env".to_string();
        let custom_relative_env_path = self.relative_env_path.clone();

        let relative_env_path = if let Some(val) = custom_relative_env_path {
            match val.trim().parse::<String>() {
                Ok(value) => value,
                Err(_) => default_relative_env_path,
            }
        } else {
            default_relative_env_path
        };

        let env_path = cwd.display().to_string() + relative_env_path.as_str();
        let env_path_str = env_path.as_str();
        let env_content_result = fs::read_to_string(env_path_str);
        let Ok(env_content) = env_content_result else {
            panic!("\n{} {:?}", "Can't read directory".red().bold(), env_path);
        };

        println!("Text content:\n{env_content}");

        let lines = self.read_lines(env_path_str);
        let config: Vec<_> = lines
            .iter()
            .filter(|x| x.contains('='))
            .flat_map(|x| {
                let mut split_pair = x.split('=').collect::<Vec<&str>>();
                let split_key = split_pair.first();
                let some_key = split_key.is_some();
                let key = if some_key {
                    match split_key.unwrap().trim().parse::<String>() {
                        Ok(value) => value,
                        Err(_) => String::new(),
                    }
                } else {
                    String::new()
                };
                split_pair.remove(0);
                let value = split_pair.join("");
                let map = vec![(key, value)];
                map
            })
            .collect();
        let map: HashMap<String, String> = HashMap::from_iter(config);
        Ok(map)
    }

    /// Read a file line by line.
    fn read_lines(&self, filename: &str) -> Vec<String> {
        let mut result = Vec::new();
        for line in fs::read_to_string(filename).unwrap().lines() {
            result.push(line.to_string())
        }
        result
    }
}
