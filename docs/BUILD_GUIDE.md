# Build & Installation Guide

This guide walks you through building and installing `pin-actions` from source.

## Prerequisites

### Required

- **Rust 1.70+**: Install from [rustup.rs](https://rustup.rs)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Git**: For cloning and git operations
  ```bash
  # macOS
  brew install git
  
  # Ubuntu/Debian
  sudo apt install git
  
  # Windows
  winget install Git.Git
  ```

### Optional

- **Make**: For using Makefile commands
- **Docker**: For containerized builds
- **cargo-watch**: For development
  ```bash
  cargo install cargo-watch
  ```

## Quick Install

### From Crates.io (Recommended)

```bash
cargo install pin-actions
```

### From GitHub Releases

```bash
# Linux x86_64
curl -L https://github.com/yourusername/pin-actions/releases/latest/download/pin-actions-linux-x86_64.tar.gz | tar xz
sudo mv pin-actions /usr/local/bin/

# macOS x86_64
curl -L https://github.com/yourusername/pin-actions/releases/latest/download/pin-actions-macos-x86_64.tar.gz | tar xz
sudo mv pin-actions /usr/local/bin/

# macOS ARM64
curl -L https://github.com/yourusername/pin-actions/releases/latest/download/pin-actions-macos-aarch64.tar.gz | tar xz
sudo mv pin-actions /usr/local/bin/

# Windows
# Download pin-actions-windows-x86_64.zip from releases page
```

### Using Homebrew (macOS/Linux)

```bash
brew tap yourusername/pin-actions
brew install pin-actions
```

## Building from Source

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/pin-actions.git
cd pin-actions
```

### 2. Build Debug Version

For development and testing:

```bash
cargo build
```

Binary location: `target/debug/pin-actions`

### 3. Build Release Version

Optimized for production:

```bash
cargo build --release
```

Binary location: `target/release/pin-actions`

### 4. Install System-wide

```bash
cargo install --path .
```

This installs to `~/.cargo/bin/pin-actions` (ensure this is in your PATH).

## Verification

Verify the installation:

```bash
pin-actions --version
# Should output: pin-actions 0.1.0

pin-actions --help
# Should show help text
```

## Platform-Specific Instructions

### Linux

#### Ubuntu/Debian

```bash
# Install dependencies
sudo apt update
sudo apt install -y build-essential git pkg-config libssl-dev

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Clone and build
git clone https://github.com/yourusername/pin-actions.git
cd pin-actions
cargo build --release

# Install
sudo cp target/release/pin-actions /usr/local/bin/
```

#### RHEL/CentOS/Fedora

```bash
# Install dependencies
sudo dnf install -y gcc git openssl-devel pkg-config

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Clone and build
git clone https://github.com/yourusername/pin-actions.git
cd pin-actions
cargo build --release

# Install
sudo cp target/release/pin-actions /usr/local/bin/
```

### macOS

```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install Homebrew (if not already installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Clone and build
git clone https://github.com/yourusername/pin-actions.git
cd pin-actions
cargo build --release

# Install
sudo cp target/release/pin-actions /usr/local/bin/
```

### Windows

```powershell
# Install Visual Studio Build Tools
# Download from: https://visualstudio.microsoft.com/downloads/

# Install Rust
# Download from: https://rustup.rs/

# Clone and build
git clone https://github.com/yourusername/pin-actions.git
cd pin-actions
cargo build --release

# Add to PATH or copy to desired location
copy target\release\pin-actions.exe C:\Windows\System32\
```

## Docker Build

### Build Image

```bash
docker build -t pin-actions .
```

### Run Container

```bash
docker run -v $(pwd):/workspace pin-actions \
  --workflows-dir /workspace/.github/workflows
```

### Docker Compose

```bash
# Development
docker-compose up dev

# Run pin-actions
docker-compose run pin-actions --dry-run
```

## Cross-Compilation

### Linux to Windows

```bash
# Install target
rustup target add x86_64-pc-windows-gnu

# Install MinGW
sudo apt install mingw-w64

# Build
cargo build --release --target x86_64-pc-windows-gnu
```

### macOS to Linux

```bash
# Install target
rustup target add x86_64-unknown-linux-musl

# Build
cargo build --release --target x86_64-unknown-linux-musl
```

## Development Setup

### Install Development Tools

```bash
# Formatter
rustup component add rustfmt

# Linter
rustup component add clippy

# Watch for changes
cargo install cargo-watch

# Code coverage
cargo install cargo-tarpaulin

# Benchmarking
cargo install cargo-criterion
```

### Setup Pre-commit Hooks

```bash
# Install pre-commit
pip install pre-commit

# Install hooks
pre-commit install
```

### Run Tests

```bash
# All tests
cargo test

# With output
cargo test -- --nocapture

# Integration tests only
cargo test --test integration

# Run continuously
cargo watch -x test
```

### Run Benchmarks

```bash
cargo bench
```

### Generate Documentation

```bash
cargo doc --no-deps --open
```

## Troubleshooting

### Build Fails

**Error: `linker 'cc' not found`**

```bash
# Linux
sudo apt install build-essential

# macOS
xcode-select --install
```

**Error: `failed to run custom build command for openssl-sys`**

```bash
# Linux
sudo apt install pkg-config libssl-dev

# macOS
brew install openssl
export OPENSSL_DIR=$(brew --prefix openssl)
```

**Error: `could not find libgit2`**

The git2 dependency is automatically compiled. If issues persist:

```bash
# Linux
sudo apt install libgit2-dev

# macOS
brew install libgit2
```

### Runtime Issues

**Error: `git ls-remote failed`**

Ensure git is installed and in PATH:

```bash
git --version
which git
```

**Permission Denied**

```bash
# Fix binary permissions
chmod +x ~/.cargo/bin/pin-actions

# Or for system-wide install
sudo chmod +x /usr/local/bin/pin-actions
```

## Performance Tips

### Release Builds

Always use release builds for production:

```bash
cargo build --release
# NOT: cargo build
```

### Strip Symbols

Reduce binary size:

```bash
strip target/release/pin-actions
```

### Profile-Guided Optimization (PGO)

For maximum performance:

```bash
# Build with instrumentation
RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" \
  cargo build --release

# Run with representative workload
./target/release/pin-actions --dry-run

# Build with PGO
llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data

RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata" \
  cargo build --release
```

## Updating

### From Crates.io

```bash
cargo install pin-actions --force
```

### From Source

```bash
cd pin-actions
git pull origin main
cargo install --path . --force
```

## Uninstalling

### Cargo Installation

```bash
cargo uninstall pin-actions
```

### Manual Installation

```bash
# Linux/macOS
sudo rm /usr/local/bin/pin-actions

# Windows
del C:\Windows\System32\pin-actions.exe
```

### Homebrew

```bash
brew uninstall pin-actions
```

## Next Steps

After installation:

1. Read the [Quick Reference](QUICK_REFERENCE.md)
2. Try the [examples](../examples/usage.sh)
3. Review [Security Guide](SECURITY.md)
4. Check [Advanced Usage](ADVANCED_USAGE.md)

## Support

If you encounter issues:

1. Check [Troubleshooting](#troubleshooting)
2. Search [existing issues](https://github.com/yourusername/pin-actions/issues)
3. Open a [new issue](https://github.com/yourusername/pin-actions/issues/new)

## Resources

- [Rust Installation](https://www.rust-lang.org/tools/install)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Git Documentation](https://git-scm.com/doc)
