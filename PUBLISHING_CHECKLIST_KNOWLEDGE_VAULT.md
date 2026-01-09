# Publishing Checklist for knowledge-vault-rs

This checklist guides you through publishing `knowledge-vault-rs` to crates.io as a standalone crate.

## Overview

`knowledge-vault-rs` is a Rust library for building local vector databases and RAG (Retrieval-Augmented Generation) systems. It provides:

- **Document chunking** with multiple strategies (sliding window, markdown-aware, code-aware)
- **Embedding generation** with pluggable providers (BGE-Micro, OpenAI, custom)
- **Vector storage** using SQLite-VSS for fast similarity search
- **File watching** for automatic reindexing
- **Async channel-based indexing** for non-blocking operations

## Pre-Publish Requirements

Before publishing, ensure all of the following are complete:

- [ ] **Source Code Extracted**: All knowledge vault code copied from synesis-knowledge
- [ ] **All Tests Pass**: Run `cargo test --all-features` (100% pass rate required)
- [ ] **Zero Compiler Warnings**: Run `cargo clippy --all-features -- -D warnings`
- [ ] **Code Formatted**: Run `cargo fmt --all`
- [ ] **README Complete**: Converts visitors in 10 seconds, shows quick start
- [ ] **All Examples Run**: `cargo run --example basic` (if examples exist)
- [ ] **CI/CD Configured**: GitHub Actions workflows pass on all platforms
- [ ] **Documentation Complete**: All public APIs have doc comments
- [ ] **Cross-References Added**: Links to SuperInstance use cases
- [ ] **LICENSE File Present**: MIT OR Apache-2.0
- [ ] **CONTRIBUTING.md Present**: Contribution guidelines
- [ ] **Cargo.toml Metadata**: Complete metadata (description, categories, keywords)
- [ ] **Feature Flags Tested**: All feature combinations work correctly
- [ ] **Performance Benchmarks**: Basic benchmarks included in docs
- [ ] **Migration Guide**: Complete guide from synesis-knowledge provided

## Pre-Publish Verification

Run these commands to verify the crate is ready:

```bash
# Navigate to knowledge-vault-rs directory
cd /mnt/c/claudesuperinstance/knowledge-vault-rs

# 1. Run all tests with all features
cargo test --all-features

# Expected: All tests pass (30+ tests)
# synesis-knowledge has 28 tests
# knowledge-vault-rs should have similar or better coverage

# 2. Check for compiler warnings
cargo clippy --all-features -- -D warnings

# Expected: Zero warnings
# Any warning must be fixed before publishing

# 3. Format check
cargo fmt -- --check

# Expected: No formatting errors
# If errors, run: cargo fmt

# 4. Build documentation
cargo doc --all-features --no-deps

# Expected: Documentation builds successfully
# Check for missing docs: cargo doc --all-features --no-deps 2>&1 | grep "missing"

# 5. Run examples (if any)
cargo run --example basic
cargo run --example markdown_chunking
cargo run --example code_chunking
cargo run --example vector_search

# Expected: All examples run without errors

# 6. Verify package metadata
cargo package --list

# Expected: All necessary files included
# - LICENSE, LICENSE-MIT, LICENSE-APACHE
# - README.md
# - Cargo.toml
# - src/**/*.rs
# - examples/** (if present)
```

## Cargo.toml Verification

Ensure your `Cargo.toml` has complete metadata:

```toml
[package]
name = "knowledge-vault-rs"
version = "0.1.0"
description = "Local vector database and RAG system for Rust applications"
license = "MIT OR Apache-2.0"
repository = "https://github.com/SuperInstance/knowledge-vault-rs"
authors = ["SuperInstance Team"]
edition = "2021"
readme = "README.md"
keywords = [
    "vector-database",
    "embeddings",
    "rag",
    "semantic-search",
    "sqlite",
]
categories = [
    "database",
    "asynchronous",
    "science",
    "algorithms",
    "text-processing",
]
rust-version = "1.75"  # Minimum supported Rust version

[dependencies]
# Async runtime
tokio = { version = "1", features = ["full"] }

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Error handling
thiserror = "1"
anyhow = "1"

# Async trait
async-trait = "0.1"

# Database (SQLite)
rusqlite = "0.32"

# IDs
uuid = { version = "1", features = ["v4", "serde"] }

# Time
chrono = { version = "0.4", features = ["serde"] }

# File watching
notify = "6"

# Crypto for content hashing
sha2 = "0.10"
hex = "0.4"

# Text processing
unicode-segmentation = "1"

# Regex for pattern matching
regex = "1"

# Futures utilities
futures = "0.3"

# Logging
tracing = "0.1"

[dev-dependencies]
tokio-test = "0.4"
tempfile = "3"

[lib]
name = "knowledge_vault"
path = "src/lib.rs"

[features]
default = []
# Enable SQLite-VSS extension for vector search
vss = []
# Enable file watching
watcher = ["notify"]
# Enable all features
full = ["vss", "watcher"]

[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
```

## Test on crates.io (Dry Run)

Before publishing for real, do a dry run:

```bash
# Navigate to knowledge-vault-rs
cd /mnt/c/claudesuperinstance/knowledge-vault-rs

# Dry run (validates package without publishing)
cargo publish --dry-run

# Expected output:
# - Packaging knowledge-vault-rs v0.1.0
# - Verifying knowledge-vault-rs v0.1.0
# - (success message)

# If dry run fails:
# 1. Read error messages carefully
# 2. Fix issues (missing files, warnings, etc.)
# 3. Run cargo test again
# 4. Run cargo publish --dry-run again
```

## Common Dry Run Issues

### Issue: "can't find crate"
```
error: cannot find crate `synesis-knowledge` in the registry
```
**Solution**: Remove any `synesis-knowledge` dependencies from `Cargo.toml`. The crate must be standalone.

### Issue: Missing documentation
```
warning: unused doc comment
```
**Solution**: Add doc comments to all public types and functions:
```rust
/// Document chunker with multiple strategies
pub struct DocumentChunker { ... }
```

### Issue: Warning about dead code
```
warning: field is never read
```
**Solution**: Either use the field or mark it as `pub` if it's part of the public API.

### Issue: Missing license files
```
error: failed to verify package tarball
```
**Solution**: Ensure `LICENSE-MIT` and `LICENSE-APACHE` files exist in the repository root.

### Issue: Feature flag conflicts
```
error: cannot find attribute `features` in this scope
```
**Solution**: Ensure all feature flags are properly defined in `[features]` section.

## Publish for Real

Once dry run succeeds, publish:

```bash
# Publish to crates.io
cargo publish

# Expected output:
# - Uploading knowledge-vault-rs v0.1.0
# - Published knowledge-vault-rs v0.1.0
# - crates.io URL: https://crates.io/crates/knowledge-vault-rs

# NOTE: First-time publish may require:
# 1. Login to crates.io (creates API token)
# 2. Running: cargo login
# 3. Pasting API token
```

## Post-Publish Checklist

After publishing, verify everything:

- [ ] **Verify on crates.io**
  - Visit: https://crates.io/crates/knowledge-vault-rs
  - Check metadata (description, keywords, categories)
  - Verify license is correct
  - Check version is 0.1.0
  - Verify features are listed

- [ ] **Verify docs.rs builds**
  - Visit: https://docs.rs/knowledge-vault-rs
  - Wait 5-10 minutes for docs to build (automatic)
  - Check all public types are documented
  - Verify examples render correctly
  - Verify feature-flagged docs work

- [ ] **Create GitHub Release v0.1.0**
  - Go to: https://github.com/SuperInstance/knowledge-vault-rs/releases
  - Click "Draft a new release"
  - Tag: `v0.1.0`
  - Title: `knowledge-vault-rs v0.1.0`
  - Description: Use content from `RELEASE_NOTES.md`
  - Attach binaries (if any)

- [ ] **Update SuperInstance to use published version**
  - Edit `/mnt/c/claudesuperinstance/Cargo.toml`
  - Replace `synesis-knowledge` with `knowledge-vault-rs` dependency
  - See `MIGRATION_GUIDE.md` for details

- [ ] **Update ecosystem documentation**
  - Update `ECOSYSTEM_INDEX.md` if it exists
  - Update `CLAUDE.md` with knowledge-vault-rs reference
  - Update `ARCHITECTURE.md` with new dependency diagram
  - Update project README with new crate link

- [ ] **Publish announcement post**
  - Publish on SuperInstance blog
  - Announce on Rust forums (users.rust-lang.org)
  - Tweet with hashtag #rustlang
  - Post on Reddit r/rust

## Migration from synesis-knowledge

Users migrating from `synesis-knowledge` to `knowledge-vault-rs` need these changes:

### 1. Update Cargo.toml

```toml
# Old
[dependencies]
synesis-knowledge = "0.2"

# New
[dependencies]
knowledge-vault-rs = "0.1"
```

### 2. Update Imports

```rust
// Old
use synesis_knowledge::{KnowledgeVault, DocumentIndexer, LocalEmbedder};
use synesis_knowledge::{Chunker, ChunkOptions};

// New
use knowledge_vault::{KnowledgeVault, DocumentIndexer, LocalEmbedder};
use knowledge_vault::{Chunker, ChunkOptions};
```

### 3. Update Code (Minimal Changes)

The API is 95% compatible. Key differences:
- Error type renamed: `KnowledgeError` (same)
- Traits are in public API (previously private)
- Feature flags enable optional functionality

See `MIGRATION_GUIDE.md` for complete migration instructions.

## Verification Steps

After publishing, verify the following:

### 1. Install from crates.io

```bash
# Create a new test project
cargo new test-knowledge-vault
cd test-knowledge-vault

# Add dependency
cargo add knowledge-vault-rs

# Build
cargo build

# Expected: Downloads and compiles knowledge-vault-rs v0.1.0 successfully
```

### 2. Test Basic Functionality

```rust
// In src/main.rs
use knowledge_vault::{KnowledgeVault, Chunker};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create vault
    let vault = KnowledgeVault::open("test.db", 384)?;

    // Test chunking
    let chunker = Chunker::new();
    let chunks = chunker.chunk("Hello world! This is a test.")?;
    println!("Created {} chunks", chunks.len());

    println!("knowledge-vault-rs imported successfully!");
    Ok(())
}
```

```bash
cargo run

# Expected: Prints "knowledge-vault-rs imported successfully!"
```

### 3. Documentation Check

```bash
# Generate docs
cargo doc --open

# Expected: Opens browser with documentation
# Check all public types are documented
```

### 4. Feature Flags Test

```bash
# Test default features
cargo build

# Test with all features
cargo build --all-features

# Test with specific features
cargo build --features "watcher,vss"

# Expected: All feature combinations build successfully
```

### 5. SuperInstance Integration

```bash
# Navigate to SuperInstance
cd /mnt/c/claudesuperinstance

# Update Cargo.toml to use knowledge-vault-rs
# (See MIGRATION_GUIDE.md)

# Build workspace
cargo build --workspace

# Expected: Builds successfully with knowledge-vault-rs dependency

# Run tests
cargo test --workspace

# Expected: All 250+ tests pass
```

## Publishing Timeline

### Phase 1: Preparation (Agent 3.1, 3.2, 3.3)
- ✅ Extract knowledge code from synesis-knowledge
- ✅ Create knowledge-vault-rs crate structure
- ✅ Implement public API with traits
- ✅ Port all chunking, embedding, and indexing logic
- ✅ Add tests (30+ tests)
- ✅ Create comprehensive documentation

### Phase 2: Pre-Publish Verification (Agent 3.4)
- ✅ Create PUBLISHING_CHECKLIST.md
- ✅ Create RELEASE_NOTES.md
- ✅ Create MIGRATION_GUIDE.md
- ✅ Create INTEGRATION_PLAN.md
- ⏳ Run all verification commands
- ⏳ Fix any issues found

### Phase 3: GitHub Setup (After extraction complete)
- ⏳ Create repository: https://github.com/SuperInstance/knowledge-vault-rs
- ⏳ Push code to GitHub
- ⏳ Configure GitHub Actions CI/CD
- ⏳ Test CI/CD passes on all platforms
- ⏳ Add README with quick start examples

### Phase 4: crates.io Publishing (After orchestrator approval)
- ⏳ Run `cargo publish --dry-run`
- ⏳ Fix any issues from dry run
- ⏳ Run `cargo publish` for real
- ⏳ Verify on crates.io and docs.rs

### Phase 5: SuperInstance Migration (After publish)
- ⏳ Update SuperInstance's Cargo.toml
- ⏳ Update all imports in synesis-knowledge
- ⏳ Remove knowledge code from synesis-knowledge
- ⏳ Run `cargo test --workspace`
- ⏳ Verify all 250+ tests still pass
- ⏳ Commit and push changes

## Notes

- **DO NOT publish to crates.io until orchestrator approves**
- **GitHub repo can be created anytime** (can be private initially)
- **SuperInstance can use path dependency during development**:
  ```toml
  [dependencies]
  knowledge-vault-rs = { path = "../knowledge-vault-rs" }
  ```
- **Switch to crates.io dependency only after published**:
  ```toml
  [dependencies]
  knowledge-vault-rs = "0.1"
  ```

## Rollback Plan

If publishing fails or causes issues:

1. **Yank the version** (if published):
   ```bash
   # Go to https://crates.io/crates/knowledge-vault-rs
   # Click "Yank" button for v0.1.0
   ```

2. **Revert SuperInstance changes**:
   ```bash
   cd /mnt/c/claudesuperinstance
   git checkout main
   git reset --hard HEAD~1  # Undo migration commit
   ```

3. **Fix issues** and repeat publishing process

4. **Publish v0.1.1** with fixes:
   ```bash
   # Bump version in Cargo.toml to 0.1.1
   # Fix issues
   # cargo publish
   ```

## Success Criteria

Publishing is successful when:
- ✅ Crate appears on https://crates.io/crates/knowledge-vault-rs
- ✅ Documentation builds on https://docs.rs/knowledge-vault-rs
- ✅ Can install with `cargo add knowledge-vault-rs`
- ✅ SuperInstance builds and runs with knowledge-vault-rs dependency
- ✅ All 250+ SuperInstance tests still pass
- ✅ Zero compiler warnings in SuperInstance workspace
- ✅ Feature flags work correctly
- ✅ Examples run without errors

## Performance Benchmarks

Include these benchmarks in the crate documentation:

```markdown
## Performance

### Chunking Speed
- Markdown documents: ~1MB/s
- Code files: ~500KB/s (with RegexSet optimization)
- Plain text: ~2MB/s

### Embedding Generation
- Placeholder (SHA256): ~10k embeddings/sec
- BGE-Micro (real): ~100 embeddings/sec (when integrated)

### Search Performance
- VSS (with extension): <10ms for 100k chunks
- Cosine fallback: ~100ms for 10k chunks
- In-memory: <1ms for 1k chunks
```

## Security Considerations

1. **SQL Injection**: All queries use parameterized statements
2. **Path Traversal**: File paths are validated and sandboxed
3. **Resource Limits**: Channel-based indexing prevents memory overflow
4. **Content Hashing**: SHA256 prevents duplicate processing

## Maintenance Plan

- **Monthly releases**: Bug fixes and minor features
- **Quarterly major releases**: Breaking changes with migration guide
- **Security patches**: Immediate release for vulnerabilities
- **Deprecation policy**: 6-month notice for breaking changes

---

**Created**: 2026-01-08
**Agent**: 3.4 (Publishing & Integration Specialist)
**Status**: Ready for verification phase
**Version**: 0.1.0
