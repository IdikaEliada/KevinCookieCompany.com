use anyhow::{Context, Result};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT};
use serde::Deserialize;

const GITHUB_API_BASE: &str = "https://api.github.com";

// ── Serde models ──────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct Repo {
    pub full_name: String,
    pub name: String,
    pub description: Option<String>,
    pub html_url: String,
    pub stargazers_count: u32,
    pub forks_count: u32,
    pub open_issues_count: u32,
    pub language: Option<String>,
    pub license: Option<License>,
    pub topics: Option<Vec<String>>,
    pub pushed_at: Option<String>,
    pub homepage: Option<String>,
    pub archived: bool,
}

#[derive(Debug, Deserialize)]
pub struct License {
    pub name: String,
}

#[derive(Debug, Deserialize)]
struct SearchResponse {
    items: Vec<Repo>,
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Optional filters applied to a GitHub search query.
pub struct SearchFilters {
    pub language: Option<String>,
    pub sort: String,
    pub min_stars: Option<u32>,
}

/// Search GitHub for repositories matching `query`.
pub fn search_repos(query: &str, limit: u8, filters: SearchFilters) -> Result<Vec<Repo>> {
    let client = build_client()?;

    // Build the `q` parameter: start with the raw query, then append qualifiers.
    let mut q = query.to_owned();
    if let Some(lang) = &filters.language {
        q.push_str(&format!(" language:{lang}"));
    }
    if let Some(stars) = filters.min_stars {
        q.push_str(&format!(" stars:>={stars}"));
    }

    let url = format!("{GITHUB_API_BASE}/search/repositories");
    let response = client
        .get(&url)
        .query(&[
            ("q", q.as_str()),
            ("sort", &filters.sort),
            ("order", "desc"),
            ("per_page", &limit.to_string()),
        ])
        .send()
        .context("Failed to reach GitHub API")?
        .error_for_status()
        .context("GitHub API returned an error")?
        .json::<SearchResponse>()
        .context("Failed to parse GitHub search response")?;

    Ok(response.items)
}

/// Fetch detailed information about a single repository.
pub fn get_repo_info(owner: &str, repo: &str) -> Result<Repo> {
    let client = build_client()?;
    let url = format!("{GITHUB_API_BASE}/repos/{owner}/{repo}");

    let result = client
        .get(&url)
        .send()
        .context("Failed to reach GitHub API")?
        .error_for_status()
        .context("Repository not found or API error")?
        .json::<Repo>()
        .context("Failed to parse repository details")?;

    Ok(result)
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn build_client() -> Result<Client> {
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/vnd.github+json"),
    );
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("oss-search-cli/0.1.0"),
    );

    // Honour an optional GITHUB_TOKEN env var for higher rate limits.
    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        let auth = HeaderValue::from_str(&format!("Bearer {token}"))
            .context("Invalid GITHUB_TOKEN value")?;
        headers.insert(reqwest::header::AUTHORIZATION, auth);
    }

    Client::builder()
        .default_headers(headers)
        .build()
        .context("Failed to build HTTP client")
}