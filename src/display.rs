use crate::structs::Contributions;
use crate::structs::Display;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use std::collections::{HashMap, HashSet};

pub fn add_row_with_centered_value(table: &mut Table, label: &str, value: &str) {
    table.add_row(vec![
        Cell::new(label),
        Cell::new(value).set_alignment(CellAlignment::Center),
    ]);
}

pub fn print_tables(
    display: &Display,
    full: bool,
    commits_by_author: HashMap<String, Contributions>,
    commits_by_type: HashMap<String, u32>,
    commit_messages: Vec<String>,
    commits_by_issue: HashMap<String, u32>,
) {
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
            Cell::new("Issue Ticket").add_attribute(Attribute::Bold),
            Cell::new("# of Commits")
                .add_attribute(Attribute::Bold)
                .set_alignment(CellAlignment::Center),
        ])
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);

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
        if full || display.line_diff {
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

    if display.issues && !commits_by_issue.is_empty() {
        let mut issues: Vec<_> = commits_by_issue.into_iter().collect();
        issues.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

        for (issue, count) in issues {
            add_row_with_centered_value(&mut tab_issue, &issue, &count.to_string());
        }
        println!("\n{tab_issue}");
    }
}
