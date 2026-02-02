use crate::structs::Contributions;
use chrono::{DateTime, Local, NaiveDate};
use git2::Repository;
use std::collections::{HashMap, HashSet};
use std::process::exit;

pub fn analyze_commits(
    path: &str,
    args: &clap::ArgMatches,
) -> Result<
    (
        HashMap<String, Contributions>,
        Vec<String>,
        HashMap<String, u32>,
        HashMap<String, u32>,
    ),
    git2::Error,
> {
    let repo = Repository::open(path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_glob("refs/heads/*")?;

    let today = Local::now().date_naive();
    let mut commits_by_author: HashMap<String, Contributions> = HashMap::new();
    let mut commit_messages: Vec<String> = Vec::new();

    let mut commits_by_type: HashMap<String, u32> = HashMap::new();
    let mut commits_by_issue: HashMap<String, u32> = HashMap::new();

    let keyword_map = [
        ("Bugs", vec!["bug", "fix", "fixing"]),
        ("Features", vec!["feat", "feature"]),
        ("Docs", vec!["doc", "docs"]),
        ("Tests", vec!["test", "tests"]),
        ("Refactors", vec!["refactors", "rewrite"]),
    ];

    let today_string = today.to_string();
    let start_date = NaiveDate::parse_from_str(
        args.get_one::<String>("start").unwrap_or(&today_string),
        "%Y-%m-%d",
    );
    let end_date = NaiveDate::parse_from_str(
        args.get_one::<String>("end").unwrap_or(&today_string),
        "%Y-%m-%d",
    );

    while let Some(oid) = revwalk.next() {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        let commit_time = commit.time();
        let commit_date = DateTime::from_timestamp(commit_time.seconds(), 0)
            .unwrap()
            .date_naive();

        if commit_date == today
            || args.get_flag("all")
            || ((commit_date <= end_date.unwrap()) && (commit_date >= start_date.unwrap()))
        {
            let author = commit.author();
            let author_name = author.name().unwrap_or("Unknown").to_string();
            let contributions =
                commits_by_author
                    .entry(author_name)
                    .or_insert(Contributions {
                        commits: 0,
                        lines_added: 0,
                        lines_removed: 0,
                        files_changed: HashSet::new(),
                    });
            contributions.commits += 1;

            let tree = commit.tree()?;
            let parent = if commit.parent_count() > 0 {
                Some(commit.parent(0)?)
            } else {
                None
            };
            let parent_tree = if let Some(p) = &parent {
                Some(p.tree()?)
            } else {
                None
            };

            let diff = repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), None)?;
            let stats = diff.stats()?;
            contributions.lines_added += stats.insertions();
            contributions.lines_removed += stats.deletions();

            diff.foreach(
                &mut |delta, _| {
                    if let Some(path) = delta.new_file().path() {
                        contributions
                            .files_changed
                            .insert(path.to_string_lossy().to_string());
                    }
                    true
                },
                None,
                None,
                None,
            )?;

            if commit.parent_count() > 1 {
                *commits_by_type.entry("Merges".to_string()).or_insert(0) += 1;
                continue; // if it is a merge commit, do not count it as something else
            }

            let message = commit.message().unwrap_or("");
            if message.starts_with('[') {
                if let Some(end_index) = message.find(']') {
                    let issue = &message[1..end_index];
                    if !issue.is_empty() {
                        *commits_by_issue.entry(issue.to_string()).or_insert(0) += 1;
                    }
                }
            } else if let Some(issue_part) = message.strip_prefix('#') {
                let terminator_pos = issue_part.find([' ', ':']).unwrap_or(issue_part.len());
                let issue = &issue_part[..terminator_pos];
                if !issue.is_empty() {
                    *commits_by_issue.entry(issue.to_string()).or_insert(0) += 1;
                }
            }

            let lower_message = message.to_lowercase();
            for (category, keywords) in &keyword_map {
                if keywords
                    .iter()
                    .any(|keyword| lower_message.contains(keyword))
                {
                    *commits_by_type.entry(category.to_string()).or_insert(0) += 1;
                }
            }

            commit_messages.push(message.to_string());
        }
    }

    if commit_messages.is_empty() {
        println!("No commits today 😿");
        exit(0);
    }

    Ok((
        commits_by_author,
        commit_messages,
        commits_by_type,
        commits_by_issue,
    ))
}
