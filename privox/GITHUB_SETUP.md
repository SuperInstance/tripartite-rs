# GitHub Repository Setup Instructions

## Prerequisites

You need admin access to the SuperInstance GitHub organization.

## Step 1: Create GitHub Repository

Since `gh` CLI is not installed, create the repo manually:

1. Go to: https://github.com/organizations/SuperInstance/repositories/new
2. Settings:
   - **Repository name**: `privox`
   - **Description**: `Privacy-first redaction engine for LLM applications - 18 built-in PII patterns, reversible tokenization, 100K+ ops/sec`
   - **Visibility**: Public ✅
   - **Initialize**: ❌ (we have code ready)

## Step 2: Push Code

```bash
# From Tripartite1 repository
cd /mnt/c/claudesuperinstance

# Create a separate worktree for privox (or export subdirectory)
git subtree push --prefix privox origin privox-first-push

# OR: Manual approach (if subtree doesn't work)
mkdir -p /tmp/privox-standalone
cd /mnt/c/claudesuperinstance/privox
tar czf /tmp/privox.tar.gz --exclude=target --exclude=.git .
cd /tmp/privox-standalone
tar xzf /tmp/privox.tar.gz
git init
git add .
git commit -m "Initial commit: privox v0.1.0"
git remote add origin https://github.com/SuperInstance/privox.git
git push -u origin main
```

## Step 3: Verify Repository

Visit: https://github.com/SuperInstance/privox

Should show:
- ✅ README.md renders correctly
- ✅ LICENSE file present
- ✅ Examples visible
- ✅ CI/CD workflows ready

## Step 4: Create GitHub Release

1. Go to: https://github.com/SuperInstance/privox/releases/new
2. Tag: `v0.1.0`
3. Title: `privox v0.1.0 - Privacy-First LLM Redaction`
4. Description: Copy from `RELEASE_NOTES.md`

## Step 5: Publish to crates.io

```bash
cd /mnt/c/claudesuperinstance/privox

# Login to crates.io (first time only)
cargo login

# Dry run (verifies package is valid)
cargo publish --dry-run

# If dry-run succeeds, publish for real
cargo publish
```

## Post-Publish Verification

1. **crates.io**: Visit https://crates.io/crates/privox
2. **docs.rs**: Wait 10-20 minutes, then visit https://docs.rs/privox
3. **Dependencies**: Verify SuperInstance can use `privox = "0.1"`

## Migration for SuperInstance

```toml
# In Tripartite1/Cargo.toml
[dependencies]
# Old:
# synesis-privacy = { path = "crates/synesis-privacy" }

# New (during development):
privox = { path = "privox" }

# After crates.io publish:
privox = "0.1"
```

## Next Steps After Publishing

1. Update SuperInstance to use `privox`
2. Remove `crates/synesis-privacy` from workspace
3. Update all imports: `synesis_privacy` → `privox`
4. Run tests to verify migration
5. Begin Round 2: tripartite-rs extraction

---

**Status**: Ready for manual setup
**Blocker**: GitHub CLI not installed
**Workaround**: Manual repo creation required
