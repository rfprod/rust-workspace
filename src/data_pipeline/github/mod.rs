//! GitHub module for the data pipeline.

use octorust::{types::Order, types::SearchReposSort};

pub use self::repositories::ReposFetchResult;
pub use self::workflows::WorkflowRunsFetchResult;

/// Custom result type for fetch results.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

mod rate_limit_handler;
mod repositories;
mod workflows;

pub struct DataPipelineGitHubFsConfiguration {
    pub repos_output: String,
    pub workflows_output: String,
}

/// The entry point of the program.
pub fn main() -> DataPipelineGitHub {
    DataPipelineGitHub::new()
}

pub struct DataPipelineGitHub {
    pub configuration: DataPipelineGitHubFsConfiguration,
    repos_ctx: repositories::DataPipelineGitHubRepos,
    workflows_ctx: workflows::DataPipelineGitHubWorkflows,
}

impl DataPipelineGitHub {
    /// Program constructor.
    fn new() -> DataPipelineGitHub {
        let configuration = DataPipelineGitHubFsConfiguration {
            repos_output: String::from("/.data/output/github/repos"),
            workflows_output: String::from("/.data/output/github/workflows"),
        };
        let repos_ctx = repositories::DataPipelineGitHubRepos {
            rate_limit_handler: rate_limit_handler::GitHubRateLimitHandler,
        };
        let workflows_ctx = workflows::DataPipelineGitHubWorkflows {
            rate_limit_handler: rate_limit_handler::GitHubRateLimitHandler,
        };
        DataPipelineGitHub {
            configuration,
            repos_ctx,
            workflows_ctx,
        }
    }

    /// GitHub repositories request.
    pub async fn repos_request(
        &self,
        q: &str,
        sort: SearchReposSort,
        order: Order,
        per_page: i64,
        page: i64,
    ) -> Result<ReposFetchResult> {
        self.repos_ctx
            .repos_request(q, sort, order, per_page, page)
            .await
    }

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
        self.workflows_ctx
            .workflow_runs_request(owner, repo, branch, created, per_page, page)
            .await
    }
}
