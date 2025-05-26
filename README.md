# Todo App (Rust CLI)

A simple command-line todo application written in Rust.

## Features
- Add, list, complete, remove tasks
- Output in plain or JSON format
- Autocompletion for bash/zsh/fish

## Usage

```bash
todo_app add "Write Rust code"
todo_app list --format json
todo_app completions --shell zsh

## 🧩 CLI Completions (Zsh)

This CLI supports auto-completion, making it easier to discover and use commands and flags.

### 🔧 Enable Zsh Completions

   ```bash
   source <(todo_app completions --shell zsh)
