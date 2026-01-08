# Tool Extraction & Publishing Guide

**Step-by-step guide for extracting and publishing independent tools**

---

## Overview

This guide walks through the process of extracting each of the 8 independent tools from the SuperInstance codebase and publishing them as standalone crates.

**Prerequisites**:
- Read [INDEPENDENT_TOOLS_NAMING_STRATEGY.md](./INDEPENDENT_TOOLS_NAMING_STRATEGY.md)
- Read [NAMING_QUICK_REFERENCE.md](./NAMING_QUICK_REFERENCE.md)
- Verify name availability using `check-crate-availability.sh`

---

## Table of Contents

1. [Phase 1: Preparation](#phase-1-preparation)
2. [Phase 2: Code Extraction](#phase-2-code-extraction)
3. [Phase 3: Publishing](#phase-3-publishing)
4. [Phase 4: Post-Publishing](#phase-4-post-publishing)
5. [Tool-Specific Guides](#tool-specific-guides)

---

## Phase 1: Preparation

### Step 1.1: Name Reservation

**Priority Order** (based on competitive analysis):

```bash
# Run availability check
./check-crate-availability.sh

# Reserve top 3 names immediately
cargo new --lib tripartite-rs
cargo new --lib privox
cargo new --lib quicunnel

# Create placeholder crates (version 0.0.1)
cd tripartite-rs
# Edit Cargo.toml with proper metadata
cargo publish
```

### Step 1.2: Repository Setup

For each tool, create a GitHub repository:

```bash
# GitHub organization: superinstance

# Repository naming:
superinstance/tripartite-rs
superinstance/privox
superinstance/quicunnel
superinstance/knowledgr
superinstance/hwscan-rs
superinstance/usemeter
superinstance/model-registry
superinstance/token-keeper
```

**Repository Structure**:
```
tripartite-rs/
├── Cargo.toml              # Metadata
├── README.md               # User-facing docs
├── LICENSE                 # MIT OR Apache-2.0
├── CONTRIBUTING.md         # Contribution guide
├── src/
│   ├── lib.rs              # Public API
│   ├── consensus.rs        # Core functionality
│   └── ...
├── examples/               # Usage examples
│   └── basic.rs
├── tests/                  # Integration tests
│   └── integration_test.rs
├── benches/                # Benchmarks
│   └── consensus_bench.rs
└── docs/                   # Additional docs
    └── architecture.md
```

### Step 1.3: Documentation Setup

Create documentation sites:

```bash
# For each tool
cd tripartite-rs
mkdir docs
echo "# tripartite-rs Documentation" > docs/index.md

# Set up GitHub Pages
# Or use docs.rs for API docs
```

---

## Phase 2: Code Extraction

### General Extraction Pattern

For each tool, follow this pattern:

#### Step 2.1: Identify Dependencies

```bash
# In the monorepo
grep -r "use synesis" crates/synesis-privacy/src/
grep -r "use synesis_core" crates/synesis-privacy/src/

# Identify internal dependencies
# Plan how to handle them (copy, refactor, or create dependency)
```

#### Step 2.2: Extract Core Functionality

```rust
// Original: crates/synesis-privacy/src/redact.rs

// Extracted: privox/src/lib.rs
pub use redaction::{RedactConfig, RedactionEngine};

pub mod redaction {
    // ... extracted code
}
```

#### Step 2.3: Remove SuperInstance-Specific Code

```rust
// REMOVE:
// - References to tripartite council
// - SuperInstance config structures
// - Integration-specific code

// KEEP:
// - Core algorithms
// - Public APIs
// - Utility functions
```

#### Step 2.4: Add Standalone Examples

```rust
// examples/basic_redaction.rs
use privox::{RedactConfig, RedactionEngine};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = RedactConfig::default();
    let engine = RedactionEngine::new(config);

    let text = "My email is john@example.com";
    let redacted = engine.redact(text)?;

    println!("Original: {}", text);
    println!("Redacted: {}", redacted);

    Ok(())
}
```

---

## Phase 3: Publishing

### Step 3.1: Prepare Cargo.toml

```toml
# Example: privox/Cargo.toml

[package]
name = "privox"
version = "0.1.0"
edition = "2021"
authors = ["SuperInstance AI <dev@superinstance.ai>"]
license = "MIT OR Apache-2.0"
repository = "https://github.com/superinstance/privox"
documentation = "https://docs.rs/privox"
homepage = "https://privox.dev"  # Optional
keywords = ["privacy", "pii", "redaction", "security"]
categories = ["development-tools", "parser-tools"]
description = """
Privacy-first PII redaction for AI applications
"""
readme = "README.md"

[dependencies]
# Minimal dependencies for standalone use
regex = "1"
thiserror = "1"
serde = { version = "1", features = ["derive"], optional = true }

[dev-dependencies]
criterion = "0.5"

[features]
default = []
serde = ["dep:serde"]  # Optional serialization

[[bench]]
name = "redaction_bench"
harness = false
```

### Step 3.2: Write Comprehensive README.md

```markdown
# privox

[![Crates.io](https://img.shields.io/crates/v/privox)](https://crates.io/crates/privox)
[![Documentation](https://docs.rs/privox/badge.svg)](https://docs.rs/privox)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

**Privacy-first PII redaction for AI applications**

## Overview

`privox` automatically detects and redacts personally identifiable information (PII) from text, making it safe to send sensitive data to LLMs and other AI services.

## Features

- 🔍 **Automatic PII Detection**: Emails, phone numbers, SSNs, credit cards, API keys
- 🚀 **Streaming Support**: Process data in real-time without buffering
- 🎯 **LLM-Aware**: Token-aware redaction preserves context
- 🔒 **Privacy-First**: No data leaves your machine
- ⚡ **Fast**: Zero-copy design, minimal allocations
- 🛠️ **Configurable**: Easy to add custom patterns

## Quick Start

\`\`\`toml
# Cargo.toml
[dependencies]
privox = "0.1"
\`\`\`

\`\`\`rust
use privox::{RedactConfig, RedactionEngine};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = RedactionEngine::new(RedactConfig::default());

    let text = "Contact me at john@example.com or 555-123-4567";
    let redacted = engine.redact(text)?;

    assert!(redacted.contains("[EMAIL]"));
    assert!(redacted.contains("[PHONE]"));

    println!("{}", redacted);
    Ok(())
}
\`\`\`

## Documentation

- [API Documentation](https://docs.rs/privox)
- [Examples](https://github.com/superinstance/privox/tree/main/examples)
- [Architecture Guide](https://github.com/superinstance/privox/blob/main/docs/architecture.md)

## License

MIT OR Apache-2.0
```

### Step 3.3: Pre-Publish Checklist

```bash
# For each tool:

# 1. Run tests
cargo test

# 2. Check for warnings
cargo clippy -- -D warnings

# 3. Format code
cargo fmt

# 4. Build documentation
cargo doc --no-deps --open

# 5. Run examples
cargo run --example basic

# 6. Check metadata
cargo package

# 7. Publish dry-run
cargo publish --dry-run

# 8. Actual publish
cargo publish
```

### Step 3.4: Version 0.1.0 Requirements

For a 0.1.0 release, ensure:

- ✅ All tests passing
- ✅ Zero compiler warnings
- ✅ Public APIs documented
- ✅ README with examples
- ✅ LICENSE file included
- ✅ At least 1 working example
- ✅ Basic benchmark (if performance-critical)

---

## Phase 4: Post-Publishing

### Step 4.1: Create GitHub Release

```bash
# Tag the release
git tag -a v0.1.0 -m "Release privox v0.1.0"
git push origin v0.1.0

# Create GitHub release with:
# - Release notes
# - Binary artifacts (if applicable)
# - Migration guide (if updating)
```

### Step 4.2: Announce to Community

```markdown
# Reddit r/rust post title: [ANN] privox: Privacy-first PII redaction for AI applications

Body:
I'm excited to announce privox, a new Rust crate for automatic PII redaction.

[Short description]

[Key features]

[Example usage]

[Link to crates.io]

[Link to GitHub]

Looking forward to feedback!
```

### Step 4.3: Add to Awesome Lists

Submit to:
- [awesome-rust](https://github.com/rust-unofficial/awesome-rust)
- [rust-search-extension/awesome-rust](https://github.com/rust-search-extension/awesome-rust)

### Step 4.4: Monitor Engagement

```bash
# Track downloads (after a few days)
curl https://crates.io/api/v1/crates/privox | jq '.downloads'

# Check GitHub stars
curl https://api.github.com/repos/superinstance/privox | jq '.stargazers_count'

# Monitor issues/discussions
# Respond quickly to build community
```

---

## Tool-Specific Guides

### Tool 1: privox (Privacy Redaction)

**Extract From**: `crates/synesis-privacy/src/redact.rs`

**Key Components**:
- `RedactionEngine` - Main API
- `RedactConfig` - Configuration
- Pattern matchers (email, phone, SSN, etc.)

**Dependencies to Remove**:
- `synesis_core` types
- Tripartite council integration

**Dependencies to Add**:
- None (standalone)

**Publishing Name**: `privox`
**GitHub**: `superinstance/privox`

---

### Tool 2: tripartite-rs (Consensus Engine)

**Extract From**: `crates/synesis-core/src/consensus/`

**Key Components**:
- `ConsensusEngine` - Main consensus logic
- `Agent` trait - Agent interface
- `Voter` struct - Voting mechanism
- `RoundManager` - Multi-round coordination

**Dependencies to Remove**:
- Specific agent implementations (Pathos, Logos, Ethos)
- SuperInstance config

**Dependencies to Keep**:
- Async runtime (tokio)
- Serialization (serde)

**Publishing Name**: `tripartite-rs`
**GitHub**: `superinstance/tripartite-rs`

---

### Tool 3: knowledgr (Knowledge Vault)

**Extract From**: `crates/synesis-knowledge/src/`

**Key Components**:
- `KnowledgeVault` - Main storage API
- `VectorStore` - Vector operations
- `EmbeddingGenerator` - BGE-Micro integration
- `Retrieval` - RAG retrieval

**Dependencies to Remove**:
- `synesis_core` types
- File watcher (keep as optional feature)

**Dependencies to Keep**:
- SQLite-VSS integration
- Embedding models

**Publishing Name**: `knowledgr`
**GitHub**: `superinstance/knowledgr`

---

### Tool 4: hwscan-rs (Hardware Detection)

**Extract From**: `crates/synesis-models/src/detection.rs`

**Key Components**:
- `HardwareDetector` - Main API
- `GPUInfo` - GPU detection
- `CPUInfo` - CPU detection
- `Capability` - Capability flags

**Dependencies to Remove**:
- Model management code
- Synesis types

**Dependencies to Keep**:
- System info libraries
- GPU detection libraries

**Publishing Name**: `hwscan-rs`
**GitHub**: `superinstance/hwscan-rs`

---

### Tool 5: model-registry (Model Management)

**Extract From**: `crates/synesis-models/src/registry.rs`

**Key Components**:
- `ModelRegistry` - Main API
- `ModelVersion` - Version tracking
- `ModelMetadata` - Metadata storage

**Dependencies to Remove**:
- Hardware detection (move to hwscan-rs)
- Synesis config

**Dependencies to Add**:
- None (standalone)

**Publishing Name**: `model-registry`
**GitHub**: `superinstance/model-registry`

---

### Tool 6: quicunnel (QUIC Tunnel)

**Extract From**: `crates/synesis-cloud/src/tunnel.rs`

**Key Components**:
- `QuicTunnel` - Main tunnel API
- `TunnelConfig` - Configuration
- `BiDirectionalStream` - Stream management

**Dependencies to Remove**:
- Cloud-specific code
- Billing integration

**Dependencies to Keep**:
- Quinn (QUIC library)
- Tokio (async runtime)

**Publishing Name**: `quicunnel`
**GitHub**: `superinstance/quicunnel`

---

### Tool 7: usemeter (Billing Metering)

**Extract From**: `crates/synesis-cloud/src/billing.rs`

**Key Components**:
- `UsageMeter` - Main API
- `MeterRecord` - Usage record
- `BillingCalculator` - Cost calculation

**Dependencies to Remove**:
- Cloudflare integration
- Stripe integration (keep as optional)

**Dependencies to Keep**:
- Time tracking
- Serialization

**Publishing Name**: `usemeter`
**GitHub**: `superinstance/usemeter`

---

### Tool 8: token-keeper (Token Vault)

**Extract From**: `crates/synesis-privacy/src/vault.rs`

**Key Components**:
- `TokenVault` - Main API
- `TokenEntry` - Token storage
- `Encryption` - Token encryption

**Dependencies to Remove**:
- Redaction integration (that's privox)
- Synesis types

**Dependencies to Keep**:
- Encryption libraries
- Secure storage

**Publishing Name**: `token-keeper`
**GitHub**: `superinstance/token-keeper`

---

## Maintenance Strategy

### Keeping Tools in Sync with Monorepo

**Option 1: Git Submodules**

```bash
# In monorepo
git submodule add https://github.com/superinstance/privox.git vendor/privox

# In synesis-privacy/Cargo.toml
[dependencies]
privox = { path = "../vendor/privox" }
```

**Option 2: Path Dependencies**

```bash
# Keep extracted code in crates/
# Update both during development
# Publish independently

# In monorepo Cargo.toml
[dependencies]
privox = { path = "crates/privox" }

# In privox/Cargo.toml
# Use relative path during dev, crates.io for publish
[dependencies]
# (no internal dependencies)
```

**Option 3: Dual Maintenance (RECOMMENDED)**

```bash
# Maintain separate codebases
# Share code via copy-paste initially
# Re-synchronize quarterly
# Tools evolve independently over time
```

### Version Management

**Semantic Versioning**:

- **0.1.x**: Initial release, API may change
- **0.2.x**: Feature additions, breaking changes OK
- **1.0.x**: Stable API, no breaking changes
- **1.x.x**: Backward-compatible features

**Release Cadence**:
- Major tools: Quarterly releases
- Bug fixes: As needed (patch versions)
- Features: Monthly (minor versions)

---

## Success Metrics

Track these metrics for each tool:

| Metric | Target (3 months) | Target (6 months) |
|--------|-------------------|-------------------|
| **crates.io downloads** | 1,000 | 5,000 |
| **GitHub stars** | 50 | 200 |
| **External adopters** | 2 | 10 |
| **Issues resolved** | 90% | 95% |
| **Documentation coverage** | 100% | 100% |

---

## Conclusion

Extracting independent tools from SuperInstance requires:

1. **Strategic naming** - Use recommended names from strategy docs
2. **Clean extraction** - Remove internal dependencies
3. **Standalone value** - Each tool must work independently
4. **Professional polish** - Documentation, examples, tests
5. **Community engagement** - Announce, support, iterate

**Timeline Estimate**:
- Phase 1: 1 week
- Phase 2: 2-4 weeks per tool
- Phase 3: 1 week per tool
- Phase 4: Ongoing

**Total**: 3-6 months for all 8 tools

---

*Version: 1.0*
*Last Updated: 2026-01-08*
*Author: SuperInstance AI Team*
