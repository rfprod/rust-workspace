use colored::Colorize;
use octorust::{
    auth::Credentials,
    types::SearchReposSort,
    types::{Order, RepoSearchResultItem},
    Client,
};
use std::{
    cmp::Ordering,
    collections::HashMap,
    env::{self, args, Args},
    fs::{self, File},
    io,
    process::Command,
};

mod artifact;
mod mongo;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

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

/// GitHub repos fetch result.
struct FetchResult {
    items: Vec<RepoSearchResultItem>,
    total: i64,
    retry: bool,
}

/// Supported contexts.
type Contexts<'a> = [&'a str; 2];

/// Supported collections.
type Collections<'a> = [&'a str; 1];

struct DataPipeline<'a> {
    contexts: Contexts<'a>,
    collections: Collections<'a>,
}

impl<'a> DataPipeline<'a> {
    /// Creates a new data pipeline.
    fn new() -> DataPipeline<'a> {
        let contexts: Contexts = ["Create artifact", "Restore artifact"];
        let collections: Collections = ["repos"];
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

        let _result = self.load_env_vars();

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
                artifact::main(self.contexts, args.context);
                mongo::main(self.collections, args.collection);
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

    /// Parses the data pipeline arguments.
    fn args(&mut self) -> InuputArguments {
        let mut args: Args = args();

        println!("\n{}:\n{:?}", "Arguments".cyan().bold(), args);

        InuputArguments {
            context: args.nth(2),
            collection: args.nth(3),
            search_term: args.nth(4),
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

    /// Prints the context selection instructions.
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

    /// Prints the collection selection instructions.
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

    /// Resets the input argument to start over if the program does not exist.
    fn reset_input_arg(&self) -> String {
        println!("\n{}", "The subprogram does not exist.".red());
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
            println!("\n{}", "Please input a search term:".yellow().bold());

            io::stdin()
                .read_line(&mut search_term_input)
                .expect("Failed to read line");
        } else if search_term_input.trim().is_empty() {
            search_term_input = search_term_arg_input;
        }

        let mut page = 0;

        loop {
            let search_term = search_term_input.as_str().trim().to_string();

            println!("\n{}: {}", "Your search term".cyan(), search_term);

            let query_string = search_term + " in:name in:description in:readme user:rfprod";
            let q = query_string.as_str();
            let sort = SearchReposSort::Noop;
            let order = Order::Asc;
            let per_page = 5;
            page += 1;

            let mut fetch_result = FetchResult {
                items: Vec::<RepoSearchResultItem>::new(),
                total: 0,
                retry: false,
            };

            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            runtime.block_on(async {
                let result = self.repos_request(q, sort, order, per_page, page).await;
                fetch_result = match result {
                    Ok(data) => {
                        if data.retry {
                            page -= 1;
                        } else {
                            let cwd = env::current_dir().unwrap();
                            println!("The current directory is {}", cwd.display());
                            let base_path = cwd.display().to_string() + "/.data/output/github";
                            let create_dir_result = fs::create_dir_all(&base_path);
                            if let Ok(_tmp) = create_dir_result {
                                let path = base_path + "/github" + &page.to_string() + ".json";
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
                        FetchResult {
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
                artifact::main(self.contexts, Some(context));
                mongo::main(self.collections, Some(collection));
                break;
            }
        }
    }

    /// GitHub repositories request.
    async fn repos_request(
        &self,
        q: &str,
        sort: SearchReposSort,
        order: Order,
        per_page: i64,
        page: i64,
    ) -> Result<FetchResult> {
        let token_env = env::var("GITHUB_TOKEN");
        let token = match token_env.unwrap().trim().parse::<String>() {
            Ok(value) => value,
            Err(_) => String::new(),
        };

        let github = Client::new(String::from("user-agent-name"), Credentials::Token(token));

        let mut retry: bool = false;

        let client = github.unwrap();
        let search = client.search();
        let result = search.repos(q, sort, order, per_page, page).await;
        let raw_res = match result {
            Ok(res) => Ok(res),
            Err(error) => {
                println!(
                    "\n{}: {:?}",
                    "There was an error getting data from GitHub".red(),
                    error
                );

                let err = error.to_string();

                let rate_limit_regx =
                    regex::Regex::new(r"(Rate limited for the next)\s+(\d+)\s+(seconds)").unwrap();
                let captures = rate_limit_regx.captures(&err).map(|captures| {
                    captures
                        .iter() // All the captured groups
                        .skip(1) // Skipping the complete match
                        // .flat_map(|c| c) // Ignoring all empty optional matches
                        .flatten()
                        .map(|c| c.as_str()) // Grab the original strings
                        .collect::<Vec<_>>() // Create a vector
                });
                let wait_timeout = match captures.as_deref() {
                    Some(["Rate limited for the next", x, "seconds"]) => {
                        let x: i64 = x.parse().expect("Can't parse number");
                        x
                    }
                    _ => panic!("Unknown Command: {}", &err),
                };
                println!(
                    "\n{}: {:?}",
                    "GitHub API rate limit hit. Will wait for".red(),
                    wait_timeout
                );

                retry = true;

                let mut child = Command::new("sleep")
                    .arg(wait_timeout.to_string())
                    .spawn()
                    .unwrap();
                let _result = child.wait().unwrap();

                Err(())
            }
        };

        if retry {
            let result = FetchResult {
                items: Vec::<RepoSearchResultItem>::new(),
                total: 0,
                retry: true,
            };
            return Ok(result);
        }

        let res = raw_res.unwrap();
        let body = res.body.to_owned();
        let items = body.items;
        for item in items {
            let name = item.full_name;
            println!("\n{}: {}", "Data".cyan(), name);
        }

        println!("{}: {}", "Response".green(), res.status);
        println!("{}: {:#?}\n", "Headers".green(), res.headers);
        println!("{}: {:#?}\n", "Body".green(), res.body);

        println!("\n\n{}", "Done!".green().bold());

        let items = res.body.items.to_owned();
        let total = res.body.total_count.to_owned();

        let result = FetchResult {
            items,
            total,
            retry: false,
        };
        Ok(result)
    }

    /// Load environment variables read from the .env file.
    fn load_env_vars(&self) -> HashMap<String, String> {
        let result = self.read_env();
        let config = match result {
            Ok(env) => {
                let iter = env.iter();
                for record in iter {
                    let key = record.0;
                    let value = record.1;
                    env::set_var(key, value);
                    println!("key: {key}\nvalue: {value}");
                }
                env
            }
            Err(error) => {
                println!(
                    "\n{}: {:?}",
                    "There was an error reading the .env file".red(),
                    error
                );
                HashMap::from([])
            }
        };
        config
    }

    /// Read variables from the .env file.
    fn read_env(&self) -> std::io::Result<HashMap<String, String>> {
        let cwd = env::current_dir()?;

        println!("The current directory is {}", cwd.display());

        let env_path = cwd.display().to_string() + "/.env";
        let env_path_str = env_path.as_str();
        let contents = fs::read_to_string(env_path_str).expect("Can't read file");

        println!("Text content:\n{contents}");

        let lines = self.read_lines(env_path_str);
        let config: Vec<_> = lines
            .iter()
            .flat_map(|x| {
                let split_pair = x.split('=').collect::<Vec<&str>>();
                let split_key = split_pair.first();
                let some_key = split_key.is_some();
                let key = if some_key {
                    match split_key.unwrap().trim().parse::<String>() {
                        Ok(value) => value,
                        Err(_) => String::new(),
                    }
                } else {
                    String::new()
                };
                let split_value = split_pair.get(1);
                let some_value = split_value.is_some();
                let value = if some_value {
                    match split_value.unwrap().trim().parse::<String>() {
                        Ok(value) => value,
                        Err(_) => String::new(),
                    }
                } else {
                    String::new()
                };
                let map = vec![(key, value)];
                map
            })
            .collect();
        let map: HashMap<String, String> = HashMap::from_iter(config);
        Ok(map)
    }

    /// Read a file line by line.
    fn read_lines(&self, filename: &str) -> Vec<String> {
        let mut result = Vec::new();
        for line in fs::read_to_string(filename).unwrap().lines() {
            result.push(line.to_string())
        }
        result
    }
}
