#!/bin/bash
# Version consistency check script
# Ensures Cargo.toml, CHANGELOG.md, and pkg/package.json versions are in sync

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "Checking version consistency..."

# Extract version from Cargo.toml [package] section
CARGO_VERSION=$(grep -A 10 '^\[package\]' "$PROJECT_ROOT/Cargo.toml" | grep '^version = ' | head -1 | sed 's/version = "\(.*\)"/\1/')

if [ -z "$CARGO_VERSION" ]; then
    echo -e "${RED}Error: Could not extract version from Cargo.toml${NC}"
    exit 1
fi

echo "  Cargo.toml version: $CARGO_VERSION"

# Extract version from CHANGELOG.md (latest release)
CHANGELOG_VERSION=$(grep -E '^## \[[0-9]+\.[0-9]+\.[0-9]+\]' "$PROJECT_ROOT/CHANGELOG.md" | head -1 | sed -E 's/^## \[([0-9]+\.[0-9]+\.[0-9]+)\].*/\1/')

if [ -z "$CHANGELOG_VERSION" ]; then
    echo -e "${YELLOW}Warning: Could not find version in CHANGELOG.md${NC}"
    echo "  Expected format: ## [X.Y.Z] - YYYY-MM-DD"
    CHANGELOG_VERSION="(not found)"
fi

echo "  CHANGELOG.md version: $CHANGELOG_VERSION"

# Extract version from pkg/package.json if it exists
PKG_JSON="$PROJECT_ROOT/pkg/package.json"
if [ -f "$PKG_JSON" ]; then
    PKG_VERSION=$(grep '"version"' "$PKG_JSON" | head -1 | sed -E 's/.*"version"[[:space:]]*:[[:space:]]*"([^"]+)".*/\1/')
    echo "  pkg/package.json version: $PKG_VERSION"
else
    echo -e "${YELLOW}  pkg/package.json: (not present - will be generated during WASM build)${NC}"
    PKG_VERSION=""
fi

# Check for mismatches
ERRORS=0

if [ "$CARGO_VERSION" != "$CHANGELOG_VERSION" ] && [ "$CHANGELOG_VERSION" != "(not found)" ]; then
    echo -e "${RED}Error: Version mismatch between Cargo.toml ($CARGO_VERSION) and CHANGELOG.md ($CHANGELOG_VERSION)${NC}"
    ERRORS=$((ERRORS + 1))
fi

if [ -n "$PKG_VERSION" ] && [ "$CARGO_VERSION" != "$PKG_VERSION" ]; then
    echo -e "${RED}Error: Version mismatch between Cargo.toml ($CARGO_VERSION) and pkg/package.json ($PKG_VERSION)${NC}"
    echo -e "${YELLOW}  Hint: Run 'wasm-pack build' to regenerate pkg/package.json, then update version${NC}"
    ERRORS=$((ERRORS + 1))
fi

if [ $ERRORS -gt 0 ]; then
    echo -e "${RED}Version check failed with $ERRORS error(s)${NC}"
    echo ""
    echo "To fix version mismatches:"
    echo "  1. Update Cargo.toml with the new version"
    echo "  2. Add a CHANGELOG.md entry: ## [X.Y.Z] - YYYY-MM-DD"
    echo "  3. If pkg/package.json exists, update its version to match"
    exit 1
fi

echo -e "${GREEN}All versions are consistent!${NC}"
exit 0
