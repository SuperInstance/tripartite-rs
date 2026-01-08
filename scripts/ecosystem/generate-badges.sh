#!/bin/bash
# generate-badges.sh
# Generate ecosystem badges for READMEs

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
ASSETS_DIR="$REPO_ROOT/assets/badges"

echo "==================================="
echo "Ecosystem Badge Generator"
echo "==================================="
echo ""

# Create assets directory
mkdir -p "$ASSETS_DIR"

# Generate badge markdown
cat > "$ASSETS_DIR/ecosystem-badge.md" << 'EOF'
# SuperInstance Ecosystem Badge

Add this badge to your README to show your tool is part of the SuperInstance ecosystem:

## Markdown (Recommended)

```markdown
[![SuperInstance Ecosystem](https://img.shields.io/badge/SuperInstance-Ecosystem-blue.svg)](https://github.com/SuperInstance/Tripartite1/blob/main/docs/ECOSYSTEM.md)
```

## HTML

```html
<a href="https://github.com/SuperInstance/Tripartite1/blob/main/docs/ECOSYSTEM.md">
  <img src="https://img.shields.io/badge/SuperInstance-Ecosystem-blue.svg" alt="SuperInstance Ecosystem">
</a>
```

## Preview

[![SuperInstance Ecosystem](https://img.shields.io/badge/SuperInstance-Ecosystem-blue.svg)](https://github.com/SuperInstance/Tripartite1/blob/main/docs/ECOSYSTEM.md)
EOF

echo "Generated badge documentation: $ASSETS_DIR/ecosystem-badge.md"

# Generate badge insertion script
cat > "$ASSETS_DIR/insert-badge.sh" << 'EOF'
#!/bin/bash
# insert-badge.sh - Insert ecosystem badge into README

if [ -z "$1" ]; then
    echo "Usage: $0 <readme-file>"
    exit 1
fi

README="$1"
BADGE_LINE='[![SuperInstance Ecosystem](https://img.shields.io/badge/SuperInstance-Ecosystem-blue.svg)](https://github.com/SuperInstance/Tripartite1/blob/main/docs/ECOSYSTEM.md)'

# Check if badge already exists
if grep -q "SuperInstance-Ecosystem" "$README"; then
    echo "Badge already exists in $README"
    exit 0
fi

# Find badges section (usually near top)
if grep -q "^\[!" "$README"; then
    # Insert after first badge
    sed -i "0,/^\\\[!/{s//&\n$BADGE_LINE/}" "$README"
else
    # Insert after title
    sed -i "1,/^# /{s//&\n\n$BADGE_LINE/}" "$README"
fi

echo "Inserted ecosystem badge into $README"
EOF

chmod +x "$ASSETS_DIR/insert-badge.sh"

echo "Generated badge insertion script: $ASSETS_DIR/insert-badge.sh"

# List all badges
echo ""
echo "==================================="
echo "Available Badges"
echo "==================================="
echo ""
echo "1. SuperInstance Ecosystem Badge"
echo "   Purpose: Show tool is part of the ecosystem"
echo "   Markdown: [![SuperInstance Ecosystem](https://img.shields.io/badge/SuperInstance-Ecosystem-blue.svg)](https://github.com/SuperInstance/Tripartite1/blob/main/docs/ECOSYSTEM.md)"
echo ""
echo "2. Standard Badges (use as appropriate):"
echo "   - Build Status: [![CI](https://github.com/SuperInstance/Tripartite1/actions/workflows/ci.yml/badge.svg)](https://github.com/SuperInstance/Tripartite1/actions/workflows/ci.yml)"
echo "   - License: [![License](https://img.shields.io/badge/license-MIT%20%7C%20Apache--2.0-blue.svg)](LICENSE-APACHE)"
echo "   - Rust: [![Rust Version](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)"
echo "   - Tests: [![Tests](https://img.shields.io/badge/tests-passing-brightgreen.svg)](https://github.com/SuperInstance/Tripartite1)"
echo ""

echo "✅ Badge generation complete!"
echo ""
echo "To insert badge into a README:"
echo "  $ASSETS_DIR/insert-badge.sh <readme-file>"
