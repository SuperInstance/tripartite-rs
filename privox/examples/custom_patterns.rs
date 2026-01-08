//! Custom patterns example
//!
//! Demonstrates adding custom regex patterns

use privox::{CustomPatternConfig, Redactor, RedactorConfig, TokenVault};

fn main() -> privox::PrivacyResult<()> {
    println!("=== Custom Patterns Example ===\n");

    // Create vault and redactor with custom pattern
    let vault = TokenVault::in_memory()?;
    let custom_patterns = vec![
        CustomPatternConfig {
            name: "employee_id".to_string(),
            pattern: r"EMP-[0-9]{6}".to_string(),
        },
        CustomPatternConfig {
            name: "project_code".to_string(),
            pattern: r"PROJ-[A-Z]{3}-[0-9]{4}".to_string(),
        },
    ];

    let config = RedactorConfig {
        custom_patterns,
        ..Default::default()
    };

    let mut redactor = Redactor::new(config, vault)?;

    // Text with custom patterns
    let text = r#"
        Employee Information:
        Name: John Doe
        ID: EMP-123456
        Project: PROJ-ABC-7890

        Contact: john@example.com
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

    for (pattern_type, count) in &result.stats.by_type {
        println!("  {}: {}", pattern_type, count);
    }

    // Reinflate
    let reinflated = redactor.reinflate(&result.redacted_text);
    println!("\n=== Reinflated text ===");
    println!("{}", reinflated);

    assert_eq!(text, reinflated);
    println!("\n✓ Custom patterns working correctly!");

    Ok(())
}
