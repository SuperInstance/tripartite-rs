# Ecosystem Automation Scripts

This directory contains automation scripts for maintaining the SuperInstance ecosystem documentation and cross-references.

## Available Scripts

### `validate-ecosystem.sh`

Validates that all crates have proper ecosystem cross-references in their READMEs.

**Usage**:
```bash
./scripts/ecosystem/validate-ecosystem.sh
```

**Checks**:
- ✓ Ecosystem section exists
- ✓ Required subsections present (Used By, Requires, Complementary Tools, See Also)
- ✓ Ecosystem badge present
- ✓ Link to ECOSYSTEM.md present

### `check-links.sh`

Validates all external links in markdown documentation.

**Usage**:
```bash
./scripts/ecosystem/check-links.sh
```

**Requirements**:
```bash
npm install -g markdown-link-check
```

**What it checks**:
- All markdown files in repository
- External HTTP/HTTPS links
- Reports broken links

### `generate-badges.sh`

Generates ecosystem badge documentation and insertion scripts.

**Usage**:
```bash
./scripts/ecosystem/generate-badges.sh
```

**Output**:
- `assets/badges/ecosystem-badge.md` - Badge documentation
- `assets/badges/insert-badge.sh` - Script to insert badge into README

## Quick Start

```bash
# Validate all cross-references
./scripts/ecosystem/validate-ecosystem.sh

# Check all links
./scripts/ecosystem/check-links.sh

# Generate badges
./scripts/ecosystem/generate-badges.sh
```

## Adding New Scripts

1. Create script in this directory
2. Make executable: `chmod +x script-name.sh`
3. Add shebang: `#!/bin/bash`
4. Add error handling: `set -e`
5. Document in this README

## Script Standards

- **Shebang**: `#!/bin/bash` or `#!/usr/bin/env bash`
- **Error handling**: `set -e` at start
- **Logging**: Use echo with status messages
- **Colors**: Use ANSI colors for output (green = success, red = error, yellow = warning)
- **Exit codes**: 0 = success, 1 = failure
- **Help**: Each script should show usage when called with `--help`

## Troubleshooting

### Permission Denied

If you get "Permission denied" error:
```bash
chmod +x scripts/ecosystem/*.sh
```

### Script Not Found

Make sure you're running from repository root:
```bash
cd /path/to/Tripartite1
./scripts/ecosystem/validate-ecosystem.sh
```

### Missing Dependencies

Install required tools:
```bash
# For link checking
npm install -g markdown-link-check

# For dependency graphs
cargo install cargo-graph

# For GitHub operations
# Install GitHub CLI from https://cli.github.com/
```

## Related Documentation

- **[ECOSYSTEM_AUTOMATION.md](../../docs/architecture/ECOSYSTEM_AUTOMATION.md)** - Full automation documentation
- **[ECOSYSTEM_GUIDE.md](../../docs/architecture/ECOSYSTEM_GUIDE.md)** - Implementation guide
- **[README_STANDARDS.md](../../docs/architecture/README_STANDARDS.md)** - README format standards

---

**Last Updated**: 2026-01-08
**Maintained By**: [SuperInstance AI](https://github.com/SuperInstance)
