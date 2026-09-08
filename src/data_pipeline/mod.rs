//! Data Pipeline module.
//!
//! Environment variables should be placed in the `.env` file:
//!
//! GITHUB_TOKEN=...
//! GPG_PASSPHRASE=...
//! MONGODB_CONNECTION_STRING=...
//! MONGODB_DATABASE=...

use colored::Colorize;
use octorust::{
    types::SearchReposSort,
    types::{Order, RepoSearchResultItem, Repository, WorkflowRun},
};
use std::{
    env,
    fs::{self, File},
    io,
};

mod artifact;
mod checkpoint;
mod configuration;
mod environment;
mod github;
mod mongo;

/// The entry point of the program.
pub fn main() {
    DataPipeline::new();
}

struct DataPipeline<'a> {
    checkpoint: checkpoint::DataPipelineCheckpoint,
    contexts: configuration::Contexts<'a>,
    collections: configuration::Collections<'a>,
    configuration: configuration::DataPipelineConfiguration<'a>,
    github: github::DataPipelineGitHub,
    runtime: tokio::runtime::Runtime,
}

impl<'a> DataPipeline<'a> {
    /// Program constructor.
    fn new() -> DataPipeline<'a> {
        let checkpoint: checkpoint::DataPipelineCheckpoint = checkpoint::main();
        let contexts: configuration::Contexts = configuration::CONTEXTS;
        let collections: configuration::Collections = configuration::COLLECTIONS;
        let configuration: configuration::DataPipelineConfiguration =
            configuration::main(contexts, collections);
        let github: github::DataPipelineGitHub = github::main();
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let mut program = DataPipeline {
            checkpoint,
            contexts,
            collections,
            configuration,
            github,
            runtime,
        };
        program.init();
        program
    }

    /// Initializes the program.
    fn init(&mut self) {
        println!("\n{}", "DataPipeline initialized.".blue().bold());

        environment::main(None);

        let args = self.configuration.args();

        let context_arg = args.context.to_owned();

        let context_index = self.configuration.choose_context(context_arg);

        let context = self.contexts[context_index];

        let collection_arg = args.collection.to_owned();

        let collection_index = self.configuration.choose_collection(collection_arg);

        let collection = self.collections[collection_index];

        match context_index {
            0 => self.execute(
                args.search_term,
                args.force_restart,
                context.to_owned(),
                collection.to_owned(),
            ),
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

    /// Clears a checkpoint for the operation.
    fn force_restart(&mut self, operation: &str, context: &str) {
        let clear_result = self.checkpoint.clear_checkpoint(operation, context);
        if clear_result.is_ok() {
            println!("Cleared {:?} checkpoint", operation);
        } else {
            println!("Error clearing {:?} checkpoint", operation);
        }
    }

    /// The data pipeline program for the provided search_term.
    fn execute(
        &mut self,
        search_term_arg: Option<String>,
        force_restart: bool,
        context: String,
        collection: String,
    ) {
        let is_some = search_term_arg.is_some();
        let search_term_arg_input = if is_some {
            search_term_arg
                .unwrap()
                .trim()
                .parse::<String>()
                .unwrap_or_default()
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
            "repos" => {
                let operation = "repos_collector";
                if force_restart {
                    self.force_restart(operation, &context);
                }
                self.execute_repos_collector(operation, search_term_input.clone());
            }
            "workflows" => {
                let operation = "workflows_collector";
                if force_restart {
                    self.force_restart(operation, &context);
                }
                self.execute_workflows_collector(operation);
            }
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
    fn execute_repos_collector(&mut self, operation: &str, search_term_input: String) {
        let search_term = search_term_input.as_str().trim().to_string();

        println!("\n{}: {}", "Your search term".cyan(), search_term);

        let mut checkpoint_state = self
            .checkpoint
            .load_checkpoint(operation, &search_term)
            .unwrap_or_else(|| {
                checkpoint::DataPipelineCheckpoint::new_state(
                    operation.to_string(),
                    0,
                    5,
                    None,
                    0,
                    chrono::Local::now().to_rfc3339(),
                    search_term.clone(),
                )
            });

        let query_string =
            "  in:name in:description in:readme user:".to_string() + search_term.as_str();
        let q = query_string.as_str();
        // let per_page = 5;

        loop {
            let page = checkpoint_state.position + 1;

            let mut fetch_result = github::ReposFetchResult {
                items: Vec::<RepoSearchResultItem>::new(),
                total: 0,
                retry: false,
            };

            self.runtime.block_on(async {
                let result = self
                    .github
                    .repos_request(
                        q,
                        SearchReposSort::Noop,
                        Order::Asc,
                        checkpoint_state.per_page,
                        page,
                    )
                    .await;
                fetch_result = match result {
                    Ok(data) => {
                        if !data.retry {
                            let cwd = env::current_dir().unwrap();
                            println!("The current directory is {}", cwd.display());
                            let base_path = cwd.display().to_string()
                                + self.github.configuration.repos_output.as_str();
                            let create_dir_result = fs::create_dir_all(&base_path);
                            if let Ok(_tmp) = create_dir_result {
                                let path =
                                    base_path + "/github-repos-" + &page.to_string() + ".json";
                                let file = File::create(path).unwrap();
                                let _result = serde_json::to_writer_pretty(file, &data.items);
                            }

                            // Update and save checkpoint
                            checkpoint_state.position = page;
                            checkpoint_state.completed_count +=
                                i64::try_from(data.items.len()).unwrap();
                            checkpoint_state.total_items = Some(data.total);
                            checkpoint_state.last_updated = chrono::Local::now().to_rfc3339();

                            if let Err(e) = self.checkpoint.save_checkpoint(&checkpoint_state) {
                                eprintln!("Failed to save checkpoint: {:?}", e);
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

            let progress = checkpoint_state.completed_count;
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
                self.checkpoint
                    .clear_checkpoint(operation, &search_term)
                    .ok();
                break;
            }
        }
    }

    /// Collects repository workflow metadata.
    fn execute_workflows_collector(&mut self, operation: &str) {
        let context = "workflows".to_string();
        let mut checkpoint_state = self
            .checkpoint
            .load_checkpoint(operation, &context)
            .unwrap_or_else(|| {
                checkpoint::DataPipelineCheckpoint::new_state(
                    operation.to_string(),
                    0,
                    100,
                    None,
                    0,
                    chrono::Local::now().to_rfc3339(),
                    context.to_string(),
                )
            });

        let records = self.collect_documents();
        let mut record_index = checkpoint_state.position;
        let records_len = i64::try_from(records.len()).unwrap();

        let mut workflows_len = 0;

        while record_index < records_len {
            let record = &records[record_index as usize];
            let mut fetch_result = github::WorkflowRunsFetchResult {
                items: Vec::<WorkflowRun>::new(),
                total: records_len,
                retry: false,
            };

            let owner = &record.owner.clone().unwrap().login;
            println!("owner {:?}", owner);
            let repo = &record.name;
            println!("repo {:?}", repo);
            let branch = &record.default_branch;
            println!("branch {:?}", branch);

            self.runtime.block_on(async {
                let result = self
                    .github
                    .workflow_runs_request(owner, repo, branch, "", checkpoint_state.per_page, 1)
                    .await;
                fetch_result = match result {
                    Ok(data) => {
                        if data.retry {
                            record_index -= 1;
                        } else {
                            let cwd = env::current_dir().unwrap();
                            println!("The current directory is {}", cwd.display());
                            let base_path = cwd.display().to_string()
                                + self.github.configuration.workflows_output.as_str();
                            let create_dir_result = fs::create_dir_all(&base_path);
                            if let Ok(_tmp) = create_dir_result {
                                if !data.items.is_empty() {
                                    let path = base_path + "/" + &record.name + ".json";
                                    let file = File::create(path).unwrap();
                                    let _result = serde_json::to_writer_pretty(file, &data.items);
                                }
                            }

                            workflows_len += data.total;

                            // Update checkpoint on successful completion
                            checkpoint_state.position = record_index + 1;
                            checkpoint_state.completed_count += 1;
                            checkpoint_state.total_items = Some(workflows_len);
                            checkpoint_state.last_updated = chrono::Local::now().to_rfc3339();

                            if let Err(e) = self.checkpoint.save_checkpoint(&checkpoint_state) {
                                eprintln!("Failed to save checkpoint: {:?}", e);
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
                "\n{}: {:?}/{:?} (Workflows: {:?})",
                "Progress/Total".green().bold(),
                record_index,
                records_len,
                workflows_len
            );
        }

        // Clear checkpoint on completion
        self.checkpoint.clear_checkpoint(operation, &context).ok();
    }

    /// Collects repository records for the repository workflow collector.
    fn collect_documents(&self) -> Vec<Repository> {
        let cwd = env::current_dir().unwrap();
        println!(
            "\n{}:\n{:?}",
            "The current directory is".cyan().bold(),
            cwd.display()
        );
        let json_data_dir = self.github.configuration.repos_output.as_str();
        let base_path = cwd.display().to_string() + json_data_dir;
        println!("\n{}:\n{:?}", "Base path".cyan().bold(), base_path);
        let dir_content_result = fs::read_dir(&base_path);

        let Ok(dir_content) = dir_content_result else {
            panic!("\n{} {:?}", "Can't read directory".red().bold(), base_path);
        };

        let mut docs: Vec<Repository> = vec![];

        for dir_entries_result in dir_content {
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
