//! Integration with LLM APIs example
//!
//! Demonstrates how to integrate privox with an LLM API (like OpenAI)
//! to protect sensitive data before sending to the cloud.
//!
//! This example shows the complete flow:
//! 1. Redact user input
//! 2. Send redacted text to LLM
//! 3. Reinflate the response
//!
//! Note: This is a simplified example. In production, you'd want:
//! - Better error handling
//! - Session management
//! - Async/await throughout
//! - Proper API key management

use privox::{Redactor, RedactorConfig, TokenVault};

/// Simulated LLM API response
/// In production, this would call actual APIs like OpenAI, Anthropic, etc.
fn mock_llm_api(prompt: &str) -> String {
    format!(
        "Based on your inquiry about {}, I can help you with that request.",
        prompt
    )
}

/// Ask an LLM with privacy protection
fn ask_llm_protected(question: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Step 1: Create redactor (in production, reuse this across calls)
    let vault = TokenVault::in_memory()?;
    let mut redactor = Redactor::new(RedactorConfig::default(), vault)?;

    println!("🔒 Original question: {}", question);
    println!();

    // Step 2: Redact sensitive data
    let redacted = redactor.redact(question, "session-1")?;
    println!("🛡️  Redacted question: {}", redacted.redacted_text);
    println!("   Redacted {} items", redacted.stats.patterns_redacted);
    println!();

    // Step 3: Send redacted text to LLM (simulated here)
    let llm_response = mock_llm_api(&redacted.redacted_text);
    println!("🤖 LLM Response (redacted): {}", llm_response);
    println!();

    // Step 4: Reinflate to restore original values
    let restored_response = redactor.reinflate(&llm_response);
    println!("✅ Restored Response: {}", restored_response);
    println!();

    Ok(restored_response)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== LLM Integration Example ===\n");
    println!("This example shows how to protect sensitive data when calling LLM APIs\n");

    // Example 1: Email redaction
    println!("--- Example 1: Email Protection ---");
    let question1 = "My email is john@example.com, can you help me reset my password?";
    ask_llm_protected(question1)?;

    println!();

    // Example 2: API key protection
    println!("--- Example 2: API Key Protection ---");
    let question2 = "I'm getting an error with API key sk_test_1234567890abcdef, what should I do?";
    ask_llm_protected(question2)?;

    println!();

    // Example 3: Multiple sensitive items
    println!("--- Example 3: Multiple Sensitive Items ---");
    let question3 = "Contact me at jane@company.com or call 555-123-4567. My SSN is 123-45-6789.";
    ask_llm_protected(question3)?;

    println!();
    println!("=== All examples completed ===");
    println!();
    println!("Key benefits:");
    println!("  ✓ Cloud LLM never sees your sensitive data");
    println!("  ✓ Responses are intelligible after reinflation");
    println!("  ✓ Session-based isolation for cleanup");
    println!("  ✓ Works with any LLM API");

    Ok(())
}
