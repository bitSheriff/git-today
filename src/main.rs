use chrono::{DateTime, Local};
use clap::{Arg, Command};
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
                .help("Print commit messages"),
        )
        .get_matches();

    let path = args.get_one::<String>("path").unwrap();
    let full = args.get_flag("full");

    match run(path, full) {
        Ok(_) => {}
        Err(e) => eprintln!("error: {}", e),
    }
}

fn run(path: &str, full: bool) -> Result<(), git2::Error> {
    let repo = Repository::open(path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let today = Local::now().date_naive();
    let mut commits_by_author: HashMap<String, u32> = HashMap::new();
    let mut commit_messages: Vec<String> = Vec::new();

    let mut bug_commits = 0;
    let mut feature_commits = 0;
    let mut doc_commits = 0;

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

            commit_messages.push(commit.message().unwrap_or("").to_string());
        }
    }

    if commit_messages.len() == 0 {
        println!("No commits today 😿");
        exit(0);
    }
    println!("Commits per author today:");
    for (author, count) in commits_by_author {
        println!("\t{}: {}", author, count);
    }

    if bug_commits > 0 || feature_commits > 0 || doc_commits > 0 {
        println!("Commits per issue type today:");
        if bug_commits > 0 {
            println!("\t🐛: {}", bug_commits);
        }
        if feature_commits > 0 {
            println!("\t🚀: {}", feature_commits);
        }
        if doc_commits > 0 {
            println!("\t📝: {}", doc_commits);
        }
    }

    if full {
        println!("\nCommit messages today:");
        for msg in commit_messages {
            println!("- {}", msg.trim());
        }
    }

    Ok(())
}
