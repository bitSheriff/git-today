use chrono::{DateTime, Local};
use clap::{Arg, Command};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use git2::Repository;
use std::{
    collections::{HashMap, HashSet},
    process::exit,
};

struct Contributions {
    commits: usize,
    lines_added: usize,
    lines_removed: usize,
    files_changed: HashSet<String>,
}

struct Display {
    authors: bool,
    messages: bool,
    issue_types: bool,
    files: bool,
    issues: bool,
}

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
    // config what to display with default values
    let mut display = Display {
        authors: true,
        messages: true,
        issue_types: true,
        files: false,
        issues: false,
    };
    let repo = Repository::open(path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_glob("refs/heads/*")?;

    let today = Local::now().date_naive();
    let mut commits_by_author: HashMap<String, Contributions> = HashMap::new();
    let mut commit_messages: Vec<String> = Vec::new();

    let mut commits_by_type: HashMap<String, u32> = HashMap::new();
    let mut commits_by_issue: HashMap<String, u32> = HashMap::new();
    let mut tab_author = Table::new();
    let mut tab_types = Table::new();
    let mut tab_issue = Table::new();
    let mut tab_author_header = vec![
        Cell::new("Author").add_attribute(Attribute::Bold),
        Cell::new("# of Commits")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ];

    if full {
        tab_author_header.push(Cell::new("Adds").add_attribute(Attribute::Bold));
        tab_author_header.push(Cell::new("Dels").add_attribute(Attribute::Bold));
        tab_author_header.push(Cell::new("Files").add_attribute(Attribute::Bold));
        display.messages = true;
        display.files = true;
    }

    tab_author
        .set_header(tab_author_header)
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);

    tab_types
        .set_header(vec![
            Cell::new("Issue Type").add_attribute(Attribute::Bold),
            Cell::new("# of Commits")
                .add_attribute(Attribute::Bold)
                .set_alignment(CellAlignment::Center),
        ])
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);

    tab_issue
        .set_header(vec![
            Cell::new("Issue").add_attribute(Attribute::Bold),
            Cell::new("# of Commits")
                .add_attribute(Attribute::Bold)
                .set_alignment(CellAlignment::Center),
        ])
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);

    let keyword_map = [
        ("Bugs", vec!["bug", "fix", "fixing"]),
        ("Features", vec!["feat", "feature"]),
        ("Docs", vec!["doc", "docs"]),
        ("Tests", vec!["test", "tests"]),
        ("Refactors", vec!["refactors", "rewrite"]),
    ];

    while let Some(oid) = revwalk.next() {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        let commit_time = commit.time();
        let commit_date = DateTime::from_timestamp(commit_time.seconds(), 0)
            .unwrap()
            .date_naive();

        if commit_date == today {
            let author = commit.author();
            let author_name = author.name().unwrap_or("Unknown").to_string();
            let contributions = commits_by_author
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

            let message = commit.message().unwrap_or("").to_lowercase();
            for (category, keywords) in &keyword_map {
                if keywords.iter().any(|keyword| message.contains(keyword)) {
                    *commits_by_type.entry(category.to_string()).or_insert(0) += 1;
                }
            }

            commit_messages.push(commit.message().unwrap_or("").to_string());
        } else if commit_date < today {
            revwalk.hide(oid)?; // skip parent commits if already this is older than today (they can only get older in this history)
        }
    }

    if commit_messages.is_empty() {
        println!("No commits today 😿");
        exit(0);
    }
    let all_changed_files: HashSet<String> = if full {
        commits_by_author
            .values()
            .flat_map(|c| &c.files_changed)
            .cloned()
            .collect()
    } else {
        HashSet::new()
    };

    let mut authors: Vec<_> = commits_by_author.into_iter().collect();
    authors.sort_by(|a, b| b.1.commits.cmp(&a.1.commits).then_with(|| a.0.cmp(&b.0)));
    for (author, contributions) in authors {
        if full {
            tab_author.add_row(vec![
                Cell::new(author),
                Cell::new(contributions.commits.to_string()).set_alignment(CellAlignment::Center),
                Cell::new(contributions.lines_added.to_string())
                    .set_alignment(CellAlignment::Center)
                    .fg(Color::Green),
                Cell::new(contributions.lines_removed.to_string())
                    .set_alignment(CellAlignment::Center)
                    .fg(Color::Red),
                Cell::new(contributions.files_changed.len().to_string())
                    .set_alignment(CellAlignment::Center),
            ]);
        } else {
            add_row_with_centered_value(
                &mut tab_author,
                &author,
                &contributions.commits.to_string(),
            );
        }
    }

    if display.authors {
        println!("{tab_author}");
    }

    if (!commits_by_type.is_empty() || full) && display.issue_types {
        let issue_types = [
            ("Bugs", "🐛 Bugs"),
            ("Features", "🚀 Features"),
            ("Refactors", "♻️ Refactrors"),
            ("Docs", "📝 Docs"),
            ("Merges", "🧬 Merges"),
            ("Tests", "🔍 Tests"),
        ];

        for (key, display_name) in &issue_types {
            let count = commits_by_type.get(*key).unwrap_or(&0);
            if *count > 0 || full {
                add_row_with_centered_value(&mut tab_types, display_name, &count.to_string());
            }
        }

        println!("{tab_types}");
    }

    if display.files && !all_changed_files.is_empty() {
        println!("\nChanged files today:");
        let mut sorted_files: Vec<_> = all_changed_files.into_iter().collect();
        sorted_files.sort();
        for file in sorted_files {
            println!("- {}", file);
        }
    }

    if display.messages && !commit_messages.is_empty() {
        println!("\nCommit messages today:");
        for msg in commit_messages {
            println!("- {}", msg.trim());
        }
    }

    if display.issues {}

    Ok(())
}
