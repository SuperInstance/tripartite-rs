# privox v0.1.0

First release of privox as independent crate!

## What's New

- **Extracted from synesis-privacy** as standalone tool for broader ecosystem use
- **18 built-in PII redaction patterns**: emails, phones, SSNs, credit cards, API keys, and more
- **Reversible tokenization** for LLM workflows: redact before cloud, reinflate after
- **High performance**: 100K+ redactions/second on typical workloads
- **100% API compatible** with synesis-privacy - drop-in replacement
- **Thread-safe token vault** with SQLite backend
- **Session-based token isolation** for security
- **Comprehensive test coverage**: 37 tests, 100% passing

## Use Cases

privox is designed for **LLM applications** that need to:

1. **Redact sensitive data** before sending to cloud LLMs
2. **Maintain context** by reinflating tokens in responses
3. **Protect privacy** without losing functionality
4. **Scale efficiently** with minimal performance overhead

### Example Workflows

```rust
use privox::{Redactor, RedactorConfig, TokenVault};

// 1. Create redactor with in-memory vault
let vault = TokenVault::in_memory()?;
let mut redactor = Redactor::new(RedactorConfig::default(), vault)?;

// 2. Redact user query
let original = "My email is john@example.com and SSN is 123-45-6789";
let result = redactor.redact(original, "session-123");
// result.redacted_text: "My email is [EMAIL_0001] and SSN is [SSN_0001]"

// 3. Send redacted text to cloud (no PII leaked!)
let cloud_response = llm_api.complete(&result.redacted_text).await?;

// 4. Reinflate response to restore context
let final_response = redactor.reinflate(&cloud_response);
// Tokens replaced with original values
```

## Built-in Patterns

privox detects and redacts 18 types of sensitive data:

| Pattern | Description | Example |
|---------|-------------|---------|
| Email | Email addresses | `user@example.com` |
| Phone (US) | US/Canada phone numbers | `555-123-4567` |
| Phone (Intl) | International numbers | `+44 20 7946 0958` |
| SSN | Social Security Numbers | `123-45-6789` |
| Credit Card | Visa, MC, Amex, Discover | `4111...1111` |
| API Key (Generic) | Generic API keys | `api_key=XXXXX` |
| AWS Key | AWS access/secret keys | `AKIA...` |
| GitHub Token | GitHub personal access tokens | `ghp_...` |
| Slack Token | Slack bot/user tokens | `xoxb-...` |
| Stripe/OpenAI Key | sk- prefix keys | `sk_test_...` |
| Private Key | PEM-formatted private keys | `-----BEGIN...` |
| IPv4 | IPv4 addresses | `192.168.1.1` |
| IPv6 | IPv6 addresses | `2001:db8::1` |
| URL Token | URLs with sensitive params | `?token=XXX` |
| Password | Passwords in context | `password=secret` |
| Unix Path | Unix file paths | `/home/user/...` |
| Windows Path | Windows file paths | `C:\Users\...` |
| Custom | User-defined patterns | - |

## Performance

- **Redaction speed**: 100K+ operations/second
- **Token lookup**: O(1) with SQLite indexes
- **Memory overhead**: ~1KB per unique token
- **Thread safety**: Arc<Mutex<>> for concurrent access

## Migration from synesis-privacy

### Before (synesis-privacy)

```toml
[dependencies]
synesis-privacy = "0.2"
```

```rust
use synesis_privacy::{Redactor, TokenVault};
```

### After (privox)

```toml
[dependencies]
privox = "0.1"
```

```rust
use privox::{Redactor, TokenVault};
```

**That's it!** The API is 100% compatible. No code changes required.

## Documentation

- **GitHub**: https://github.com/SuperInstance/privox
- **crates.io**: https://crates.io/crates/privox
- **Docs.rs**: https://docs.rs/privox
- **Examples**: See `/examples` directory in repo

## SuperInstance Integration

privox is the **privacy engine** behind SuperInstance AI:

```
User Query → privox.redact() → [TOKEN_XXXX] → Cloud LLM
                                          ↓
                                   TokenVault (local)
                                          ↓
Response ← privox.reinflate() ← [TOKEN_XXXX] ← Cloud Response
```

SuperInstance uses privox to:
1. **Redact PII** before cloud escalation
2. **Protect secrets** in code repositories
3. **Maintain context** in multi-turn conversations
4. **Isolate sessions** for multi-user environments

## What's Next

Future releases may include:
- **Async API** for tokio integration
- **Custom pattern DSL** for domain-specific redaction
- **Streaming redaction** for large payloads
- **WASM support** for browser environments
- **Performance optimizations** for bulk operations

## Contributing

We welcome contributions! See `CONTRIBUTING.md` for details.

Areas where we'd love help:
- Additional PII patterns (international formats)
- Performance benchmarks and optimizations
- Documentation and examples
- Integration with popular LLM frameworks

## License

MIT OR Apache-2.0 (your choice)

## Acknowledgments

Extracted from **SuperInstance AI** (https://github.com/SuperInstance/Tripartite1)
where it powers the privacy layer of the tripartite agent system.

---

**Version**: 0.1.0
**Release Date**: 2026-01-08
**Compatible with**: synesis-privacy 0.2.x (drop-in replacement)
