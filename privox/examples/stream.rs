//! Stream processing example
//!
//! Demonstrates processing stdin line-by-line, redacting each line
//! and writing to stdout. This is useful for log file processing.
//!
//! Run with:
//!   echo "Contact john@example.com" | cargo run --example stream
//!
//! Or process a file:
//!   cat log.txt | cargo run --example stream > redacted.txt

use privox::{Redactor, RedactorConfig, TokenVault};
use std::io::{self, BufRead, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create vault and redactor
    let vault = TokenVault::in_memory()?;
    let mut redactor = Redactor::new(RedactorConfig::default(), vault)?;

    // Lock stdout for efficient writing
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    let mut line_count = 0;
    let mut total_redacted = 0;

    // Process each line from stdin
    for line in stdin.lock().lines() {
        let line = line?;
        line_count += 1;

        // Redact the line
        let result = redactor.redact(&line, "session-1");
        total_redacted += result.stats.patterns_redacted;

        // Write redacted line to stdout
        writeln!(stdout_lock, "{}", result.redacted_text)?;
    }

    // Flush stdout
    stdout_lock.flush()?;

    // Statistics (written to stderr to not mix with stdout output)
    eprintln!();
    eprintln!("Processing complete:");
    eprintln!("  Lines processed: {}", line_count);
    eprintln!("  Items redacted:  {}", total_redacted);

    Ok(())
}
