# SuperInstance Tool Extraction Checklist

> **Step-by-step guide for extracting tools from SuperInstance as standalone crates**

---

## 📋 Overview

This checklist guides you through extracting a SuperInstance component (like `synesis-privacy`, `synesis-knowledge`, etc.) as a standalone, production-ready Rust crate.

**Using**: [Standalone Crate Template](./STANDALONE_CRATE_TEMPLATE.md)

**Estimated Time**: 3-4 hours per crate

**Quality Goal**: Production-ready, publishable to crates.io

---

## 🎯 Pre-Extraction Planning

### 1. Evaluate Extraction Candidacy

**Questions to ask**:

- [ ] **Is it loosely coupled?** (< 5 dependencies on other SuperInstance crates)
- [ ] **Does it solve a general problem?** (Useful outside SuperInstance)
- [ ] **Is it well-tested?** (> 80% test coverage)
- [ ] **Is it well-documented?** (All public APIs documented)
- [ ] **Does it have clear boundaries?** (Well-defined interface)

**Score**: 4-5 yes = Extract | 2-3 yes = Maybe | 0-1 yes = Don't extract

### 2. Identify Dependencies

**List all dependencies**:

```bash
# In the crate directory
cd crates/synesis-<tool>/

# Check Cargo.toml dependencies
cat Cargo.toml | grep -A 20 "\[dependencies\]"

# Check code imports
rg "^use synesis_" src/ | sort -u
```

**Categorize**:
- ✅ **External crates** (tokio, regex, etc.) - Keep
- ⚠️ **Internal dependencies** (synesis-core, etc.) - May need refactoring
- ❌ **Tightly coupled** - Don't extract (or refactor first)

### 3. Plan the Crate Name

**Naming convention**:
- Current: `synesis-privacy` → New: `privacy-proxy`
- Current: `synesis-knowledge` → New: `knowledge-vault`
- Current: `synesis-models` → New: `hardware-detection`

**Check availability**:
```bash
# Check crates.io
curl https://crates.io/api/v1/crates/<new-name>

# Check GitHub
curl https://github.com/<new-name>
```

### 4. Estimate Scope

**Files to extract**:
- Source code: ~N files
- Tests: ~N files
- Examples: ~N files
- Documentation: ~N files

**Total**: ~N files

---

## 🚀 Extraction Steps

### Phase 1: Setup (15 minutes)

#### 1.1 Create New Repository

```bash
# Create new crate
cargo new <new-name> --lib
cd <new-name>

# Create directories
mkdir -p .github/workflows .github/ISSUE_TEMPLATE \
    examples/basic examples/intermediate examples/advanced \
    benches docs/tutorials docs/guides docs/reference \
    tests
```

#### 1.2 Copy Template Files

```bash
# From templates directory
cp /path/to/templates/*.md .

# Copy GitHub templates
cp -r /path/to/templates/.github/* .github/

# Copy example structure
cp /path/to/templates/examples/README.md examples/

# Copy docs structure
cp /path/to/templates/docs/README.md docs/
```

#### 1.3 Customize Template

```bash
# Replace placeholders
find . -type f -name "*.md" -o -name "*.yml" | \
    xargs sed -i 's/tool-name/<new-name>/g'

find . -type f -name "*.md" -o -name "*.yml" | \
    xargs sed -i 's/username/<your-username>/g'
```

### Phase 2: Code Migration (45 minutes)

#### 2.1 Copy Source Code

```bash
# Copy source files
cp -r ../crates/synesis-<tool>/src/* src/

# Copy tests
cp -r ../crates/synesis-<tool>/tests/* tests/ 2>/dev/null || true
```

#### 2.2 Update Imports

**Find internal dependencies**:
```bash
cd src/
rg "^use synesis_" --type rust
```

**Replace**:
- `synesis_core::` → Remove or replace
- `synesis_` → Internal only (refactor needed)

**Example**:
```rust
// Before
use synesis_core::{Error, Result};

// After
use crate::{Error, Result};
```

#### 2.3 Update Cargo.toml

**Copy dependencies from original**:
```bash
# Get original dependencies
cat ../crates/synesis-<tool>/Cargo.toml
```

**Paste into new Cargo.toml**:
```toml
[package]
name = "<new-name>"
version = "0.1.0"
edition = "2021"
rust-version = "1.75"
description = "<description from original>"
# ... (use template)

[dependencies]
# Copy from original, remove synesis_* dependencies
```

#### 2.4 Fix Compilation Errors

```bash
# Try to build
cargo build

# Fix errors one by one
# Common issues:
# 1. Missing dependencies → Add to Cargo.toml
# 2. Internal types → Copy or refactor
# 3. Unclear imports → Update paths
```

### Phase 3: Testing (30 minutes)

#### 3.1 Run All Tests

```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test --test '*'

# Documentation tests
cargo test --doc

# Examples
cargo test --examples
```

**Goal**: 100% tests passing

#### 3.2 Check Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage
cargo tarpaulin --out Html

# Open report
open html/index.html
```

**Goal**: > 80% coverage

#### 3.3 Run Quality Checks

```bash
# Format
cargo fmt --all -- --check

# Lint
cargo clippy --all-targets -- -D warnings

# Build docs
cargo doc --no-deps --all-features

# Check all
cargo check --all-targets --all-features
```

**Goal**: Zero warnings, zero errors

### Phase 4: Documentation (45 minutes)

#### 4.1 Write README.md

**Structure** (from template):
1. Badges (CI, docs, crates.io, license)
2. What is it? (3 sentences)
3. Key features (4-6 items)
4. Quick start (working example)
5. Use cases (2-3 examples)
6. Architecture (diagram)
7. Performance (table)
8. Contributing (link)
9. License
10. Support/acknowledgments

**Time**: 20 minutes

#### 4.2 Create Getting Started Tutorial

**File**: `docs/tutorials/getting_started.md`

**Structure**:
1. Prerequisites
2. Installation
3. First example (hello world)
4. Next steps
5. Troubleshooting

**Time**: 15 minutes

#### 4.3 Create Examples

**Minimum**:
1. `examples/hello_world.rs` - Minimal working example
2. `examples/basic/quick_start.rs` - Quick start guide

**Template**:
```rust
//! Example Name - Brief description
//!
//! # Expected Output
//! ```text
//! Output here
//! ```

use <new_name>::MainType;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Code here
    Ok(())
}
```

**Time**: 10 minutes

### Phase 5: CI/CD (20 minutes)

#### 5.1 Update CI Workflow

**File**: `.github/workflows/ci.yml`

**Customize**:
- Repository name
- Branch names (if not main/develop)
- Additional steps (if needed)

#### 5.2 Test CI Locally

```bash
# Install act (local GitHub Actions runner)
# See: https://github.com/nektos/act

# Run CI locally
act push
```

**Goal**: CI passes on all platforms

### Phase 6: GitHub Setup (15 minutes)

#### 6.1 Create Repository

```bash
# On GitHub, create new repository

# Add remote
git remote add origin https://github.com/<username>/<new-name>.git

# Push
git branch -M main
git push -u origin main
```

#### 6.2 Configure Repository

**Settings**:
- [ ] Topics: Add relevant topics (rust, <category>)
- [ ] Description: Copy from README
- [ ] Website: Add docs.rs link
- [ ] Branch protection: Protect main branch
- [ ] Rulesets: Require PR reviews, require CI pass

#### 6.3 Enable Features

**Actions**:
- [ ] Enable GitHub Actions
- [ ] Enable Dependabot
- [ ] Enable CodeQL (security)
- [ ] Enable Discussions (optional)

**Secrets** (if needed):
- [ ] CRATES_IO_TOKEN (for publishing)
- [ ] DOC_RS_API_TOKEN (for docs.rs builds)

### Phase 7: Publishing (20 minutes)

#### 7.1 Pre-Publish Checklist

**Code Quality**:
- [ ] All tests pass
- [ ] Zero warnings
- [ ] All public APIs documented
- [ ] README.md complete
- [ ] Examples work

**Legal**:
- [ ] LICENSE files present
- [ ] License specified in Cargo.toml
- [ ] No copyrighted material

**Metadata**:
- [ ] Crate name available on crates.io
- [ ] Description clear and concise
- [ ] Keywords relevant (max 5)
- [ ] Categories appropriate (max 5)
- [ ] Repository URL correct
- [ ] Documentation URL correct

#### 7.2 Dry Run

```bash
# Login to crates.io
cargo login

# Dry run
cargo publish --dry-run
```

**Fix any errors** before actual publish.

#### 7.3 Publish

```bash
# Publish!
cargo publish
```

#### 7.4 Verify

```bash
# Check it's on crates.io
curl https://crates.io/api/v1/crates/<new-name>

# Check docs.rs builds
# (Automatic, takes ~5-10 minutes)
# Visit: https://docs.rs/<new-name>
```

#### 7.5 Release on GitHub

1. Go to: https://github.com/<username>/<new-name>/releases/new
2. Tag: `v0.1.0`
3. Title: `Version 0.1.0`
4. Description: Copy from CHANGELOG.md
5. Publish release

---

## 📊 Post-Extraction Tasks

### 1. Update SuperInstance

**In `Cargo.toml`**:
```toml
[dependencies]
# Old
synesis-privacy = { path = "crates/synesis-privacy" }

# New
privacy-proxy = "0.1"
```

**Update imports**:
```bash
# Find all uses
cd /mnt/c/claudesuperinstance
rg "use synesis_privacy" --type rust

# Replace
# (Use find + sed or IDE refactoring)
```

**Remove old crate**:
```bash
# After confirming everything works
rm -rf crates/synesis-privacy
```

### 2. Document Extraction

**Create**: `status/<NEW_NAME>_EXTRACTED.md`

**Content**:
```markdown
# <NEW-NAME> Extraction Complete

## Date
YYYY-MM-DD

## Extraction Details
- **Original**: `synesis-privacy`
- **New**: `privacy-proxy`
- **Repository**: https://github.com/SuperInstance/privacy-proxy
- **Crates.io**: https://crates.io/crates/privacy-proxy
- **Docs.rs**: https://docs.rs/privacy-proxy

## Changes Made
- Removed dependencies on: <list>
- Refactored: <list>
- Added: <list>

## Migration Guide
For users upgrading:

```toml
# Old
synesis-privacy = "0.2"

# New
privacy-proxy = "0.1"
```

Update imports:
```rust
// Old
use synesis_privacy::PrivacyProxy;

// New
use privacy_proxy::PrivacyProxy;
```

## Status
✅ Extracted
✅ Published
✅ Integrated
```

### 3. Announce

**Internal**:
- [ ] Update CLAUDE.md
- [ ] Update README.md
- [ ] Add to status/
- [ ] Team announcement

**External**:
- [ ] Reddit (r/rust)
- [ ] Twitter / Mastodon
- [ ] Discord / Slack
- [ ] Blog post

---

## ✅ Quality Checklist

### Before Publishing

**Code**:
- [ ] `cargo test` passes (100%)
- [ ] `cargo clippy` passes (0 warnings)
- [ ] `cargo fmt` passes (formatted)
- [ ] `cargo doc` passes (all documented)
- [ ] `cargo bench` runs successfully

**Documentation**:
- [ ] README.md converts users in 10 seconds
- [ ] getting_started.md works end-to-end
- [ ] All examples compile and run
- [ ] API docs complete
- [ ] No broken links

**CI/CD**:
- [ ] CI passes on Linux
- [ ] CI passes on macOS
- [ ] CI passes on Windows
- [ ] Coverage > 80%
- [ ] Security scans pass

**Legal**:
- [ ] LICENSE files included
- [ ] Cargo.toml license correct
- [ ] No unauthorized code
- [ ] Attribution given

**Metadata**:
- [ ] Name available
- [ ] Description clear
- [ ] Keywords relevant
- [ ] Categories appropriate
- [ ] URLs correct

### After Publishing

**Verification**:
- [ ] Appears on crates.io
- [ ] Docs.rs builds successfully
- [ ] Can be installed: `cargo add <name>`
- [ ] Examples work in fresh project
- [ ] No broken dependencies

**Integration**:
- [ ] SuperInstance updated
- [ ] Tests still pass
- [ ] Documentation updated
- [ ] Migration guide written

---

## 🎯 Extraction Candidates

### High Priority (Loosely Coupled)

1. **synesis-privacy** → `privacy-proxy`
   - Dependencies: Low (regex, rusqlite)
   - Value: High (general privacy tool)
   - Effort: Low (2-3 hours)

2. **synesis-knowledge** → `knowledge-vault`
   - Dependencies: Medium (sqlite-vss)
   - Value: High (RAG is popular)
   - Effort: Medium (3-4 hours)

3. **synesis-models** → `hardware-detection`
   - Dependencies: Low (system info)
   - Value: Medium (specialized)
   - Effort: Low (2 hours)

### Medium Priority (Some Refactoring Needed)

4. **synesis-cloud** → `quic-tunnel`
   - Dependencies: Medium (quinn, tokio)
   - Value: Medium (networking)
   - Effort: Medium (3-4 hours)
   - **Note**: Wait for Phase 2 completion

### Low Priority (Tightly Coupled)

5. **synesis-core** → Not recommended (too coupled)
6. **synesis-cli** → Not recommended (SuperInstance-specific)

---

## 📚 Resources

### Templates

- **[STANDALONE_CRATE_TEMPLATE.md](STANDALONE_CRATE_TEMPLATE.md)** - Full template
- **[CRATE_QUICK_REFERENCE.md](CRATE_QUICK_REFERENCE.md)** - Quick setup
- **[CRATE_EXAMPLE_README.md](CRATE_EXAMPLE_README.md)** - Practical example
- **[VISUAL_STRUCTURE.md](VISUAL_STRUCTURE.md)** - Visual guide

### External Resources

- **[The Cargo Book](https://doc.rust-lang.org/cargo/)** - Publishing
- **[crates.io](https://crates.io/)** - Package registry
- **[docs.rs](https://docs.rs/)** - Documentation
- **[API Guidelines](https://rust-lang.github.io/api-guidelines/)** - API design

---

## 🎉 Success Criteria

Your extraction is successful when:

✅ **Code Quality**
- All tests pass (100%)
- Zero warnings
- > 80% coverage

✅ **Documentation**
- README converts users in 10 seconds
- Tutorial works end-to-end
- Examples run successfully
- API documentation complete

✅ **CI/CD**
- Multi-platform testing passes
- Coverage tracked
- Security scanning enabled

✅ **Published**
- On crates.io
- Docs.rs builds
- GitHub release created

✅ **Integrated**
- SuperInstance updated
- Migration guide written
- Tests still pass

---

## 📞 Support

**Questions?**
- Check templates
- Review examples
- Ask in team chat

**Issues?**
- Document in status/
- Open GitHub discussion
- Get help from team

---

**Extraction Checklist Version**: 1.0.0
**Part of**: Standalone Crate Templates
**Last Updated**: 2026-01-08
**Maintained By**: SuperInstance AI Team
