# `git-today`

<img src="./doc/cover.png" style="zoom: 25%;" />

A tool to recap your daily git work.

## Motivation

In my optinion, projects which solve a problem firstly to use, are the most valueable. So the idea for this project was born while I was writing my bachelor thesis:
working all day in a project which gets bigger and bigger, the commits are piling up (big fan of commiting small chunks when working with LaTeX) and at the end of the day I had no idea how *much* I worked and how. Pretty early in the process I commited to "mark" my commits with little tags inside the message (not like `git tag`) to get a quick overview what this commit is about.

So I said to myself, why not analyze the commit history of today and print nice little statistics.

## Installation

Currently no binaries are provided yet, so the only way is to install it with `cargo` directly from the repository url#

```sh
# Use the GitHub version
cargo install --git https://github.com/bitSheriff/git-today

# Use my selfhosted version
cargo install --git https://code.bitsheriff.dev/bitSheriff/git-today
```

## Usage

```
A tool to recap your daily git work

Usage: git-today [OPTIONS] [path]

Arguments:
  [path]  Path to the git repository [default: .]

Options:
  -v, --version  Print version information
      --full     Print commit messages
  -h, --help     Print help
```

To use this tool with `git today`, you can create a git alias.

### Set the alias in the current repository:

```sh
git config alias.today "!git-today"
```

### Set the alias globally for your user:

```sh
git config --global alias.today "!git-today"
```
