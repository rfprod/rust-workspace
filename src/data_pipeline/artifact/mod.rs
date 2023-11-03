use colored::Colorize;
/// Artifact module for the data pipeline.
///
use std::{
    env::{self},
    fs,
};

mod create_artifact;
mod restore_artifact;

/// Supported contexts.
pub type Contexts<'a> = [&'a str; 2];
/// Supported contexts.
pub const CONTEXTS: Contexts = ["Create artifact", "Restore artifact"];

/// The entry point of the program.
pub fn main(contexts: Contexts, context_arg: Option<String>, collection: String) {
    DataPipelineArtifact::new(contexts, context_arg, collection);
}

struct ArtifactFileConfig {
    json_collection_path: String,
    artifact_base_path: String,
    artifact_file_name: String,
    encrypted_artifact_file_name: String,
}

struct DataPipelineArtifact<'a> {
    contexts: Contexts<'a>,
}

impl<'a> DataPipelineArtifact<'a> {
    /// Program constructor.
    fn new(
        contexts: Contexts,
        context_arg: Option<String>,
        collection: String,
    ) -> DataPipelineArtifact {
        let mut program = DataPipelineArtifact { contexts };
        program.init(context_arg, collection);
        program
    }

    /// Initializes the program.
    fn init(&mut self, context: Option<String>, collection: String) {
        println!("\n{}", "DataPipelineArtifact initialized.".blue().bold());

        println!("\n{} {:?}", "Selected context".blue().bold(), context);

        let context_index = self.choose_context(context);

        let config = self.fs_config(collection);

        match context_index {
            0 => {
                let create_dir_result = fs::create_dir_all(&config.artifact_base_path);
                if let Ok(_tmp) = create_dir_result {
                    let source_path = config.json_collection_path;
                    let output_path = config.artifact_base_path + &config.artifact_file_name;
                    create_artifact::main(&output_path, &source_path);
                }
            }
            1 => restore_artifact::main(
                &config.artifact_base_path,
                &config.artifact_file_name,
                &config.encrypted_artifact_file_name,
            ),
            _ => {
                println!(
                    "\n{}",
                    "Nothing to execute. The context is not supported"
                        .red()
                        .bold()
                )
            }
        }
    }

    /// Prompts input from the user, processes it, and returns the selected context index.
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
