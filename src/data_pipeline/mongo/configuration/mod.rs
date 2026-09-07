//! MongoDb configuration submodule.

use colored::Colorize;
use mongodb::sync::{Client, Database};
use std::env::{self};

use crate::data_pipeline::configuration::Collections;

pub fn main(collections: Collections<'_>) -> MongoDbConfiguration<'_> {
    MongoDbConfiguration::new(collections)
}

pub struct MongoDbFileConfig {
    pub json_data_dir: String,
}

pub struct MongoDbConfiguration<'a> {
    collections: Collections<'a>,
}

impl<'a> MongoDbConfiguration<'a> {
    /// Program constructor.
    fn new(collections: Collections<'a>) -> MongoDbConfiguration<'a> {
        MongoDbConfiguration { collections }
    }

    /// MongoDb module collection configuration.
    pub fn choose_collection(&self, collection: Option<String>) -> usize {
        let is_some = collection.is_some();
        let collection_input = if is_some {
            collection
                .unwrap()
                .trim()
                .parse::<String>()
                .unwrap_or_default()
        } else {
            String::new()
        };

        let mut index = usize::MAX;
        for (i, ctx) in self.collections.iter().enumerate() {
            if ctx.to_owned().eq(collection_input.as_str()) {
                index = i;
                break;
            }
        }

        index
    }

    /// Connects to the MongoDB instance and returns the database reference.
    pub fn connect(&self) -> Database {
        let connection_url_env = env::var("MONGODB_CONNECTION_STRING");
        let connection_url = connection_url_env
            .unwrap()
            .trim()
            .parse::<String>()
            .unwrap_or_default();

        let client_connection = Client::with_uri_str(&connection_url);
        let client = match client_connection {
            Ok(value) => value,
            Err(_) => {
                panic!("\nUnable to connect, connection URL: {}", connection_url);
            }
        };

        let db_name_env = env::var("MONGODB_DATABASE");
        let db_name = db_name_env
            .unwrap()
            .trim()
            .parse::<String>()
            .unwrap_or_default();

        let db = client.database(db_name.as_str());

        match db.list_collection_names().run() {
            Ok(value) => {
                for col in value.iter() {
                    println!("\n{}: {:?}", "Collection".bold().cyan(), col);
                }
            }
            Err(err) => {
                panic!("\nUnable to list collection names\n {:?}", err);
            }
        };
        db
    }

    /// MongoDb module file system configuration.
    pub fn fs_config(&self, collection: String) -> MongoDbFileConfig {
        let cwd = match env::current_dir() {
            Ok(value) => {
                println!("{}: {:?}", "Current directory".cyan().bold(), value);
                value.display().to_string()
            }
            Err(error) => {
                panic!("{:?}", error);
            }
        };

        let json_base_path = cwd + "/.data/output/github/";
        let json_data_dir = json_base_path + collection.as_str() + "/";

        MongoDbFileConfig { json_data_dir }
    }
}

#[cfg(test)]
mod tests;
