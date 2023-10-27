use std::{
    env::{self},
    fs,
};

use colored::Colorize;
use mongodb::sync::Client;
use octorust::types::Repository;

/// The entry point of the program.
pub fn main(collection_arg: Option<String>) {
    DataPipelineMongoDb::new(collection_arg);
}

/// Supported collections.
type Collections<'a> = [&'a str; 1];

struct DataPipelineMongoDb;

impl DataPipelineMongoDb {
    /// Creates a new program for working with MongoDb.
    fn new(collection_arg: Option<String>) -> DataPipelineMongoDb {
        let mut program = DataPipelineMongoDb;
        program.init(collection_arg);
        program
    }

    /// Initializes the program.
    fn init(&mut self, collection: Option<String>) {
        println!("\n{}", "DataPipelineMongoDb initialized.".blue().bold());

        println!("\n{} {:?}", "Selected collection".blue().bold(), collection);

        let collections: Collections = ["repos"];

        let collection_index = self.choose_collection(collections, collection);

        match collection_index {
            0 => {
                let collection = collections[collection_index];
                self.create_collection(collection);
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
    fn choose_collection(&self, collections: Collections, collection_arg: Option<String>) -> usize {
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
        for (i, ctx) in collections.iter().enumerate() {
            if ctx.to_owned().eq(collection_arg_input.as_str()) {
                index = i;
                break;
            }
        }

        index
    }

    /// Creates a collection and inserts data.
    fn create_collection(&self, collection: &str) {
        println!("\n{} {:?}", "Creating collection".cyan().bold(), collection);
        let cwd = env::current_dir().unwrap();
        println!(
            "\n{}:\n{:?}",
            "The current directory is".cyan().bold(),
            cwd.display()
        );
        let base_path = cwd.display().to_string() + "/.data/output/github/";
        let dir_content_result = fs::read_dir(&base_path);

        let Ok(dir_content) = dir_content_result else {
            panic!("\n{} {:?}", "Can't read directory", base_path);
        };

        let mut docs: Vec<Repository> = vec![];

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
            if let Ok(mut json) = parse_result {
                docs.append(&mut json);
            } else {
                println!("Error serializing JSON file: {:?}", dir_entry.path());
            }
        }

        let connection_url_env = env::var("MONGODB_CONNECTION_STRING");
        let connection_url = match connection_url_env.unwrap().trim().parse::<String>() {
            Ok(value) => value,
            Err(_) => String::new(),
        };

        let client = Client::with_uri_str(connection_url).expect("Can't get Mongo client");

        let db = client.database("local");

        if let Ok(collection_name) = db.list_collection_names(None) {
            for (_i, col) in collection_name.iter().enumerate() {
                println!("{}", col);
            }
        }

        let collection_ref = db.collection::<Repository>(collection);

        match collection_ref.drop(None) {
            Ok(_) => {
                println!("\n{}: {:?}", "Dropped collection".green(), collection);
            }
            Err(err) => {
                println!(
                    "\n{}: {:?}\n{:?}",
                    "Can't drop collection".bold().red(),
                    collection,
                    err
                );
            }
        };

        match collection_ref.insert_many(docs, None) {
            Ok(_) => {
                println!(
                    "\n{}: {:?}",
                    "Inserted documents, collection".green(),
                    collection
                );
            }
            Err(err) => {
                println!(
                    "\n{}: {:?}\n{:?}",
                    "Can't insert documents, collection".bold().red(),
                    collection,
                    err
                );
            }
        };
    }
}