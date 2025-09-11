use chrono::{DateTime, Local};
use clap::{Arg, Command};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use git2::Repository;
use std::{collections::HashMap, process::exit};

fn main() {
    let args = Command::new("git-today")
        .disable_version_flag(true)
        .version(env!("CARGO_PKG_VERSION"))
        .author("Your Name <youremail@example.com>")
        .about("A tool to recap your daily git work")
        .arg(
            Arg::new("version")
                .short('v')
                .long("version")
                .action(clap::ArgAction::Version)
                .help("Print version information"),
        )
        .arg(
            Arg::new("path")
                .help("Path to the git repository")
                .default_value("."),
        )
        .arg(
            Arg::new("full")
                .long("full")
                .action(clap::ArgAction::SetTrue)
                .help("Print commit messages and full table"),
        )
        .get_matches();

    let path = args.get_one::<String>("path").unwrap();
    let full = args.get_flag("full");

    match run(path, full) {
        Ok(_) => {}
        Err(e) => eprintln!("error: {}", e),
    }
}

fn add_row_with_centered_value(table: &mut Table, label: &str, value: &str) {
    table.add_row(vec![
        Cell::new(label),
        Cell::new(value).set_alignment(CellAlignment::Center),
    ]);
}

fn run(path: &str, full: bool) -> Result<(), git2::Error> {
    let repo = Repository::open(path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_glob("refs/heads/*")?;

    let today = Local::now().date_naive();
    let mut commits_by_author: HashMap<String, u32> = HashMap::new();
    let mut commit_messages: Vec<String> = Vec::new();

    let mut bug_commits = 0;
    let mut feature_commits = 0;
    let mut doc_commits = 0;
    let mut merge_commits = 0;
    let mut test_commits = 0;
    let mut tab_author = Table::new();
    let mut tab_issue = Table::new();
    tab_author
        .set_header(vec![
            Cell::new("Author").add_attribute(Attribute::Bold),
            Cell::new("# of Commits")
                .add_attribute(Attribute::Bold)
                .set_alignment(CellAlignment::Center),
        ])
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);
    tab_issue
        .set_header(vec![
            Cell::new("Issue Type").add_attribute(Attribute::Bold),
            Cell::new("# of Commits")
                .add_attribute(Attribute::Bold)
                .set_alignment(CellAlignment::Center),
        ])
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);

    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        let commit_time = commit.time();
        let commit_date = DateTime::from_timestamp(commit_time.seconds(), 0)
            .unwrap()
            .date_naive();

        if commit_date == today {
            let author = commit.author();
            let author_name = author.name().unwrap_or("Unknown").to_string();
            *commits_by_author.entry(author_name).or_insert(0) += 1;

            if commit.parent_count() > 1 {
                merge_commits += 1;
            }

            let message = commit.message().unwrap_or("").to_lowercase();
            if message.contains("bug") || message.contains("fix") || message.contains("fixing") {
                bug_commits += 1;
            }
            if message.contains("feat") || message.contains("feature") {
                feature_commits += 1;
            }
            if message.contains("doc") || message.contains("docs") {
                doc_commits += 1;
            }
            if message.contains("test") || message.contains("tests") {
                test_commits += 1;
            }

            commit_messages.push(commit.message().unwrap_or("").to_string());
        }
    }

    if commit_messages.is_empty() {
        println!("No commits today 😿");
        exit(0);
    }
    for (author, count) in commits_by_author {
        add_row_with_centered_value(&mut tab_author, &author, &count.to_string());
    }
    println!("{tab_author}");

    if (bug_commits + feature_commits + doc_commits + merge_commits + test_commits) > 0 || full {
        if bug_commits > 0 || full {
            add_row_with_centered_value(&mut tab_issue, "🐛 Bugs", &bug_commits.to_string());
        }
        if feature_commits > 0 || full {
            add_row_with_centered_value(
                &mut tab_issue,
                "🚀 Features",
                &feature_commits.to_string(),
            );
        }
        if doc_commits > 0 || full {
            add_row_with_centered_value(&mut tab_issue, "📝 Docs", &doc_commits.to_string());
        }
        if merge_commits > 0 || full {
            add_row_with_centered_value(&mut tab_issue, "🧬 Merges", &merge_commits.to_string());
        }
        if test_commits > 0 || full {
            add_row_with_centered_value(&mut tab_issue, "🔍 Tests", &test_commits.to_string());
        }

        println!("{tab_issue}");
    }

    if full {
        println!("\nCommit messages today:");
        for msg in commit_messages {
            println!("- {}", msg.trim());
        }
    }

    Ok(())
}
