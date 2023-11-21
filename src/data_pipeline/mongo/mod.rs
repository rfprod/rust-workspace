//! MongoDB module for the data pipeline.

use colored::Colorize;

mod configuration;
mod repos_collection;
mod workflows_collection;

/// The entry point of the program.
pub fn main(collections: Collections, collection_arg: Option<String>) {
    DataPipelineMongoDb::new(collections, collection_arg);
}

/// Supported collections.
pub type Collections<'a> = [&'a str; 2];
/// Supported collections.
pub const COLLECTIONS: Collections = ["repos", "workflows"];

struct DataPipelineMongoDb<'a> {
    collections: Collections<'a>,
}

impl<'a> DataPipelineMongoDb<'a> {
    /// Program constructor.
    fn new(collections: Collections, collection_arg: Option<String>) -> DataPipelineMongoDb {
        let mut program = DataPipelineMongoDb { collections };
        program.init(collection_arg);
        program
    }

    /// Initializes the program.
    fn init(&mut self, collection_arg: Option<String>) {
        println!("\n{}", "DataPipelineMongoDb initialized.".blue().bold());

        println!(
            "\n{} {:?}",
            "Selected collection".blue().bold(),
            collection_arg
        );

        let config = configuration::main(self.collections);

        let collection_index = config.choose_collection(collection_arg);

        match collection_index {
            0 => {
                let collection = self.collections[collection_index];
                let db = config.connect();
                let fs_config = config.fs_config(collection.to_owned());
                repos_collection::main(db, collection, &fs_config.json_data_dir);
            }
            1 => {
                let collection = self.collections[collection_index];
                let db = config.connect();
                let fs_config = config.fs_config(collection.to_owned());
                workflows_collection::main(db, collection, &fs_config.json_data_dir);
            }
            _ => {
                println!(
                    "\n{}",
                    "Nothing to execute. The collection is not supported"
                        .red()
                        .bold()
                )
            }
        }
    }
}
