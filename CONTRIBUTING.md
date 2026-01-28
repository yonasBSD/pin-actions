# Contributing to pin-actions

Thank you for your interest in contributing to pin-actions! We welcome contributions from the community.

## 📋 Code of Conduct

This project adheres to a code of conduct. By participating, you are expected to uphold this code.

## 🚀 Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/yourusername/pin-actions.git`
3. Create a feature branch: `git checkout -b feature/amazing-feature`
4. Make your changes
5. Run tests: `cargo test`
6. Commit your changes: `git commit -m 'Add amazing feature'`
7. Push to the branch: `git push origin feature/amazing-feature`
8. Open a Pull Request

## 🛠️ Development Setup

### Prerequisites

- Rust 1.70 or higher
- Git

### Building

```bash
# Clone the repository
git clone https://github.com/yourusername/pin-actions.git
cd pin-actions

# Build the project
cargo build

# Run tests
cargo test

# Run the CLI
cargo run -- --help
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run integration tests only
cargo test --test integration
```

### Code Quality

Before submitting a PR, ensure your code passes all checks:

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy -- -D warnings

# Run tests
cargo test
```

## 📝 Pull Request Guidelines

1. **Keep PRs focused**: One feature or fix per PR
2. **Write tests**: Add tests for new features
3. **Update documentation**: Update README and docs as needed
4. **Follow code style**: Run `cargo fmt` before committing
5. **Write good commit messages**: Clear and descriptive
6. **Update CHANGELOG**: Add entry for your changes

### PR Checklist

- [ ] Tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation is updated
- [ ] CHANGELOG is updated
- [ ] Commits are clear and descriptive

## 🐛 Reporting Bugs

When reporting bugs, please include:

1. **Description**: Clear description of the bug
2. **Steps to reproduce**: Detailed steps to reproduce the issue
3. **Expected behavior**: What you expected to happen
4. **Actual behavior**: What actually happened
5. **Environment**: OS, Rust version, etc.
6. **Logs**: Any relevant error messages or logs

## 💡 Feature Requests

We welcome feature requests! Please:

1. Check existing issues first
2. Provide a clear use case
3. Explain why this feature would be useful
4. Include examples if possible

## 🏗️ Project Structure

```
pin-actions/
├── src/
│   ├── main.rs       # CLI entry point
│   ├── action.rs     # Action representation
│   ├── git.rs        # Git operations
│   ├── parser.rs     # YAML parsing
│   └── workflow.rs   # Workflow processing
├── tests/            # Integration tests
├── .github/          # GitHub Actions workflows
└── docs/             # Documentation
```

## 📖 Documentation

- Code should be self-documenting when possible
- Add doc comments for public APIs
- Update README for user-facing changes
- Keep examples up to date

## 🎨 Code Style

We follow the standard Rust style guide:

- Use `cargo fmt` for formatting
- Follow Rust naming conventions
- Write idiomatic Rust code
- Keep functions focused and small
- Use meaningful variable names

## 🧪 Testing Guidelines

- Write unit tests for new functions
- Add integration tests for features
- Aim for high test coverage
- Test edge cases
- Use meaningful test names

## 📦 Release Process

Releases are automated via GitHub Actions:

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md
3. Create and push a tag: `git tag v0.1.0 && git push origin v0.1.0`
4. GitHub Actions will build and publish automatically

## 💬 Getting Help

- Open an issue for bugs or features
- Check existing issues and PRs
- Join discussions in issues

## 📄 License

By contributing, you agree that your contributions will be licensed under the same license as the project (MIT or Apache-2.0).

## 🙏 Thank You!

Thank you for contributing to pin-actions! Your efforts help make GitHub Actions more secure for everyone.
