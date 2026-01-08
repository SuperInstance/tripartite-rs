#!/bin/bash
# check-crate-availability.sh
# Script to check availability of recommended crate names on crates.io

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Recommended names from naming strategy
NAMES=(
    # Top Priority
    "privox"
    "privox-redact"
    "tripartite-rs"
    "tripartite"
    "knowledgr"
    "quicunnel"
    "usemeter"

    # Secondary Priority
    "hwscan-rs"
    "token-keeper"
    "model-registry"
    "model-keep"

    # Alternative options
    "pii-guardian"
    "sanctum-rs"
    "consensus-council"
    "rag-vault"
    "hardware-probe"
    "synesis-redact"
    "synesis-vault"
    "synesis-consensus"
    "synesis-mem"
    "synesis-hw"
    "synesis-tunnel"
    "synesis-billing"
)

echo "========================================"
echo "  Crate Availability Check"
echo "  SuperInstance AI - Naming Strategy"
echo "========================================"
echo ""
echo "Checking availability of recommended names..."
echo ""

# Create results array
declare -a AVAILABLE=()
declare -a TAKEN=()
declare -a ERROR=()

for name in "${NAMES[@]}"; do
    # Check crates.io API
    response=$(curl -s -o /dev/null -w "%{http_code}" "https://crates.io/api/v1/crates/$name")

    if [ "$response" = "404" ]; then
        echo -e "${GREEN}✅ $name${NC} - Available"
        AVAILABLE+=("$name")
    elif [ "$response" = "200" ]; then
        echo -e "${RED}❌ $name${NC} - Taken"
        TAKEN+=("$name")
    else
        echo -e "${YELLOW}⚠️  $name${NC} - Error (HTTP $response)"
        ERROR+=("$name")
    fi

    # Be nice to the API
    sleep 0.5
done

echo ""
echo "========================================"
echo "  Summary"
echo "========================================"
echo ""
echo -e "${GREEN}Available: ${#AVAILABLE[@]}${NC}"
for name in "${AVAILABLE[@]}"; do
    echo "  ✅ $name"
done

echo ""
echo -e "${RED}Taken: ${#TAKEN[@]}${NC}"
for name in "${TAKEN[@]}"; do
    echo "  ❌ $name"
done

if [ ${#ERROR[@]} -gt 0 ]; then
    echo ""
    echo -e "${YELLOW}Errors: ${#ERROR[@]}${NC}"
    for name in "${ERROR[@]}"; do
        echo "  ⚠️  $name"
    done
fi

echo ""
echo "========================================"
echo "  Next Steps"
echo "========================================"
echo ""
echo "1. Reserve available names immediately"
echo "2. Create placeholder crates (version 0.0.1)"
echo "3. Set up GitHub repositories"
echo "4. Consider domain purchases for top brands"
echo ""
echo "To create a placeholder crate:"
echo "  cargo new --lib $name"
echo "  cd $name"
echo "  # Edit Cargo.toml with proper metadata"
echo "  cargo publish"
echo ""
