# Git Rewang

<p align="center">
  <img src="assets/gr-lg.png" alt="Git Rewang Logo"/>
</p>

Git Rewang is a lightweight CLI tool for developers who use multiple GitHub accounts. It simplifies commit operations with auto templates, branch management, and simplified status display.

## Features

- **Auto Commit Template** - Consistent commit messages with emoji and standard format
- **Branch Cleaner** - Remove local branches that have been merged
- **Simplified Status** - Cleaner repository status display
- **Flexible Configuration** - Support for per-project and global config

## Installation

### Build from Source

```bash
# Clone repository
git clone https://github.com/your-username/git-rewang.git
cd git-rewang

# Build
cargo build --release

# Binary available at target/release/git-rewang
```

### Add to PATH (Optional)

```bash
# Windows (PowerShell)
$env:Path += ";D:\path\to\git-rewang\target\release"

# Linux/macOS
export PATH="$PATH:/path/to/git-rewang/target/release"
```

## Usage

### Show Help

```bash
git-rewang --help
```

### Repository Status

Display repository status in a simplified format.

```bash
git-rewang status
```

Output:
```
Repository Status
──────────────────────────────
  branch:    main
  staged:    3 files
  modified:  2 files
  untracked: 1 files
```

### Commit with Template

Create a commit with auto template based on type.

```bash
# Commit with default type (feat)
git-rewang commit "add login feature"

# Commit with specific type
git-rewang commit "fix validation error" -t fix
git-rewang commit "update installation guide" -t docs
```

**Available commit types:**

| Type | Template |
|------|----------|
| `feat` | ✨ feat: {message} |
| `fix` | 🐛 fix: {message} |
| `docs` | 📝 docs: {message} |
| `style` | 💄 style: {message} |
| `refactor` | ♻️ refactor: {message} |
| `test` | ✅ test: {message} |
| `chore` | 🔧 chore: {message} |

### List Branches

Display all local branches.

```bash
git-rewang list-branches
```

### Clean Merged Branches

Remove local branches that have been merged to the main branch.

```bash
git-rewang clean-branches
```

## Configuration

### Configuration File

Git Rewang searches for configuration with the following priority:
1. **Project config** - `git-helper.toml` in project root
2. **Global config** - `~/.config/git-helper/config.toml`
3. **Default** - Built-in configuration

### Example `git-helper.toml`

```toml
default_branch = "main"

[commit_types]
feat = "✨ feat: {message}"
fix = "🐛 fix: {message}"
docs = "📝 docs: {message}"
style = "💄 style: {message}"
refactor = "♻️ refactor: {message}"
test = "✅ test: {message}"
chore = "🔧 chore: {message}"

[user]
name = "Your Name"
email = "your@email.com"
```

## Project Structure

```
git-rewang/
├── src/
│   ├── main.rs           # Entry point
│   ├── cli.rs            # CLI argument parser
│   ├── git/
│   │   ├── mod.rs
│   │   ├── repo.rs       # Repository operations
│   │   ├── branch.rs     # Branch management
│   │   ├── commit.rs     # Commit operations
│   │   └── status.rs     # Status display
│   ├── template/
│   │   ├── mod.rs
│   │   └── loader.rs     # Template loading & rendering
│   ├── config/
│   │   ├── mod.rs
│   │   └── config.rs     # Configuration management
│   └── utils/
│       ├── mod.rs
│       └── fs.rs         # Filesystem utilities
├── git-helper.toml       # Project config example
└── Cargo.toml
```

## Dependencies

| Crate | Purpose |
|-------|----------|
| `clap` | CLI argument parsing |
| `git2` | Git operations (native binding) |
| `serde` | Serialization/deserialization |
| `toml` | Config file parsing |
| `colored` | Terminal colors |
| `chrono` | Date/time handling |
| `anyhow` | Error handling |
| `dirs` | System directory paths |

## Requirements

- Rust 1.70+
- Git repository (for git operations)

## License

MIT License