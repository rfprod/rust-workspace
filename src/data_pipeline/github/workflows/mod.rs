//! GitHubWorkflows module for the data pipeline.

use colored::Colorize;
use octorust::{auth::Credentials, types::WorkflowRun, types::WorkflowRunStatus, Client};
use std::env::{self};

use super::rate_limit_handler::GitHubRateLimitHandler;

/// Custom result type for fetch results.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// GitHub repo workflow runs fetch result.
pub struct WorkflowRunsFetchResult {
    pub items: Vec<WorkflowRun>,
    pub total: i64,
    pub retry: bool,
}

pub struct DataPipelineGitHubWorkflows {
    pub rate_limit_handler: GitHubRateLimitHandler,
}

impl DataPipelineGitHubWorkflows {
    /// GitHub repositories request.
    pub async fn workflow_runs_request(
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

                let wait_timeout = self.rate_limit_handler.error_handler(error);
                if wait_timeout > 0 {
                    retry = true;
                    self.rate_limit_handler.sleep_for(wait_timeout);
                }

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
        // println!("{}: {:#?}\n", "Body".green(), res.body);

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
