# Migration Guide: synesis-privacy → privox

This guide helps you migrate from `synesis-privacy` to `privox`. The good news: **the API is 100% compatible**, so migration is straightforward.

## Quick Migration

### 1. Update Cargo.toml

```toml
# Old
[dependencies]
synesis-privacy = "0.2"

# New
[dependencies]
privox = "0.1"
```

### 2. Update Imports

```rust
// Old
use synesis_privacy::{Redactor, TokenVault, RedactorConfig};

// New
use privox::{Redactor, TokenVault, RedactorConfig};
```

### 3. Update Error Types

```rust
// Old
use synesis_privacy::PrivacyError;

// New
use privox::PrivacyError;
```

## Complete Example

### Before (synesis-privacy)

```rust
use synesis_privacy::{Redactor, RedactorConfig, TokenVault};

fn main() -> synesis_privacy::PrivacyResult<()> {
    let vault = TokenVault::in_memory()?;
    let mut redactor = Redactor::new(RedactorConfig::default(), vault)?;

    let text = "Contact: john@example.com";
    let result = redactor.redact(text, "session-1")?;

    println!("{}", result.redacted_text);

    Ok(())
}
```

### After (privox)

```rust
use privox::{Redactor, RedactorConfig, TokenVault};

fn main() -> privox::PrivacyResult<()> {
    let vault = TokenVault::in_memory()?;
    let mut redactor = Redactor::new(RedactorConfig::default(), vault)?;

    let text = "Contact: john@example.com";
    let result = redactor.redact(text, "session-1")?;

    println!("{}", result.redacted_text);

    Ok(())
}
```

**That's it!** Only the import paths changed.

## Breaking Changes

**None!** The `privox` crate is a direct extraction of `synesis-privacy` with the same API.

## Feature Parity

All features from `synesis-privacy` are available in `privox`:

| Feature | synesis-privacy | privox |
|---------|----------------|--------|
| 18 built-in patterns | ✅ | ✅ |
| Custom patterns | ✅ | ✅ |
| Reversible tokenization | ✅ | ✅ |
| SQLite vault | ✅ | ✅ |
| In-memory vault | ✅ | ✅ |
| Thread-safe | ✅ | ✅ |
| Session isolation | ✅ | ✅ |
| Statistics | ✅ | ✅ |

## Type Compatibility

All types are the same:

```rust
// These types are identical in both crates:
Redactor                // Main redaction engine
TokenVault              // Token storage
RedactorConfig          // Configuration
RedactionResult         // Result of redaction
RedactionStats          // Statistics
PrivacyError            // Error type
PrivacyResult<T>        // Result type
Pattern                 // Pattern definition
PatternMatch            // Match result
PatternSet              // Pattern collection
PatternType             // Pattern type enum
```

## Why the Rename?

The `synesis-privacy` crate was renamed to `privox` (short for "privacy proxy") to:

1. **Make it standalone**: Can be used independently of SuperInstance
2. **Clearer branding**: "privox" clearly indicates it's a privacy/redaction tool
3. **Easier discovery**: Shorter, more memorable name for crates.io
4. **Broader adoption**: Encourage use in other projects

## Testing Your Migration

After updating imports, run your tests:

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_redaction
```

All tests should pass without modification.

## Need Help?

If you encounter issues during migration:

1. **Check imports**: Make sure all `synesis_privacy` references are updated to `privox`
2. **Check version**: Verify you're using `privox = "0.1"` or later
3. **Review examples**: See `/examples` directory for usage patterns
4. **Report issues**: https://github.com/SuperInstance/privox/issues

## Advanced Migration

### Using Both Crates (Not Recommended)

If you need to run both crates during a gradual migration:

```rust
// Use aliases to distinguish
use privox as pv;
use synesis_privacy as sp;

// This is temporary - remove old code as you migrate
```

**Warning**: Don't run both in production. They maintain separate token vaults and won't share state.

### Workspace Migration

If you're migrating a workspace with multiple packages:

```toml
# In workspace root Cargo.toml
[workspace.dependencies]
# Old
# synesis-privacy = "0.2"

# New
privox = "0.1"

# In each package's Cargo.toml
[dependencies]
privox = { workspace = true }
```

Then update all imports across the workspace.

## Summary

- ✅ API is 100% compatible
- ✅ Only imports need to change
- ✅ All tests should pass
- ✅ No breaking changes
- ✅ Better branding and standalone usage

Welcome to `privox`! 🎉
