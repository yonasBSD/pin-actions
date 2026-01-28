# 📌 pin-actions

[![Crates.io](https://img.shields.io/crates/v/pin-actions.svg)](https://crates.io/crates/pin-actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![CI](https://github.com/yourusername/pin-actions/workflows/CI/badge.svg)](https://github.com/yourusername/pin-actions/actions)

A blazingly fast CLI tool to pin GitHub Actions to specific commit SHAs for improved security and reproducibility.

## 🎯 Why Pin Actions?

Using tags (like `@v4`) or branches in your GitHub Actions workflows can be risky:

- **Security**: Tags and branches can be moved to point to malicious code
- **Reproducibility**: Your workflows might behave differently over time
- **Supply Chain**: Reduces attack surface for supply chain attacks

Pinning to commit SHAs ensures your workflows always run the exact same code.

## ✨ Features

- 🚀 **Fast**: Concurrent SHA resolution with configurable parallelism
- 🎯 **Smart**: Automatically detects and skips already-pinned actions
- 💾 **Safe**: Optional backup creation before modifications
- 🧪 **Dry Run**: Preview changes without modifying files
- 🎨 **Beautiful**: Colorful terminal output with progress bars
- 📊 **Flexible**: JSON output for CI/CD integration
- ✅ **Tested**: Comprehensive test suite
- 🦀 **Reliable**: Written in Rust for performance and safety

## 📦 Installation

### From Cargo

```bash
cargo install pin-actions
```

### From Source

```bash
git clone https://github.com/yourusername/pin-actions
cd pin-actions
cargo install --path .
```

### Using Homebrew (macOS/Linux)

```bash
brew install pin-actions
```

### Pre-built Binaries

Download from the [releases page](https://github.com/yourusername/pin-actions/releases).

## 🚀 Usage

### Basic Usage

```bash
# Pin all actions in .github/workflows
pin-actions

# Specify a custom workflows directory
pin-actions -w path/to/workflows

# Dry run to preview changes
pin-actions --dry-run

# Create backups before modifying
pin-actions --backup

# Increase concurrency for faster resolution
pin-actions -j 20

# Enable verbose logging
pin-actions --verbose

# Output results as JSON
pin-actions --format json
```

### Example

**Before:**
```yaml
name: CI
on: [push]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v3
```

**After:**
```yaml
name: CI
on: [push]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@b4ffde65f46336ab88eb53be808477a3936bae11 # v4
      - uses: actions/setup-node@1e60f620b9541d16bece96c5465dc8ee9832be0b # v3
```

## 🔧 Options

```
Options:
  -w, --workflows-dir <PATH>    Path to workflows directory [default: .github/workflows]
  -n, --dry-run                 Preview changes without modifying files
  -b, --backup                  Create .bak files before modifying
  -j, --jobs <N>                Number of concurrent SHA resolutions [default: 10]
  -v, --verbose                 Enable verbose output
      --skip-pinned             Skip actions that are already pinned [default: true]
  -f, --format <FORMAT>         Output format: text or json [default: text]
  -h, --help                    Print help
  -V, --version                 Print version
```

## 📊 Output Example

```
🔍 Scanning workflows in .github/workflows
Found 3 workflow file(s)
Resolving 5 unique action(s)
#####################################> 5/5 Resolution complete

  📌 actions/checkout@v4 → b4ffde65
  📌 actions/setup-node@v3 → 1e60f620
  📌 actions/cache@v3 → 704facf5
  📌 github/codeql-action/analyze@v2 → 959cbb7c
  📌 actions/upload-artifact@v3 → a8a3f3ad

📊 Summary
──────────────────────────────────────────────────
  Files processed:  3
  Actions found:    8
  Actions pinned:   5
  Already pinned:   3
  Errors:           0
──────────────────────────────────────────────────

✅ All unpinned actions have been pinned to commit SHAs
```

## 🔒 Security Best Practices

1. **Always review changes**: Use `--dry-run` first to see what will be changed
2. **Use backups**: Add `--backup` flag for safety
3. **Verify SHAs**: Check that resolved SHAs match expected versions
4. **Keep updated**: Regularly update pinned SHAs to get security fixes
5. **Use Dependabot**: Configure Dependabot to update pinned actions

## 🏗️ Architecture

```
pin-actions/
├── src/
│   ├── main.rs       # CLI entry point
│   ├── action.rs     # Action representation
│   ├── git.rs        # Git SHA resolution
│   ├── parser.rs     # Workflow YAML parsing
│   └── workflow.rs   # Workflow processing logic
├── tests/            # Integration tests
└── Cargo.toml        # Dependencies
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run integration tests only
cargo test --test integration

# Run with coverage
cargo tarpaulin --out Html
```

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development

```bash
# Clone the repository
git clone https://github.com/yourusername/pin-actions
cd pin-actions

# Run in development mode
cargo run -- --help

# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy -- -D warnings

# Build release
cargo build --release
```

## 📝 License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 🙏 Acknowledgments

- Inspired by the need for better GitHub Actions security
- Built with amazing Rust crates: clap, tokio, git2, regex, and more

## 📚 Related Projects

- [Dependabot](https://github.com/dependabot/dependabot-core) - Automated dependency updates
- [actionlint](https://github.com/rhysd/actionlint) - GitHub Actions workflow linter
- [act](https://github.com/nektos/act) - Run GitHub Actions locally

## 🐛 Reporting Issues

Found a bug? Have a feature request? Please [open an issue](https://github.com/yourusername/pin-actions/issues/new).

## 📮 Contact

- GitHub: [@yourusername](https://github.com/yourusername)
- Email: you@example.com

---

Made with ❤️ and 🦀 by the community
