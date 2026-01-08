# Example: Creating a Standalone Crate

> **Practical example using the template**

This example shows how to extract a hypothetical `synesis-privacy-proxy` crate from the SuperInstance project using the template.

---

## Step 1: Plan the Crate

**Crate Name**: `privacy-proxy`

**Purpose**: Standalone privacy proxy with redaction and tokenization

**Key Features**:
- Pattern-based redaction
- Token vault
- Re-inflation
- Custom patterns

**Repository**: https://github.com/SuperInstance/privacy-proxy

---

## Step 2: Create Crate Structure

```bash
# 1. Create new crate
cargo new privacy-proxy --lib
cd privacy-proxy

# 2. Create directories
mkdir -p .github/workflows .github/ISSUE_TEMPLATE \
    examples/basic examples/intermediate examples/advanced \
    benches docs/tutorials docs/guides docs/reference \
    tests

# 3. Verify structure
tree -L 2
```

**Output**:
```
privacy-proxy/
├── .github/
│   ├── ISSUE_TEMPLATE/
│   └── workflows/
├── benches/
├── docs/
│   ├── guides/
│   ├── reference/
│   └── tutorials/
├── examples/
│   ├── advanced/
│   ├── basic/
│   └── intermediate/
├── src/
└── tests/
```

---

## Step 3: Create Essential Files

### README.md

```markdown
# Privacy Proxy

> **Privacy-first redaction and tokenization for Rust**

[![CI](https://github.com/SuperInstance/privacy-proxy/actions/workflows/ci.yml/badge.svg)](https://github.com/SuperInstance/privacy-proxy/actions/workflows/ci.yml)
[![Documentation](https://docs.rs/privacy-proxy/badge.svg)](https://docs.rs/privacy-proxy/)
[![Crates.io](https://img.shields.io/crates/v/privacy-proxy.svg)](https://crates.io/crates/privacy-proxy)
[![License](https://img.shields.io/badge/license-MIT%20%7C%20Apache--2.0-blue.svg)](LICENSE-APACHE)

## 🎯 What is Privacy Proxy?

**Privacy Proxy** is a Rust library that automatically redacts sensitive information before it leaves your system and re-inflates it locally when responses return.

- ✅ **18 built-in patterns** - Emails, API keys, SSNs, credit cards, and more
- ✅ **Token vault** - SQLite-based local storage
- ✅ **Re-inflation** - Automatic restoration on your device
- ✅ **Custom patterns** - Add your own redaction rules
- ✅ **Zero dependencies** - (besides SQLite)

## 🚀 Quick Start

```bash
cargo add privacy-proxy
```

```rust
use privacy_proxy::{PrivacyProxy, RedactionPattern};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create proxy with default patterns
    let proxy = PrivacyProxy::new().await?;

    // Redact sensitive data
    let input = "Contact me at user@example.com";
    let redacted = proxy.redact(input).await?;
    println!("Redacted: {}", redacted);
    // Output: Contact me at [EMAIL_01]

    // Re-inflate locally
    let restored = proxy.reinflate(&redacted).await?;
    println!("Restored: {}", restored);
    // Output: Contact me at user@example.com

    Ok(())
}
```

## 📚 Documentation

- **[Getting Started](docs/tutorials/getting_started.md)** - Installation and first steps
- **[Examples](examples/)** - Runnable code examples
- **[API Reference](https://docs.rs/privacy-proxy/)** - Complete API documentation
- **[Contributing](CONTRIBUTING.md)** - Development guide

## 💡 Use Cases

### Cloud Escalation

Redact before sending to cloud:

```rust
let proxy = PrivacyProxy::new().await?;
let query = "My API key is sk-1234567890abcdef";
let redacted = proxy.redact(query).await?;

// Send redacted to cloud
send_to_cloud(&redacted).await?;

// Re-inflate response locally
let response = receive_from_cloud().await?;
let restored = proxy.reinflate(&response).await?;
```

### Logging

Sanitize logs automatically:

```rust
let proxy = PrivacyProxy::new().await?;
let log_entry = "User login: john@example.com";
let sanitized = proxy.redact(log_entry).await?;
log::info!("{}", sanitized); // User login: [EMAIL_01]
```

### API Responses

Filter sensitive data in API responses:

```rust
let proxy = PrivacyProxy::new().await?;
let api_response = r#"{"email": "user@example.com", "name": "John"}"#;
let filtered = proxy.redact(api_response).await?;
// {"email": "[EMAIL_01]", "name": "John"}
```

## 🏗️ Architecture

```text
┌─────────────────┐
│  Input Data     │
└────────┬────────┘
         │
    ┌────▼────┐
    │ Redact  │ ← Pattern matching
    └────┬────┘
         │
    ┌────▼────┐
    │  Token  │ ← UUID replacement
    │  Vault  │ ← SQLite storage
    └────┬────┘
         │
    ┌────▼────┐
    │ Re-inflate │ ← Local restoration
    └─────────┘
```

**Learn More**: [Architecture Guide](docs/guides/architecture.md)

## 📊 Performance

| Operation | Throughput | Latency |
|-----------|------------|---------|
| Redaction | 1M+ ops/sec | <1μs |
| Re-inflation | 2M+ ops/sec | <0.5μs |
| Token vault | 100K ops/sec | <10μs |

*Benchmarks on: Intel i7-12700K, 32GB RAM, NVMe SSD*

## 🔧 System Requirements

- **Rust**: 1.75+
- **OS**: Linux, macOS, Windows
- **Dependencies**: SQLite 3.35+

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

### Quick Start

```bash
git clone https://github.com/SuperInstance/privacy-proxy.git
cd privacy-proxy
cargo test
cargo run --example hello_world
```

## 📝 License

Licensed under either of:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT))
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

at your option.

## 🙏 Acknowledgments

Built with amazing projects:
- [regex](https://docs.rs/regex/) - Pattern matching
- [rusqlite](https://docs.rs/rusqlite/) - SQLite storage
- [uuid](https://docs.rs/uuid/) - Unique tokens

## 📞 Support

- **[Documentation](docs/)** - Start here
- **[FAQ](docs/reference/faq.md)** - Common questions
- **[GitHub Issues](https://github.com/SuperInstance/privacy-proxy/issues)** - Report bugs
- **[GitHub Discussions](https://github.com/SuperInstance/privacy-proxy/discussions)** - Ask questions

---

**Privacy Proxy** - *Your secrets stay yours.*

**Version**: 0.1.0 | **Status**: Alpha

*Last Updated: 2026-01-08*
```

### Cargo.toml

```toml
[package]
name = "privacy-proxy"
version = "0.1.0"
edition = "2021"
rust-version = "1.75"
authors = ["SuperInstance AI Team"]
description = "Privacy-first redaction and tokenization for Rust"
documentation = "https://docs.rs/privacy-proxy/"
repository = "https://github.com/SuperInstance/privacy-proxy"
homepage = "https://github.com/SuperInstance/privacy-proxy"
readme = "README.md"
keywords = ["privacy", "redaction", "tokenization", "security", "pii"]
categories = ["cryptography", "authentication", "data-structures"]
license = "MIT OR Apache-2.0"
exclude = ["/.github", "/docs", "/examples", "/benches", "/tests"]

[features]
default = ["sqlite"]
sqlite = ["rusqlite"]
custom-patterns = []

[dependencies]
# Core dependencies
regex = "1.10"
uuid = { version = "1.6", features = ["v4", "serde"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Optional SQLite support
rusqlite = { version = "0.30", optional = true, features = ["bundled"] }

# Async runtime
tokio = { version = "1.35", features = ["full"], optional = true }

# Error handling
thiserror = "1.0"
anyhow = "1.0"

[dev-dependencies]
# Testing
criterion = "0.5"
tokio-test = "0.4"
tempfile = "3.8"

[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]

[[bench]]
name = "redaction_benchmark"
harness = false

[[example]]
name = "hello_world"
path = "examples/hello_world.rs"

[[example]]
name = "custom_patterns"
path = "examples/basic/custom_patterns.rs"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true

[profile.bench]
inherits = "release"
debug = true
```

### CHANGELOG.md

```markdown
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial development

## [0.1.0] - 2026-01-08

### Added
- Initial release
- 18 built-in redaction patterns
- SQLite token vault
- Re-inflation functionality
- Custom pattern support
- Comprehensive documentation
- 100+ tests
```

### src/lib.rs

```rust
//! Privacy Proxy - Privacy-first redaction and tokenization
//!
//! This library provides privacy protection through automatic redaction
//! and tokenization of sensitive information.
//!
//! # Quick Start
//!
//! ```rust
//! use privacy_proxy::PrivacyProxy;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let proxy = PrivacyProxy::new().await?;
//! let redacted = proxy.redact("Contact user@example.com").await?;
//! println!("{}", redacted); // "Contact [EMAIL_01]"
//! # Ok(())
//! # }
//! ```
//!
//! # Features
//!
//! - **18 built-in patterns**: Emails, API keys, SSNs, credit cards, etc.
//! - **Token vault**: SQLite-based local storage
//! - **Re-inflation**: Automatic local restoration
//! - **Custom patterns**: Add your own rules
//!
//! # Architecture
//!
//! The library uses a three-step process:
//!
//! 1. **Redaction**: Replace sensitive data with UUID tokens
//! 2. **Storage**: Store mappings in local SQLite vault
//! 3. **Re-inflation**: Restore tokens to original values locally
//!
//! # Examples
//!
//! See the [examples](https://github.com/SuperInstance/privacy-proxy/tree/main/examples)
//! directory for complete examples.

pub mod error;
pub mod pattern;
pub mod proxy;
pub mod vault;

// Re-exports for convenience
pub use error::{PrivacyError, PrivacyResult};
pub use pattern::RedactionPattern;
pub use proxy::PrivacyProxy;
pub use vault::TokenVault;

/// Current version of the library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_basic_redaction() {
        let proxy = PrivacyProxy::new().await.unwrap();
        let input = "Contact user@example.com";
        let redacted = proxy.redact(input).await.unwrap();
        assert!(redacted.contains("[EMAIL_"));
    }
}
```

---

## Step 4: Create GitHub Configuration

### .github/workflows/ci.yml

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
```

### .github/ISSUE_TEMPLATE/bug_report.md

*(Use template from main template, customize for privacy-proxy)*

---

## Step 5: Create Examples

### examples/hello_world.rs

```rust
//! Hello World - Your first redaction
//!
//! # Expected Output
//! ```text
//! Original: Contact me at user@example.com
//! Redacted: Contact me at [EMAIL_01]
//! Restored: Contact me at user@example.com
//! ```

use privacy_proxy::PrivacyProxy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new privacy proxy
    let proxy = PrivacyProxy::new().await?;

    // Original text with email
    let original = "Contact me at user@example.com";
    println!("Original: {}", original);

    // Redact the email
    let redacted = proxy.redact(original).await?;
    println!("Redacted: {}", redacted);

    // Re-inflate (restore) the email
    let restored = proxy.reinflate(&redacted).await?;
    println!("Restored: {}", restored);

    Ok(())
}
```

### examples/basic/custom_patterns.rs

```rust
//! Custom Patterns - Add your own redaction rules
//!
//! This example shows how to add custom redaction patterns.

use privacy_proxy::{PrivacyProxy, RedactionPattern};
use regex::Regex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create proxy
    let mut proxy = PrivacyProxy::new().await?;

    // Add custom pattern for internal IDs
    let id_pattern = RedactionPattern {
        name: "INTERNAL_ID".to_string(),
        pattern: Regex::new(r"ID-\d{4}")?,
        description: "Internal ID format".to_string(),
    };

    proxy.add_pattern(id_pattern);

    // Test custom pattern
    let text = "User ID-1234 accessed the system";
    let redacted = proxy.redact(text).await?;
    println!("Original: {}", text);
    println!("Redacted: {}", redacted);
    // Output: User [INTERNAL_ID_01] accessed the system

    Ok(())
}
```

---

## Step 6: Create Documentation

### docs/tutorials/getting_started.md

```markdown
# Getting Started with Privacy Proxy

This tutorial will get you up and running in under 10 minutes.

## Prerequisites

- **Rust** 1.75+ ([install](https://rustup.rs/))

## Installation

```bash
cargo add privacy-proxy
```

## Your First Redaction

Create `src/main.rs`:

```rust
use privacy_proxy::PrivacyProxy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proxy = PrivacyProxy::new().await?;

    let input = "My email is user@example.com";
    let redacted = proxy.redact(input).await?;

    println!("{}", redacted);
    Ok(())
}
```

Run it:

```bash
cargo run
```

**Output**:
```
My email is [EMAIL_01]
```

## Built-in Patterns

The library includes 18 built-in patterns:

| Pattern | Example |
|---------|---------|
| EMAIL | user@example.com |
| API_KEY_GITHUB | ghp_xxx |
| API_KEY_AWS | AKIAxxx |
| SSN | 123-45-6789 |
| CREDIT_CARD | 4111-1111-1111-1111 |
| ... | ... |

## Next Steps

- **[Custom Patterns](custom_patterns.md)** - Add your own patterns
- **[Examples](../../examples/)** - More examples
- **[API Reference](https://docs.rs/privacy-proxy/)** - Complete API

---

**Tutorial Version**: 0.1.0
**Last Updated**: 2026-01-08
```

---

## Step 7: Test Everything

```bash
# Build
cargo build

# Test
cargo test

# Examples
cargo run --example hello_world
cargo run --example custom_patterns

# Format
cargo fmt

# Lint
cargo clippy --all-targets -- -D warnings

# Documentation
cargo doc --no-deps --open
```

---

## Step 8: Push to GitHub

```bash
git init
git add .
git commit -m "Initial commit: Privacy Proxy v0.1.0"

git remote add origin https://github.com/SuperInstance/privacy-proxy.git
git branch -M main
git push -u origin main
```

---

## Step 9: Publish to Crates.io

```bash
# Login (first time only)
cargo login

# Dry run
cargo publish --dry-run

# Publish
cargo publish
```

---

## Result

✅ **Production-ready crate** in under 2 hours with:
- Complete documentation
- CI/CD pipeline
- Working examples
- Professional structure

**Time breakdown**:
- Planning: 10 minutes
- Structure: 10 minutes
- Essential files: 45 minutes
- Examples: 30 minutes
- Documentation: 20 minutes
- Testing: 15 minutes
- GitHub setup: 10 minutes
- Publishing: 10 minutes

**Total**: ~2 hours 30 minutes

---

**Example Version**: 1.0.0
**Last Updated**: 2026-01-08
