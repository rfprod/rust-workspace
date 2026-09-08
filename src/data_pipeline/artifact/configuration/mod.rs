//! Artifact Configuration submodule.

use colored::Colorize;
use std::env::{self};

use crate::data_pipeline::configuration::Contexts;

/// Artifact module context configuration.
pub fn choose_context(contexts: Contexts, context: Option<String>) -> usize {
    let p = ArtifactConfiguration::new(contexts);
    p.choose_context(context)
}

/// Artifact module file system configuration.
pub fn fs_config(contexts: Contexts, collection: String) -> ArtifactFileConfig {
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
    contexts: Contexts<'a>,
}

impl<'a> ArtifactConfiguration<'a> {
    /// Program constructor.
    fn new(contexts: Contexts<'a>) -> ArtifactConfiguration<'a> {
        ArtifactConfiguration { contexts }
    }

    /// Artifact module context configuration.
    fn choose_context(&self, context: Option<String>) -> usize {
        let is_some = context.is_some();
        let context_input = if is_some {
            context
                .unwrap()
                .trim()
                .parse::<String>()
                .unwrap_or_default()
        } else {
            String::new()
        };

        let mut index = usize::MAX;
        for (i, ctx) in self.contexts.iter().enumerate() {
            if ctx.to_owned().eq(context_input.as_str()) {
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

#[cfg(test)]
mod tests;
