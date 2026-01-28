# pin-actions Quick Reference

## Installation

```bash
# Cargo
cargo install pin-actions

# Homebrew
brew install pin-actions

# From source
git clone https://github.com/yourusername/pin-actions
cd pin-actions
cargo install --path .

# Docker
docker pull pin-actions:latest
```

## Common Commands

```bash
# Pin all actions (default)
pin-actions

# Preview changes (dry run)
pin-actions --dry-run

# Custom workflows directory
pin-actions -w path/to/workflows

# Create backups
pin-actions --backup

# Verbose output
pin-actions --verbose

# JSON output
pin-actions --format json

# Increase concurrency
pin-actions -j 20

# Combine options
pin-actions --dry-run --verbose --backup
```

## Quick Start

1. Navigate to your repository root
2. Run dry-run to preview:
   ```bash
   pin-actions --dry-run
   ```
3. Review the changes
4. Apply changes with backup:
   ```bash
   pin-actions --backup
   ```
5. Commit and push:
   ```bash
   git add .github/workflows/
   git commit -m "chore: pin GitHub Actions"
   git push
   ```

## Flags

| Flag | Short | Description | Default |
|------|-------|-------------|---------|
| `--workflows-dir` | `-w` | Workflows directory | `.github/workflows` |
| `--dry-run` | `-n` | Preview without changes | `false` |
| `--backup` | `-b` | Create .bak files | `false` |
| `--jobs` | `-j` | Concurrent SHA resolves | `10` |
| `--verbose` | `-v` | Verbose logging | `false` |
| `--skip-pinned` | | Skip already pinned | `true` |
| `--format` | `-f` | Output format | `text` |
| `--help` | `-h` | Show help | |
| `--version` | `-V` | Show version | |

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | Errors occurred |

## Output Format

### Text (default)

```
🔍 Scanning workflows in .github/workflows
Found 3 workflow file(s)
Resolving 5 unique action(s)
#####> 5/5 Resolution complete

  📌 actions/checkout@v4 → b4ffde65

📊 Summary
──────────────────────────────────────────
  Files processed:  3
  Actions found:    8
  Actions pinned:   5
  Already pinned:   3
  Errors:           0
──────────────────────────────────────────
```

### JSON

```json
{
  "files_processed": 3,
  "actions_found": 8,
  "actions_pinned": 5,
  "already_pinned": 3,
  "errors": 0,
  "pinned_actions": [
    {
      "file": ".github/workflows/ci.yml",
      "action": "actions/checkout",
      "old_ref": "v4",
      "sha": "b4ffde65f46336ab88eb53be808477a3936bae11"
    }
  ]
}
```

## Example Workflows

### Before

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

### After

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

## Common Use Cases

### CI/CD Integration

```bash
# In your CI pipeline
pin-actions --format json > pins.json
cat pins.json | jq '.errors'
```

### Pre-commit Hook

```yaml
# .pre-commit-config.yaml
repos:
  - repo: local
    hooks:
      - id: pin-actions
        name: Pin GitHub Actions
        entry: pin-actions --dry-run
        language: system
```

### Docker

```bash
docker run -v $(pwd):/workspace pin-actions \
  --workflows-dir /workspace/.github/workflows
```

### Scheduled Updates

```yaml
# .github/workflows/pin-actions.yml
name: Update Pins
on:
  schedule:
    - cron: '0 0 * * 0'  # Weekly
jobs:
  pin:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: pin-actions
      - uses: peter-evans/create-pull-request@v5
```

## Troubleshooting

### Can't resolve SHA

```bash
# Check manually
git ls-remote https://github.com/actions/checkout.git v4

# Try with verbose
pin-actions --verbose
```

### Permission denied

```bash
# Check permissions
ls -la .github/workflows/

# Fix permissions
chmod 644 .github/workflows/*.yml
```

### Network issues

```bash
# Check connectivity
ping github.com

# Retry with backoff
for i in {1..3}; do pin-actions && break || sleep 5; done
```

## Tips

- Always `--dry-run` first
- Use `--backup` for safety
- Increase `-j` for faster resolution
- Use `--format json` in CI/CD
- Keep pins updated monthly
- Review changes before committing

## Environment Variables

```bash
# Set log level
export RUST_LOG=pin_actions=debug

# Custom temp directory
export TMPDIR=/custom/tmp
```

## Performance

| Repository Size | Actions | Time (j=10) | Time (j=20) |
|----------------|---------|-------------|-------------|
| Small (1-5 workflows) | 5-10 | ~2s | ~1s |
| Medium (5-20 workflows) | 10-50 | ~5s | ~3s |
| Large (20+ workflows) | 50+ | ~15s | ~8s |

## Links

- [GitHub](https://github.com/yourusername/pin-actions)
- [Documentation](https://github.com/yourusername/pin-actions/tree/main/docs)
- [Report Issues](https://github.com/yourusername/pin-actions/issues)
- [Releases](https://github.com/yourusername/pin-actions/releases)

## Getting Help

```bash
# Show help
pin-actions --help

# Show version
pin-actions --version

# Check documentation
open https://github.com/yourusername/pin-actions/tree/main/docs
```
