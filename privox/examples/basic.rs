//! Basic redaction example
//!
//! Demonstrates the simplest use of privox:
//! - Create a vault and redactor
//! - Redact sensitive data
//! - See the results

use privox::{Redactor, RedactorConfig, TokenVault};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an in-memory token vault (for testing)
    let vault = TokenVault::in_memory()?;

    // Create a redactor with default settings
    let mut redactor = Redactor::new(RedactorConfig::default(), vault)?;

    // Text containing sensitive information
    let text = "Contact me at john@example.com or call 555-123-4567";

    // Redact the text (returns RedactionResult, not Result)
    let result = redactor.redact(text, "session-1");

    // Display results
    println!("Original:  {}", text);
    println!("Redacted:  {}", result.redacted_text);
    println!();
    println!("Statistics:");
    println!("  Patterns detected: {}", result.stats.patterns_detected);
    println!("  Patterns redacted: {}", result.stats.patterns_redacted);
    println!("  Tokens created:     {}", result.stats.tokens_created);
    println!();
    println!("By type:");
    for (pattern_type, count) in &result.stats.by_type {
        println!("  {}: {}", pattern_type, count);
    }

    Ok(())
}
