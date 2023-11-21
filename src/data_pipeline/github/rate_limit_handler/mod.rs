//! GitHub API rate limit handler submodule for the data pipeline.

use colored::Colorize;
use octorust::ClientError;
use std::process::Command;

/// Sleep for N seconds defined by the `wait_timeout` value.
pub fn sleep_for(wait_timeout: i64) {
    let p = GitHubRateLimitHandler::new();
    p.sleep_for(wait_timeout)
}

/// Process GitHub API `ClientError` and derive the value of the `wait_timeout` variable.
pub fn error_handler(error: ClientError) -> i64 {
    let p = GitHubRateLimitHandler::new();
    p.error_handler(error)
}

struct GitHubRateLimitHandler;

impl GitHubRateLimitHandler {
    /// Program constructor.
    fn new() -> GitHubRateLimitHandler {
        GitHubRateLimitHandler
    }

    /// Sleep for N seconds defined by the `wait_timeout` value.
    fn sleep_for(&self, wait_timeout: i64) {
        match Command::new("sleep").arg(wait_timeout.to_string()).spawn() {
            Ok(mut child) => {
                println!("\n{}", "Can sleep".bold().green());
                match child.wait() {
                    Ok(exit_status) => {
                        println!("{}: {:?}", "Sleep success".bold().green(), exit_status);
                    }
                    Err(err) => {
                        panic!("{}\n{:?}", "Sleep error".bold().red(), err);
                    }
                }
            }
            Err(err) => {
                panic!("{}\n{:?}", "Can't sleep".bold().red(), err);
            }
        }
    }

    /// Process GitHub API `ClientError` and derive the value of the `wait_timeout` variable.
    fn error_handler(&self, error: ClientError) -> i64 {
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
                let x: i64 = x.parse().expect("Can't parse a number");
                x
            }
            _ => {
                println!(
                    "{}\n{:?}",
                    "It looks like the root cause of the error is not a rate limit hit.".cyan(),
                    &err
                );
                0
            }
        };
        println!(
            "{}: {:?}",
            "GitHub API rate limit hit. Have to wait for".red(),
            wait_timeout
        );

        wait_timeout
    }
}
