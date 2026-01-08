# Getting Started with privox

This tutorial will walk you through using privox to protect sensitive data in your applications.

## Table of Contents

1. [Installation](#installation)
2. [Basic Usage](#basic-usage)
3. [Configuration](#configuration)
4. [Custom Patterns](#custom-patterns)
5. [Token Vault](#token-vault)
6. [LLM Integration](#llm-integration)
7. [Best Practices](#best-practices)

## Installation

Add privox to your `Cargo.toml`:

```toml
[dependencies]
privox = "0.1"
```

## Basic Usage

### Hello World

The simplest use case - redact sensitive data from text:

```rust
use privox::{Redactor, RedactorConfig, TokenVault};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create vault (in-memory for testing)
    let vault = TokenVault::in_memory()?;

    // Create redactor with default patterns
    let mut redactor = Redactor::new(RedactorConfig::default(), vault)?;

    // Redact sensitive data
    let text = "Contact me at john@example.com or call 555-123-4567";
    let result = redactor.redact(text, "session-1")?;

    println!("Original: {}", text);
    println!("Redacted: {}", result.redacted_text);
    // Output: Contact me at [EMAIL_0001] or call [PHONE_0001]

    Ok(())
}
```

### Reinflation

Restore original values after processing:

```rust
use privox::{Redactor, RedactorConfig, TokenVault};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vault = TokenVault::in_memory()?;
    let mut redactor = Redactor::new(RedactorConfig::default(), vault)?;

    let text = "Email: john@example.com";
    let redacted = redactor.redact(text, "session-1")?;

    // Send redacted.redacted_text to cloud/LLM...
    let cloud_response = format!("Received: {}", redacted.redacted_text);

    // Restore original values
    let restored = redactor.reinflate(&cloud_response);
    println!("Restored: {}", restored);
    // Output: Received: Email: john@example.com

    Ok(())
}
```

## Configuration

### Enable/Disable Patterns

```rust
use privox::{Redactor, RedactorConfig, TokenVault};

let vault = TokenVault::in_memory()?;

// Customize which patterns to enable
let config = RedactorConfig {
    redact_emails: true,
    redact_phones: true,
    redact_ssns: true,
    redact_credit_cards: true,
    redact_api_keys: true,
    redact_ips: true,
    redact_paths: true,
    redact_urls: true,
    custom_patterns: vec![],
};

let redactor = Redactor::new(config, vault)?;
```

### Persistent Token Vault

Use SQLite for persistent storage:

```rust
use privox::TokenVault;

// File-based vault (persistent)
let vault = TokenVault::new("/var/lib/privox/tokens.db")?;

// In-memory vault (ephemeral, for testing)
let vault = TokenVault::in_memory()?;
```

## Custom Patterns

Add domain-specific patterns:

```rust
use privox::{Redactor, RedactorConfig, CustomPatternConfig, TokenVault};

let vault = TokenVault::in_memory()?;

let config = RedactorConfig {
    custom_patterns: vec![
        CustomPatternConfig {
            name: "employee_id".to_string(),
            pattern: r"EMP-[0-9]{6}".to_string(),
        },
        CustomPatternConfig {
            name: "ticket_number".to_string(),
            pattern: r"TICKET-[0-9]{4}-[0-9]{4}".to_string(),
        },
    ],
    ..Default::default()
};

let redactor = Redactor::new(config, vault)?;
```

## Token Vault

### Direct Vault Access

While `Redactor` handles vault operations automatically, you can use the vault directly:

```rust
use privox::TokenVault;

let vault = TokenVault::in_memory()?;

// Store a token manually
let token = vault.store("EMAIL", "john@example.com", "session-1")?;
println!("Token: {}", token); // [EMAIL_0001]

// Retrieve original value
let original = vault.retrieve(&token);
assert_eq!(original, Some("john@example.com".to_string()));

// Get session statistics
let stats = vault.session_stats("session-1")?;
println!("Tokens in session: {}", stats.total_tokens);

// Clear session tokens
vault.clear_session("session-1")?;
```

### Thread-Safe Access

The vault is thread-safe and can be shared:

```rust
use privox::TokenVault;
use std::sync::Arc;

let vault = Arc::new(TokenVault::in_memory()?);

// Clone Arc and use in multiple threads
let vault1 = Arc::clone(&vault);
let vault2 = Arc::clone(&vault);

// Both can access concurrently
```

## LLM Integration

### Complete LLM Flow

```rust
use privox::{Redactor, RedactorConfig, TokenVault};

async fn ask_llm(question: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Step 1: Redact user input
    let vault = TokenVault::in_memory()?;
    let mut redactor = Redactor::new(RedactorConfig::default(), vault)?;
    let redacted = redactor.redact(question, "session-1")?;

    // Step 2: Send redacted text to LLM
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .json(&serde_json::json!({
            "model": "gpt-4",
            "messages": [{"role": "user", "content": redacted.redacted_text}]
        }))
        .send()
        .await?;

    let text = response.text().await?;

    // Step 3: Restore original values
    let restored = redactor.reinflate(&text);

    Ok(restored)
}
```

### Session Management

For better performance, reuse the redactor across calls:

```rust
use privox::{Redactor, RedactorConfig, TokenVault};
use std::sync::{Arc, Mutex};

// Create once
let vault = TokenVault::in_memory()?;
let redactor = Arc::new(Mutex::new(
    Redactor::new(RedactorConfig::default(), vault)?
));

// Use across multiple requests
async fn process_request(
    redactor: Arc<Mutex<Redactor>>,
    user_input: &str,
    session_id: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut r = redactor.lock().unwrap();
    let redacted = r.redact(user_input, session_id)?;

    // Send to LLM...

    let response = "LLM response here";
    let restored = r.reinflate(response);

    Ok(restored)
}
```

## Best Practices

### 1. Use Session IDs

Always use session IDs for token isolation:

```rust
// Good
let result = redactor.redact(text, &session.user_id)?;
let result = redactor.redact(text, &uuid::Uuid::new_v4().to_string())?;

// Bad (global session)
let result = redactor.redact(text, "global")?;
```

### 2. Clear Sessions

Clean up tokens after use:

```rust
// After user request completes
redactor.clear_session(&session_id)?;

// Or periodically
tokio::spawn(async move {
    tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
    let _ = redactor.clear_session(&session_id);
});
```

### 3. Preview Before Redaction

Check what will be redacted without storing:

```rust
let matches = redactor.preview(text);

for m in &matches {
    println!("Found {}: {}", m.pattern_name, m.matched_text);
}

// Then redact
let result = redactor.redact(text, session_id)?;
```

### 4. Monitor Statistics

Track redaction statistics:

```rust
let result = redactor.redact(text, session_id)?;

println!("Redacted {} items", result.stats.patterns_redacted);

for (pattern_type, count) in &result.stats.by_type {
    println!("  {}: {}", pattern_type, count);
}
```

### 5. Handle Errors Gracefully

```rust
use privox::PrivacyError;

match redactor.redact(text, session_id) {
    Ok(result) => {
        // Process redacted text
    },
    Err(PrivacyError::PatternError(e)) => {
        eprintln!("Pattern error: {}", e);
    },
    Err(PrivacyError::VaultError(e)) => {
        eprintln!("Vault error: {}", e);
    },
    Err(e) => {
        eprintln!("Error: {}", e);
    },
}
```

### 6. Use Persistent Storage in Production

```rust
// In production, use file-based vault
let vault = TokenVault::new("/var/lib/privox/tokens.db")?;

// In tests, use in-memory
let vault = TokenVault::in_memory()?;
```

### 7. Set Vault Permissions

Ensure vault database has proper permissions:

```bash
# Restrict to owner only
chmod 600 /var/lib/privox/tokens.db
chown appuser:appuser /var/lib/privox/tokens.db
```

## Next Steps

- Explore [examples/](../examples/) for more usage patterns
- Read the [API documentation](https://docs.rs/privox)
- Check the [README](../README.md) for features and performance
- Review [MIGRATION_GUIDE.md](../MIGRATION_GUIDE.md) if upgrading from synesis-privacy

## Support

- GitHub: https://github.com/SuperInstance/privox
- Issues: https://github.com/SuperInstance/privox/issues
- Docs: https://docs.rs/privox
