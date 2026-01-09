//! Privox - Privacy Proxy and Redaction Engine
//!
//! This crate is being extracted from synesis-privacy. For now, it re-exports
//! the synesis_privacy module to enable CI/CD testing.
//!
//! # Quick Start
//!
//! ```rust
//! use privox::{Redactor, RedactorConfig, TokenVault};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let vault = TokenVault::in_memory()?;
//! let mut redactor = Redactor::new(RedactorConfig::default(), vault)?;
//! let result = redactor.redact("Contact john@example.com", "session-1").await?;
//! assert!(result.redacted_text.contains("[EMAIL_"));
//! # Ok(())
//! # }
//! ```

// Re-export synesis-privacy functionality
pub use synesis_privacy::*;

// Type aliases for the new naming
pub use synesis_privacy::PrivacyError as PrivoxError;
pub use synesis_privacy::PrivacyResult as PrivoxResult;
