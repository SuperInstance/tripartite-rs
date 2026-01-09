//! Custom pattern example for Privox
//!
//! This example demonstrates how to add custom redaction patterns

use privox::{Redactor, RedactorConfig, TokenVault, CustomPatternConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a configuration with custom patterns
    let mut config = RedactorConfig::default();

    // Add a custom pattern for employee IDs
    config.custom_patterns.push(vec![
        CustomPatternConfig {
            name: "employee_id".to_string(),
            pattern: r"EMP-[0-9]{6}".to_string(),
        }
    ]);

    // Create vault and redactor
    let vault = TokenVault::in_memory()?;
    let mut redactor = Redactor::new(config, vault)?;

    // Redact text with custom pattern
    let text = "Employee EMP-123456 started today. Contact at john@example.com";
    let result = redactor.redact(text, "session-1");

    println!("Original: {}", text);
    println!("Redacted: {}", result.redacted_text);
    println!("Tokens created: {}", result.stats.tokens_created);

    Ok(())
}
