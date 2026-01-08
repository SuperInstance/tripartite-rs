# Standalone Rust Crate Template

> **Comprehensive repository template for extracting SuperInstance tools as independent, production-ready crates**

---

## Table of Contents

1. [Quick Start](#quick-start)
2. [Directory Structure](#directory-structure)
3. [File-by-File Template](#file-by-file-template)
4. [Priority Levels](#priority-levels)
5. [Usage Instructions](#usage-instructions)
6. [Best Practices](#best-practices)

---

## Quick Start

### Template Overview

This template is based on analysis of three world-class Rust repositories:

1. **[huggingface/candle](https://github.com/huggingface/candle)** - ML framework
   - Excellent examples organization
   - Clear feature-based documentation
   - Progressive learning path

2. **[burn-rs/burn](https://github.com/burn-rs/burn)** - Deep learning framework
   - Clean workspace structure
   - Comprehensive benchmarking
   - Strong visual branding

3. **[rust-lang/cargo](https://github.com/rust-lang/cargo)** - Package manager
   - Professional CI/CD
   - Extensive contribution guidelines
   - Clear documentation hierarchy

### What This Template Provides

✅ **Turnkey structure** - Copy-paste ready
✅ **Progressive examples** - Hello world → advanced
✅ **Comprehensive testing** - Unit, integration, benchmarks
✅ **Professional CI/CD** - Multi-platform, coverage, security
✅ **Complete documentation** - API, guides, tutorials
✅ **Community templates** - Issues, PRs, discussions

---

## Directory Structure

```
tool-name/
├── .github/
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.md
│   │   ├── feature_request.md
│   │   ├── documentation.md
│   │   └── question.md
│   ├── PULL_REQUEST_TEMPLATE.md
│   ├── workflows/
│   │   ├── ci.yml
│   │   ├── coverage.yml
│   │   ├── security.yml
│   │   ├── release.yml
│   │   └── documentation.yml
│   ├── CODEOWNERS
│   └── dependabot.yml
├── examples/
│   ├── README.md
│   ├── hello_world.rs
│   ├── basic/
│   │   ├── quick_start.rs
│   │   └── configuration.rs
│   ├── intermediate/
│   │   ├── custom_types.rs
│   │   └── error_handling.rs
│   └── advanced/
│       ├── integration.rs
│       └── performance.rs
├── benches/
│   ├── README.md
│   ├── basic_benchmark.rs
│   └── comparison_benchmark.rs
├── docs/
│   ├── README.md
│   ├── tutorials/
│   │   ├── getting_started.md
│   │   └── common_patterns.md
│   ├── guides/
│   │   ├── architecture.md
│   │   └── api_reference.md
│   └── reference/
│       ├── faq.md
│       └── glossary.md
├── src/
│   ├── lib.rs
│   ├── error.rs
│   └── ...
├── tests/
│   └── integration_test.rs
├── CHANGELOG.md
├── CONTRIBUTING.md
├── Cargo.toml
├── LICENSE-APACHE
├── LICENSE-MIT
├── README.md
├── RELEASE_CHECKLIST.md
├── rust-toolchain.toml
└── rustfmt.toml
```

---

## File-by-File Template

### 🚀 README.md (MUST-HAVE)

**Purpose**: The 10-second conversion. Converts visitors to users in under 10 seconds.

**Template**:

```markdown
# Tool Name

> **One-line value proposition** (what it does, who it's for, why it's better)

[![CI](https://github.com/username/tool-name/actions/workflows/ci.yml/badge.svg)](https://github.com/username/tool-name/actions/workflows/ci.yml)
[![Documentation](https://docs.rs/tool-name/badge.svg)](https://docs.rs/tool-name/)
[![Crates.io](https://img.shields.io/crates/v/tool-name.svg)](https://crates.io/crates/tool-name)
[![License](https://img.shields.io/badge/license-MIT%20%7C%20Apache--2.0-blue.svg)](LICENSE-APACHE)
[![Rust Version](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)

## 🎯 What is Tool Name?

**Tool Name** is a brief description (2-3 sentences) that explains:

- What problem it solves
- Who it's for
- Why it's better than alternatives

### Key Features

- ✅ **Feature 1** - Brief explanation
- ✅ **Feature 2** - Brief explanation
- ✅ **Feature 3** - Brief explanation
- ✅ **Feature 4** - Brief explanation

## 🚀 Quick Start

### Installation

```bash
# Add to Cargo.toml
[dependencies]
tool-name = "0.1.0"

# Or install via cargo
cargo install tool-name
```

### Basic Usage

```rust
use tool_name::Tool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tool = Tool::new();
    let result = tool.do_something()?;
    println!("{:?}", result);
    Ok(())
}
```

**Output**:
```
The result is...
```

## 📚 Documentation

- **[Getting Started](docs/tutorials/getting_started.md)** - Installation and first steps
- **[Examples](examples/)** - Runnable code examples
- **[API Reference](https://docs.rs/tool-name/)** - Complete API documentation
- **[Contributing](CONTRIBUTING.md)** - Development guide

## 💡 Use Cases

### Use Case 1

**Description**: Brief description of use case

```rust
// Example code
```

### Use Case 2

**Description**: Brief description of use case

```rust
// Example code
```

## 🏗️ Architecture

Brief architecture overview or diagram.

```text
┌─────────────┐
│   Component │
└─────────────┘
```

**Learn More**: [Architecture Guide](docs/guides/architecture.md)

## 📊 Performance

| Metric | Value |
|--------|-------|
| Metric 1 | Value 1 |
| Metric 2 | Value 2 |
| Metric 3 | Value 3 |

*Benchmarks on: Hardware description*

## 🔧 System Requirements

- **Rust**: 1.75+
- **OS**: Linux, macOS, Windows
- **Memory**: X MB minimum

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

### Quick Start

```bash
# Clone repository
git clone https://github.com/username/tool-name.git
cd tool-name

# Run tests
cargo test

# Run examples
cargo run --example hello_world
```

## 📝 License

Licensed under either of:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT))
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

at your option.

## 🙏 Acknowledgments

Built with amazing projects:
- [Dependency 1](link)
- [Dependency 2](link)

## 📞 Support

- **[Documentation](docs/)** - Start here
- **[FAQ](docs/reference/faq.md)** - Common questions
- **[GitHub Issues](https://github.com/username/tool-name/issues)** - Report bugs
- **[GitHub Discussions](https://github.com/username/tool-name/discussions)** - Ask questions

---

**Tool Name** - *Your tagline here*

**Version**: 0.1.0 | **Status**: Alpha/Beta/Production

*Last Updated: YYYY-MM-DD*
```

**Priority**: MUST-HAVE

**Key Elements**:
1. Badge row (CI, docs, crates.io, license, Rust version)
2. Clear value proposition in first screen
3. Quick start that works in under 30 seconds
4. Visual badges (✅) for features
5. Architecture diagram
6. Performance table
7. All links relative (no broken URLs)

---

### 📦 Cargo.toml (MUST-HAVE)

**Purpose**: Package manifest with all metadata, dependencies, and features.

**Template**:

```toml
[package]
name = "tool-name"
version = "0.1.0"
edition = "2021"
rust-version = "1.75"
authors = ["Your Name <email@example.com>"]
description = "Brief description (will appear in crates.io)"
documentation = "https://docs.rs/tool-name/"
repository = "https://github.com/username/tool-name"
homepage = "https://github.com/username/tool-name"
readme = "README.md"
keywords = ["keyword1", "keyword2", "keyword3"]
categories = ["category1", "category2"]
license = "MIT OR Apache-2.0"
exclude = ["/.github", "/docs", "/examples", "/benches", "/tests"]

[features]
default = []

# Feature: Feature name
feature-name = ["dependency1"]
feature-two = ["dependency2"]

[dependencies]
# Core dependencies
dependency1 = { version = "1.0", optional = true }
dependency2 = { version = "2.0", optional = true }

# Async runtime
tokio = { version = "1.35", features = ["full"], optional = true }

# Error handling
thiserror = "1.0"
anyhow = "1.0"

[dev-dependencies]
# Testing
criterion = "0.5"  # Benchmarking
proptest = "1.4"   # Property testing
quickcheck = "1.0" # Property testing (alternative)

# Mocking
mockall = "0.12"

# Async testing
tokio-test = "0.4"

[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]

[[bench]]
name = "basic_benchmark"
harness = false

[[example]]
name = "hello_world"
path = "examples/hello_world.rs"

[[example]]
name = "quick_start"
path = "examples/basic/quick_start.rs"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true

[profile.bench]
inherits = "release"
debug = true
```

**Priority**: MUST-HAVE

**Key Elements**:
1. Complete metadata for crates.io
2. Feature flags for optional dependencies
3. Dev-dependencies separated
4. Docs.rs configuration
5. Release profile optimizations
6. Benchmark configuration

---

### 📝 CHANGELOG.md (MUST-HAVE)

**Purpose**: Track changes across versions for users and contributors.

**Template**:

```markdown
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- New feature 1
- New feature 2

### Changed
- Updated dependency X to version Y

### Deprecated
- Old function (will be removed in 0.2.0)

### Removed
- Removed deprecated feature

### Fixed
- Bug fix 1
- Bug fix 2

### Security
- Security fix 1

## [0.1.0] - YYYY-MM-DD

### Added
- Initial release
- Core functionality
- Basic documentation
```

**Priority**: MUST-HAVE

---

### 🤝 CONTRIBUTING.md (MUST-HAVE)

**Purpose**: Guide contributors on how to participate in the project.

**Template** (abbreviated - full version in repository):

```markdown
# Contributing to Tool Name

Thank you for your interest in contributing!

## Quick Start

```bash
git clone https://github.com/username/tool-name.git
cd tool-name
cargo build
cargo test
```

## Development Workflow

1. Read [Development Guide](docs/contributing/development.md)
2. Set up environment
3. Pick an issue
4. Fork and branch
5. Make changes
6. Add tests
7. Submit PR

## Code Standards

- **Formatting**: `cargo fmt`
- **Linting**: `cargo clippy -- -D warnings`
- **Testing**: `cargo test`
- **Documentation**: All public APIs documented

## Testing Guidelines

- Unit tests: `cargo test`
- Integration tests: `cargo test --test integration_test`
- Benchmarks: `cargo bench`

## Documentation Standards

- Code comments: Explain "why", not "what"
- Examples: Runnable, well-commented
- Guides: Clear, progressive

## Submitting Changes

1. Update CHANGELOG.md
2. Add tests
3. Update documentation
4. Run `cargo test && cargo clippy && cargo fmt`
5. Submit PR with template

## Community Guidelines

- Be respectful
- Welcome newcomers
- Assume good intent
- Focus on what is best for the community

## Getting Help

- [Documentation](docs/)
- [GitHub Discussions](https://github.com/username/tool-name/discussions)
- [GitHub Issues](https://github.com/username/tool-name/issues)
```

**Priority**: MUST-HAVE

---

### 🔧 .github/workflows/ci.yml (MUST-HAVE)

**Purpose**: Continuous integration testing across platforms.

**Template**:

```yaml
name: CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main, develop]

concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true

env:
  CARGO_TERM_COLOR: always
  RUSTFLAGS: -Dwarnings
  RUST_BACKTRACE: 1

jobs:
  test:
    name: Test (${{ matrix.os }}, ${{ matrix.rust }})
    runs-on: ${{ matrix.os }}
    strategy:
      fail-fast: false
      matrix:
        include:
          - os: ubuntu-latest
            rust: stable
          - os: macos-latest
            rust: stable
          - os: windows-latest
            rust: stable
          - os: ubuntu-latest
            rust: nightly

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: ${{ matrix.rust }}
          components: rustfmt, clippy

      - name: Cache dependencies
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ matrix.rust }}-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: |
            ${{ runner.os }}-cargo-${{ matrix.rust }}-
            ${{ runner.os }}-cargo-

      - name: Check formatting
        run: cargo fmt --all -- --check

      - name: Run clippy
        run: cargo clippy --all-targets --all-features -- -D warnings

      - name: Run tests
        run: cargo test --workspace --all-features --verbose

      - name: Run documentation tests
        run: cargo test --doc

      - name: Check all crates compile
        run: cargo check --all-targets

  coverage:
    name: Code Coverage
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: stable

      - name: Install tarpaulin
        run: cargo install cargo-tarpaulin

      - name: Generate coverage report
        run: cargo tarpaulin --workspace --all-features --verbose --timeout 120 --out Xml

      - name: Upload to codecov.io
        uses: codecov/codecov-action@v4
        with:
          fail_ci_if_error: true

  msrv:
    name: Check MSRV
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: "1.75.0"

      - name: Check compiles with MSRV
        run: cargo check
```

**Priority**: MUST-HAVE

---

### 🐛 .github/ISSUE_TEMPLATE/bug_report.md (MUST-HAVE)

**Purpose**: Standardize bug reports for reproducibility.

**Template**:

```markdown
---
name: Bug report
about: Create a report to help us improve
title: '[BUG] '
labels: bug
assignees: ''
---

## Bug Description

A clear and concise description of what the bug is.

## Steps to Reproduce

1. Go to '...'
2. Click on '....'
3. Scroll down to '....'
4. See error

**Code snippet or command:**
```rust
// Add code snippet here
```

## Expected Behavior

A clear and concise description of what you expected to happen.

## Actual Behavior

A clear and concise description of what actually happened.

**Error message:**
```
Paste error message here
```

## Environment

- **OS**: [e.g. Linux 5.15, macOS 13.0, Windows 11]
- **Architecture**: [e.g. x86_64, ARM64]
- **Rust Version**: [e.g. rustc 1.75.0]
- **Tool Version**: [e.g. v0.1.0]

## Additional Context

- **Related Issues**: [List related issue numbers]
- **Relevant Logs**: [Attach logs if applicable]
- **Screenshots**: [Add screenshots if applicable]

## Severity

- [ ] Critical - Blocks development/production
- [ ] High - Major functionality broken
- [ ] Medium - Workaround available
- [ ] Low - Minor inconvenience

---

**Thank you for reporting this bug! 🐛**
```

**Priority**: MUST-HAVE

---

### 🎯 .github/ISSUE_TEMPLATE/feature_request.md (MUST-HAVE)

**Template**:

```markdown
---
name: Feature request
about: Suggest an idea for this project
title: '[FEATURE] '
labels: enhancement
assignees: ''
---

## Feature Description

A clear and concise description of the feature you'd like to see.

## Problem Statement

What problem does this feature solve? What pain point does it address?

## Proposed Solution

How would you like this feature to work?

## Alternatives Considered

What alternative solutions have you considered? Why are they not suitable?

## Use Cases

Describe specific use cases for this feature:

1. Use case 1
2. Use case 2
3. Use case 3

## Additional Context

- **Related Issues**: [List related issue numbers]
- **Examples**: [Links to similar features in other projects]
- **Implementation Ideas**: [Any ideas on how to implement]

## Would You Be Willing to Contribute?

- [ ] Yes, I'd like to implement this feature
- [ ] Yes, but I'd need guidance
- [ ] No, I don't have time right now
- [ ] No, I don't have the technical skills

---

**Thank you for your suggestion! 💡**
```

**Priority**: MUST-HAVE

---

### 🔄 .github/PULL_REQUEST_TEMPLATE.md (MUST-HAVE)

**Template**:

```markdown
## Pull Request Description

**Summary:**
[2-3 sentence summary of changes]

**Related Issue:**
Fixes #[issue number]
Related to #[issue number]

## Type of Change

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update (changes only to documentation)
- [ ] Refactoring (no functional changes, code improvements)
- [ ] Performance improvement (no functional changes, better performance)
- [ ] Tests (adding or updating tests)
- [ ] Other: ________

## How Has This Been Tested?

- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing performed
- [ ] All existing tests pass: `cargo test`
- [ ] No compiler warnings: `cargo clippy -- -D warnings`

**Test Command:**
```bash
cargo test
```

**Test Results:**
```
test result: ok. X passed in Ys
```

## Checklist

- [ ] My code follows the style guidelines of this project (run `cargo fmt`)
- [ ] I have performed a self-review of my code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have made corresponding changes to the documentation
- [ ] I have updated the CHANGELOG.md (if applicable)
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] New and existing unit tests pass locally with my changes
- [ ] Any dependent changes have been merged and published in downstream modules

## Breaking Changes

If this PR introduces breaking changes, please describe them here:

**What changes?**

**Migration path:**

**Why is this necessary?**

## Additional Context

- **Performance Impact**: [Describe any performance improvements or regressions]
- **Documentation**: [Link to updated documentation]
- **Known Limitations**: [Any known limitations or future work]

---

**Thank you for your contribution! 🎉**
```

**Priority**: MUST-HAVE

---

### 📖 examples/README.md (MUST-HAVE)

**Purpose**: Guide users through examples progressively.

**Template**:

```markdown
# Tool Name - Code Examples

This directory contains runnable code examples demonstrating various aspects of Tool Name.

## Running Examples

### Run from Source

```bash
# From repository root
cargo run --example hello_world

# Or navigate and run
cd examples
cargo run --example hello_world
```

### Run All Examples

```bash
# Test all examples compile and run
cargo test --examples
```

## Examples by Category

### 📦 Basic ([`basic/`](basic/))

Simple examples to get you started:

| Example | Description |
|---------|-------------|
| [`hello_world.rs`](hello_world.rs) | Run your first example |
| [`quick_start.rs`](basic/quick_start.rs) | Quick start guide |
| [`configuration.rs`](basic/configuration.rs) | Configuration options |

**Prerequisites**: None

### 🚀 Intermediate ([`intermediate/`](intermediate/))

More advanced usage patterns:

| Example | Description |
|---------|-------------|
| [`custom_types.rs`](intermediate/custom_types.rs) | Working with custom types |
| [`error_handling.rs`](intermediate/error_handling.rs) | Error handling patterns |

**Prerequisites**: Complete basic examples

### 🔥 Advanced ([`advanced/`](advanced/))

Advanced usage and integration:

| Example | Description |
|---------|-------------|
| [`integration.rs`](advanced/integration.rs) | Integration patterns |
| [`performance.rs`](advanced/performance.rs) | Performance optimization |

**Prerequisites**: Complete intermediate examples

## Example Standards

All examples follow these standards:

### ✅ Characteristics

- **Runnable**: Every example compiles and runs
- **Self-contained**: Each example is complete and independent
- **Well-commented**: Code explains what's happening
- **Best practices**: Demonstrates recommended patterns
- **Error handling**: Shows proper error handling

### 📝 Structure

Each example includes:

1. **File header**: Description and prerequisites
2. **Imports**: Clear and organized
3. **Main logic**: Well-commented code
4. **Output**: Shows expected results
5. **Related docs**: Links to documentation

### Example Template

```rust
//! Example Name - Brief description
//!
//! This example demonstrates [what it does].
//!
//! # Prerequisites
//!
//! - [Requirement 1]
//! - [Requirement 2]
//!
//! # Related Documentation
//!
//! - [Link to relevant docs]
//!
//! # Expected Output
//!
//! ```text
//! [Show what the output looks like]
//! ```

use tool_name::Tool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize
    let tool = Tool::new();

    // Do something
    let result = tool.do_something()?;
    println!("{:?}", result);

    Ok(())
}
```

## Contributing Examples

Want to add an example? Great!

### Guidelines

1. **Keep it simple**: Focus on one concept per example
2. **Make it runnable**: Test before submitting
3. **Add comments**: Explain the "why", not just the "what"
4. **Update docs**: Link from relevant documentation
5. **Follow template**: Use the example template above

### Adding a New Example

1. Create the example file in appropriate directory
2. Add it to this README's table of contents
3. Test it works: `cargo run --example your_example`
4. Add documentation links
5. Submit PR

## Need Help?

- Example not working? Check [Troubleshooting](../docs/reference/troubleshooting.md)
- Want to understand more? Read [Tutorials](../docs/tutorials/)
- Found a bug? [Open an issue](https://github.com/username/tool-name/issues)

---

**Examples Version**: 0.1.0
**Last Updated**: YYYY-MM-DD
```

**Priority**: MUST-HAVE

---

### 📊 benches/README.md (NICE-TO-HAVE)

**Purpose**: Document benchmarking approach and results.

**Template**:

```markdown
# Benchmarks

This directory contains performance benchmarks for Tool Name.

## Running Benchmarks

### Run All Benchmarks

```bash
cargo bench
```

### Run Specific Benchmark

```bash
cargo bench --bench basic_benchmark
```

### Run with Specific Filter

```bash
cargo bench --bench basic_benchmark -- 'benchmark_name'
```

## Benchmark Descriptions

| Benchmark | Description | Command |
|-----------|-------------|---------|
| [`basic_benchmark`](basic_benchmark.rs) | Basic performance tests | `cargo bench --bench basic_benchmark` |
| [`comparison_benchmark`](comparison_benchmark.rs) | Compare with alternatives | `cargo bench --bench comparison_benchmark` |

## Benchmark Results

Results from CI runs:

| Date | Hardware | Result |
|------|----------|--------|
| YYYY-MM-DD | Intel i7-12700K, 32GB RAM | [Link] |

## Writing Benchmarks

### Template

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

fn benchmark_function(c: &mut Criterion) {
    let mut group = c.benchmark_group("my_group");

    for size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                // Code to benchmark
                black_box(function_to_test(size))
            })
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_function);
criterion_main!(benches);
```

## Guidelines

1. **Use Criterion**: Provides statistical analysis
2. **Black box inputs**: Prevents compiler optimizations
3. **Multiple sizes**: Test different input sizes
4. **Clear names**: Describe what's being tested
5. **Document**: Comment complex benchmarks

## CI Integration

Benchmarks run on main branch pushes. Results are tracked over time.

---

**Benchmarks Version**: 0.1.0
**Last Updated**: YYYY-MM-DD
```

**Priority**: NICE-TO-HAVE (but recommended for performance-critical crates)

---

### 📚 docs/README.md (MUST-HAVE)

**Purpose**: Central hub for all documentation.

**Template**:

```markdown
# Tool Name Documentation

Welcome to the official documentation for Tool Name!

## Quick Navigation

### For Users

- **[Getting Started](tutorials/getting_started.md)** - Installation and first steps
- **[Tutorials](tutorials/)** - Progressive tutorials
- **[API Reference](https://docs.rs/tool-name/)** - Complete API reference
- **[FAQ](reference/faq.md)** - Frequently asked questions
- **[Glossary](reference/glossary.md)** - Terminology and concepts

### For Developers

- **[Contributing](../CONTRIBUTING.md)** - Contribution guide
- **[Architecture](guides/architecture.md)** - System design
- **[Examples](../examples/)** - Runnable code examples
- **[Benchmarks](../benches/)** - Performance data
- **[Internals](internals/)** - Implementation details

### For Maintainers

- **[Release Checklist](../RELEASE_CHECKLIST.md)** - Release process
- **[Governance](governance.md)** - Project governance

## Documentation Structure

```
docs/
├── tutorials/        # Step-by-step guides for users
│   ├── getting_started.md
│   └── common_patterns.md
├── guides/           # In-depth guides
│   ├── architecture.md
│   └── api_reference.md
├── reference/        # Reference material
│   ├── faq.md
│   └── glossary.md
└── internals/        # Implementation details
    ├── design_decisions.md
    └── performance.md
```

## Reading Path

### New Users

1. Start with [Getting Started](tutorials/getting_started.md)
2. Try the [Examples](../examples/)
3. Read [API Reference](https://docs.rs/tool-name/)

### Contributors

1. Read [Contributing Guide](../CONTRIBUTING.md)
2. Study [Architecture](guides/architecture.md)
3. Review [Examples](../examples/)

## Documentation Standards

### ✅ Standards

- **Clear**: Simple language, avoid jargon
- **Complete**: Cover all aspects
- **Current**: Keep up-to-date with code
- **Tested**: All examples are runnable
- **Linked**: Cross-reference related docs

### 📝 Writing Guide

1. **Audience**: Know who you're writing for
2. **Structure**: Use clear headings
3. **Examples**: Provide code snippets
4. **Links**: Connect related concepts
5. **Review**: Have others review before publishing

## Contributing Documentation

We welcome documentation improvements!

### Quick Start

1. Edit the relevant markdown file
2. Test any code examples: `cargo test --doc`
3. Build docs: `cargo doc --no-deps --open`
4. Submit PR

### Documentation PR Checklist

- [ ] All examples compile and run
- [ ] Cross-references work
- [ ] Links are valid
- [ ] Spelling and grammar checked
- [ ] Added to relevant index

## Need Help?

- Question? [Open a discussion](https://github.com/username/tool-name/discussions)
- Documentation issue? [Open a documentation issue](https://github.com/username/tool-name/issues/new?template=documentation.md)
- Want to contribute? See [Contributing Guide](../CONTRIBUTING.md)

---

**Documentation Version**: 0.1.0
**Last Updated**: YYYY-MM-DD
```

**Priority**: MUST-HAVE

---

### 📖 docs/tutorials/getting_started.md (MUST-HAVE)

**Purpose**: Get users from zero to working in minimal time.

**Template**:

```markdown
# Getting Started with Tool Name

Welcome! This tutorial will get you up and running with Tool Name in under 10 minutes.

## Prerequisites

- **Rust** 1.75+ ([install](https://rustup.rs/))
- Basic understanding of Rust

## Installation

### Option 1: Add to Your Project

Add to `Cargo.toml`:

```toml
[dependencies]
tool-name = "0.1.0"
```

### Option 2: Install Binary

```bash
cargo install tool-name
```

## Your First Example

Create `src/main.rs`:

```rust
use tool_name::Tool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new instance
    let tool = Tool::new();

    // Use the tool
    let result = tool.do_something("input")?;
    println!("Result: {}", result);

    Ok(())
}
```

Run it:

```bash
cargo run
```

**Output**:
```
Result: Your output here
```

## Next Steps

### Learn More

- **[Common Patterns](common_patterns.md)** - Typical usage patterns
- **[API Reference](https://docs.rs/tool-name/)** - Complete API
- **[Examples](../../examples/)** - More examples

### Try These Examples

```bash
# Run hello world
cargo run --example hello_world

# Run quick start
cargo run --example quick_start

# Run configuration example
cargo run --example configuration
```

## Troubleshooting

### Problem: Compilation Errors

**Solution**: Ensure you're using Rust 1.75+:

```bash
rustc --version
```

### Problem: Runtime Errors

**Solution**: Check the [FAQ](../reference/faq.md) or [open an issue](https://github.com/username/tool-name/issues).

## Getting Help

- **[FAQ](../reference/faq.md)** - Common questions
- **[GitHub Discussions](https://github.com/username/tool-name/discussions)** - Ask questions
- **[GitHub Issues](https://github.com/username/tool-name/issues)** - Report bugs

---

**Tutorial Version**: 0.1.0
**Last Updated**: YYYY-MM-DD
```

**Priority**: MUST-HAVE

---

### ⚙️ rust-toolchain.toml (MUST-HAVE)

**Purpose**: Ensure consistent Rust version across environments.

**Template**:

```toml
[toolchain]
channel = "1.75"
components = ["rustfmt", "clippy"]
profile = "minimal"
```

**Priority**: MUST-HAVE

---

### 🎨 rustfmt.toml (MUST-HAVE)

**Purpose**: Consistent code formatting.

**Template**:

```toml
max_width = 100
hard_tabs = false
tab_spaces = 4
edition = "2021"
merge_derives = true
use_try_shorthand = true
use_field_init_shorthand = true
```

**Priority**: MUST-HAVE

---

### 📄 LICENSE-MIT & LICENSE-APACHE (MUST-HAVE)

**Purpose**: Legal licensing for open source distribution.

**Templates**: Use standard SPDX license templates.

**Priority**: MUST-HAVE

---

### 🚀 RELEASE_CHECKLIST.md (MICE-TO-HAVE)

**Purpose**: Ensure consistent, complete releases.

**Template**:

```markdown
# Release Checklist

Use this checklist when preparing a new release.

## Pre-Release

### Code Quality

- [ ] All tests pass: `cargo test --workspace --all-features`
- [ ] No compiler warnings: `cargo clippy --all-targets -- -D warnings`
- [ ] Code formatted: `cargo fmt --all -- --check`
- [ ] Documentation builds: `cargo doc --no-deps`
- [ ] No documentation warnings: `cargo doc --no-deps 2>&1 | grep warning`

### Documentation

- [ ] CHANGELOG.md updated with all changes
- [ ] README.md version updated
- [ ] Cargo.toml version updated
- [ ] All new APIs documented
- [ ] Examples tested and working
- [ ] Migration guide added (if breaking changes)

### Testing

- [ ] Unit tests pass on all platforms (Linux, macOS, Windows)
- [ ] Integration tests pass
- [ ] Benchmarks run successfully
- [ ] Manual testing completed
- [ ] Edge cases tested

### Security

- [ ] No vulnerabilities in dependencies: `cargo audit`
- [ ] Secrets not included in repository
- [ ] License compliance checked

## Release Process

### Create Release Branch

```bash
git checkout main
git pull
git checkout -b release/v0.X.0
```

### Update Version

```bash
# Update version in Cargo.toml
# Update version in README.md
# Update CHANGELOG.md

git add .
git commit -m "Prepare release v0.X.0"
```

### Tag and Push

```bash
git tag -a v0.X.0 -m "Release v0.X.0"
git push origin release/v0.X.0
git push origin v0.X.0
```

### Publish to Crates.io

```bash
# Login (first time only)
cargo login

# Publish
cargo publish --dry-run
# Check output, then:
cargo publish
```

### Create GitHub Release

1. Go to https://github.com/username/tool-name/releases/new
2. Tag: `v0.X.0`
3. Title: `Version 0.X.0`
4. Description: Copy from CHANGELOG.md
5. Attach binaries (if applicable)
6. Publish release

## Post-Release

### Announcements

- [ ] GitHub release published
- [ ] Reddit post (r/rust)
- [ ] Twitter post
- [ ] Discord/Slack announcement
- [ ] Blog post (for major releases)

### Cleanup

- [ ] Merge release branch back to main
- [ ] Delete release branch
- [ ] Update website (if applicable)
- [ ] Update documentation links
- [ ] Close related issues

### Metrics

- [ ] Track download count
- [ ] Monitor issues and PRs
- [ ] Gather user feedback

## Verification

### Smoke Test

```bash
# Create new project
cargo new test-tool-name
cd test-tool-name

# Add dependency
echo 'tool-name = "0.X.0"' >> Cargo.toml

# Test it works
echo 'use tool_name::Tool; fn main() { println!("OK"); }' > src/main.rs
cargo run
```

### Documentation Check

```bash
# Verify docs.rs builds
cargo doc --no-deps --all-features

# Check examples work
cargo run --example hello_world
cargo run --example quick_start
```

## Rollback Plan (If Issues Found)

1. **Yank from crates.io**: `cargo yank --vers 0.X.0`
2. **Update GitHub release**: Mark as "Yanked"
3. **Announce**: Post announcement about issues
4. **Fix**: Create patch release (0.X.1)

---

**Release Checklist Version**: 0.1.0
**Last Updated**: YYYY-MM-DD
```

**Priority**: NICE-TO-HAVE (but recommended for production crates)

---

## Priority Levels

### MUST-HAVE (Essential for Production)

1. ✅ **README.md** - User conversion
2. ✅ **Cargo.toml** - Package manifest
3. ✅ **CHANGELOG.md** - Version tracking
4. ✅ **CONTRIBUTING.md** - Contributor onboarding
5. ✅ **.github/workflows/ci.yml** - Continuous integration
6. ✅ **.github/ISSUE_TEMPLATE/** (bug_report.md, feature_request.md) - Issue standardization
7. ✅ **.github/PULL_REQUEST_TEMPLATE.md** - PR standardization
8. ✅ **examples/README.md** - Example guide
9. ✅ **examples/hello_world.rs** - Minimal working example
10. ✅ **docs/README.md** - Documentation hub
11. ✅ **docs/tutorials/getting_started.md** - User onboarding
12. ✅ **rust-toolchain.toml** - Version consistency
13. ✅ **rustfmt.toml** - Code formatting
14. ✅ **LICENSE-MIT** & **LICENSE-APACHE** - Legal licensing

### NICE-TO-HAVE (Recommended for Professional Crates)

1. 📊 **benches/README.md** + **benches/basic_benchmark.rs** - Performance tracking
2. 📄 **RELEASE_CHECKLIST.md** - Release process
3. 🔐 **.github/workflows/security.yml** - Security scanning
4. 📚 **.github/workflows/documentation.yml** - Documentation testing
5. 📦 **.github/dependabot.yml** - Dependency updates
6. 👥 **.github/CODEOWNERS** - Code ownership
7. 📖 **docs/tutorials/common_patterns.md** - Advanced usage
8. 🏗️ **docs/guides/architecture.md** - System design
9. ❓ **docs/reference/faq.md** - Common questions
10. 🔤 **docs/reference/glossary.md** - Terminology

### EXTRAS (Polish for Mature Projects)

1. 🎨 **branding/** - Logos, colors, visual identity
2. 📊 **.github/workflows/benchmarks.yml** - Benchmark CI
3. 📈 **.github/workflows/release.yml** - Automated releases
4. 📝 **examples/advanced/** - Complex examples
5. 🧪 **examples/integration/** - Integration examples
6. 📚 **docs/internals/** - Implementation details
7. 🏛️ **docs/governance.md** - Project governance

---

## Usage Instructions

### Quick Start (Copy-Paste)

```bash
# 1. Create new crate
cargo new tool-name --lib
cd tool-name

# 2. Copy template files
# (Copy all files from this template)

# 3. Customize
# - Replace "tool-name" with your actual name
# - Update descriptions
# - Set your own GitHub username

# 4. Test
cargo build
cargo test
cargo fmt
cargo clippy -- -D warnings

# 5. Commit and push
git init
git add .
git commit -m "Initial commit from template"
git branch -M main
git remote add origin https://github.com/username/tool-name.git
git push -u origin main
```

### Customization Checklist

- [ ] Replace all instances of `tool-name` with actual name
- [ ] Replace `username` with your GitHub username
- [ ] Update descriptions in README.md
- [ ] Update keywords and categories in Cargo.toml
- [ ] Set appropriate Rust version
- [ ] Customize CI/CD workflows
- [ ] Add your own examples
- [ ] Write your own tutorials
- [ ] Create custom issue templates (if needed)
- [ ] Add branding assets (logo, colors)

---

## Best Practices

### 1. Progressive Learning Path

```
README.md (10 seconds)
    ↓
examples/hello_world.rs (1 minute)
    ↓
docs/tutorials/getting_started.md (10 minutes)
    ↓
examples/basic/* (30 minutes)
    ↓
docs/tutorials/common_patterns.md (1 hour)
    ↓
API Reference (ongoing)
```

### 2. Documentation Pyramid

```
README.md (1 page - summary)
    ↓
docs/tutorials/ (10 pages - learning)
    ↓
docs/guides/ (50 pages - understanding)
    ↓
docs/reference/ (100+ pages - reference)
    ↓
API Docs (auto-generated - complete)
```

### 3. Testing Pyramid

```
Unit tests (70% - fast, focused)
    ↓
Integration tests (20% - slower, comprehensive)
    ↓
Benchmarks (10% - performance validation)
```

### 4. CI/CD Best Practices

- **Test matrix**: Multiple OS versions, Rust stable/nightly
- **Fail fast**: Cancel outdated runs
- **Cache aggressively**: Speed up builds
- **Separate jobs**: Test, coverage, security, benchmarks
- **Clear status**: Easy to see what failed

### 5. Issue Tracking Best Practices

- **Standard templates**: Bug, feature, question
- **Required fields**: Environment, reproduction steps
- **Labels**: Bug, enhancement, documentation, help wanted
- **Severity levels**: Critical, high, medium, low

### 6. Pull Request Best Practices

- **Template**: All PRs follow same structure
- **Checklist**: Ensures quality
- **Linked issues**: Every PR references issue
- **CHANGELOG**: Update for user-facing changes
- **Tests**: All changes tested

### 7. Release Best Practices

- **SemVer**: Follow semantic versioning
- **CHANGELOG**: Detailed change list
- **Tagged commits**: Every release tagged
- **Release notes**: Human-readable summary
- **Rollback plan**: Prepare for issues

### 8. Community Best Practices

- **Code of Conduct**: Clear behavior expectations
- **Contributing guide**: Easy to contribute
- **Responsive**: Address issues promptly
- **Welcoming**: Help newcomers
- **Documented**: All processes written down

---

## Sources and Inspiration

This template was created by analyzing three world-class Rust repositories:

1. **[huggingface/candle](https://github.com/huggingface/candle)**
   - Excellent examples organization (candle-examples/)
   - Progressive learning path (simple → complex)
   - Clear feature-based documentation

2. **[burn-rs/burn](https://github.com/burn-rs/burn)**
   - Clean workspace structure
   - Comprehensive benchmarking (Criterion-based)
   - Strong visual branding

3. **[rust-lang/cargo](https://github.com/rust-lang/cargo)**
   - Professional CI/CD (multi-platform, extensive checks)
   - Comprehensive contribution guidelines
   - Clear documentation hierarchy

And incorporating best practices from SuperInstance AI:

- Issue/PR templates
- Progressive examples
- Comprehensive docs structure
- Release checklist

---

## Support

For questions or issues with this template:

1. Check the [Best Practices](#best-practices) section
2. Review the source repositories (Candle, Burn, Cargo)
3. Open an issue on the SuperInstance repository

---

**Template Version**: 1.0.0
**Last Updated**: 2026-01-08
**Maintained By**: SuperInstance AI Team
**License**: MIT OR Apache-2.0
