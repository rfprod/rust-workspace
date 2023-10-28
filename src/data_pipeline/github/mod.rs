/// GitHub module for the data pipeline.
///
use colored::Colorize;
use octorust::{
    auth::Credentials,
    types::{Order, RepoSearchResultItem, WorkflowRun},
    types::{SearchReposSort, WorkflowRunStatus},
    Client,
};
use std::{
    env::{self},
    process::Command,
};

/// Custom result type for fetch results.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Fetch repositories from GitHub.
pub async fn repos(
    q: &str,
    sort: SearchReposSort,
    order: Order,
    per_page: i64,
    page: i64,
) -> Result<ReposFetchResult> {
    let p = DataPipelineGitHub::new();
    p.repos_request(q, sort, order, per_page, page).await
}

/// Fetch a repository workflow runs from GitHub.
pub async fn workflow_runs(
    owner: &str,
    repo: &str,
    branch: &str,
    created: &str,
    per_page: i64,
    page: i64,
) -> Result<WorkflowRunsFetchResult> {
    let p = DataPipelineGitHub::new();
    p.workflow_runs_request(owner, repo, branch, created, per_page, page)
        .await
}

/// GitHub repos fetch result.
pub struct ReposFetchResult {
    pub items: Vec<RepoSearchResultItem>,
    pub total: i64,
    pub retry: bool,
}

/// GitHub repo workflow runs fetch result.
pub struct WorkflowRunsFetchResult {
    pub items: Vec<WorkflowRun>,
    pub total: i64,
    pub retry: bool,
}

struct DataPipelineGitHub;

impl DataPipelineGitHub {
    /// Program constructor.
    fn new() -> DataPipelineGitHub {
        DataPipelineGitHub
    }

    /// Artificial timeout to comply with GitHub API rate limits.
    fn sleep_for(&self, wait_timeout: i64) {
        match Command::new("sleep").arg(wait_timeout.to_string()).spawn() {
            Ok(mut child) => {
                println!("\n{}", "Can sleep".bold().green());
                match child.wait() {
                    Ok(exit_status) => {
                        println!("\n{}: {:?}", "Sleep success".bold().green(), exit_status);
                    }
                    Err(err) => {
                        println!("\n{}: {:?}", "Sleep error".bold().red(), err);
                    }
                }
            }
            Err(err) => {
                println!("\n{}: {:?}", "Can't sleep".bold().red(), err);
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
    ) -> Result<ReposFetchResult> {
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
                    "GitHub API rate limit hit. Have to wait for".red(),
                    wait_timeout
                );

                retry = true;

                self.sleep_for(wait_timeout);

                Err(())
            }
        };

        if retry {
            let result = ReposFetchResult {
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

        let result = ReposFetchResult {
            items,
            total,
            retry: false,
        };
        Ok(result)
    }

    /// GitHub repositories request.
    async fn workflow_runs_request(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        created: &str,
        per_page: i64,
        page: i64,
    ) -> Result<WorkflowRunsFetchResult> {
        let token_env = env::var("GITHUB_TOKEN");
        let token = match token_env.unwrap().trim().parse::<String>() {
            Ok(value) => value,
            Err(_) => String::new(),
        };

        let github = Client::new(String::from("user-agent-name"), Credentials::Token(token));

        let mut retry: bool = false;

        let client = github.unwrap();
        let actions = client.actions();
        let result = actions
            .list_workflow_runs_for_repo(
                owner,
                repo,
                "",
                branch,
                "",
                WorkflowRunStatus::Noop,
                per_page,
                page,
                created,
            )
            .await;
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
                    "GitHub API rate limit hit. Have to wait for".red(),
                    wait_timeout
                );

                retry = true;

                self.sleep_for(wait_timeout);

                Err(())
            }
        };

        if retry {
            let result = WorkflowRunsFetchResult {
                items: Vec::<WorkflowRun>::new(),
                total: 0,
                retry: true,
            };
            return Ok(result);
        }

        let res = raw_res.unwrap();
        let body = res.body.to_owned();
        let items = body.workflow_runs;
        for item in items {
            let name = item.name;
            println!("\n{}: {}", "Data".cyan(), name);
        }

        println!("{}: {}", "Response".green(), res.status);
        println!("{}: {:#?}\n", "Headers".green(), res.headers);
        println!("{}: {:#?}\n", "Body".green(), res.body);

        println!("\n\n{}", "Done!".green().bold());

        let items = res.body.workflow_runs.to_owned();
        let total = res.body.total_count.to_owned();

        let result = WorkflowRunsFetchResult {
            items,
            total,
            retry: false,
        };
        Ok(result)
    }
}
