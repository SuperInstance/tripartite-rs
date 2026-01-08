//! Pattern detection example
//!
//! Demonstrates pattern detection without redaction

use privox::{PatternSet, Redactor, RedactorConfig, TokenVault};

fn main() -> privox::PrivacyResult<()> {
    println!("=== Pattern Detection Example ===\n");

    let vault = TokenVault::in_memory()?;
    let config = RedactorConfig::default();
    let redactor = Redactor::new(config, vault)?;

    // Text with various sensitive patterns
    let text = r#"
        Configuration:
        - Database URL: postgresql://user:password123@localhost:5432/db
        - API Key: sk_test_1234567890abcdefghijklmnopqrst
        - AWS Access Key: AKIAIOSFODNN7EXAMPLE
        - Email: admin@example.com
        - Phone: 555-123-4567
        - File: /home/user/config/secrets.env
        - IP: 192.168.1.1
        - URL: https://api.example.com?token=abc123def456
        - SSN: 123-45-6789
        - Credit Card: 4111111111111111
        - GitHub: ghp_1234567890abcdefghijklmnopqrstuvwxyz123456
    "#;

    // Preview (detection without storage)
    let matches = redactor.preview(text);

    println!("Detected {} sensitive patterns:\n", matches.len());

    for (i, pattern_match) in matches.iter().enumerate() {
        println!("{}. {:?}", i + 1, pattern_match.pattern_type);
        println!("   Pattern: {}", pattern_match.pattern_name);
        println!("   Text: {}", pattern_match.matched_text);
        println!(
            "   Position: {}-{}\n",
            pattern_match.start, pattern_match.end
        );
    }

    // Check if text contains sensitive data
    let has_sensitive = redactor.contains_sensitive(text);
    println!("Contains sensitive data: {}", has_sensitive);

    // Pattern set statistics
    let pattern_set = PatternSet::with_builtins();
    println!("\nBuilt-in patterns available: {}", pattern_set.len());

    Ok(())
}
