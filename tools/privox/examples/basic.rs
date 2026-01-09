//! Basic redaction example for Privox
//!
//! This example demonstrates the simplest way to redact sensitive information

use privox::{Redactor, RedactorConfig, TokenVault};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an in-memory token vault
    let vault = TokenVault::in_memory()?;

    // Create a redactor with default patterns
    let mut redactor = Redactor::new(RedactorConfig::default(), vault)?;

    // Redact sensitive information
    let text = "Contact me at john@example.com or call 555-123-4567";
    let result = redactor.redact(text, "demo-session");

    println!("Original: {}", text);
    println!("Redacted: {}", result.redacted_text);
    println!("Stats: {:?}", result.stats);

    Ok(())
}
