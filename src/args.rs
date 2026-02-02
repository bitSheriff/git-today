use crate::structs::Display;
use clap::{Arg, Command};

pub fn get_displaying_data(args: &clap::ArgMatches) -> Display {
    let full = args.get_flag("full");
    let only = args.get_flag("only");

    // config what to display with default values
    let mut display = if only {
        Display {
            authors: false,
            messages: false,
            issue_types: false,
            files: false,
            issues: false,
            line_diff: false,
        }
    } else {
        Display {
            authors: true,
            messages: false,
            issue_types: true,
            files: false,
            issues: false,
            line_diff: false,
        }
    };

    if args.get_flag("author") {
        display.authors = true;
    }
    if args.get_flag("files") {
        display.files = true;
    }
    if args.get_flag("issues") {
        display.issues = true;
    }
    if args.get_flag("messages") {
        display.messages = true;
    }
    if args.get_flag("types") {
        display.issue_types = true;
    }
    if args.get_flag("diff") {
        display.line_diff = true;
    }

    if full {
        display.messages = true;
        display.files = true;
        display.issues = true;
    }

    display
}

pub fn get_args() -> Command {
    Command::new("git-today")
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
        .arg(
            Arg::new("author")
                .long("author")
                .action(clap::ArgAction::SetTrue)
                .help("Display authors table"),
        )
        .arg(
            Arg::new("files")
                .long("files")
                .action(clap::ArgAction::SetTrue)
                .help("Display changed files"),
        )
        .arg(
            Arg::new("issues")
                .long("issues")
                .action(clap::ArgAction::SetTrue)
                .help("Display issues table"),
        )
        .arg(
            Arg::new("messages")
                .long("messages")
                .action(clap::ArgAction::SetTrue)
                .help("Display commit messages"),
        )
        .arg(
            Arg::new("types")
                .long("types")
                .action(clap::ArgAction::SetTrue)
                .help("Display issue types table"),
        )
        .arg(
            Arg::new("diff")
                .long("diff")
                .action(clap::ArgAction::SetTrue)
                .help("Display line diffs in authors table"),
        )
        .arg(
            Arg::new("only")
                .long("only")
                .action(clap::ArgAction::SetTrue)
                .help("Display only the selected items"),
        )
        .arg(
            Arg::new("all")
                .long("all")
                .action(clap::ArgAction::SetTrue)
                .help("Parse whole history"),
        )
        .arg(Arg::new("start").long("start").help("Start date"))
        .arg(Arg::new("end").long("end").help("End date"))
}
