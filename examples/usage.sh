#!/usr/bin/env bash
# Example usage script for pin-actions

set -euo pipefail

echo "🔍 pin-actions - Example Usage"
echo "================================"
echo

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Create example workflow directory
TEMP_DIR=$(mktemp -d)
WORKFLOWS_DIR="$TEMP_DIR/.github/workflows"
mkdir -p "$WORKFLOWS_DIR"

echo -e "${BLUE}Creating example workflow files...${NC}"

# Create example CI workflow
cat > "$WORKFLOWS_DIR/ci.yml" << 'EOF'
name: CI
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v3
      - name: Install dependencies
        run: npm install
      - name: Run tests
        run: npm test
  
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v3
      - run: npm run lint
EOF

# Create example release workflow
cat > "$WORKFLOWS_DIR/release.yml" << 'EOF'
name: Release
on:
  push:
    tags:
      - 'v*'
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v3
      - uses: docker/build-push-action@v5
      - uses: actions/upload-artifact@v3
EOF

echo -e "${GREEN}✓ Created example workflows${NC}"
echo

# Example 1: Dry run
echo -e "${BLUE}Example 1: Dry run (preview changes)${NC}"
echo "$ pin-actions --workflows-dir $WORKFLOWS_DIR --dry-run"
echo

pin-actions --workflows-dir "$WORKFLOWS_DIR" --dry-run || true

echo
echo "---"
echo

# Example 2: Verbose mode
echo -e "${BLUE}Example 2: Verbose output${NC}"
echo "$ pin-actions --workflows-dir $WORKFLOWS_DIR --verbose --dry-run"
echo

pin-actions --workflows-dir "$WORKFLOWS_DIR" --verbose --dry-run || true

echo
echo "---"
echo

# Example 3: With backup
echo -e "${BLUE}Example 3: Pin actions with backup${NC}"
echo "$ pin-actions --workflows-dir $WORKFLOWS_DIR --backup"
echo

pin-actions --workflows-dir "$WORKFLOWS_DIR" --backup || true

# Show backup files
echo -e "${YELLOW}Backup files created:${NC}"
ls -la "$WORKFLOWS_DIR"/*.bak 2>/dev/null || echo "No backup files"

echo
echo "---"
echo

# Example 4: JSON output
echo -e "${BLUE}Example 4: JSON output (for CI/CD)${NC}"
echo "$ pin-actions --workflows-dir $WORKFLOWS_DIR --format json"
echo

pin-actions --workflows-dir "$WORKFLOWS_DIR" --format json || true

echo
echo "---"
echo

# Show final result
echo -e "${GREEN}Final workflow (with pinned actions):${NC}"
cat "$WORKFLOWS_DIR/ci.yml"

# Cleanup
echo
echo -e "${BLUE}Cleaning up temporary directory...${NC}"
rm -rf "$TEMP_DIR"

echo
echo -e "${GREEN}✓ Examples complete!${NC}"
echo
echo "Try these commands yourself:"
echo "  pin-actions --help"
echo "  pin-actions --dry-run"
echo "  pin-actions --backup --verbose"
