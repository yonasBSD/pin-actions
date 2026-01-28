# Advanced Usage Guide

This guide covers advanced usage patterns and best practices for `pin-actions`.

## Table of Contents

- [CI/CD Integration](#cicd-integration)
- [Performance Tuning](#performance-tuning)
- [Error Handling](#error-handling)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)

## CI/CD Integration

### GitHub Actions

Use pin-actions in your CI pipeline to automatically pin actions:

```yaml
name: Pin Actions
on:
  schedule:
    - cron: '0 0 * * 0'  # Weekly
  workflow_dispatch:

jobs:
  pin:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install pin-actions
        run: |
          curl -L https://github.com/yourusername/pin-actions/releases/latest/download/pin-actions-linux-x86_64.tar.gz | tar xz
          sudo mv pin-actions /usr/local/bin/
      
      - name: Pin actions
        run: pin-actions --format json > results.json
      
      - name: Create PR if changes
        uses: peter-evans/create-pull-request@v5
        with:
          title: "chore: pin GitHub Actions to SHAs"
          body: |
            Automated PR to pin GitHub Actions to commit SHAs.
            
            See results.json for details.
          branch: pin-actions-update
```

### GitLab CI

```yaml
pin-actions:
  image: rust:latest
  script:
    - cargo install pin-actions
    - pin-actions --backup
  artifacts:
    paths:
      - .github/workflows/*.bak
  only:
    - schedules
```

### Pre-commit Hook

Add to `.pre-commit-config.yaml`:

```yaml
repos:
  - repo: local
    hooks:
      - id: pin-actions
        name: Pin GitHub Actions
        entry: pin-actions
        language: system
        pass_filenames: false
        always_run: true
```

## Performance Tuning

### Concurrency Control

Adjust the number of concurrent SHA resolutions:

```bash
# Default (10 concurrent)
pin-actions

# More aggressive (20 concurrent)
pin-actions -j 20

# Conservative (5 concurrent)
pin-actions -j 5
```

### Caching

Pin-actions caches resolved SHAs during execution. For repeated runs:

1. The first run will fetch all SHAs
2. Subsequent runs reuse cached results

### Large Repositories

For repositories with many workflows:

```bash
# Process workflows in batches
for dir in .github/workflows/*/; do
  pin-actions --workflows-dir "$dir" --verbose
done
```

## Error Handling

### Handling Resolution Failures

When an action can't be resolved:

```bash
# Continue despite errors
pin-actions || echo "Some actions failed to resolve"

# Get error details
pin-actions --format json | jq '.errors'
```

### Network Issues

For flaky networks:

```bash
# Retry on failure
retry_count=0
max_retries=3

while [ $retry_count -lt $max_retries ]; do
  if pin-actions; then
    break
  fi
  retry_count=$((retry_count + 1))
  sleep 5
done
```

## Best Practices

### 1. Always Use Dry Run First

```bash
# Preview changes
pin-actions --dry-run

# Review output, then apply
pin-actions
```

### 2. Create Backups

```bash
# Always create backups for safety
pin-actions --backup

# Restore if needed
cp .github/workflows/ci.yml.bak .github/workflows/ci.yml
```

### 3. Version Control Integration

```bash
# Check what will change
pin-actions --dry-run

# Apply changes
pin-actions

# Review diff
git diff .github/workflows/

# Commit if satisfied
git add .github/workflows/
git commit -m "chore: pin GitHub Actions to SHAs"
```

### 4. Keep SHAs Updated

Set up automated updates:

```yaml
# .github/workflows/update-pins.yml
name: Update Pinned Actions
on:
  schedule:
    - cron: '0 0 1 * *'  # Monthly

jobs:
  update:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Update pins
        run: |
          pin-actions
          
      - name: Create PR
        if: git diff --quiet
        run: |
          # Create PR with updated pins
```

### 5. Document Pinned Versions

Add comments to workflow files:

```yaml
# Before pin-actions runs:
- uses: actions/checkout@v4

# After pin-actions runs:
- uses: actions/checkout@b4ffde65f46336ab88eb53be808477a3936bae11 # v4
```

### 6. Validate Workflows

After pinning, validate workflows:

```bash
# Pin actions
pin-actions

# Validate syntax
actionlint .github/workflows/*.yml

# Test locally
act -l
```

## Troubleshooting

### Problem: Can't Resolve SHA

**Symptom:**
```
WARN: could not resolve actions/some-action@v1
```

**Solutions:**

1. Check if the ref exists:
   ```bash
   git ls-remote https://github.com/actions/some-action.git v1
   ```

2. Verify network access:
   ```bash
   ping github.com
   ```

3. Try with verbose output:
   ```bash
   pin-actions --verbose
   ```

### Problem: Wrong SHA Resolved

**Symptom:**
Actions pinned to unexpected SHAs

**Solutions:**

1. Check the ref manually:
   ```bash
   git ls-remote https://github.com/actions/checkout.git v4
   ```

2. Use dry-run to verify:
   ```bash
   pin-actions --dry-run
   ```

3. Review the output carefully before applying

### Problem: File Permissions

**Symptom:**
```
Error: Permission denied
```

**Solutions:**

```bash
# Check file permissions
ls -la .github/workflows/

# Fix permissions
chmod 644 .github/workflows/*.yml

# Run as appropriate user
sudo pin-actions  # Not recommended
```

### Problem: Large Repository Performance

**Symptom:**
Slow performance with many workflows

**Solutions:**

```bash
# Increase concurrency
pin-actions -j 30

# Process in parallel
find .github/workflows -name "*.yml" | \
  xargs -P 4 -I {} pin-actions --workflows-dir {}

# Use selective processing
pin-actions --workflows-dir .github/workflows/critical/
```

## Advanced Patterns

### Selective Pinning

Pin only specific workflows:

```bash
# Pin only CI workflows
find .github/workflows -name "ci*.yml" -o -name "test*.yml" | \
  while read file; do
    dir=$(dirname "$file")
    pin-actions --workflows-dir "$dir"
  done
```

### Automated Rotation

Regularly update pinned SHAs:

```bash
#!/bin/bash
# rotate-pins.sh

# Pin actions
pin-actions

# Check if changes were made
if ! git diff --quiet .github/workflows/; then
  # Run tests
  make test
  
  # If tests pass, commit
  if [ $? -eq 0 ]; then
    git add .github/workflows/
    git commit -m "chore: rotate pinned action SHAs"
    git push origin update-pins
  fi
fi
```

### Audit Trail

Keep track of SHA updates:

```bash
# Before pinning
git log -1 --format="%H %s" > sha-audit.log

# Pin actions
pin-actions --format json >> sha-audit.log

# After pinning
git log -1 --format="%H %s" >> sha-audit.log
```

## Integration Examples

### With Dependabot

Configure Dependabot to update pinned actions:

```yaml
# .github/dependabot.yml
version: 2
updates:
  - package-ecosystem: "github-actions"
    directory: "/"
    schedule:
      interval: "weekly"
    # Dependabot will update the SHAs
```

### With Renovate

```json
{
  "extends": ["config:base"],
  "github-actions": {
    "enabled": true,
    "pinDigests": true
  }
}
```

### With Pre-commit

```yaml
# .pre-commit-config.yaml
repos:
  - repo: local
    hooks:
      - id: pin-actions
        name: Pin GitHub Actions
        entry: pin-actions
        language: system
        files: '^\.github/workflows/.*\.ya?ml$'
        pass_filenames: false
```

## Resources

- [GitHub Actions Security Best Practices](https://docs.github.com/en/actions/security-guides/security-hardening-for-github-actions)
- [Supply Chain Security](https://slsa.dev/)
- [Action Pinning Discussion](https://github.com/actions/toolkit/issues/646)
