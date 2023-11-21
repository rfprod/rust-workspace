//! Artifact creation module for the data pipeline.

use std::{
    env::{self},
    process::Command,
};

use colored::Colorize;

/// The entry point of the program.
pub fn main(output_path: &str, source_path: &str) {
    ArtifactCreator::new(output_path, source_path);
}

struct ArtifactCreator;

impl ArtifactCreator {
    /// Program constructor.
    fn new(output_path: &str, source_path: &str) -> ArtifactCreator {
        let mut program = ArtifactCreator;
        program.init(output_path, source_path);
        program
    }

    /// Initializes the program.
    fn init(&mut self, output_path: &str, source_path: &str) {
        println!("\n{}", "ArtifactCreator initialized.".blue().bold());

        println!("{} {:?}", "Output path".cyan(), output_path);
        println!("{} {:?}", "Source path".cyan(), source_path);

        self.create_artifact(output_path, source_path)
    }

    /// Create an encrypted archive containing downloaded artifacts.
    fn create_artifact(&self, output_path: &str, source_path: &str) {
        println!("\n{}", "Creating artifact...".cyan().bold());

        match Command::new("tar")
            .args(["-czf", output_path, source_path])
            .output()
        {
            Ok(output) => {
                println!("{}\n{:?}", "Create artifact success".bold().green(), output);
            }
            Err(error) => {
                panic!("{}\n{:?}", "Create artifact error".bold().red(), error);
            }
        };

        let gpg_passphrase_env = env::var("GPG_PASSPHRASE");
        let gpg_passphrase = match gpg_passphrase_env.unwrap().trim().parse::<String>() {
            Ok(value) => value,
            Err(_) => String::new(),
        };

        let encrypted_artifact_path = output_path.to_owned() + ".gpg";
        match Command::new("gpg")
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
                output_path,
            ])
            .output()
        {
            Ok(output) => {
                println!(
                    "{}\n{:?}",
                    "Encrypt artifact success".bold().green(),
                    output
                );
            }
            Err(error) => {
                panic!("{}\n{:?}", "Encrypt artifact error".bold().red(), error);
            }
        }

        println!(
            "\n{}:\n{:?}",
            "Encrypted artifact path".green().bold(),
            encrypted_artifact_path
        );
    }
}
