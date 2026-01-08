//! Basic redaction example
//!
//! Demonstrates simple redaction and reinflation

use privox::{Redactor, RedactorConfig, TokenVault};

fn main() -> privox::PrivacyResult<()> {
    println!("=== Basic Redaction Example ===\n");

    // Create vault and redactor
    let vault = TokenVault::in_memory()?;
    let config = RedactorConfig::default();
    let mut redactor = Redactor::new(config, vault)?;

    // Text with sensitive information
    let text = r#"
        Contact Information:
        Email: john.doe@example.com
        Phone: 555-123-4567

        API Keys:
        GitHub: ghp_1234567890abcdefghijklmnopqrstuvwxyz123456
        Stripe: sk_test_1234567890abcdefghijklmnopqrst

        SSN: 123-45-6789
        Credit Card: 4111111111111111
    "#;

    println!("Original text:");
    println!("{}", text);

    // Redact
    let session_id = uuid::Uuid::new_v4().to_string();
    let result = redactor.redact(text, &session_id);

    println!("\n=== Redacted text ===");
    println!("{}", result.redacted_text);

    println!("\n=== Statistics ===");
    println!(
        "Total patterns detected: {}",
        result.stats.patterns_detected
    );
    println!("Total tokens created: {}", result.stats.tokens_created);

    for (pattern_type, count) in &result.stats.by_type {
        println!("  {}: {}", pattern_type, count);
    }

    // Reinflate
    let reinflated = redactor.reinflate(&result.redacted_text);
    println!("\n=== Reinflated text ===");
    println!("{}", reinflated);

    // Verify perfect restoration
    assert_eq!(text, reinflated);
    println!("\n✓ Perfect restoration verified!");

    Ok(())
}
