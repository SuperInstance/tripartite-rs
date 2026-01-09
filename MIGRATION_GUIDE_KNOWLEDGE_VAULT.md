# Migration Guide: synesis-knowledge → knowledge-vault-rs

This guide helps you migrate from `synesis-knowledge` (the SuperInstance internal crate) to `knowledge-vault-rs` (the standalone library).

---

## Table of Contents

1. [Overview](#overview)
2. [Why Migrate?](#why-migrate)
3. [Migration Strategy](#migration-strategy)
4. [Step-by-Step Migration](#step-by-step-migration)
5. [API Changes](#api-changes)
6. [Code Examples](#code-examples)
7. [Testing Migration](#testing-migration)
8. [Rollback Plan](#rollback-plan)
9. [Troubleshooting](#troubleshooting)

---

## Overview

### What's Migrating?

The knowledge vault system is being extracted from SuperInstance's monorepo into a standalone Rust library:

**Before** (monorepo internal):
```toml
[dependencies]
synesis-knowledge = { path = "../crates/synesis-knowledge" }
```

**After** (standalone library):
```toml
[dependencies]
knowledge-vault-rs = "0.1"
```

### What Stays the Same?

- **95% API compatibility** - Most code works without changes
- **Same functionality** - All features preserved
- **Same performance** - No speed regression
- **Same data format** - SQLite databases compatible

### What's Different?

- **Crate name**: `synesis_knowledge` → `knowledge_vault`
- **Traits are public**: Can now implement custom providers
- **Feature flags**: Optional functionality via Cargo features
- **Standalone**: No workspace dependencies

---

## Why Migrate?

### Benefits for SuperInstance

1. **Separation of Concerns**
   - Knowledge system is now a generic library
   - SuperInstance focuses on agent orchestration
   - Clear boundaries between components

2. **Ecosystem Growth**
   - Other projects can use knowledge-vault-rs
   - External contributors improve the library
   - Faster bug fixes and features

3. **Easier Testing**
   - Test knowledge system in isolation
   - Mock implementations for testing
   - Clear test boundaries

4. **Independent Versioning**
   - Knowledge vault can evolve at its own pace
   - SuperInstance not blocked by knowledge changes
   - Semantic versioning for stability

### Benefits for Users

1. **Reusability**
   - Use knowledge vault in your own projects
   - Build RAG systems without SuperInstance
   - Local-first, privacy-focused vector search

2. **Community Contributions**
   - More eyes on the code
   - Bug reports and fixes from community
   - Feature requests from diverse use cases

3. **Better Documentation**
   - Standalone docs focused on vector search
   - Examples for various use cases
   - Migration guides and tutorials

---

## Migration Strategy

### Recommended Approach: Incremental Migration

**Phase 1: Add Dependency** (Day 1)
- Add `knowledge-vault-rs` alongside `synesis-knowledge`
- No code changes yet
- Verify both compile together

**Phase 2: Update Imports** (Day 1-2)
- Replace imports in one module at a time
- Test each module after updating
- Commit frequently for easy rollback

**Phase 3: Remove Old Crate** (Day 2-3)
- Once all imports updated, remove `synesis-knowledge` from workspace
- Run full test suite
- Fix any remaining issues

**Phase 4: Clean Up** (Day 3)
- Remove old `synesis-knowledge` code from repository
- Update documentation
- Create migration commit

### Timeline

**Estimated effort**: 2-3 developer days

- **Day 1**: Add dependency, update imports in 50% of files
- **Day 2**: Update remaining imports, run tests
- **Day 3**: Remove old crate, clean up, final testing

---

## Step-by-Step Migration

### Step 1: Add knowledge-vault-rs Dependency

**File**: `/mnt/c/claudesuperinstance/Cargo.toml`

```toml
[workspace.dependencies]
# Keep old for now
synesis-knowledge = { path = "crates/synesis-knowledge" }

# Add new dependency
knowledge-vault-rs = "0.1"
```

**Build and verify**:
```bash
cd /mnt/c/claudesuperinstance
cargo build --workspace
```

Expected: Both crates compile successfully.

### Step 2: Update Imports (Module by Module)

**Example**: Update `synesis-core` first.

**File**: `/mnt/c/claudesuperinstance/crates/synesis-core/src/agents/logos.rs`

**Before**:
```rust
use synesis_knowledge::{KnowledgeVault, DocumentIndexer, LocalEmbedder};
use synesis_knowledge::{Chunker, ChunkOptions, DocType};
use synesis_knowledge::KnowledgeError;
```

**After**:
```rust
use knowledge_vault::{KnowledgeVault, DocumentIndexer, LocalEmbedder};
use knowledge_vault::{Chunker, ChunkOptions, DocType};
use knowledge_vault::KnowledgeError;
```

**Test after each module**:
```bash
cargo test --package synesis-core
```

**Repeat for**:
- `synesis-core/src/agents/*.rs`
- `synesis-cli/src/commands/knowledge.rs`
- Any other files using `synesis_knowledge`

### Step 3: Update Type Names (If Any)

Most types have the same name, but verify:

| Old Type | New Type | Notes |
|----------|----------|-------|
| `synesis_knowledge::KnowledgeError` | `knowledge_vault::KnowledgeError` | Same name |
| `synesis_knowledge::KnowledgeVault` | `knowledge_vault::KnowledgeVault` | Same name |
| `synesis_knowledge::DocumentIndexer` | `knowledge_vault::DocumentIndexer` | Same name |
| `synesis_knowledge::LocalEmbedder` | `knowledge_vault::LocalEmbedder` | Same name |
| `synesis_knowledge::Chunker` | `knowledge_vault::Chunker` | Same name |

**All types should have the same name** - only the crate path changes.

### Step 4: Update Feature Flags

If using workspace features:

**Before** (in workspace Cargo.toml):
```toml
[workspace.dependencies]
synesis-knowledge = { path = "crates/synesis-knowledge" }
```

**After**:
```toml
[workspace.dependencies]
knowledge-vault-rs = { version = "0.1", features = ["watcher", "vss"] }
```

**Available features**:
- `watcher` - File watching support
- `vss` - SQLite-VSS extension support
- `full` - All features

### Step 5: Update Test Code

Tests also use the knowledge system:

**File**: `/mnt/c/claudesuperinstance/crates/synesis-knowledge/tests/integration_test.rs`

**Before**:
```rust
use synesis_knowledge::{KnowledgeVault, LocalEmbedder};
```

**After**:
```rust
use knowledge_vault::{KnowledgeVault, LocalEmbedder};
```

**Run all tests**:
```bash
cargo test --workspace
```

Expected: All 250+ tests pass.

### Step 6: Remove Old Dependency

Once all imports updated:

**File**: `/mnt/c/claudesuperinstance/Cargo.toml`

```toml
[workspace.dependencies]
# REMOVE THIS LINE:
# synesis-knowledge = { path = "crates/synesis-knowledge" }

# KEEP THIS LINE:
knowledge-vault-rs = "0.1"
```

**Remove from workspace members**:

**File**: `/mnt/c/claudesuperinstance/Cargo.toml`

```toml
[workspace.members]
# REMOVE THIS LINE:
# "crates/synesis-knowledge",

# KEEP OTHERS:
"crates/synesis-core",
"crases/synesis-cli",
# ...
```

**Build and verify**:
```bash
cargo build --workspace
cargo test --workspace
```

### Step 7: Remove Old Code Directory

**⚠️ WARNING**: Only do this after all tests pass!

```bash
cd /mnt/c/claudesuperinstance
rm -rf crates/synesis-knowledge
```

**Verify build still works**:
```bash
cargo build --workspace
cargo test --workspace
```

### Step 8: Update Documentation

**File**: `/mnt/c/claudesuperinstance/CLAUDE.md`

Update references from `synesis-knowledge` to `knowledge-vault-rs`:

```markdown
## Architecture Overview

The knowledge system uses [knowledge-vault-rs](https://crates.io/crates/knowledge-vault-rs)
for vector storage and semantic search.
```

**File**: `/mnt/c/claudesuperinstance/README.md`

Update quick start:

```toml
[dependencies]
knowledge-vault-rs = "0.1"
```

---

## API Changes

### No Breaking Changes

The public API is **95% compatible**. Most code works with only import path changes.

### Public Traits (New)

Previously private traits are now public:

#### `EmbeddingProvider` Trait

You can now implement custom embedding providers:

```rust
use knowledge_vault::EmbeddingProvider;

struct MyCustomEmbedder {
    api_key: String,
}

#[async_trait]
impl EmbeddingProvider for MyCustomEmbedder {
    async fn embed(&self, text: &str) -> Result<Vec<f32>, KnowledgeError> {
        // Call your custom API
        Ok(vec![0.0; 384])
    }

    async fn embed_batch(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, KnowledgeError> {
        // Batch implementation
        let mut results = Vec::new();
        for text in texts {
            results.push(self.embed(text).await?);
        }
        Ok(results)
    }

    fn dimensions(&self) -> u32 {
        384
    }

    fn model_name(&self) -> &str {
        "my-custom-model"
    }
}
```

### Feature Flags (New)

Optional functionality via Cargo features:

```toml
# Default (minimal features)
knowledge-vault-rs = "0.1"

# With file watching
knowledge-vault-rs = { version = "0.1", features = ["watcher"] }

# With SQLite-VSS
knowledge-vault-rs = { version = "0.1", features = ["vss"] }

# Everything
knowledge-vault-rs = { version = "0.1", features = ["full"] }
```

### Error Types (Same)

Error types remain the same:

```rust
use knowledge_vault::KnowledgeError;

match error {
    KnowledgeError::NotFound(id) => { /* handle */ }
    KnowledgeError::EmbeddingError(msg) => { /* handle */ }
    KnowledgeError::DatabaseError(msg) => { /* handle */ }
    // ... all other variants
}
```

---

## Code Examples

### Example 1: Basic Usage (Before/After)

**Before** (synesis-knowledge):
```rust
use synesis_knowledge::{KnowledgeVault, LocalEmbedder};

let vault = KnowledgeVault::open("knowledge.db", 384)?;
let embedder = LocalEmbedder::new(384)?;

let query_embedding = embedder.embed("search query").await?;
let results = vault.search(&query_embedding, 5)?;
```

**After** (knowledge-vault-rs):
```rust
use knowledge_vault::{KnowledgeVault, LocalEmbedder};

let vault = KnowledgeVault::open("knowledge.db", 384)?;
let embedder = LocalEmbedder::new(384)?;

let query_embedding = embedder.embed("search query").await?;
let results = vault.search(&query_embedding, 5)?;
```

**Difference**: Only the import path changed!

### Example 2: Document Indexing (Before/After)

**Before**:
```rust
use synesis_knowledge::{DocumentIndexer, IndexerConfig};
use std::sync::Arc;
use tokio::sync::Mutex;

let config = IndexerConfig::default();
let (indexer, handle) = DocumentIndexer::new(
    Arc::new(Mutex::new(vault)),
    Arc::new(Mutex::new(embedder)),
    config,
);

indexer.index_file("README.md")?;
```

**After**:
```rust
use knowledge_vault::{DocumentIndexer, IndexerConfig};
use std::sync::Arc;
use tokio::sync::Mutex;

let config = IndexerConfig::default();
let (indexer, handle) = DocumentIndexer::new(
    Arc::new(Mutex::new(vault)),
    Arc::new(Mutex::new(embedder)),
    config,
);

indexer.index_file("README.md")?;
```

**Difference**: Only import path changed!

### Example 3: Custom Embedding Provider (New Capability)

This wasn't possible before - traits were private:

```rust
use knowledge_vault::{EmbeddingProvider, KnowledgeError};
use async_trait::async_trait;

struct OpenAIEmbedder {
    api_key: String,
    model: String,
}

#[async_trait]
impl EmbeddingProvider for OpenAIEmbedder {
    async fn embed(&self, text: &str) -> Result<Vec<f32>, KnowledgeError> {
        // Call OpenAI API
        let client = reqwest::Client::new();
        let response = client
            .post("https://api.openai.com/v1/embeddings")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({
                "input": text,
                "model": &self.model
            }))
            .send()
            .await
            .map_err(|e| KnowledgeError::EmbeddingError(e.to_string()))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| KnowledgeError::EmbeddingError(e.to_string()))?;

        let embedding = json["data"][0]["embedding"]
            .as_array()
            .ok_or_else(|| KnowledgeError::EmbeddingError("Invalid response".to_string()))?
            .iter()
            .map(|v| v.as_f64().unwrap_or(0.0) as f32)
            .collect();

        Ok(embedding)
    }

    async fn embed_batch(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, KnowledgeError> {
        // Implement batch embedding
        let mut results = Vec::new();
        for text in texts {
            results.push(self.embed(text).await?);
        }
        Ok(results)
    }

    fn dimensions(&self) -> u32 {
        1536 // for text-embedding-ada-002
    }

    fn model_name(&self) -> &str {
        &self.model
    }
}
```

### Example 4: File Watching (Before/After)

**Before** (with workspace features):
```rust
use synesis_knowledge::{FileWatcher, WatchConfig};

let config = WatchConfig::default();
let watcher = FileWatcher::new(config)?;
watcher.start().await?;
```

**After** (with explicit feature):
```toml
# Cargo.toml
knowledge-vault-rs = { version = "0.1", features = ["watcher"] }
```

```rust
use knowledge_vault::{FileWatcher, WatchConfig};

let config = WatchConfig::default();
let watcher = FileWatcher::new(config)?;
watcher.start().await?;
```

---

## Testing Migration

### Test Checklist

Run these tests to verify migration:

#### 1. Unit Tests

```bash
# Test each package
cargo test --package synesis-core
cargo test --package synesis-cli
cargo test --package synesis-models

# Expected: All tests pass
```

#### 2. Integration Tests

```bash
# Run all tests
cargo test --workspace

# Expected: 250+ tests pass (100%)
```

#### 3. Build Check

```bash
# Debug build
cargo build --workspace

# Release build
cargo build --release --workspace

# Expected: Zero warnings
```

#### 4. Clippy Check

```bash
cargo clippy --workspace -- -D warnings

# Expected: Zero warnings
```

#### 5. Documentation Build

```bash
cargo doc --workspace --no-deps

# Expected: Documentation builds successfully
```

#### 6. Functionality Tests

Test actual knowledge vault operations:

```rust
#[tokio::test]
async fn test_vault_operations() {
    use knowledge_vault::{KnowledgeVault, LocalEmbedder};

    // Open vault
    let vault = KnowledgeVault::open(":memory:", 384).unwrap();

    // Add document
    let content = "Test document content";
    let doc_id = vault.add_document("test.txt", content, "text").unwrap();

    // Verify document exists
    let doc = vault.get_document(&doc_id).unwrap().unwrap();
    assert_eq!(doc.title, "test.txt");
}
```

### Performance Verification

Ensure no performance regression:

```bash
# Run benchmarks (if available)
cargo bench --package synesis-core

# Expected: Similar or better performance than before
```

---

## Rollback Plan

If migration causes issues, here's how to rollback:

### Quick Rollback (Git Reset)

```bash
cd /mnt/c/claudesuperinstance

# Commit migration changes first (in case you want to revisit)
git add .
git commit -m "Migration attempt: knowledge-vault-rs"

# If issues found, reset to before migration
git reset --hard HEAD~1

# Revert to working state
git checkout main
```

### Partial Rollback (Keep Both Crates)

If you need to use both temporarily:

```toml
[workspace.dependencies]
# Keep both
synesis-knowledge = { path = "crates/synesis-knowledge" }
knowledge-vault-rs = "0.1"
```

Use imports selectively:

```rust
// Old code (still works)
use synesis_knowledge::KnowledgeVault;

// New code (testing migration)
use knowledge_vault::DocumentIndexer;
```

### Data Migration (Not Required)

**Good news**: SQLite databases are compatible!

No data migration needed - databases created with `synesis-knowledge` work with `knowledge-vault-rs` without changes.

**Why?**
- Same schema
- Same binary format for embeddings
- Same hashing algorithm (SHA256)
- Same chunking logic

---

## Troubleshooting

### Issue 1: "Cannot find synesis_knowledge"

**Error**:
```
error[E0433]: failed to resolve: use of undeclared crate or module `synesis_knowledge`
```

**Cause**: Some imports still use old crate name.

**Solution**:
```bash
# Find all occurrences
grep -r "synesis_knowledge" crates/

# Update each file
sed -i 's/synthesis_knowledge/knowledge_vault/g' crates/synesis-core/src/agents/logos.rs
```

### Issue 2: Feature Flags Not Working

**Error**:
```
error[E0432]: unresolved import `knowledge_vault::FileWatcher`
```

**Cause**: Missing feature flag.

**Solution**:
```toml
# Add feature flag
knowledge-vault-rs = { version = "0.1", features = ["watcher"] }
```

### Issue 3: Version Conflict

**Error**:
```
error: duplicate dependency for knowledge-vault-rs
```

**Cause**: Both workspace and local dependency.

**Solution**:
```toml
# Remove local dependency
[dependencies]
# knowledge-vault-rs = { path = "../knowledge-vault-rs" }  # REMOVE THIS

# Keep workspace dependency
knowledge-vault-rs = { workspace = true }
```

### Issue 4: Type Mismatch

**Error**:
```
error[E0308]: mismatched types: expected synesis_knowledge::KnowledgeError, found knowledge_vault::KnowledgeError
```

**Cause**: Error types from both crates.

**Solution**: Ensure all imports use the same crate:

```rust
// Don't do this:
use synesis_knowledge::KnowledgeVault;
use knowledge_vault::DocumentIndexer;  // Different error types!

// Do this instead:
use knowledge_vault::{KnowledgeVault, DocumentIndexer};  // Consistent
```

### Issue 5: Database Schema Mismatch

**Error**:
```
Error: DatabaseError("no such table: documents")
```

**Cause**: Old database format (unlikely).

**Solution**: Rebuild database:
```bash
rm knowledge.db
# Re-run indexing
```

### Issue 6: Embedding Dimension Mismatch

**Error**:
```
Error: EmbeddingError("Dimension mismatch: expected 384, got 768")
```

**Cause**: Different embedder configuration.

**Solution**: Use consistent dimensions:
```rust
// Wrong
let vault = KnowledgeVault::open("db.db", 384)?;
let embedder = LocalEmbedder::new(768)?;  // Mismatch!

// Correct
let vault = KnowledgeVault::open("db.db", 384)?;
let embedder = LocalEmbedder::new(384)?;  // Matches!
```

---

## Success Criteria

Migration is successful when:

- ✅ All 250+ tests pass
- ✅ Zero compiler warnings
- ✅ Zero clippy warnings
- ✅ All examples run without errors
- ✅ No performance regression
- ✅ Documentation updated
- ✅ Old crate removed from workspace
- ✅ Git history clean

---

## Post-Migration Tasks

### 1. Update CI/CD

**File**: `.github/workflows/test.yml`

Update to test with knowledge-vault-rs:
```yaml
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run tests
        run: cargo test --workspace --all-features
```

### 2. Update Documentation

**Files to update**:
- `CLAUDE.md` - Architecture overview
- `README.md` - Quick start
- `ARCHITECTURE.md` - System diagrams
- `CONTRIBUTING.md` - Development setup

### 3. Create Migration Commit

```bash
git add .
git commit -m "Migrate from synesis-knowledge to knowledge-vault-rs

- Replace synesis-knowledge with knowledge-vault-rs dependency
- Update all imports from synesis_knowledge to knowledge_vault
- Remove synesis-knowledge from workspace
- Update documentation

All 250+ tests pass.
"
```

### 4. Announce Migration

**Internally** (SuperInstance team):
- Update development blog
- Post in Discord
- Update Sprint board

**Externally** (if applicable):
- Publish blog post
- Reddit announcement
- Twitter post

---

## Summary

### What Changed

| Aspect | Before | After |
|--------|--------|-------|
| **Crate name** | `synesis-knowledge` | `knowledge-vault-rs` |
| **Import path** | `use synesis_knowledge::...` | `use knowledge_vault::...` |
| **Dependency** | `path = "../crates/synesis-knowledge"` | `"0.1"` |
| **Traits** | Private | Public (can implement) |
| **Features** | Workspace-based | Cargo features |

### What Stayed the Same

- ✅ All type names (KnowledgeVault, DocumentIndexer, etc.)
- ✅ All method signatures
- ✅ Error types and variants
- ✅ Database schema
- ✅ Performance characteristics
- ✅ Data format (embeddings, chunks)

### Migration Effort

- **Time**: 2-3 developer days
- **Complexity**: Low (95% API compatible)
- **Risk**: Low (easy rollback)
- **Benefits**: High (ecosystem growth, better testing)

---

**Need Help?**

- **GitHub Issues**: https://github.com/SuperInstance/knowledge-vault-rs/issues
- **Discord**: https://discord.gg/superinstance
- **Email**: support@superinstance.ai

---

**Created**: 2026-01-08
**For**: knowledge-vault-rs v0.1.0
**From**: synesis-knowledge v0.2.0
