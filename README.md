# Rust Todo CLI 🦀

A blazing fast, command-line based To-Do list manager written in Rust. It supports batch task creation, color-coded output, and persistent storage via JSON.

## Features ✨

* **Batch Add:** Add multiple tasks in one line.
* **Persistence:** Automatically saves tasks to a local JSON file.
* **Shell Integration:** Generates completion scripts for Zsh, Bash, Fish, etc.
* **Visuals:** Color-coded task status (Green for done, Red for pending).

## Installation ⚙️

### From Source

Ensure you have Rust installed. Clone the repo and run:

```bash
cargo install --path .
```

## Usage 🚀

### Adding Tasks

You can add one or multiple tasks at once:

```bash
todo add "Buy Milk" "Walk the dog" "Learn Rust"
```

### Viewing Tasks

See all your current tasks with their IDs and status:

```bash
todo view
```

### Completing Tasks

Mark tasks as done by their ID (supports multiple IDs):

```bash
todo complete 1 3
```

### Deleting Tasks

Remove tasks permanently by ID:

```bash
todo delete 2
```

## Shell Completion 🐚

To enable tab autocomplete (e.g., for Zsh), run:

```bash
# 1. Create the completion directory
mkdir -p ~/.zsh/completions

# 2. Generate the script (hidden command)
todo completion zsh > ~/.zsh/completions/_todo

# 3. Add to your .zshrc if not already present
# fpath=(~/.zsh/completions $fpath)
# autoload -Uz compinit
# compinit
```

## Roadmap 🗺️

- [ ] Pretty printing with tables
- [ ] Edit existing tasks
- [ ] Global binary installation

## License ⚖️

[MIT License]
