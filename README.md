# git-push-check
![Crates.io Version](https://img.shields.io/crates/v/git-push-check)
![Crates.io License](https://img.shields.io/crates/l/git-push-check)

A CLI tool to check if HEAD is pushed to remote.

> [!TIP]
> This is only my personal project. Depending on your needs, running `git branch -r --contains HEAD` or `git tag --contains HEAD` and checking if the output contains the remote branch or tag may be enough.

## Installation

```bash
cargo install git-push-check
```

## Usage

Run `git-push-check` in a git repository to check if the current HEAD is pushed to the remote.

> [!TIP]
> You may want to `git fetch` before running `git-push-check`.

Options:

- `--only`: Which remote references to check: branches, tags, or all (default: all)
- `-h`, `--help`: Print help
- `-V`, `--version`: Print version

Exit codes:

- `0`: HEAD is pushed to the remote
- `2`: HEAD is not pushed to the remote
- `1`: An error occurred
