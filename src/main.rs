use clap::{Arg, Command};
use git2::{Repository, Commit};
use chrono::{Local, DateTime, TimeZone};
use std::collections::HashMap;

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
        .get_matches();

    if let Some(path) = args.get_one::<String>("path") {
        match run(path) {
            Ok(_) => {}
            Err(e) => eprintln!("error: {}", e),
        }
    }
}

fn run(path: &str) -> Result<(), git2::Error> {
    let repo = Repository::open(path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let today = Local::now().date_naive();
    let mut commits_by_author: HashMap<String, u32> = HashMap::new();
    let mut commit_messages: Vec<String> = Vec::new();

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
            commit_messages.push(commit.message().unwrap_or("").to_string());
        }
    }

    println!("Commits per author today:");
    for (author, count) in commits_by_author {
        println!("{}: {}", author, count);
    }

    println!("
Commit messages today:");
    for msg in commit_messages {
        println!("- {}", msg.trim());
    }

    Ok(())
}