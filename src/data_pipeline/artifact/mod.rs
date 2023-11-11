/// Artifact module for the data pipeline.
///
use colored::Colorize;
use std::fs;

mod config;
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

        let context_index = config::choose_context(self.contexts, context);

        let fs_config = config::fs_config(self.contexts, collection);

        match context_index {
            0 => {
                let create_dir_result = fs::create_dir_all(&fs_config.artifact_base_path);
                if let Ok(_tmp) = create_dir_result {
                    let source_path = fs_config.json_collection_path;
                    let output_path = fs_config.artifact_base_path + &fs_config.artifact_file_name;
                    create_artifact::main(&output_path, &source_path);
                }
            }
            1 => restore_artifact::main(
                &fs_config.artifact_base_path,
                &fs_config.artifact_file_name,
                &fs_config.encrypted_artifact_file_name,
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
}
