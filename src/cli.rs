use clap::{Parser, Subcommand};

/// Search and explore open source projects from the command line
#[derive(Parser)]
#[command(
    name = "oss-search",
    version,
    about,
    long_about = "A CLI tool for discovering open source projects on GitHub.\n\nExamples:\n  oss-search search tokio\n  oss-search search \"web framework\" --language rust --stars 1000\n  oss-search info tokio-rs/tokio"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Search for open source projects by name or keyword
    Search {
        /// Search query (project name, keyword, or description)
        query: String,

        /// Maximum number of results to display
        #[arg(short, long, default_value_t = 10, value_parser = clap::value_parser!(u8).range(1..=50))]
        limit: u8,

        /// Filter by programming language (e.g. rust, python, go)
        #[arg(short = 'L', long)]
        language: Option<String>,

        /// Sort results by: stars, forks, updated, best-match
        #[arg(short, long, default_value = "stars")]
        sort: String,

        /// Filter to only repos with at least this many stars
        #[arg(short = 'S', long)]
        stars: Option<u32>,
    },

    /// Show detailed information about a specific repository
    Info {
        /// Repository in owner/repo format (e.g. rust-lang/rust)
        repo: String,
    },
}