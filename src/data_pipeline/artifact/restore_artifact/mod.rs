/// Artifact restoration module for the data pipeline.
///
use std::{
    env::{self},
    process::Command,
};

use colored::Colorize;

/// The entry point of the program.
pub fn main(
    artifact_base_path: &str,
    artifact_file_name: &str,
    encrypted_artifact_file_name: &str,
) {
    DataPipelineArtifactRestorer::new(
        artifact_base_path,
        artifact_file_name,
        encrypted_artifact_file_name,
    );
}

struct DataPipelineArtifactRestorer;

impl DataPipelineArtifactRestorer {
    /// Program constructor.
    fn new(
        artifact_base_path: &str,
        artifact_file_name: &str,
        encrypted_artifact_file_name: &str,
    ) -> DataPipelineArtifactRestorer {
        let program = DataPipelineArtifactRestorer;
        program.init(
            artifact_base_path,
            artifact_file_name,
            encrypted_artifact_file_name,
        );
        program
    }

    /// Initializes the program.
    fn init(
        &self,
        artifact_base_path: &str,
        artifact_file_name: &str,
        encrypted_artifact_file_name: &str,
    ) {
        println!(
            "\n{}",
            "DataPipelineArtifactRestorer initialized.".blue().bold()
        );

        println!("{} {:?}", "Artifact base path".cyan(), artifact_base_path);
        println!("{} {:?}", "Artifact file name".cyan(), artifact_file_name);
        println!(
            "{} {:?}",
            "Encrypted artifact file name".cyan(),
            encrypted_artifact_file_name
        );

        self.restore_artifact(
            artifact_base_path,
            artifact_file_name,
            encrypted_artifact_file_name,
        )
    }

    fn restore_artifact(
        &self,
        artifact_base_path: &str,
        artifact_file_name: &str,
        encrypted_artifact_file_name: &str,
    ) {
        println!("\n{}", "Restoring artifact...".cyan().bold());

        let artifact_path = artifact_base_path.to_owned() + artifact_file_name;
        let encrypted_artifact_path = artifact_base_path.to_owned() + encrypted_artifact_file_name;

        let gpg_passphrase_env = env::var("GPG_PASSPHRASE");
        let gpg_passphrase = match gpg_passphrase_env.unwrap().trim().parse::<String>() {
            Ok(value) => value,
            Err(_) => String::new(),
        };

        println!(
            "\n{}:\n{:?}",
            "Encrypted artifact path".green().bold(),
            encrypted_artifact_path
        );

        match Command::new("gpg")
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
        {
            Ok(output) => {
                println!(
                    "\n{}\n{:?}",
                    "Decrypt artifact success".bold().green(),
                    output
                );
            }
            Err(error) => {
                panic!("\n{}\n{:?}", "Decrypt artifact error".bold().red(), error);
            }
        }

        println!("\n{}: {:?}", "Artifact path".green().bold(), artifact_path);

        let output_path = "./";

        match Command::new("tar")
            .args(["-xzf", &artifact_path, "--directory", output_path])
            .output()
        {
            Ok(output) => {
                println!(
                    "\n{}\n{:?}",
                    "Unpack artifact success".bold().green(),
                    output
                );
            }
            Err(error) => {
                panic!("\n{}\n{:?}", "Unpack artifact error".bold().red(), error);
            }
        }
    }
}
