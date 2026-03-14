mod cli;
mod github;
mod display;

use anyhow::Result;
use cli::{Cli, Commands};
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Search {
            query,
            limit,
            language,
            sort,
            stars,
        } => {
            let filters = github::SearchFilters {
                language,
                sort,
                min_stars: stars,
            };
            let results = github::search_repos(&query, limit, filters)?;

            if results.is_empty() {
                println!("No results found for '{}'", query);
            } else {
                display::print_results(&results, &query);
            }
        }
        Commands::Info { repo } => {
            let parts: Vec<&str> = repo.splitn(2, '/').collect();
            if parts.len() != 2 {
                anyhow::bail!("Expected format: owner/repo (e.g. rust-lang/rust)");
            }
            let info = github::get_repo_info(parts[0], parts[1])?;
            display::print_repo_detail(&info);
        }
    }

    Ok(())
}