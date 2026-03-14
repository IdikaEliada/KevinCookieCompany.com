use colored::Colorize;
use crate::github::Repo;

/// Print a compact ranked list of search results.
pub fn print_results(repos: &[Repo], query: &str) {
    println!(
        "\n{} {}\n",
        "Search results for:".dimmed(),
        query.bold().cyan()
    );

    for (i, repo) in repos.iter().enumerate() {
        let rank = format!("{:>2}.", i + 1).dimmed();
        let name = repo.full_name.bold().green();

        let stars = format!("★ {}", format_count(repo.stargazers_count)).yellow();
        let forks = format!("⑂ {}", format_count(repo.forks_count)).blue();

        let lang = repo
            .language
            .as_deref()
            .map(|l| format!("[{l}]").magenta().to_string())
            .unwrap_or_default();

        let archived = if repo.archived {
            " (archived)".red().to_string()
        } else {
            String::new()
        };

        println!("{rank} {name}{archived}  {stars}  {forks}  {lang}");

        if let Some(desc) = &repo.description {
            let trimmed = truncate(desc, 90);
            println!("     {}", trimmed.dimmed());
        }

        println!("     {}", repo.html_url.dimmed());

        if i < repos.len() - 1 {
            println!();
        }
    }

    println!();
}

/// Print a detailed view of a single repository.
pub fn print_repo_detail(repo: &Repo) {
    let sep = "─".repeat(60).dimmed();

    println!("\n{sep}");
    println!("{}", repo.full_name.bold().green());
    println!("{sep}");

    if let Some(desc) = &repo.description {
        println!("{desc}");
    }

    println!();
    print_field("URL", &repo.html_url);

    if let Some(hp) = &repo.homepage {
        if !hp.is_empty() {
            print_field("Homepage", hp);
        }
    }

    print_field("Stars",  &format_count(repo.stargazers_count));
    print_field("Forks",  &format_count(repo.forks_count));
    print_field("Issues", &format_count(repo.open_issues_count));

    if let Some(lang) = &repo.language {
        print_field("Language", lang);
    }

    if let Some(license) = &repo.license {
        print_field("License", &license.name);
    }

    if let Some(pushed) = &repo.pushed_at {
        print_field("Last push", &pushed[..10]);  // YYYY-MM-DD
    }

    if let Some(topics) = &repo.topics {
        if !topics.is_empty() {
            print_field("Topics", &topics.join(", "));
        }
    }

    if repo.archived {
        println!("\n  {}", "⚠  This repository is archived.".yellow());
    }

    println!();
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn print_field(label: &str, value: &str) {
    println!("  {:12} {}", format!("{label}:").dimmed(), value);
}

fn format_count(n: u32) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}k", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_owned()
    } else {
        format!("{}…", &s[..max])
    }
}