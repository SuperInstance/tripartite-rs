#!/bin/bash
# check-links.sh
# Validate all external links in markdown files

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

echo "==================================="
echo "Markdown Link Validator"
echo "==================================="
echo ""

# Check if markdown-link-check is installed
if ! command -v markdown-link-check &> /dev/null; then
    echo "markdown-link-check not found."
    echo "Install with: npm install -g markdown-link-check"
    exit 1
fi

# Create config file if it doesn't exist
CONFIG_FILE="$REPO_ROOT/.markdown-link-check.json"
if [ ! -f "$CONFIG_FILE" ]; then
    cat > "$CONFIG_FILE" << 'EOF'
{
  "ignorePatterns": [
    {
      "pattern": "^http://localhost"
    },
    {
      "pattern": "^http://127.0.0.1"
    }
  ],
  "timeout": "20s",
  "retryOn429": true,
  "retryCount": 2,
  "fallbackRetryDelay": "5s"
}
EOF
    echo "Created config file: $CONFIG_FILE"
fi

# Find and check all markdown files (excluding target and .git)
echo "Scanning for markdown files..."
mapfile -t md_files < <(find "$REPO_ROOT" -name "*.md" \
    -not -path "*/target/*" \
    -not -path "*/.git/*" \
    -not -path "*/node_modules/*")

echo "Found ${#md_files[@]} markdown files"
echo ""

FAILED=0
PASSED=0

for file in "${md_files[@]}"; do
    echo "Checking: $(basename "$file")"

    # Run markdown-link-check
    if markdown-link-check "$file" --config "$CONFIG_FILE" > /dev/null 2>&1; then
        echo -e "  ✓ Passed"
        ((PASSED++))
    else
        echo -e "  ✗ Failed"
        ((FAILED++))

        # Run again to show errors
        markdown-link-check "$file" --config "$CONFIG_FILE" || true
    fi
done

echo ""
echo "==================================="
echo "Link Check Results"
echo "==================================="
echo "Passed: $PASSED"
echo "Failed: $FAILED"
echo ""

if [ $FAILED -gt 0 ]; then
    echo -e "❌ $FAILED file(s) have broken links"
    exit 1
else
    echo -e "✅ All links validated!"
    exit 0
fi
