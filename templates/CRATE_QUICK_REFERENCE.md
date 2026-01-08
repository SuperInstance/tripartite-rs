# Standalone Crate Template - Quick Reference

> **Fast-path reference for the comprehensive template**

---

## 🚀 60-Second Setup

```bash
# 1. Create crate
cargo new my-tool --lib && cd my-tool

# 2. Create essential directories
mkdir -p .github/workflows .github/ISSUE_TEMPLATE \
    examples/basic examples/intermediate examples/advanced \
    benches docs/tutorials docs/guides docs/reference \
    tests src

# 3. Copy template files (from template directory)
# - README.md
# - Cargo.toml (with metadata)
# - .github/workflows/ci.yml
# - .github/ISSUE_TEMPLATE/*
# - .github/PULL_REQUEST_TEMPLATE.md
# - examples/README.md
# - examples/hello_world.rs
# - docs/README.md
# - docs/tutorials/getting_started.md
# - CHANGELOG.md
# - CONTRIBUTING.md
# - rust-toolchain.toml
# - rustfmt.toml
# - LICENSE-MIT
# - LICENSE-APACHE

# 4. Customize
find . -type f -exec sed -i 's/tool-name/my-tool/g' {} \;
find . -type f -exec sed -i 's/username/your-github-username/g' {} \;

# 5. Test
cargo build && cargo test && cargo fmt && cargo clippy -- -D warnings

# 6. Push
git init && git add . && git commit -m "Initial commit" && \
git remote add origin https://github.com/your-github-username/my-tool.git && \
git push -u origin main
```

---

## 📋 Essential Files (Must-Have 14)

### 1. Root Directory (7 files)

```
my-tool/
├── README.md                  # User conversion (10 seconds)
├── CHANGELOG.md               # Version tracking
├── CONTRIBUTING.md            # Contributor guide
├── Cargo.toml                 # Package manifest
├── rust-toolchain.toml        # Rust version pinning
├── rustfmt.toml               # Code formatting
├── LICENSE-MIT / LICENSE-APACHE  # Legal
```

### 2. GitHub (6 files)

```
.github/
├── workflows/
│   └── ci.yml                 # Testing (Linux, macOS, Windows)
├── ISSUE_TEMPLATE/
│   ├── bug_report.md          # Bug reports
│   └── feature_request.md     # Feature requests
└── PULL_REQUEST_TEMPLATE.md   # PR template
```

### 3. Documentation (1 file)

```
docs/
└── tutorials/
    └── getting_started.md     # User onboarding
```

---

## 📝 README.md Template (Critical!)

```markdown
# My Tool

> **One-line value proposition**

[![CI](https://github.com/username/my-tool/actions/workflows/ci.yml/badge.svg)](https://github.com/username/my-tool/actions/workflows/ci.yml)
[![Documentation](https://docs.rs/my-tool/badge.svg)](https://docs.rs/my-tool/)
[![Crates.io](https://img.shields.io/crates/v/my-tool.svg)](https://crates.io/crates/my-tool)
[![License](https://img.shields.io/badge/license-MIT%20%7C%20Apache--2.0-blue.svg)](LICENSE-APACHE)

## 🎯 What is My Tool?

Brief description (2-3 sentences).

## 🚀 Quick Start

```bash
cargo add my-tool
```

```rust
use my_tool::Tool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tool = Tool::new();
    let result = tool.do_something()?;
    println!("{:?}", result);
    Ok(())
}
```

## 📚 Documentation

- [Getting Started](docs/tutorials/getting_started.md)
- [Examples](examples/)
- [API Reference](https://docs.rs/my-tool/)

## 🤝 Contributing

[CONTRIBUTING.md](CONTRIBUTING.md)

## 📝 License

MIT OR Apache-2.0
```

---

## 📦 Cargo.toml Template (Critical!)

```toml
[package]
name = "my-tool"
version = "0.1.0"
edition = "2021"
rust-version = "1.75"
description = "Brief description for crates.io"
documentation = "https://docs.rs/my-tool/"
repository = "https://github.com/username/my-tool"
license = "MIT OR Apache-2.0"
keywords = ["keyword1", "keyword2"]
categories = ["category1"]

[dependencies]
thiserror = "1.0"

[dev-dependencies]
criterion = "0.5"

[package.metadata.docs.rs]
all-features = true
```

---

## 🔄 CI/CD Template

**File**: `.github/workflows/ci.yml`

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always
  RUSTFLAGS: -Dwarnings

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        rust: [stable]

    steps:
      - uses: actions/checkout@v4

      - uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: ${{ matrix.rust }}
          components: rustfmt, clippy

      - name: Cache
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

      - run: cargo fmt --all -- --check
      - run: cargo clippy --all-targets -- -D warnings
      - run: cargo test --workspace --verbose
```

---

## 📖 Example Structure

```
examples/
├── README.md                  # Example guide
├── hello_world.rs             # Minimal working example
├── basic/
│   ├── quick_start.rs         # Quick start
│   └── configuration.rs       # Configuration
├── intermediate/
│   ├── custom_types.rs        # Custom types
│   └── error_handling.rs      # Error handling
└── advanced/
    └── integration.rs         # Integration
```

**hello_world.rs Template**:

```rust
//! Hello World - Minimal example
//!
//! # Expected Output
//! ```text
//! Hello from my-tool!
//! ```

use my_tool::Tool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tool = Tool::new();
    println!("Hello from my-tool!");
    Ok(())
}
```

---

## 📚 Documentation Structure

```
docs/
├── README.md                  # Documentation hub
└── tutorials/
    └── getting_started.md     # Getting started
```

**getting_started.md Template**:

```markdown
# Getting Started

## Prerequisites

- Rust 1.75+

## Installation

```bash
cargo add my-tool
```

## Your First Example

```rust
use my_tool::Tool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tool = Tool::new();
    // ...
    Ok(())
}
```

## Next Steps

- [Examples](../../examples/)
- [API Reference](https://docs.rs/my-tool/)
```

---

## ✅ Pre-Commit Checklist

Before committing, run:

```bash
# 1. Format
cargo fmt

# 2. Lint
cargo clippy --all-targets -- -D warnings

# 3. Test
cargo test --workspace

# 4. Build docs
cargo doc --no-deps

# 5. Check all
cargo check --all-targets
```

---

## 🎯 Priority Levels Summary

### MUST-HAVE (14 files) - Essential for Production

**Root (7)**:
- ✅ README.md
- ✅ CHANGELOG.md
- ✅ CONTRIBUTING.md
- ✅ Cargo.toml
- ✅ rust-toolchain.toml
- ✅ rustfmt.toml
- ✅ LICENSE-MIT + LICENSE-APACHE

**GitHub (6)**:
- ✅ .github/workflows/ci.yml
- ✅ .github/ISSUE_TEMPLATE/bug_report.md
- ✅ .github/ISSUE_TEMPLATE/feature_request.md
- ✅ .github/PULL_REQUEST_TEMPLATE.md
- ✅ .github/CODEOWNERS
- ✅ .github/dependabot.yml

**Documentation (1)**:
- ✅ docs/tutorials/getting_started.md

**Examples (1)**:
- ✅ examples/hello_world.rs

### NICE-TO-HAVE (10+ files) - Professional Polish

- 📊 benches/ + benchmarks
- 📄 RELEASE_CHECKLIST.md
- 🔐 Security scanning workflow
- 📚 More tutorials and guides
- ❓ FAQ and glossary
- 🎨 Branding assets

---

## 🔗 File Templates Location

All complete file templates are in: `/mnt/c/claudesuperinstance/templates/STANDALONE_CRATE_TEMPLATE.md`

This quick reference is for fast access. See the full template for:

- Complete file contents
- Detailed explanations
- Best practices
- Usage instructions

---

## 📦 What to Copy When

### New Crate (Minimal)

```bash
# Essential 14 files only
README.md, CHANGELOG.md, CONTRIBUTING.md, Cargo.toml,
rust-toolchain.toml, rustfmt.toml, LICENSE-*,
.github/workflows/ci.yml,
.github/ISSUE_TEMPLATE/bug_report.md,
.github/ISSUE_TEMPLATE/feature_request.md,
.github/PULL_REQUEST_TEMPLATE.md,
docs/tutorials/getting_started.md,
examples/hello_world.rs
```

### Professional Crate (Recommended)

```bash
# All must-haves + nice-to-haves
# (Add benches, more examples, more docs, etc.)
```

### Production Crate (Complete)

```bash
# Everything from template
# (Add branding, advanced workflows, etc.)
```

---

## 💡 Pro Tips

1. **Start minimal**: 14 essential files only
2. **Iterate**: Add more as you grow
3. **Automate**: Use GitHub Actions
4. **Document**: Write docs as you code
5. **Test early**: Set up CI from day one
6. **Release often**: Use semantic versioning
7. **Be welcoming**: Good contribution guide

---

## 🎯 Learning Path

```
1. Read this Quick Reference (5 minutes)
   ↓
2. Copy essential files (10 minutes)
   ↓
3. Customize for your tool (15 minutes)
   ↓
4. Test everything (10 minutes)
   ↓
5. Push to GitHub (5 minutes)
   ↓
Total: 45 minutes to production-ready crate!
```

---

**Quick Reference Version**: 1.0.0
**Companion to**: STANDALONE_CRATE_TEMPLATE.md
**Last Updated**: 2026-01-08
