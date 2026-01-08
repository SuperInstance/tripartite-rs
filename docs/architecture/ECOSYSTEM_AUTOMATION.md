# Ecosystem Automation Tools

> **Scripts and templates for maintaining cross-references**

**Version**: 1.0.0
**Last Updated**: 2026-01-08

---

## Overview

This document provides scripts, templates, and automation tools to keep the SuperInstance ecosystem documentation synchronized and cross-references accurate.

---

## Table of Contents

1. [Quick Start](#quick-start)
2. [Script Directory Structure](#script-directory-structure)
3. [Cross-Reference Generation](#cross-reference-generation)
4. [Link Validation](#link-validation)
5. [Ecosystem Documentation Sync](#ecosystem-documentation-sync)
6. [Badge Generation](#badge-generation)
7. [CI/CD Integration](#cicd-integration)

---

## Quick Start

### Installation

```bash
# Clone repository
git clone https://github.com/SuperInstance/Tripartite1.git
cd Tripartite1

# Install dependencies
cargo install mdbook
cargo install cargo-mdbook
npm install -g markdown-link-check
```

### Run All Checks

```bash
# Validate all cross-references
./scripts/validate-ecosystem.sh

# Generate cross-reference sections
./scripts/generate-cross-refs.sh

# Check all links
./scripts/check-links.sh
```

---

## Script Directory Structure

```
scripts/
├── ecosystem/
│   ├── generate-cross-refs.sh      # Generate ecosystem sections
│   ├── validate-ecosystem.sh       # Validate cross-references
│   ├── check-links.sh              # Check all external links
│   ├── update-topics.sh            # Update GitHub topics
│   └── sync-ecosystem-docs.sh      # Sync ecosystem documentation
├── templates/
│   ├── readme-ecosystem-section.md # README ecosystem template
│   ├── cargo-toml-metadata.toml    # Cargo.toml metadata template
│   └── ecosystem-entry.md          # New tool entry template
└── ci/
    ├── ecosystem-check.yml         # CI workflow for ecosystem checks
    └── link-check.yml              # CI workflow for link validation
```

---

## Cross-Reference Generation

### Script: `generate-cross-refs.sh`

```bash
#!/bin/bash
# generate-cross-refs.sh
# Generate ecosystem cross-reference sections for all crates

set -e

ECOSYSTEM_FILE="docs/ECOSYSTEM.md"
CRATES_DIR="crates"

echo "Generating cross-reference sections..."

# Get all crate directories
crates=($(find "$CRATES_DIR" -name "Cargo.toml" -exec dirname {} \;))

for crate in "${crates[@]}"; do
    name=$(basename "$crate")
    cargo_file="$crate/Cargo.toml"
    readme_file="$crate/README.md"

    echo "Processing: $name"

    # Extract metadata from Cargo.toml
    description=$(grep "^description = " "$cargo_file" | sed 's/description = //; s/"//g')
    version=$(grep "^version = " "$cargo_file" | sed 's/version = //; s/"//g')

    # Generate ecosystem section if README exists
    if [ -f "$readme_file" ]; then
        ./scripts/ecosystem/generate-ecosystem-section.sh \
            --name "$name" \
            --desc "$description" \
            --version "$version" \
            >> "$readme_file.tmp"

        echo "Generated ecosystem section for $name"
    fi
done

echo "✓ Cross-reference generation complete"
```

### Script: `generate-ecosystem-section.sh`

```bash
#!/bin/bash
# generate-ecosystem-section.sh
# Generate ecosystem section for a single crate

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --name) NAME="$2"; shift 2 ;;
        --desc) DESC="$2"; shift 2 ;;
        --version) VERSION="$2"; shift 2 ;;
        *) echo "Unknown option: $1"; exit 1 ;;
    esac
done

# Generate ecosystem section
cat << EOF
## 🌍 Ecosystem

### Used By

- **[SuperInstance AI](https://github.com/SuperInstance/Tripartite1)** - Main project using ${NAME}

### Requires

EOF

# Extract dependencies from Cargo.toml
cargo metadata --format-version 1 --no-deps 2>/dev/null | \
    jq -r ".packages[] | select(.name == \"${NAME}\") | .dependencies[] | select(.source == null) | .name" | \
    while read dep; do
        url=$(cargo search "$dep" --limit 1 2>/dev/null | grep "^${dep} =" | sed 's/.*# //; s/\".*//' || echo "")
        if [ -n "$url" ]; then
            echo "- **[${dep}](${url})**"
        else
            echo "- **${dep}**"
        fi
    done

cat << EOF

### Complementary Tools

- **[synesis-core](https://github.com/SuperInstance/Tripartite1)** - Agent orchestration
- **[synesis-privacy](https://github.com/SuperInstance/Tripartite1)** - Privacy proxy
- **[synesis-knowledge](https://github.com/SuperInstance/Tripartite1)** - Knowledge vault
- **[synesis-cloud](https://github.com/SuperInstance/Tripartite1)** - Cloud connectivity

### See Also

- **[llama.cpp](https://github.com/ggerganov/llama.cpp)** - Local LLM inference
- **[SQLite-VSS](https://github.com/asg017/sqlite-vss)** - Vector search extension

📖 **[View Full Ecosystem](https://github.com/SuperInstance/Tripartite1/blob/main/docs/ECOSYSTEM.md)**
EOF
```

---

## Link Validation

### Script: `check-links.sh`

```bash
#!/bin/bash
# check-links.sh
# Validate all external links in documentation

set -e

echo "Checking documentation links..."

# Install markdown-link-check if not present
if ! command -v markdown-link-check &> /dev/null; then
    echo "Installing markdown-link-check..."
    npm install -g markdown-link-check
fi

# Find all markdown files
find . -name "*.md" -not -path "./target/*" -not -path "./.git/*" | while read file; do
    echo "Checking: $file"

    # Check links
    markdown-link-check "$file" \
        --config .markdown-link-check.json \
        || {
            echo "✗ Broken links in $file"
            exit 1
        }
done

echo "✓ All links valid"
```

### Config: `.markdown-link-check.json`

```json
{
  "ignorePatterns": [
    {
      "pattern": "^http://localhost"
    },
    {
      "pattern": "^http://127.0.0.1"
    },
    {
      "pattern": "^https://github.com/SuperInstance/.*"
    }
  ],
  "timeout": "20s",
  "retryOn429": true,
  "retryCount": 2,
  "fallbackRetryDelay": "5s"
}
```

### Script: `check-internal-links.sh`

```bash
#!/bin/bash
# check-internal-links.sh
# Validate internal cross-references between ecosystem tools

set -e

echo "Checking internal cross-references..."

# Extract all tool references
grep -r "github.com/SuperInstance" --include="*.md" . | \
    grep -v "target/" | \
    grep -v ".git/" | \
    while read line; do
        file=$(echo "$line" | cut -d: -f1)
        link=$(echo "$line" | grep -oP 'https://github\.com/SuperInstance/[^)\s]*')

        # Check if link is valid
        if curl -s -o /dev/null -w "%{http_code}" "$link" | grep -q "200\|301\|302"; then
            echo "✓ $file: $link"
        else
            echo "✗ $file: $link (broken)"
            exit 1
        fi
    done

echo "✓ All internal links valid"
```

---

## Ecosystem Documentation Sync

### Script: `sync-ecosystem-docs.sh`

```bash
#!/bin/bash
# sync-ecosystem-docs.sh
# Synchronize ecosystem documentation across all crates

set -e

ECOSYSTEM_FILE="docs/ECOSYSTEM.md"
README_MAIN="README.md"

echo "Syncing ecosystem documentation..."

# Update main README reference
if ! grep -q "docs/ECOSYSTEM.md" "$README_MAIN"; then
    # Add ecosystem link to main README
    sed -i '/## 📖 Documentation/a\\n### 🌍 Ecosystem\n\n- **[Tool Ecosystem](docs/ECOSYSTEM.md)** - Discover tools and integrations' "$README_MAIN"
    echo "Added ecosystem link to main README"
fi

# Extract tools from Cargo.toml
tools=()
for cargo_file in crates/*/Cargo.toml; do
    name=$(grep "^name = " "$cargo_file" | sed 's/name = //; s/"//g')
    tools+=("$name")
done

# Generate markdown list
{
    echo "## Ecosystem Tools"
    echo ""
    for tool in "${tools[@]}"; do
        echo "- **[$tool](crates/$tool/)** - $(grep "^description = " "crates/$tool/Cargo.toml" | sed 's/description = //; s/"//g')"
    done
} > /tmp/ecosystem-list.md

# Update ecosystem file
if grep -q "## Ecosystem Tools" "$ECOSYSTEM_FILE"; then
    # Replace existing section
    sed -i '/## Ecosystem Tools/,/^##/d' "$ECOSYSTEM_FILE"
fi

cat /tmp/ecosystem-list.md | sed -i '/^## 📊 Ecosystem Stats/r /dev/stdin' "$ECOSYSTEM_FILE"

echo "✓ Ecosystem documentation synchronized"
```

---

## Badge Generation

### Script: `generate-badges.sh`

```bash
#!/bin/bash
# generate-badges.sh
# Generate ecosystem badges for READMEs

set -e

REPO="SuperInstance/Tripartite1"
BRANCH=$(git branch --show-current)

echo "Generating badges..."

# Ecosystem badge (SVG)
cat > /tmp/ecosystem-badge.svg << 'EOF'
<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="160" height="20">
  <linearGradient id="b" x2="0" y2="100%">
    <stop offset="0" stop-color="#bbb" stop-opacity=".1"/>
    <stop offset="1" stop-opacity=".1"/>
  </linearGradient>
  <mask id="a">
    <rect width="160" height="20" rx="3" fill="#fff"/>
  </mask>
  <g mask="url(#a)">
    <path fill="#555" d="M0 0h93v20H0z"/>
    <path fill="#007ec6" d="M93 0h67v20H93z"/>
    <path fill="url(#b)" d="M0 0h160v20H0z"/>
  </g>
  <g fill="#fff" text-anchor="middle" font-family="DejaVu Sans,Verdana,Geneva,sans-serif" font-size="11">
    <text x="46.5" y="15" fill="#010101" fill-opacity=".3">SuperInstance</text>
    <text x="46.5" y="14">SuperInstance</text>
    <text x="126.5" y="15" fill="#010101" fill-opacity=".3">Ecosystem</text>
    <text x="126.5" y="14">Ecosystem</text>
  </g>
</svg>
EOF

# Copy to assets directory
mkdir -p assets/badges
cp /tmp/ecosystem-badge.svg assets/badges/

# Generate markdown badge code
cat << 'EOF'

## Ecosystem Badge

Add this badge to your README:

```markdown
[![SuperInstance Ecosystem](https://img.shields.io/badge/SuperInstance-Ecosystem-blue.svg)](https://github.com/SuperInstance/Tripartite1/blob/main/docs/ECOSYSTEM.md)
```

Or use Shields.io:

```markdown
[![SuperInstance Ecosystem](https://img.shields.io/badge/SuperInstance-Ecosystem-0066cc.svg?logo=data:image/svg%2Bxml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAxMDAgMTAwIj48Y2lyY2xlIGN4PSI1MCIgY3k9IjUwIiByPSI0MCIgZmlsbD0iIzAwNjZjYyIvPjwvc3ZnPg==)](https://github.com/SuperInstance/Tripartite1/blob/main/docs/ECOSYSTEM.md)
```

EOF

echo "✓ Badges generated in assets/badges/"
```

---

## Dependency Graph Generation

### Script: `generate-deps-graph.sh`

```bash
#!/bin/bash
# generate-deps-graph.sh
# Generate dependency graph in multiple formats

set -e

OUTPUT_DIR="docs/diagrams"
mkdir -p "$OUTPUT_DIR"

echo "Generating dependency graphs..."

# Install cargo-graph if not present
if ! command -v cargo graph &> /dev/null; then
    echo "Installing cargo-graph..."
    cargo install cargo-graph
fi

# Generate Mermaid graph
cargo graph --package synesis-cli | \
    grep "synesis-" | \
    awk '{
        gsub(/"/, "", $0);
        if (NF == 2) {
            print $1 "[label=\"" $1 "\"]";
            print $2 "[label=\"" $2 "\"]";
            print $1 " -> " $2 ";";
        }
    }' > "$OUTPUT_DIR/deps.mermaid"

echo "graph TD;" > "$OUTPUT_DIR/ecosystem-graph.mermaid"

# Add nodes
for cargo_file in crates/*/Cargo.toml; do
    name=$(grep "^name = " "$cargo_file" | sed 's/name = //; s/"//g')
    desc=$(grep "^description = " "$cargo_file" | sed 's/description = //; s/"//g')
    echo "  ${name}[\"${name}<br/>${desc}\"]" >> "$OUTPUT_DIR/ecosystem-graph.mermaid"
done

# Add edges
for cargo_file in crates/*/Cargo.toml; do
    name=$(grep "^name = " "$cargo_file" | sed 's/name = //; s/"//g')
    grep "synesis-" "$cargo_file" | grep "^synesis-" | sed 's/^synesis-/  /; s/ = .*//' | \
        while read dep; do
            if [ -f "crates/$dep/Cargo.toml" ]; then
                echo "  ${name} --> ${dep}" >> "$OUTPUT_DIR/ecosystem-graph.mermaid"
            fi
        done
done

echo "}" >> "$OUTPUT_DIR/ecosystem-graph.mermaid"

echo "✓ Dependency graphs generated in $OUTPUT_DIR/"
```

---

## CI/CD Integration

### Workflow: `ecosystem-check.yml`

```yaml
name: Ecosystem Checks

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]
  schedule:
    # Run daily at 2 AM UTC
    - cron: '0 2 * * *'

jobs:
  validate-ecosystem:
    name: Validate Ecosystem
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install dependencies
        run: |
          npm install -g markdown-link-check
          cargo install cargo-graph

      - name: Check cross-references
        run: ./scripts/ecosystem/validate-ecosystem.sh

      - name: Check all links
        run: ./scripts/ecosystem/check-links.sh

      - name: Generate dependency graph
        run: ./scripts/ecosystem/generate-deps-graph.sh

      - name: Check for broken internal links
        run: ./scripts/ecosystem/check-internal-links.sh

  sync-topics:
    name: Sync GitHub Topics
    runs-on: ubuntu-latest
    if: github.event_name == 'push' && github.ref == 'refs/heads/main'
    steps:
      - uses: actions/checkout@v4

      - name: Sync topics
        run: ./scripts/ecosystem/update-topics.sh
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

### Workflow: `link-check.yml`

```yaml
name: Link Check

on:
  pull_request:
    branches: [main]
  workflow_dispatch:

jobs:
  markdown-link-check:
    name: Markdown Link Check
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Link Checker
        uses: gaurav-nelson/github-action-markdown-link-check@v1
        with:
          use-quiet-mode: 'yes'
          use-verbose-mode: 'yes'
          config-file: '.markdown-link-check.json'
          folder-path: 'docs'
          file-path: '.'
          check-modified-files-only: 'no'
```

---

## Templates

### Template: `readme-ecosystem-section.md`

```markdown
## 🌍 Ecosystem

### Used By

- **[SuperInstance AI](https://github.com/SuperInstance/Tripartite1)** - Main project using {{ TOOL_NAME }}

### Requires

{% for dep in dependencies %}
- **[{{ dep.name }}]({{ dep.url }})** - {{ dep.reason }}
{% endfor %}

### Complementary Tools

- **[synesis-core](https://github.com/SuperInstance/Tripartite1)** - Agent orchestration
- **[synesis-privacy](https://github.com/SuperInstance/Tripartite1)** - Privacy proxy
- **[synesis-knowledge](https://github.com/SuperInstance/Tripartite1)** - Knowledge vault
- **[synesis-cloud](https://github.com/SuperInstance/Tripartite1)** - Cloud connectivity

### See Also

- **[llama.cpp](https://github.com/ggerganov/llama.cpp)** - Local LLM inference
- **[SQLite-VSS](https://github.com/asg017/sqlite-vss)** - Vector search extension

📖 **[View Full Ecosystem](https://github.com/SuperInstance/Tripartite1/blob/main/docs/ECOSYSTEM.md)**
```

### Template: `ecosystem-entry.md`

```markdown
## {{ TOOL_NAME }}

{{ TOOL_DESCRIPTION }}

### Repository

- **URL**: {{ REPO_URL }}
- **Language**: {{ LANGUAGE }}
- **License**: {{ LICENSE }}
- **Status**: {{ STATUS }}

### Used By

- **[SuperInstance AI](https://github.com/SuperInstance/Tripartite1)** - Main project

### Requires

{{ DEPENDENCIES_LIST }}

### Complementary Tools

{{ COMPLEMENTARY_TOOLS_LIST }}

### Integration

```rust
use {{ TOOL_NAME }}::*;

// Example usage
```

### Documentation

- [API Reference](https://docs.rs/{{ TOOL_NAME }})
- [Examples](https://github.com/SuperInstance/Tripartite1/tree/main/examples)

### See Also

{{ SEE_ALSO_LIST }}
```

---

## Quick Reference

### Common Commands

```bash
# Validate ecosystem
./scripts/ecosystem/validate-ecosystem.sh

# Check all links
./scripts/ecosystem/check-links.sh

# Generate cross-references
./scripts/ecosystem/generate-cross-refs.sh

# Sync documentation
./scripts/ecosystem/sync-ecosystem-docs.sh

# Generate badges
./scripts/ecosystem/generate-badges.sh

# Update topics
./scripts/ecosystem/update-topics.sh

# Generate dependency graphs
./scripts/ecosystem/generate-deps-graph.sh
```

### File Locations

```
scripts/ecosystem/          # Automation scripts
docs/ECOSYSTEM.md           # Main ecosystem documentation
docs/diagrams/              # Generated graphs
assets/badges/              # Generated badges
```

---

## Troubleshooting

### Script Failures

**Problem**: `generate-cross-refs.sh` fails with "Cargo.toml not found"

**Solution**: Ensure you're running from repository root

**Problem**: `check-links.sh` reports false positives

**Solution**: Add patterns to `.markdown-link-check.json`

**Problem**: Dependency graph missing edges

**Solution**: Ensure all workspace crates are in `Cargo.toml` `[members]`

### CI/CD Issues

**Problem**: Ecosystem check fails in CI

**Solution**: Check that scripts have execute permissions:
```bash
chmod +x scripts/ecosystem/*.sh
```

**Problem**: Link check times out

**Solution**: Increase timeout in `.markdown-link-check.json`

---

## Contributing

### Adding New Scripts

1. Place in `scripts/ecosystem/`
2. Make executable: `chmod +x script-name.sh`
3. Add to this documentation
4. Test locally
5. Update CI/CD workflows if needed

### Script Standards

- **Shebang**: `#!/bin/bash` or `#!/usr/bin/env bash`
- **Error handling**: `set -e` at start
- **Logging**: Echo progress messages
- **Exit codes**: Use appropriate exit codes
- **Comments**: Document complex logic

---

## Related Documentation

- **[Ecosystem Documentation](../ECOSYSTEM.md)**
- **[README Standards](README_STANDARDS.md)**
- **[GitHub Topics Strategy](GITHUB_TOPICS_STRATEGY.md)**
- **[Implementation Guide](ECOSYSTEM_GUIDE.md)**

---

**Automation Version**: 1.0.0
**Last Updated**: 2026-01-08
**Maintained By**: [SuperInstance AI](https://github.com/SuperInstance)
