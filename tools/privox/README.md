# Privox

> **Privacy that doesn't leak. Data that doesn't leave.**

Privox is a blazingly fast, zero-dependency privacy proxy and redaction engine for Rust. Detect sensitive data, redact it with reversible tokens, and ensure your users' secrets never leave their machine.

**Status**: 🚧 Under Extraction from [SuperInstance](https://github.com/SuperInstance/Tripartite1)

## Quick Start

```rust
use privox::{Redactor, RedactorConfig, TokenVault};

# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
let vault = TokenVault::in_memory()?;
let mut redactor = Redactor::new(RedactorConfig::default(), vault)?;
let result = redactor.redact("Contact john@example.com", "session-1").await?;

assert!(result.redacted_text.contains("[EMAIL_"));
# Ok(())
# }
```

## Features

- ✅ 18+ built-in pattern detection (emails, API keys, SSNs, credit cards)
- ✅ Reversible redaction with token-based substitution
- ✅ SQLite token vault (in-memory and persistent)
- ✅ Thread-safe operations
- ✅ Constant-time token lookups (timing-attack resistant)
- ✅ Session-based token isolation
- ✅ Custom pattern support
- ✅ Comprehensive test suite (37 tests)

## Installation

```bash
cargo add privox
```

## Testing

This crate is currently being extracted from `synesis-privacy`. Run tests with:

```bash
cargo test --package synesis-privacy
```

## Documentation

Full documentation coming soon after extraction is complete.

## License

MIT OR Apache-2.0
