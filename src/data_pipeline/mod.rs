/// Data pipeline module.
///
/// Environment variables should be placed in the `.env` file:
///
/// GITHUB_TOKEN=...
/// GPG_PASSPHRASE=...
/// MONGODB_CONNECTION_STRING=...
/// MONGODB_DATABASE=...
///
use colored::Colorize;
use octorust::{
    types::SearchReposSort,
    types::{Order, RepoSearchResultItem, Repository, WorkflowRun},
};
use std::{
    cmp::Ordering,
    env::{self, args},
    fs::{self, File},
    io,
};

mod artifact;
mod environment;
mod github;
mod mongo;

/// The entry point of the program.
pub fn main() {
    DataPipeline::new();
}

/// Input arguments of the program.
struct InuputArguments {
    context: Option<String>,
    collection: Option<String>,
    search_term: Option<String>,
}

struct DataPipeline<'a> {
    contexts: artifact::Contexts<'a>,
    collections: mongo::Collections<'a>,
}

impl<'a> DataPipeline<'a> {
    /// Program constructor.
    fn new() -> DataPipeline<'a> {
        let contexts: artifact::Contexts = artifact::CONTEXTS;
        let collections: mongo::Collections = mongo::COLLECTIONS;
        let mut program = DataPipeline {
            contexts,
            collections,
        };
        program.init();
        program
    }

    /// Initializes the program.
    fn init(&mut self) {
        println!("\n{}", "DataPipeline initialized.".blue().bold());

        environment::main(None);

        let args = self.args();

        let context_arg = args.context.to_owned();

        let context_index = self.choose_context(context_arg);

        let context = self.contexts[context_index];

        let collection_arg = args.collection.to_owned();

        let collection_index = self.choose_collection(collection_arg);

        let collection = self.collections[collection_index];

        match context_index {
            0 => self.execute(args.search_term, context.to_owned(), collection.to_owned()),
            1 => {
                artifact::main(
                    self.contexts,
                    Some(context.to_owned()),
                    collection.to_owned(),
                );
                mongo::main(self.collections, Some(collection.to_owned()));
            }
            _ => {
                println!(
                    "\n{}",
                    "Nothing to execute. The context is not supported"
                        .red()
                        .bold()
                )
            }
        }
    }

    /// Parses arguments passed to the program.
    fn args(&mut self) -> InuputArguments {
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
    fn choose_context(&self, context_arg: Option<String>) -> usize {
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
    fn choose_collection(&self, collection_arg: Option<String>) -> usize {
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

    /// The data pipeline program for the provided search_term.
    fn execute(&mut self, search_term_arg: Option<String>, context: String, collection: String) {
        let is_some = search_term_arg.is_some();
        let search_term_arg_input = if is_some {
            match search_term_arg.unwrap().trim().parse::<String>() {
                Ok(value) => value,
                Err(_) => String::new(),
            }
        } else {
            String::new()
        };

        let mut search_term_input = String::new();

        if search_term_arg_input.trim().is_empty() && search_term_input.trim().is_empty() {
            println!(
                "\n{}",
                "Please input a search term (GitHub user):".yellow().bold()
            );

            io::stdin()
                .read_line(&mut search_term_input)
                .expect("Failed to read line");
        } else if search_term_input.trim().is_empty() {
            search_term_input = search_term_arg_input;
        }

        let col = collection.as_str();

        match col {
            "repos" => self.execute_repos_collector(search_term_input.clone()),
            "workflows" => self.execute_workflows_collector(),
            _ => panic!(
                "\n{}: {:?}",
                "Nothing to execute. The collection is not supported"
                    .red()
                    .bold(),
                col
            ),
        }

        artifact::main(self.contexts, Some(context), collection.to_owned());
        mongo::main(self.collections, Some(collection));
    }

    /// Collects repository metadata.
    fn execute_repos_collector(&self, search_term_input: String) {
        let mut page = 0;

        loop {
            let search_term = search_term_input.as_str().trim().to_string();

            println!("\n{}: {}", "Your search term".cyan(), search_term);

            let query_string =
                "  in:name in:description in:readme user:".to_string() + search_term.as_str();
            let q = query_string.as_str();
            let per_page = 5;
            page += 1;

            let mut fetch_result = github::ReposFetchResult {
                items: Vec::<RepoSearchResultItem>::new(),
                total: 0,
                retry: false,
            };

            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            runtime.block_on(async {
                let result =
                    github::repos(q, SearchReposSort::Noop, Order::Asc, per_page, page).await;
                fetch_result = match result {
                    Ok(data) => {
                        if data.retry {
                            page -= 1;
                        } else {
                            let cwd = env::current_dir().unwrap();
                            println!("The current directory is {}", cwd.display());
                            let base_path =
                                cwd.display().to_string() + "/.data/output/github/repos";
                            let create_dir_result = fs::create_dir_all(&base_path);
                            if let Ok(_tmp) = create_dir_result {
                                let path =
                                    base_path + "/github-repos-" + &page.to_string() + ".json";
                                let file = File::create(path).unwrap();
                                let _result = serde_json::to_writer_pretty(file, &data.items);
                            }
                        }

                        data
                    }
                    Err(error) => {
                        println!("\n{}: {:?}", "There was an error".red(), error);
                        let items = Vec::<RepoSearchResultItem>::new();
                        let total: i64 = 0;
                        github::ReposFetchResult {
                            items,
                            total,
                            retry: false,
                        }
                    }
                };
            });

            let progress = page * per_page;
            println!(
                "\n{}: {:?}/{:?}",
                "Progress/Total".green().bold(),
                progress,
                fetch_result.total
            );

            if fetch_result.total > progress {
                continue;
            } else {
                println!("\n{}", "Download complete".green().bold());
                break;
            }
        }
    }

    /// Collects repository workflow metadata.
    fn execute_workflows_collector(&self) {
        let records = self.collect_documents();
        let mut record_index = 0;
        let records_len = records.len();

        while record_index < records_len {
            let record = &records[record_index];
            let mut fetch_result = github::WorkflowRunsFetchResult {
                items: Vec::<WorkflowRun>::new(),
                total: 0,
                retry: false,
            };

            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            let owner = &record.owner.clone().unwrap().login;
            println!("owner {:?}", owner);
            let repo = &record.name;
            println!("repo {:?}", repo);
            let branch = &record.default_branch;
            println!("branch {:?}", branch);

            runtime.block_on(async {
                let result = github::workflow_runs(owner, repo, branch, "", 100, 1).await;
                fetch_result = match result {
                    Ok(data) => {
                        if data.retry {
                            record_index -= 1;
                        } else {
                            let cwd = env::current_dir().unwrap();
                            println!("The current directory is {}", cwd.display());
                            let base_path =
                                cwd.display().to_string() + "/.data/output/github/workflows";
                            let create_dir_result = fs::create_dir_all(&base_path);
                            if let Ok(_tmp) = create_dir_result {
                                if !data.items.is_empty() {
                                    let path = base_path + "/" + &record.name + ".json";
                                    let file = File::create(path).unwrap();
                                    let _result = serde_json::to_writer_pretty(file, &data.items);
                                }
                            }
                            record_index += 1;
                        }
                        data
                    }
                    Err(error) => {
                        println!("\n{}: {:?}", "There was an error".red(), error);
                        let items = Vec::<WorkflowRun>::new();
                        let total: i64 = 0;
                        github::WorkflowRunsFetchResult {
                            items,
                            total,
                            retry: false,
                        }
                    }
                };
            });

            println!(
                "\n{}: {:?}/{:?}",
                "Progress/Total".green().bold(),
                record_index,
                records_len
            );
        }
    }

    /// Collects repository records for the repository workflow collector.
    fn collect_documents(&self) -> Vec<Repository> {
        let cwd = env::current_dir().unwrap();
        println!(
            "\n{}:\n{:?}",
            "The current directory is".cyan().bold(),
            cwd.display()
        );
        let json_data_dir = "/.data/output/github/repos/";
        let base_path = cwd.display().to_string() + json_data_dir;
        println!("\n{}:\n{:?}", "Base path".cyan().bold(), base_path);
        let dir_content_result = fs::read_dir(&base_path);

        let Ok(dir_content) = dir_content_result else {
            panic!("\n{} {:?}", "Can't read directory".red().bold(), base_path);
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
        docs
    }
}
