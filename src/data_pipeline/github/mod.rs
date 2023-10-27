/// GitHub module for the data pipeline.
///
use colored::Colorize;
use octorust::{
    auth::Credentials,
    types::SearchReposSort,
    types::{Order, RepoSearchResultItem},
    Client,
};
use std::{
    env::{self},
    process::Command,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn repos(
    q: &str,
    sort: SearchReposSort,
    order: Order,
    per_page: i64,
    page: i64,
) -> Result<FetchResult> {
    let p = DataPipelineGitHub::new();
    p.repos_request(q, sort, order, per_page, page).await
}

/// GitHub repos fetch result.
pub struct FetchResult {
    pub items: Vec<RepoSearchResultItem>,
    pub total: i64,
    pub retry: bool,
}

struct DataPipelineGitHub;

impl DataPipelineGitHub {
    /// Program constructor.
    fn new() -> DataPipelineGitHub {
        DataPipelineGitHub
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
}
