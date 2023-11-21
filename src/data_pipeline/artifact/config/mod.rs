//! Artifact configuration submodule.

use colored::Colorize;
use std::env::{self};

/// Artifact module context configuration.
pub fn choose_context(contexts: [&str; 2], context_arg: Option<String>) -> usize {
    let p = ArtifactConfiguration::new(contexts);
    p.choose_context(context_arg)
}

/// Artifact module file system configuration.
pub fn fs_config(contexts: [&str; 2], collection: String) -> ArtifactFileConfig {
    let p = ArtifactConfiguration::new(contexts);
    p.fs_config(collection)
}

pub struct ArtifactFileConfig {
    pub json_collection_path: String,
    pub artifact_base_path: String,
    pub artifact_file_name: String,
    pub encrypted_artifact_file_name: String,
}

struct ArtifactConfiguration<'a> {
    contexts: [&'a str; 2],
}

impl<'a> ArtifactConfiguration<'a> {
    /// Program constructor.
    fn new(contexts: [&'a str; 2]) -> ArtifactConfiguration {
        ArtifactConfiguration { contexts }
    }

    /// Artifact module context configuration.
    fn choose_context(&self, context_arg: Option<String>) -> usize {
        let is_some = context_arg.is_some();
        let context_arg_input = if is_some {
            match context_arg.unwrap().trim().parse::<String>() {
                Ok(value) => value,
                Err(_) => String::new(),
            }
        } else {
            String::new()
        };

        let mut index = usize::MAX;
        for (i, ctx) in self.contexts.iter().enumerate() {
            if ctx.to_owned().eq(context_arg_input.as_str()) {
                index = i;
                break;
            }
        }

        index
    }

    /// Artifact module file system configuration.
    fn fs_config(&self, collection: String) -> ArtifactFileConfig {
        let cwd = match env::current_dir() {
            Ok(value) => {
                println!("{}: {:?}", "Current directory".cyan().bold(), value);
                value.display().to_string()
            }
            Err(error) => {
                panic!("{:?}", error);
            }
        };

        let json_base_path = "./.data/output/github/";
        let json_collection_path = json_base_path.to_owned() + collection.as_str() + "/";

        let artifact_base_path = cwd + "/.data/artifact/github/";
        let artifact_file_name = "github-".to_string() + collection.as_str() + ".tar.gz";
        let encrypted_artifact_file_name =
            "github-".to_string() + collection.as_str() + ".tar.gz.gpg";

        ArtifactFileConfig {
            json_collection_path,
            artifact_base_path,
            artifact_file_name,
            encrypted_artifact_file_name,
        }
    }
}
