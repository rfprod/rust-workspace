//! MongoDB repos module for the data pipeline.

use std::fs;

use colored::Colorize;
use mongodb::sync::Database;
use mongodb::{bson::doc, options::FindOneAndUpdateOptions};
use octorust::types::WorkflowRun;

/// The entry point of the program.
pub fn main(db: Database, collection_input: &str, json_data_dir: &str) {
    MongoDbWorkflowsCollection::new(db, collection_input, json_data_dir);
}

struct MongoDbWorkflowsCollection {
    db: Database,
}

impl MongoDbWorkflowsCollection {
    /// Program constructor.
    fn new(
        db: Database,
        collection_input: &str,
        json_data_dir: &str,
    ) -> MongoDbWorkflowsCollection {
        let mut program = MongoDbWorkflowsCollection { db };
        program.init(collection_input, json_data_dir);
        program
    }

    /// Initializes the program.
    fn init(&mut self, collection_input: &str, json_data_dir: &str) {
        println!(
            "\n{}",
            "MongoDbWorkflowsCollection initialized.".blue().bold()
        );

        let collection = "workflows";

        if collection_input.ne(collection) {
            panic!(
                "\n{}\n{:?} != {:?}\n",
                "Something went wrong. Input does not match expectation."
                    .bold()
                    .red(),
                collection_input,
                collection
            );
        }

        println!("{}: {:?}", "JSON data dir".cyan(), json_data_dir);

        if !json_data_dir.contains(collection) {
            panic!(
                "\n{}\nData dir: {:?}\nCollection: {:?}",
                "JSON data dir does not contain collection name".red(),
                json_data_dir,
                collection
            );
        }

        self.execute(collection, json_data_dir);
    }

    fn execute(&self, collection: &str, json_data_dir: &str) {
        let mut exists = false;
        match self.db.list_collection_names(None) {
            Ok(value) => {
                for col in value.iter() {
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
            self.update_collection(collection, json_data_dir);
        } else {
            self.create_collection(collection, json_data_dir);
        }
    }

    /// Collects documents for further processing.
    fn collect_documents(&self, json_data_dir: &str) -> Vec<Vec<WorkflowRun>> {
        let read_dir_result = fs::read_dir(json_data_dir);

        let Ok(dir_content) = read_dir_result else {
            panic!(
                "\n{} {:?}",
                "Can't read directory".red().bold(),
                json_data_dir
            );
        };

        let mut docs: Vec<Vec<WorkflowRun>> = vec![];

        for dir_entries_result in dir_content {
            let Ok(dir_entry) = dir_entries_result else {
                panic!("\n{}: {:?}", "Can't get dir entry", dir_entries_result);
            };
            println!("\n{}: {:?}", "Dir entry".green().bold(), dir_entry);

            let file_content_result = fs::read_to_string(dir_entry.path());
            let Ok(file_content) = file_content_result else {
                panic!("\n{}: {:?}", "Can't get file content", file_content_result);
            };

            let parse_result = serde_json::from_str::<Vec<WorkflowRun>>(&file_content);
            if let Ok(json) = parse_result {
                docs.push(json);
            } else {
                println!("Error serializing JSON file: {:?}", dir_entry.path());
            }
        }
        docs
    }

    /// Creates a collection and inserts data.
    fn create_collection(&self, collection: &str, json_data_dir: &str) {
        println!("\n{} {:?}", "Creating collection".cyan().bold(), collection);

        let docs: Vec<Vec<WorkflowRun>> = self.collect_documents(json_data_dir);

        let collection_ref = self.db.collection::<WorkflowRun>(collection);

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
    fn update_collection(&self, collection: &str, json_data_dir: &str) {
        println!("\n{} {:?}", "Updating collection".cyan().bold(), collection);

        let docs: Vec<Vec<WorkflowRun>> = self.collect_documents(json_data_dir);

        let collection_ref = self.db.collection::<WorkflowRun>(collection);

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
                            println!("{}: {:?}\n{:?}", "Updated".bold().green(), collection, url);
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
