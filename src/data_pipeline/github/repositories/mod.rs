//! GitHubRepos submodule for the data pipeline.

use colored::Colorize;
use octorust::{
    auth::Credentials,
    types::SearchReposSort,
    types::{Order, RepoSearchResultItem},
    Client,
};
use std::env::{self};

use super::rate_limit_handler::GitHubRateLimitHandler;

/// Custom result type for fetch results.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// GitHub repos fetch result.
pub struct ReposFetchResult {
    pub items: Vec<RepoSearchResultItem>,
    pub total: i64,
    pub retry: bool,
}

pub struct DataPipelineGitHubRepos {
    pub rate_limit_handler: GitHubRateLimitHandler,
}

impl DataPipelineGitHubRepos {
    /// GitHub repositories request.
    pub async fn repos_request(
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

                let wait_timeout = self.rate_limit_handler.error_handler(error);
                if wait_timeout > 0 {
                    retry = true;
                    self.rate_limit_handler.sleep_for(wait_timeout);
                }

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
        // println!("{}: {:#?}\n", "Body".green(), res.body);

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
}
