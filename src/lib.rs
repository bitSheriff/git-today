pub mod args;
pub mod commit_analyzer;
pub mod display;
pub mod structs;

pub fn run() -> Result<(), git2::Error> {
    let args = args::get_args().get_matches();
    let path = args.get_one::<String>("path").unwrap();
    let full = args.get_flag("full");
    let display_options = args::get_displaying_data(&args);

    let (commits_by_author, commit_messages, commits_by_type, commits_by_issue) =
        commit_analyzer::analyze_commits(path, &args)?;

    display::print_tables(
        &display_options,
        full,
        commits_by_author,
        commits_by_type,
        commit_messages,
        commits_by_issue,
    );

    Ok(())
}
