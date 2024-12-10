# 🦀 Git Commit Assistant

A powerful Rust-based CLI tool that streamlines your Git workflow by automating commit message generation and branch management.

## ✨ Features

- 📝 **Automated Commit Message Generation** - Creates structured commit messages with file-by-file descriptions
- 🌳 **Smart Branch Management** - Easy branch switching with stash handling
- 🎯 **Selective File Inclusion** - Support for `.commitignore` to exclude specific files from commit messages
- 🚀 **Git Operations** - Simplified commit and push operations
- 🎨 **Interactive UI** - Beautiful terminal interface with colored prompts

## 🚀 Quick Start

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/git-commit-rust.git
   cd git-commit-rust
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Add to your PATH or create an alias:
   ```bash
   # In your shell config file (.bashrc, .zshrc, config.fish, etc.)
   alias gcommit="/path/to/git-commit-rust/target/release/git_commit_with_rust"
   ```

## 💡 Usage

### Generate Commit Message

```bash
gcommit -g
```
This will:
- Create a `commit_message.md` file
- Scan for modified/added files
- Generate a structured template
- Open the file in your editor

### Commit Changes

```bash
gcommit -c          # Commit only
gcommit -cp         # Commit and push
gcommit -cpa --set-upstream origin main  # Commit and push with args
```

### Branch Management

```bash
gcommit -s          # Switch branches
gcommit -s --stash  # Stash changes before switching
gcommit -s --apply-stash  # Apply stash after switching
```

## 📋 Commit Message Structure

```markdown
[commit_nb] (<type> on <branch>) TL;DR Brief description

- `file1.rs`:

    Description of changes in file1

- `file2.rs`:

    Description of changes in file2
```

Where `type` can be:
- `feat`: New features
- `fix`: Bug fixes
- `test`: Test-related changes
- `chore`: Maintenance tasks

## 🎯 .commitignore

Similar to `.gitignore`, you can create a `.commitignore` file to exclude files from appearing in the commit message template:

```plaintext
# Exclude documentation
*.md
docs/

# Exclude specific folders
target/
tests/
```

## 🛠️ Commands

| Command | Description |
|---------|-------------|
| `-g, --generate` | Generate commit message template |
| `-c, --commit` | Commit changes using template |
| `-p, --push` | Push changes to remote |
| `-s, --switch` | Interactive branch switching |
| `-a, --add-exclude` | Add files with exclusions |

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.
