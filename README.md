# git-today

A tool to recap your daily git work.

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
