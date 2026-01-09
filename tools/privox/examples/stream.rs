//! Stream processing example for Privox
//!
//! This example demonstrates redacting data from a stream (e.g., log lines)

use privox::{Redactor, RedactorConfig, TokenVault};
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vault = TokenVault::in_memory()?;
    let mut redactor = Redactor::new(RedactorConfig::default(), vault)?;

    println!("Enter log lines (Ctrl+D to finish):");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let result = redactor.redact(&line, "stream-session");
        println!("Redacted: {}", result.redacted_text);
    }

    // Show statistics
    let stats = redactor.get_stats("stream-session")?;
    println!("\nStatistics:");
    println!("Total tokens: {}", stats.tokens_created);
    println!("By type: {:?}", stats.by_type);

    Ok(())
}
