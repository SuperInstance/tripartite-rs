# README Standard Sections - SuperInstance Ecosystem

> **Standard format for all tool repository READMEs**

**Version**: 1.0.0
**Last Updated**: 2026-01-08

---

## Purpose

This document defines the standard sections that every tool repository in the SuperInstance ecosystem should include. Consistency helps users discover tools, understand their purpose, and find integrations.

---

## Required Sections

### 1. Title & Description

**What**: Clear project name and one-line description

**Format**:
```markdown
# Tool Name

> Brief, compelling description (what it does, who it's for)
```

**Example**:
```markdown
# synesis-privacy

> Privacy proxy and redaction engine for SuperInstance AI - tokenize sensitive data before cloud processing
```

**Badge Requirements**:
- Build status (CI/CD)
- License
- Version
- Rust version (if applicable)
- **Required**: Ecosystem badge

```markdown
[![SuperInstance Ecosystem](https://img.shields.io/badge/SuperInstance-Ecosystem-blue.svg)](https://github.com/SuperInstance/Tripartite1/blob/main/docs/ECOSYSTEM.md)
```

---

### 2. Quick Start

**What**: Minimal example to get users started quickly

**Format**:
```markdown
## Quick Start

### Installation

\`\`\`bash
cargo add synesis-privacy
# or
cargo install synesis-privacy
\`\`\`

### Basic Usage

\`\`\`rust
use synesis_privacy::PrivacyProxy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proxy = PrivacyProxy::new("./vault.db").await?;
    let redacted = proxy.redact("My email is user@example.com").await?;
    println!("{}", redacted); // My email is [EMAIL_01]
    Ok(())
}
\`\`\`
```

**Requirements**:
- Must be runnable code
- Must include installation instructions
- Must show core functionality
- Keep under 20 lines

---

### 3. Ecosystem Section ⭐

**What**: How this tool fits into the ecosystem (CRITICAL for cross-referencing)

**Format**:
```markdown
## 🌍 Ecosystem

### Used By

- **[SuperInstance AI](https://github.com/SuperInstance/Tripartite1)** - Main project using this tool for privacy
- **[Your Project](https://github.com/your/project)** - Brief description of how you use it

### Requires

- **[tokio](https://crates.io/crates/tokio)** - Async runtime
- **[rusqlite](https://crates.io/crates/rusqlite)** - Local token vault storage
- **[regex](https://crates.io/crates/regex)** - Pattern matching

### Complementary Tools

- **[synesis-core](https://github.com/SuperInstance/Tripartite1)** - Agent orchestration
- **[synesis-knowledge](https://github.com/SuperInstance/Tripartite1)** - Knowledge vault with RAG
- **[synesis-cloud](https://github.com/SuperInstance/Tripartite1)** - Cloud connectivity

### See Also

- **[SQLite-VSS](https://github.com/asg017/sqlite-vss)** - Vector search extension
- **[llama.cpp](https://github.com/ggerganov/llama.cpp)** - Local LLM inference

📖 **[View Full Ecosystem](https://github.com/SuperInstance/Tripartite1/blob/main/docs/ECOSYSTEM.md)**
```

**Requirements**:
- **MUST** include "Used By" section
- **MUST** include "Requires" section
- **MUST** include "Complementary Tools" section
- **MUST** link to full ecosystem documentation
- Keep links up-to-date
- List real projects (no "coming soon" without label)

---

### 4. Features

**What**: What the tool does, not how it works

**Format**:
```markdown
## Features

- 🔒 **18+ redaction patterns** - Emails, API keys, phone numbers, SSNs, etc.
- 💾 **Local token vault** - Mappings never leave your device
- 🔄 **Re-inflation** - Restore redacted content locally
- ⚡ **Zero-copy** - Efficient string processing
- 🧪 **Tested** - 37 tests, 100% passing
```

**Requirements**:
- Use emoji for visual scanning
- Focus on user benefits
- Keep descriptions concise (one line each)
- Highlight unique capabilities
- Include test coverage if available

---

### 5. Integration

**What**: How to integrate this tool with others

**Format**:
```markdown
## Integration

### With synesis-core

\`\`\`rust
use synesis_core::Council;
use synesis_privacy::PrivacyProxy;

let privacy = PrivacyProxy::new("./vault.db").await?;
let mut council = Council::new(config);
council.set_privacy_proxy(privacy);
\`\`\`

### Standalone Usage

\`\`\`rust
// Use without other SuperInstance tools
let proxy = PrivacyProxy::new("./vault.db").await?;
let safe = proxy.redact(sensitive_data).await?;
\`\`\`

### Example Projects

- **[SuperInstance AI](https://github.com/SuperInstance/Tripartite1)** - Full integration
- **[examples/privacy_proxy.rs](https://github.com/SuperInstance/Tripartite1/blob/main/examples/privacy_proxy.rs)** - Standalone example
```

**Requirements**:
- Show integration with ecosystem tools
- Show standalone usage (if applicable)
- Link to real examples
- Include runnable code snippets

---

### 6. Examples Section

**What**: Real-world usage examples

**Format**:
```markdown
## Examples

### Basic Redaction

\`\`\`rust
let proxy = PrivacyProxy::new("./vault.db").await?;
let document = "Contact: user@example.com, Phone: 555-1234";
let redacted = proxy.redact(document).await?;
// Output: "Contact: [EMAIL_01], Phone: [PHONE_01]"
\`\`\`

### Custom Patterns

\`\`\`rust
let proxy = PrivacyProxy::builder()
    .add_pattern(r"\b[A-Z]{2,}\b", "COMPANY")
    .build("./vault.db").await?;
\`\`\`

### More Examples

See **[examples/](https://github.com/SuperInstance/Tripartite1/tree/main/examples)** for more:
- \`custom_patterns.rs\` - Add your own redaction patterns
- \`token_vault.rs\` - Manage token vault
- \`reinflate.rs\` - Restore redacted content
```

**Requirements**:
- 2-3 examples in README
- Link to full examples directory
- Show progressive complexity (basic → advanced)
- Include expected output in comments

---

### 7. Documentation

**What**: Links to detailed documentation

**Format**:
```markdown
## Documentation

### User Guides

- **[Getting Started](https://github.com/SuperInstance/Tripartite1/blob/main/docs/tutorials/privacy-basics.md)** - Privacy features overview
- **[API Reference](https://docs.rs/synesis-privacy/)** - Rust API docs
- **[Examples](https://github.com/SuperInstance/Tripartite1/tree/main/examples)** - Code examples

### Technical Docs

- **[Architecture](https://github.com/SuperInstance/Tripartite1/blob/main/docs/architecture/privacy-proxy.md)** - Design and internals
- **[Testing Guide](https://github.com/SuperInstance/Tripartite1/blob/main/docs/contributing/testing-guide.md)** - How to test

### Related

- **[Thread Safety Patterns](https://github.com/SuperInstance/Tripartite1/blob/main/THREAD_SAFETY_PATTERNS.md)** - Async patterns
- **[Troubleshooting](https://github.com/SuperInstance/Tripartite1/blob/main/TROUBLESHOOTING.md)** - Common issues
```

**Requirements**:
- Categorize by audience (users, developers)
- Link to docs.rs API reference (for Rust crates)
- Include troubleshooting link
- Keep links relative when possible

---

## Optional Sections

Include these if relevant to your tool:

### Performance

```markdown
## Performance

| Metric | Value |
|--------|-------|
| Redaction speed | ~1MB/s |
| Memory usage | 10-50MB |
| Token vault size | ~100 bytes per token |

*Benchmarks on: Intel i7-12700K, 32GB RAM*
```

### Roadmap

```markdown
## Roadmap

- [x] 18 redaction patterns
- [x] Local token vault
- [ ] Custom pattern API (Q2 2026)
- [ ] Fuzzy matching (Q3 2026)
```

### Contributing

```markdown
## Contributing

We welcome contributions! See **[CONTRIBUTING.md](CONTRIBUTING.md)** for guidelines.

### Good First Issues

- [Add support for SSN redaction](https://github.com/SuperInstance/Tripartite1/issues/42)
- [Improve error messages](https://github.com/SuperInstance/Tripartite1/issues/43)
```

### License

```markdown
## License

Licensed under either of:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT))
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

at your option.
```

---

## README Template

Here's a complete template you can copy:

```markdown
# tool-name

> Brief, compelling description

[![CI](https://github.com/SuperInstance/Tripartite1/actions/workflows/ci.yml/badge.svg)](https://github.com/SuperInstance/Tripartite1/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%20%7C%20Apache--2.0-blue.svg)](LICENSE-APACHE)
[![SuperInstance Ecosystem](https://img.shields.io/badge/SuperInstance-Ecosystem-blue.svg)](https://github.com/SuperInstance/Tripartite1/blob/main/docs/ECOSYSTEM.md)

## Quick Start

### Installation

\`\`\`bash
cargo add tool-name
\`\`\`

### Basic Usage

\`\`\`rust
// Your code here
\`\`\`

## 🌍 Ecosystem

### Used By

- **[SuperInstance AI](https://github.com/SuperInstance/Tripartite1)** - Main project
- **[Your Project](https://github.com/your/project)** - Description

### Requires

- **[dependency1](url)** - Reason
- **[dependency2](url)** - Reason

### Complementary Tools

- **[tool1](url)** - Description
- **[tool2](url)** - Description

### See Also

- **[related-tool](url)** - Description

📖 **[View Full Ecosystem](https://github.com/SuperInstance/Tripartite1/blob/main/docs/ECOSYSTEM.md)**

## Features

- ✨ **Feature 1** - Description
- 🚀 **Feature 2** - Description

## Integration

### With Other Tools

\`\`\`rust
// Integration code
\`\`\`

### Example Projects

- **[Project 1](url)** - Description
- **[examples/example.rs](url)** - Example code

## Examples

### Example 1

\`\`\`rust
// Code
\`\`\`

### More Examples

See **[examples/](url)** for more

## Documentation

### User Guides

- **[Guide 1](url)**
- **[API Reference](https://docs.rs/tool-name/)**

### Technical Docs

- **[Architecture](url)**
- **[Troubleshooting](url)**

## License

Licensed under either of:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT))
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

at your option.
```

---

## Quality Checklist

Before publishing your tool's README, verify:

### Content Quality

- [ ] Title and description are clear and compelling
- [ ] Quick start works (tested it myself)
- [ ] Ecosystem section complete with real links
- [ ] Examples are runnable
- [ ] Documentation links work

### Cross-References

- [ ] "Used By" lists at least one real project
- [ ] "Requires" lists all major dependencies
- [ ] "Complementary Tools" suggests related ecosystem tools
- [ ] Links to full ecosystem docs
- [ ] All URLs are valid (test them!)

### Badges

- [ ] CI/CD badge
- [ ] License badge
- [ ] Version badge
- [ ] **Ecosystem badge (required)**

### Formatting

- [ ] Consistent heading levels
- [ ] Code blocks have language tags
- [ ] Links use descriptive text
- [ ] No broken links (use `markdown-link-check`)

---

## Maintenance

### Updating Cross-References

When a new tool joins the ecosystem:

1. **Add to ecosystem documentation** (`docs/ECOSYSTEM.md`)
2. **Update related tools' READMEs**:
   - Add to "Complementary Tools" if relevant
   - Add to "Used By" if they use it
3. **Add your tool to their lists**:
   - Add your tool to their "Used By" section
   - Add them to your "Requires" section if you depend on them

### Review Schedule

Review READMEs quarterly for:
- Broken links
- Outdated versions
- New integrations to add
- New examples to feature

---

## Tools & Automation

See **[ECOSYSTEM_AUTOMATION.md](ECOSYSTEM_AUTOMATION.md)** for:
- Scripts to generate cross-reference sections
- Link checking automation
- Badge generation
- Documentation sync tools

---

**Standard Version**: 1.0.0
**Last Updated**: 2026-01-08
**Maintained By**: [SuperInstance AI](https://github.com/SuperInstance)
