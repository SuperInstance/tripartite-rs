# GitHub Topics Strategy - SuperInstance Ecosystem

> **Consistent tagging strategy for discoverability**

**Version**: 1.0.0
**Last Updated**: 2026-01-08

---

## Purpose

This document defines a consistent set of GitHub topics/tags to use across all SuperInstance repositories. Consistent tagging helps users discover tools, understand relationships, and find what they need.

---

## Mandatory Topics

### All Repositories Must Include

Every repository in the SuperInstance ecosystem MUST tag these topics:

```
superinstance
superinstance-ecosystem
rust
privacy-first
local-first
```

**Why**: These ensure users can find all ecosystem tools by searching for `superinstance` or related concepts.

---

## Ecosystem Topics

### Core Functionality

Use these to identify the tool's role:

| Topic | When to Use | Example Repos |
|-------|-------------|---------------|
| **tripartite-consensus** | Multi-agent consensus system | synesis-core |
| **privacy-proxy** | Data redaction/tokenization | synesis-privacy |
| **vector-database** | Vector search/storage | synesis-knowledge |
| **rag-engine** | Retrieval-augmented generation | synesis-knowledge |
| **hardware-detection** | System resource detection | synesis-models |
| **quic-tunnel** | QUIC networking | synesis-cloud |
| **ai-agents** | AI agent systems | synesis-core |
| **llm** | Large language model integration | synesis-models |

### Technology Topics

Tag the technologies you use:

| Topic | When to Use |
|-------|-------------|
| **rust-lang** | Rust projects |
| **typescript** | TypeScript projects |
| **async** | Async/await code |
| **tokio** | Uses Tokio runtime |
| **sqlite** | Uses SQLite |
| **cloudflare** | Cloudflare Workers |

---

## Functional Category Topics

### Privacy & Security

```
privacy
data-redaction
tokenization
pII-protection
mTLS
encryption
```

### AI & Machine Learning

```
artificial-intelligence
machine-learning
llm
embeddings
semantic-search
retrieval-augmented-generation
agents
multi-agent-system
consensus
```

### Database & Storage

```
vector-database
sqlite
sqlite-vss
knowledge-base
local-storage
```

### Networking

```
quic
networking
tunnel
cloud-connectivity
edge-compute
```

### Developer Tools

```
cli
developer-tools
productivity
automation
```

---

## Language-Specific Topics

### Rust

All Rust projects should include:

```
rust
rust-lang
rust-crate
cargo
```

Additional Rust topics (as applicable):

```
async-rust
tokio
actix
serde
wasm
```

### TypeScript

All TypeScript projects should include:

```
typescript
javascript
nodejs
deno
```

Additional TypeScript topics (as applicable):

```
cloudflare-workers
serverless
edge-compute
```

---

## Use Case Topics

Help users find tools by what they want to do:

```
chatbot
code-search
semantic-search
local-ai
privacy-preservation
offline-first
documentation
code-review
assistant
```

---

## Project Status Topics

Indicate the project's maturity:

| Topic | When to Use | Meaning |
|-------|-------------|---------|
| **production-ready** | Stable, tested, in use | Ready for production |
| **beta** | Feature complete, testing | Mostly stable, minor changes expected |
| **alpha** | Early development | Experimental, breaking changes likely |
| **experimental** | Research/prototype | May be abandoned |
| **stable** | API stable | No breaking changes planned |
| **actively-developed** | Active commits | Regular updates |
| **maintenance-mode** | Bug fixes only | No new features planned |
| **deprecated** | No longer maintained | Use alternative |

---

## Topic Assignment by Tool

### synesis-core

```
superinstance
superinstance-ecosystem
rust
privacy-first
local-first
tripartite-consensus
ai-agents
multi-agent-system
consensus
llm
artificial-intelligence
async
tokio
production-ready
stable
actively-developed
```

### synesis-privacy

```
superinstance
superinstance-ecosystem
rust
privacy-first
local-first
privacy-proxy
data-redaction
tokenization
pII-protection
privacy
regex
sqlite
async
tokio
production-ready
stable
actively-developed
```

### synesis-knowledge

```
superinstance
superinstance-ecosystem
rust
privacy-first
local-first
vector-database
rag-engine
retrieval-augmented-generation
semantic-search
embeddings
knowledge-base
sqlite
sqlite-vss
async
tokio
production-ready
stable
actively-developed
```

### synesis-models

```
superinstance
superinstance-ecosystem
rust
privacy-first
local-first
hardware-detection
llm
model-management
gpu
cuda
production-ready
stable
actively-developed
```

### synesis-cloud

```
superinstance
superinstance-ecosystem
rust
privacy-first
local-first
quic-tunnel
cloud-connectivity
networking
quic
mtls
edge-compute
cloudflare
async
tokio
beta
actively-developed
```

### synesis-cli

```
superinstance
superinstance-ecosystem
rust
privacy-first
local-first
cli
command-line
artificial-intelligence
chatbot
assistant
production-ready
stable
actively-developed
```

---

## Adding Topics to Your Repository

### Via GitHub Web UI

1. Go to your repository
2. Click **Settings** → **Topics**
3. Add topics from the lists above

### Via GitHub CLI

```bash
# Install GitHub CLI
# https://cli.github.com/

# Add topics to a repository
gh repo edit \
  --repo SuperInstance/Tripartite1 \
  --add-topic "superinstance" \
  --add-topic "privacy-first" \
  --add-topic "rust"
```

### Via Git

Add to `.github/repository.yml` (if you use this pattern):

```yaml
name: Tripartite1
description: SuperInstance AI - Privacy-first, local-first AI with tripartite consensus
topics:
  - superinstance
  - superinstance-ecosystem
  - rust
  - privacy-first
  - local-first
  - tripartite-consensus
  - ai-agents
  - production-ready
```

---

## Topic Limits & Best Practices

### GitHub Limits

- **Maximum topics**: 30 per repository
- **Topic length**: 35 characters max
- **Recommendation**: Use 10-15 relevant topics

### Prioritization Strategy

When you're near the limit, prioritize:

1. **Mandatory topics** (always include)
2. **Core functionality** (what the tool does)
3. **Technology stack** (how it's built)
4. **Use cases** (what it enables)

### Avoid

- ❌ Overly generic topics (`software`, `code`, `app`)
- ❌ Duplicate meanings (`rust` and `rust-lang` - pick one)
- ❌ Version-specific topics (`v2`, `v3`)
- ❌ Temporary states (`wip`, `todo` - use issues instead)

---

## Discoverability Strategy

### Topic Combinations

Users often search for topic combinations. Our tools should appear in:

| Search | Topics That Match |
|--------|-------------------|
| "rust privacy" | `rust` + `privacy-first` |
| "local ai" | `local-first` + `artificial-intelligence` |
| "vector database rust" | `rust` + `vector-database` |
| "rag engine" | `rag-engine` + `rust` |
| "privacy proxy" | `privacy-proxy` + `rust` |

### Cross-Linking

Topics work with README cross-references:

```
User searches: "rust vector database"
       ↓
Finds: synesis-knowledge (via topics)
       ↓
Clicks: README
       ↓
Sees: "Ecosystem" section
       ↓
Discovers: synesis-privacy, synesis-core (via cross-refs)
```

---

## Monitoring & Updates

### Review Topics Quarterly

Check if topics are still relevant:
- [ ] Remove deprecated technology topics
- [ ] Add new feature topics (e.g., added GPU support? Add `cuda`)
- [ ] Update status (beta → stable when ready)
- [ ] Check for broken topic combinations

### Analyze Discovery

Use GitHub Insights to see which topics drive traffic:
```bash
# See traffic sources
gh repo view --web traffic

# Look for topic-driven discovery in referral URLs
```

---

## Validation Tools

### Check Topic Compliance

```bash
#!/bin/bash
# check-topics.sh - Verify repository has required topics

REPO="SuperInstance/Tripartite1"
REQUIRED_TOPICS=("superinstance" "superinstance-ecosystem" "rust" "privacy-first" "local-first")

echo "Checking required topics for $REPO..."

for topic in "${REQUIRED_TOPICS[@]}"; do
  if gh api /repos/$REPO/topics --jq '.names | contains(["'$topic'"])'; then
    echo "✓ Has topic: $topic"
  else
    echo "✗ Missing topic: $topic"
  fi
done
```

### List All Topics

```bash
# List all topics on a repository
gh api /repos/SuperInstance/Tripartite1/topics --jq '.names[]'
```

---

## Topic Templates

### For New Rust Libraries

```bash
#!/bin/bash
# topics-template.sh

# Mandatory
TOPICS="superinstance,superinstance-ecosystem,rust,privacy-first,local-first"

# Add your specific topics
TOPICS="$TOPICS,async,tokio,production-ready,stable,actively-developed"

echo "Suggested topics: $TOPICS"
echo "Count: $(echo $TOPICS | tr ',' '\n' | wc -l)"
```

### For New TypeScript Projects

```bash
#!/bin/bash
# topics-template-ts.sh

TOPICS="superinstance,superinstance-ecosystem,typescript,cloudflare,edge-compute"
TOPICS="$TOPICS,serverless,production-ready,stable,actively-developed"

echo "Suggested topics: $TOPICS"
echo "Count: $(echo $TOPICS | tr ',' '\n' | wc -l)"
```

---

## Examples: Good vs Bad

### Good Topic Set

```
superinstance, rust, privacy-first, local-first, vector-database,
rag-engine, semantic-search, sqlite, async, tokio,
production-ready, stable, actively-developed
```

✅ Specific, descriptive, discoverable

### Bad Topic Set

```
code, software, app, tool, library, rust, programming,
development, github, open-source, project, system, platform
```

❌ Generic, meaningless, not discoverable

---

## Reference

### Official Documentation

- **[GitHub Topics](https://docs.github.com/en/articles/about-topics)**
- **[Best Practices](https://docs.github.com/en/articles/classifying-your-repository-with-topics)**

### Related SuperInstance Docs

- **[Ecosystem Documentation](../ECOSYSTEM.md)**
- **[README Standards](README_STANDARDS.md)**
- **[Cross-Reference Guide](ECOSYSTEM_AUTOMATION.md)**

---

## Changelog

### 2026-01-08
- Initial topic strategy defined
- Created topic assignment for all 6 core crates
- Added validation scripts

---

**Standard Version**: 1.0.0
**Last Updated**: 2026-01-08
**Maintained By**: [SuperInstance AI](https://github.com/SuperInstance)
