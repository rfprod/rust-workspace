/// MongoDB module for the data pipeline.
///
use std::env::{self};

use colored::Colorize;
use mongodb::sync::{Client, Database};

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
    fn init(&mut self, collection: Option<String>) {
        println!("\n{}", "DataPipelineMongoDb initialized.".blue().bold());

        println!("\n{} {:?}", "Selected collection".blue().bold(), collection);

        let collection_index = self.choose_collection(collection);

        match collection_index {
            0 => {
                let collection = self.collections[collection_index];
                let db = self.connect();
                repos_collection::main(db, collection);
            }
            1 => {
                let collection = self.collections[collection_index];
                let db = self.connect();
                workflows_collection::main(db, collection);
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

    /// Finds selected collection and returns the collection index.
    fn choose_collection(&self, collection_arg: Option<String>) -> usize {
        let is_some = collection_arg.is_some();
        let collection_arg_input = if is_some {
            match collection_arg.unwrap().trim().parse::<String>() {
                Ok(value) => value,
                Err(_) => String::new(),
            }
        } else {
            String::new()
        };

        let mut index = usize::MAX;
        for (i, ctx) in self.collections.iter().enumerate() {
            if ctx.to_owned().eq(collection_arg_input.as_str()) {
                index = i;
                break;
            }
        }

        index
    }

    /// Connects to the MongoDB instance and returns the database reference.
    fn connect(&self) -> Database {
        let connection_url_env = env::var("MONGODB_CONNECTION_STRING");
        let connection_url = match connection_url_env.unwrap().trim().parse::<String>() {
            Ok(value) => value,
            Err(_) => String::new(),
        };

        let client_connection = Client::with_uri_str(&connection_url);
        let client = match client_connection {
            Ok(value) => value,
            Err(_) => {
                panic!("\nUnable to connect, connection URL: {}", connection_url);
            }
        };

        let db_name_env = env::var("MONGODB_DATABASE");
        let db_name = match db_name_env.unwrap().trim().parse::<String>() {
            Ok(value) => value,
            Err(_) => String::new(),
        };

        let db = client.database(db_name.as_str());

        match db.list_collection_names(None) {
            Ok(value) => {
                for (_i, col) in value.iter().enumerate() {
                    println!("\n{}: {:?}", "Collection".bold().cyan(), col);
                }
            }
            Err(err) => {
                panic!("\nUnable to list collection names\n {:?}", err);
            }
        };
        db
    }
}
