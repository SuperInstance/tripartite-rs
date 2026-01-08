#!/bin/bash
# validate-ecosystem.sh
# Validate ecosystem cross-references across all crates

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

echo "==================================="
echo "Ecosystem Cross-Reference Validator"
echo "==================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

ERRORS=0
WARNINGS=0

# Function to check if a crate has an ecosystem section
check_ecosystem_section() {
    local readme="$1"
    local crate_name="$2"

    if [ ! -f "$readme" ]; then
        echo -e "${YELLOW}⚠ No README found for $crate_name${NC}"
        ((WARNINGS++))
        return 1
    fi

    if ! grep -q "## 🌍 Ecosystem" "$readme"; then
        echo -e "${RED}✗ Missing ecosystem section in $crate_name${NC}"
        ((ERRORS++))
        return 1
    fi

    # Check for required subsections
    local required_sections=("Used By" "Requires" "Complementary Tools" "See Also")

    for section in "${required_sections[@]}"; do
        if ! grep -q "### $section" "$readme"; then
            echo -e "${RED}✗ Missing '$section' in $crate_name${NC}"
            ((ERRORS++))
        fi
    done

    # Check for ecosystem badge
    if ! grep -q "SuperInstance-Ecosystem" "$readme"; then
        echo -e "${YELLOW}⚠ Missing ecosystem badge in $crate_name${NC}"
        ((WARNINGS++))
    fi

    # Check for link to full ecosystem docs
    if ! grep -q "ECOSYSTEM.md" "$readme"; then
        echo -e "${RED}✗ Missing link to ECOSYSTEM.md in $crate_name${NC}"
        ((ERRORS++))
    fi

    echo -e "${GREEN}✓ $crate_name ecosystem section validated${NC}"
    return 0
}

# Function to check if links in a file are valid
check_links() {
    local file="$1"
    local crate_name="$2"

    # Extract markdown links
    grep -oP '\[([^\]]+)\]\(([^)]+)\)' "$file" | while read -r match; do
        url=$(echo "$match" | grep -oP '(?<=\()\S+(?=\))')

        # Skip relative links and anchors
        if [[ "$url" =~ ^\.\. ]] || [[ "$url" =~ ^# ]]; then
            continue
        fi

        # Skip GitHub links that might not exist yet (in development)
        if [[ "$url" =~ github\.com ]] && [[ "$url" =~ SuperInstance ]]; then
            continue
        fi

        # Check if URL is accessible
        if command -v curl &> /dev/null; then
            if ! curl -s -f -o /dev/null -L --max-time 5 "$url" 2>/dev/null; then
                echo -e "${YELLOW}⚠ Possible broken link in $crate_name: $url${NC}"
                ((WARNINGS++))
            fi
        fi
    done
}

echo "Checking all crates..."
echo ""

# Find all crate directories
for cargo_toml in "$REPO_ROOT"/crates/*/Cargo.toml; do
    [ -f "$cargo_toml" ] || continue

    crate_dir=$(dirname "$cargo_toml")
    crate_name=$(basename "$crate_dir")
    readme="$crate_dir/README.md"

    echo "Checking $crate_name..."

    check_ecosystem_section "$readme" "$crate_name"

    if [ -f "$readme" ]; then
        check_links "$readme" "$crate_name"
    fi

    echo ""
done

# Check main README
echo "Checking main README..."
main_readme="$REPO_ROOT/README.md"

if grep -q "docs/ECOSYSTEM.md" "$main_readme"; then
    echo -e "${GREEN}✓ Main README links to ecosystem docs${NC}"
else
    echo -e "${YELLOW}⚠ Main README should link to ECOSYSTEM.md${NC}"
    ((WARNINGS++))
fi

echo ""
echo "==================================="
echo "Validation Complete"
echo "==================================="
echo -e "${GREEN}Validated${NC}"
echo -e "${YELLOW}Warnings: $WARNINGS${NC}"
echo -e "${RED}Errors: $ERRORS${NC}"

if [ $ERRORS -gt 0 ]; then
    echo ""
    echo -e "${RED}❌ Validation failed with $ERRORS error(s)${NC}"
    exit 1
else
    echo ""
    echo -e "${GREEN}✅ All ecosystem cross-references validated!${NC}"
    exit 0
fi
