//! Privox CLI - Privacy Proxy Command-Line Interface
//!
//! Simple CLI for testing redaction functionality.

use privox::{Redactor, RedactorConfig, TokenVault};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Privox - Privacy Proxy v0.1.0");
    println!("Enter text to redact (press Ctrl+D when done):\n");

    let vault = TokenVault::in_memory()?;
    let config = RedactorConfig::default();
    let mut redactor = Redactor::new(config, vault)?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let session_id = uuid::Uuid::new_v4().to_string();
    let result = redactor.redact(&input, &session_id);

    println!("\n=== Redacted Text ===");
    println!("{}", result.redacted_text);

    println!("\n=== Statistics ===");
    println!("Patterns detected: {}", result.stats.patterns_detected);
    println!("Patterns redacted: {}", result.stats.patterns_redacted);
    println!("Tokens created: {}", result.stats.tokens_created);

    if !result.stats.by_type.is_empty() {
        println!("\nBy type:");
        for (pattern_type, count) in &result.stats.by_type {
            println!("  {}: {}", pattern_type, count);
        }
    }

    println!("\n=== Reinflated Text ===");
    let reinflated = redactor.reinflate(&result.redacted_text);
    println!("{}", reinflated);

    Ok(())
}
