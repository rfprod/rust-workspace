//! Data pipeline configuration module.

use colored::Colorize;
use std::{cmp::Ordering, env::args, io};

/// The entry point of the program.
pub fn main<'a>(
    contexts: [&'a str; 2],
    collections: [&'a str; 2],
) -> DataPipelineConfiguration<'a> {
    DataPipelineConfiguration::new(contexts, collections)
}

/// Input arguments of the program.
pub struct InuputArguments {
    pub(crate) context: Option<String>,
    pub(crate) collection: Option<String>,
    pub(crate) search_term: Option<String>,
}

pub struct DataPipelineConfiguration<'a> {
    contexts: [&'a str; 2],
    collections: [&'a str; 2],
}

impl<'a> DataPipelineConfiguration<'a> {
    /// Program constructor.
    fn new(contexts: [&'a str; 2], collections: [&'a str; 2]) -> DataPipelineConfiguration<'a> {
        DataPipelineConfiguration {
            contexts,
            collections,
        }
    }

    /// Parses arguments passed to the program.
    pub fn args(&mut self) -> InuputArguments {
        let arguments: Vec<String> = args().collect();

        println!("\n{}:\n{:?}", "Arguments".cyan().bold(), arguments);

        let context = arguments.get(2).cloned();
        println!("- context: {:?}", context);
        let collection = arguments.get(3).cloned();
        println!("- collection: {:?}", collection);
        let search_term = arguments.get(4).cloned();
        println!("- search_term: {:?}", search_term);

        InuputArguments {
            context,
            collection,
            search_term,
        }
    }

    /// Prompts input from the user, processes it, and returns the index of the selected context.
    pub fn choose_context(&self, context_arg: Option<String>) -> usize {
        let is_some = context_arg.is_some();
        let mut context_arg_input = if is_some {
            match context_arg.unwrap().trim().parse::<String>() {
                Ok(value) => value,
                Err(_) => String::new(),
            }
        } else {
            String::new()
        };

        loop {
            let mut context_input = String::new();

            if context_arg_input.is_empty() {
                self.print_context_instructions();

                io::stdin()
                    .read_line(&mut context_input)
                    .expect("Failed to read line");
            } else {
                context_input = context_arg_input.to_string();
            }

            let context_index = match context_input.trim().parse::<usize>() {
                Ok(num) => num,
                Err(_) => continue,
            };

            match context_index.cmp(&self.contexts.len()) {
                Ordering::Less => {
                    return self.select_context(context_index);
                }
                Ordering::Greater => context_arg_input = self.reset_input_arg(),
                Ordering::Equal => context_arg_input = self.reset_input_arg(),
            }
        }
    }

    /// Prints instructions for selecting a context.
    fn print_context_instructions(&self) {
        println!("\n{}", "Available contexts:".yellow().bold());

        let max_i = self.contexts.len() - 1;
        let mut i = 0;
        while i <= max_i {
            println!("{}: {}", i, self.contexts[i]);
            i += 1;
        }

        println!(
            "\n{}, [0-{}]:",
            "Please select a context".yellow().bold(),
            max_i
        );
    }

    /// Prints selected context and returns the context index.
    fn select_context(&self, context_index: usize) -> usize {
        let context = self.contexts[context_index];
        println!("You selected: {}", context);
        context_index
    }

    /// Prompts input from the user, processes it, and returns the index of the selected context.
    pub fn choose_collection(&self, collection_arg: Option<String>) -> usize {
        let is_some = collection_arg.is_some();
        let mut collection_arg_input = if is_some {
            match collection_arg.unwrap().trim().parse::<String>() {
                Ok(value) => value,
                Err(_) => String::new(),
            }
        } else {
            String::new()
        };

        loop {
            let mut collection_input = String::new();

            if collection_arg_input.is_empty() {
                self.print_collection_instructions();

                io::stdin()
                    .read_line(&mut collection_input)
                    .expect("Failed to read line");
            } else {
                collection_input = collection_arg_input.to_string();
            }

            let collection_index = match collection_input.trim().parse::<usize>() {
                Ok(num) => num,
                Err(_) => continue,
            };

            match collection_index.cmp(&self.collections.len()) {
                Ordering::Less => {
                    return self.select_collection(collection_index);
                }
                Ordering::Greater => collection_arg_input = self.reset_input_arg(),
                Ordering::Equal => collection_arg_input = self.reset_input_arg(),
            }
        }
    }

    /// Prints instructions for selecting a collection.
    fn print_collection_instructions(&self) {
        println!("\n{}", "Available collections:".yellow().bold());

        let max_i = self.collections.len() - 1;
        let mut i = 0;
        while i <= max_i {
            println!("{}: {}", i, self.collections[i]);
            i += 1;
        }

        println!(
            "\n{}, [0-{}]:",
            "Please select a collection".yellow().bold(),
            max_i
        );
    }

    /// Prints selected collection and returns the collection index.
    fn select_collection(&self, collection_index: usize) -> usize {
        let collection = self.collections[collection_index];
        println!("You selected: {}", collection);
        collection_index
    }

    /// Resets the input argument to start over if the option does not exist.
    fn reset_input_arg(&self) -> String {
        println!("\n{}", "Invalid option.".red());
        String::new()
    }
}

#[cfg(test)]
mod tests;
