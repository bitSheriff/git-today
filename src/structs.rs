use std::collections::HashSet;

pub struct Contributions {
    pub commits: usize,
    pub lines_added: usize,
    pub lines_removed: usize,
    pub files_changed: HashSet<String>,
}

pub struct Display {
    pub authors: bool,
    pub messages: bool,
    pub issue_types: bool,
    pub files: bool,
    pub issues: bool,
    pub line_diff: bool,
}
