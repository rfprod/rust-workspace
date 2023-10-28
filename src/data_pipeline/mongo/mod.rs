/// MongoDB module for the data pipeline.
///
use std::{
    env::{self},
    fs,
};

use colored::Colorize;
use mongodb::sync::{Client, Database};
use mongodb::{bson::doc, options::FindOneAndUpdateOptions};
use octorust::types::Repository;

/// The entry point of the program.
pub fn main(collections: Collections, collection_arg: Option<String>) {
    DataPipelineMongoDb::new(collections, collection_arg);
}

/// Supported collections.
pub type Collections<'a> = [&'a str; 1];
/// Supported collections.
pub const COLLECTIONS: Collections = ["repos"];

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
                self.execute(collection);
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

    /// Collects documents for further processing.
    fn collect_doccuments(&self) -> Vec<Vec<Repository>> {
        let cwd = env::current_dir().unwrap();
        println!(
            "\n{}:\n{:?}",
            "The current directory is".cyan().bold(),
            cwd.display()
        );
        let base_path = cwd.display().to_string() + "/.data/output/github/";
        let dir_content_result = fs::read_dir(&base_path);

        let Ok(dir_content) = dir_content_result else {
            panic!("\n{} {:?}", "Can't read directory".red().bold(), base_path);
        };

        let mut docs: Vec<Vec<Repository>> = vec![];

        let dir_entries = dir_content.enumerate();
        for (_i, dir_entries_result) in dir_entries {
            let Ok(dir_entry) = dir_entries_result else {
                panic!("\n{}: {:?}", "Can't get dir entry", dir_entries_result);
            };
            println!("\n{}: {:?}", "Dir entry".green().bold(), dir_entry);

            let file_content_result = fs::read_to_string(dir_entry.path());
            let Ok(file_content) = file_content_result else {
                panic!("\n{}: {:?}", "Can't get file content", file_content_result);
            };

            let parse_result = serde_json::from_str::<Vec<Repository>>(&file_content);
            if let Ok(json) = parse_result {
                docs.push(json);
            } else {
                println!("Error serializing JSON file: {:?}", dir_entry.path());
            }
        }
        docs
    }

    fn execute(&self, collection: &str) {
        let db = self.connect();

        let mut exists = false;
        match db.list_collection_names(None) {
            Ok(value) => {
                for (_i, col) in value.iter().enumerate() {
                    if collection.eq(col) {
                        exists = true;
                    }
                }
            }
            Err(err) => {
                panic!("\nUnable to list collection names\n {:?}", err);
            }
        };

        if exists {
            self.update_collection(collection);
        } else {
            self.create_collection(collection);
        }
    }

    /// Creates a collection and inserts data.
    fn create_collection(&self, collection: &str) {
        println!("\n{} {:?}", "Creating collection".cyan().bold(), collection);

        let docs: Vec<Vec<Repository>> = self.collect_doccuments();

        let db = self.connect();

        let collection_ref = db.collection::<Repository>(collection);

        match collection_ref.drop(None) {
            Ok(_) => {
                println!("\n{}: {:?}", "Dropped".bold().green(), collection);
            }
            Err(err) => {
                println!(
                    "\n{}: {:?}\n{:?}",
                    "Can't drop".bold().red(),
                    collection,
                    err
                );
            }
        };

        for batch in docs {
            match collection_ref.insert_many(batch, None) {
                Ok(_) => {
                    println!("\n{}: {:?}", "Inserted in".green(), collection);
                }
                Err(err) => {
                    println!(
                        "\n{}: {:?}\n{:?}",
                        "Can't insert in".bold().red(),
                        collection,
                        err
                    );
                }
            };
        }
    }

    /// Updates documents in the collection.
    fn update_collection(&self, collection: &str) {
        println!("\n{} {:?}", "Updating collection".cyan().bold(), collection);

        let docs: Vec<Vec<Repository>> = self.collect_doccuments();

        let db = self.connect();

        let collection_ref = db.collection::<Repository>(collection);

        for batch in docs {
            for record in batch.iter().cloned() {
                let url = &record.url;
                let filter = doc! { "url": url };
                let record_bson = mongodb::bson::to_bson(&record).unwrap();
                if let mongodb::bson::Bson::Document(document) = record_bson {
                    let mut options = FindOneAndUpdateOptions::default();
                    options.upsert = Some(true);
                    let update = doc! { "$set": document };
                    match collection_ref.find_one_and_update(filter, update, options) {
                        Ok(_) => {
                            println!("\n{}: {:?}", "Updated".bold().green(), collection);
                        }
                        Err(err) => {
                            println!(
                                "\n{}: {:?}\n{:?}",
                                "Can't update".bold().red(),
                                collection,
                                err
                            );
                        }
                    }
                }
            }
        }
    }
}
