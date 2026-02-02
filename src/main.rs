fn main() {
    if let Err(e) = git_today::run() {
        eprintln!("error: {}", e);
    }
}