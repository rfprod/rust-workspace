//! Data Pipeline Checkpoint State module.

use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataPipelineCheckpointState {
    /// Search term or other relevant context.
    pub context: String,
    /// Context/operation being performed (e.g., "repos", "workflows").
    pub operation: String,
    /// Current page or record index being processed.
    pub position: i64,
    /// Paginator configuration.
    pub per_page: i64,
    /// Total items to process.
    pub total_items: Option<i64>,
    /// Items already successfully processed.
    pub completed_count: i64,
    /// Timestamp of last checkpoint.
    pub last_updated: String,
}

impl DataPipelineCheckpointState {
    /// Create a checkpoint state.
    pub fn new(
        operation: String,
        position: i64,
        per_page: i64,
        total_items: Option<i64>,
        completed_count: i64,
        last_updated: String,
        context: String,
    ) -> DataPipelineCheckpointState {
        DataPipelineCheckpointState {
            operation,
            position,
            per_page,
            total_items,
            completed_count,
            last_updated,
            context,
        }
    }

    /// Save a checkpoint.
    pub fn save(
        state: &DataPipelineCheckpointState,
        checkpoint_path: &PathBuf,
    ) -> io::Result<bool> {
        let json_stringify = serde_json::to_string_pretty(&state);
        if let Ok(string) = json_stringify {
            let write_result = fs::write(checkpoint_path, string);
            if write_result.is_ok() {
                Ok(true)
            } else {
                let err_text = format!("Error writing checkpoint state: {:?}", state);
                let error = io::Error::other(err_text);
                Err(error)
            }
        } else {
            let err_text = format!("Error stringifying checkpoint state: {:?}", state);
            let error = io::Error::other(err_text);
            Err(error)
        }
    }

    /// Load a checkpoint.
    pub fn load(checkpoint_path: &PathBuf) -> io::Result<DataPipelineCheckpointState> {
        let read_json = fs::read_to_string(checkpoint_path);
        if let Ok(json) = read_json {
            let parse_result = serde_json::from_str::<DataPipelineCheckpointState>(&json);
            if let Ok(json) = parse_result {
                Ok(json)
            } else {
                let err_text = format!(
                    "Error serializing checkpoint JSON file: {:?}",
                    checkpoint_path
                );
                let error = io::Error::other(err_text);
                Err(error)
            }
        } else {
            let err_text = format!("Error reading checkpoint JSON file: {:?}", checkpoint_path);
            let error = io::Error::other(err_text);
            Err(error)
        }
    }
}
