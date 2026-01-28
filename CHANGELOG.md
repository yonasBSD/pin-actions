# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-01-28

### Added
- Initial release
- Pin GitHub Actions to commit SHAs
- Concurrent SHA resolution with configurable parallelism
- Dry run mode to preview changes
- Backup creation before modifying files
- Smart detection of already-pinned actions
- Beautiful terminal output with progress bars
- JSON output format for CI/CD integration
- Comprehensive test suite
- Full documentation

### Features
- Fast concurrent SHA resolution
- Automatic detection of workflow files
- Skip local actions (./action@ref)
- Preserve original refs in comments
- Verbose logging mode
- Cross-platform support (Linux, macOS, Windows)

[Unreleased]: https://github.com/yourusername/pin-actions/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/pin-actions/releases/tag/v0.1.0
