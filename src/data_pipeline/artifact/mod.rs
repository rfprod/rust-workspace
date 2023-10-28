/// Artifact module for the data pipeline.
///
use std::{
    env::{self},
    fs,
    process::Command,
};

use colored::Colorize;

/// Supported contexts.
type Contexts<'a> = [&'a str; 2];

/// The entry point of the program.
pub fn main(contexts: Contexts, context_arg: Option<String>) {
    DataPipelineArtifact::new(contexts, context_arg);
}

struct DataPipelineArtifact<'a> {
    contexts: Contexts<'a>,
}

impl<'a> DataPipelineArtifact<'a> {
    /// Program constructor.
    fn new(contexts: Contexts, context_arg: Option<String>) -> DataPipelineArtifact {
        let mut program = DataPipelineArtifact { contexts };
        program.init(context_arg);
        program
    }

    /// Initializes the program.
    fn init(&mut self, context: Option<String>) {
        println!("\n{}", "DataPipelineArtifact initialized.".blue().bold());

        println!("\n{} {:?}", "Selected context".blue().bold(), context);

        let context_index = self.choose_context(context);

        match context_index {
            0 => self.create_artifact(),
            1 => self.restore_from_artifact(),
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

    /// Create an encrypted archive containing downloaded artifacts.
    fn create_artifact(&self) {
        println!("\n{}", "Creating the artifact...".cyan().bold());
        let cwd = env::current_dir().unwrap();
        println!(
            "\n{}:\n{:?}",
            "The current directory is".cyan().bold(),
            cwd.display()
        );
        let base_path = cwd.display().to_string() + "/.data/artifact/github/";
        let create_dir_result = fs::create_dir_all(&base_path);
        if let Ok(_tmp) = create_dir_result {
            let source_path = "./.data/output/github";
            let output_path = base_path + "/github-repos.tar.gz";

            Command::new("tar")
                .args(["-czf", &output_path, source_path])
                .output()
                .expect("Failed to create the artifact");

            println!(
                "\n{}:\n{:?}",
                "Created the archive".green().bold(),
                output_path
            );

            let gpg_passphrase_env = env::var("GPG_PASSPHRASE");
            let gpg_passphrase = match gpg_passphrase_env.unwrap().trim().parse::<String>() {
                Ok(value) => value,
                Err(_) => String::new(),
            };

            let encrypted_artifact_path = output_path.to_owned() + ".gpg";
            Command::new("gpg")
                .args([
                    "--batch",
                    "--yes",
                    "--passphrase",
                    &gpg_passphrase,
                    "--symmetric",
                    "--cipher-algo",
                    "aes256",
                    "--output",
                    &encrypted_artifact_path,
                    &output_path,
                ])
                .output()
                .expect("Failed to encrypt the artifact");

            println!(
                "\n{}:\n{:?}",
                "Encrypted the archive".green().bold(),
                encrypted_artifact_path
            );
        }
    }

    fn restore_from_artifact(&self) {
        println!("\n{}", "Restoring data from the artifact...".cyan().bold());
        let cwd = env::current_dir().unwrap();
        println!(
            "\n{}:\n{:?}",
            "The current directory is".cyan().bold(),
            cwd.display()
        );
        let base_path = cwd.display().to_string() + "/.data/artifact/github/";
        let artifact_path = base_path.to_owned() + "github-repos.tar.gz";
        let encrypted_artifact_path = base_path + "github-repos.tar.gz.gpg";

        let gpg_passphrase_env = env::var("GPG_PASSPHRASE");
        let gpg_passphrase = match gpg_passphrase_env.unwrap().trim().parse::<String>() {
            Ok(value) => value,
            Err(_) => String::new(),
        };

        Command::new("gpg")
            .args([
                "--batch",
                "--yes",
                "--passphrase",
                &gpg_passphrase,
                "--decrypt",
                "--output",
                &artifact_path,
                &encrypted_artifact_path,
            ])
            .output()
            .expect("Failed to decrypt the artifact");

        println!(
            "\n{}:\n{:?}",
            "Decrypted the archive".green().bold(),
            encrypted_artifact_path
        );

        let output_path = "./";

        Command::new("tar")
            .args(["-xzf", &artifact_path, output_path])
            .output()
            .expect("Failed to unpack the artifact");

        println!(
            "\n{}:\n{:?}",
            "Unpacked the archive".green().bold(),
            artifact_path
        );
    }
}
