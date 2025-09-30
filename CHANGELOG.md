# Changelog

## v0.1.5:

- added issue ticket

## v0.1.4: 2025-09-21

- authors are now ordered on their commit count
    - secondary order function is author name
- full table
    - added lines added and deleted
    - added unique list of files changed (count)

## v0.1.3: 2025-09-11

- fix: optimization, exit revwalk if commit is older than today, so any parent commits are skipped (much much faster, tested it with the Linux Kernel)
- fix: analyze all branches not only the current one
- `--full` prints empty table entries too
- added `Test` issue type

## v0.1.2: 2025-09-10

- full UTF8 table

## v0.1.1: 2025-09-10

- added group for merge commits
- added table output

## v0.1.0: 2025-09-08

- release on [crates.io](https://crates.io/crates/git-today)
