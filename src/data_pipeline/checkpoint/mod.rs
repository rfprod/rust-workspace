//! Data Pipeline Checkpoint module.

use colored::Colorize;
use std::{env, fs, io, path::PathBuf};

mod state;

/// The entry point of the program.
pub fn main() -> DataPipelineCheckpoint {
    DataPipelineCheckpoint::new()
}

pub struct DataPipelineCheckpoint {
    state: Vec<state::DataPipelineCheckpointState>,
}

impl DataPipelineCheckpoint {
    /// Creates a checkpoint.
    fn new() -> DataPipelineCheckpoint {
        let state: Vec<state::DataPipelineCheckpointState> = vec![];
        DataPipelineCheckpoint { state }
    }

    /// Creates a checkpoint state.
    pub fn new_state(
        operation: String,
        position: i64,
        per_page: i64,
        total_items: Option<i64>,
        completed_count: i64,
        last_updated: String,
        context: String,
    ) -> state::DataPipelineCheckpointState {
        state::DataPipelineCheckpointState::new(
            operation,
            position,
            per_page,
            total_items,
            completed_count,
            last_updated,
            context,
        )
    }

    fn checkpoint_path(&self, operation: &str, context: &str) -> PathBuf {
        let cwd = env::current_dir().unwrap();
        cwd.join(format!(".data/checkpoints/{}-{}.json", operation, context))
    }

    /// Add or replaces an operation checkpoint in the state vector.
    fn add_or_replace_state(&mut self, checkpoint: &state::DataPipelineCheckpointState) {
        let state_index = self
            .state
            .iter()
            .position(|x| x.operation == checkpoint.operation);
        if let Some(index) = state_index {
            self.state.splice(index..index, [checkpoint.to_owned()]);
        } else {
            self.state.push(checkpoint.to_owned());
        }
    }

    /// Loads checkpoint metadata for a specified operation and updates the checkpoint state.
    pub fn load_checkpoint(
        &mut self,
        operation: &str,
        context: &str,
    ) -> Option<state::DataPipelineCheckpointState> {
        let path = self.checkpoint_path(operation, context);
        match state::DataPipelineCheckpointState::load(&path) {
            Ok(checkpoint) => {
                println!(
                    "\n{} Resuming {} from position {}",
                    "✓".green(),
                    operation,
                    checkpoint.position
                );

                self.add_or_replace_state(&checkpoint);

                Some(checkpoint)
            }
            Err(_) => None,
        }
    }

    /// Saves checkpoint metadata and updates the checkpoint state.
    pub fn save_checkpoint(
        &mut self,
        checkpoint: &state::DataPipelineCheckpointState,
    ) -> io::Result<()> {
        let path = self.checkpoint_path(&checkpoint.operation, &checkpoint.context);
        let parent_path = path.parent().unwrap();
        let create_dir_result = fs::create_dir_all(parent_path);

        self.add_or_replace_state(checkpoint);

        if create_dir_result.is_ok() {
            println!("Checkpoint directory created");
        } else {
            let err_text = format!("Error creating checkpoint directory: {:?}", parent_path);
            let error = io::Error::other(err_text);
            return Err::<(), io::Error>(error);
        }

        match state::DataPipelineCheckpointState::save(checkpoint, &path) {
            Ok(_) => {
                println!("Checkpoint saved");
                Ok(())
            }
            Err(error) => Err(error),
        }
    }

    /// Removes checkpoint metadat for a specified operation.
    pub fn clear_checkpoint(&mut self, operation: &str, context: &str) -> io::Result<()> {
        let path = self.checkpoint_path(operation, context);
        let checkpoint_path = path.parent().unwrap();
        if checkpoint_path.exists() {
            let remove_result = fs::remove_file(checkpoint_path);
            if remove_result.is_ok() {
                println!("Checkpoint '{:?}' cleared.", operation)
            } else {
                let err_text =
                    format!("Error creating checkpoint directory: {:?}", checkpoint_path);
                let error = io::Error::other(err_text);
                return Err::<(), io::Error>(error);
            }
        } else {
            println!("Checkpoint '{:?}' path does not exist.", operation,)
        }

        let state_index = self.state.iter().position(|x| x.operation == operation);
        if let Some(index) = state_index {
            self.state.remove(index);
        } else {
            println!(
                "Checkpoint state does not contain '{:?}' metadata.",
                operation
            )
        }

        Ok(())
    }
}
