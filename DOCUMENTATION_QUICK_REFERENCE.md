# Documentation Quick Reference
## Essential Guidelines for Rust Tool Documentation

**Version**: 1.0
**Last Updated**: 2026-01-08

---

## 10-Second Summary

This guide provides the essential patterns for creating PyTorch/Ollama-quality documentation for Rust tools. For complete details, see [DOCUMENTATION_PLAYBOOK.md](DOCUMENTATION_PLAYBOOK.md).

---

## Documentation Hierarchy

```
10-second sell (README) → 5-minute win (Quick Start) → 1-hour deep dive (Guides) → Complete reference (API Docs) → Community (Contributing)
```

---

## Phase 1: README (The 10-Second Sell)

### Required Sections (in order)

1. **Hook + Tagline** (3 seconds)
2. **Badges** (build trust)
3. **What Makes It Different** (differentiation)
4. **Quick Start** (5-minute win)
5. **Usage Examples** (3-5 examples)
6. **Architecture Diagram** (if complex)
7. **Documentation Links** (navigation)
8. **Features** (detailed capabilities)
9. **Project Status** (builds trust)
10. **Contributing** (invitation)
11. **License & Acknowledgments**

### Quality Checklist

- ✅ Hook in first 3 lines
- ✅ Badges functional
- ✅ Quick Start <5 minutes, copy-pasteable
- ✅ Expected output shown
- ✅ 3-5 usage examples
- ✅ Clear navigation links
- ✅ 300-500 lines total

### Template Snippet

```markdown
# [Project Name]

> **[Tagline: what + why]**

[![CI](badge)](link) [![Docs](badge)](link) [![License](badge)](link)

[Paragraph explaining what it is and why it's unique]

## 🚀 Quick Start

```bash
# Install (one command if possible)
command

# Run first example
command example
```

**Output**:
```
[Show actual output]
```
```

---

## Phase 2: Quick Start Tutorial (The 5-Minute Win)

### Structure

```markdown
# [Title: Action-Oriented]

**Time**: 5 minutes
**Difficulty**: Beginner
**Prerequisites**: [List]

## What You'll Learn

- [Concrete outcome 1]
- [Concrete outcome 2]

## Before You Start

### Prerequisites

- **[Requirement]** ([install link]) - Why needed

## Step 1: [Action-Oriented Title]

**What you're doing**: WHY this matters

```bash
# Command
```

**What you should see**:
```
[Output]
```

## [Repeat for 3-5 steps]

## What's Next?

- **[Next Tutorial](link)** - What you'll learn

## Summary

In this tutorial, you:
- ✅ [Accomplishment 1]
- ✅ [Accomplishment 2]
```

### Quality Checklist

- ✅ Time estimate accurate
- ✅ All commands copy-pasteable
- ✅ Expected output for every step
- ✅ Troubleshooting common errors
- ✅ Clear "what's next" links

---

## Phase 3: In-Depth Guides

### Guide Types

1. **Architecture Guides** - How it works
2. **Feature Guides** - Deep dive into capabilities
3. **Configuration Guides** - All options explained
4. **Performance Guides** - Optimization
5. **Integration Guides** - Using with other tools

### Structure Template

```markdown
# [Topic] Guide

## Overview

[2-3 paragraphs: what, who, outcome]

## When to Use [Feature]

- **Use case 1**: [Description]
- **Use case 2**: [Description]

## Key Concepts

### Concept 1

[Explanation + example]

### Concept 2

[Explanation + example]

## Implementation

### Step 1: [Action]

**Prerequisites**: [What you need]

[Detailed explanation + code]

## Best Practices

### ✅ DO: [Best practice]

[Explanation + good code]

### ❌ DON'T: [Common mistake]

[Explanation + bad code]

## Performance Considerations

- **Consideration**: [Impact + recommendation]

## Troubleshooting

### Problem: [Issue]

**Solution**: [Fix]

## See Also

- **[Related doc](link)**
```

---

## Phase 4: API Reference

### rustdoc Documentation

**Crate-level** (`lib.rs`):
```rust
//! # [Crate Name]
//!
//! [One-line description]
//!
//! ## Quick Start
//!
//! ```rust
//! use crate_name;
//!
//! // Example code
//! ```
//!
//! ## Modules
//!
//! - [`module1`] - [Description]
//! - [`module2`] - [Description]
```

**Module-level** (`mod.rs`):
```rust
//! # Module Name
//!
//! [Purpose and overview]
//!
//! ## Key Components
//!
//! ### Component 1
//! [Description]
//!
//! ### Component 2
//! [Description]
```

**Item-level**:
```rust
/// Brief one-line summary.
///
/// More detailed explanation.
///
/// # Arguments
///
/// * `arg1` - Description
///
/// # Returns
///
/// Description of return value
///
/// # Errors
///
/// * `ErrorType` - When this occurs
///
/// # Examples
///
/// ```
/// use crate::function_name;
///
/// let result = function_name(...)?;
/// assert_eq!(result, expected);
/// # Ok::<(), ErrorType>(())
/// ```
pub fn function_name(arg1: Type1) -> Result<ReturnType, ErrorType> {
    // implementation
}
```

### Enhanced API Docs

**File**: `docs/api/component-name.md`

```markdown
# [Component] API

## Overview

[Brief description]

## Quick Example

```rust
[Complete, runnable example]
```

## Core Types

### [Type Name]

[Purpose and usage]

#### Methods

##### `method_name(params) -> Result<Type>`

[Description]

**Example**:
```rust
[Usage example]
```

**Parameters**:
- `param`: [Description]

**Returns**:
- `Ok(value)`: [Description]
- `Err(Error)`: [Description]

**See Also**: [Related APIs]
```

---

## Phase 5: Community & Contributing

### Contributing Guide

**Required Sections**:
1. Quick Start (for contributors)
2. Development Workflow
3. Code Standards
4. Testing Guidelines
5. Documentation Standards
6. Submitting Changes
7. Community Guidelines
8. Getting Help

### Issue Templates

**Bug Report** (`.github/ISSUE_TEMPLATE/bug_report.md`):
```markdown
---
name: Bug report
about: Create a report to help us improve
title: '[BUG] '
labels: bug
---

## Bug Description
[Clear description]

## To Reproduce
1. Step 1
2. Step 2

## Expected Behavior
[What should happen]

## Actual Behavior
[What actually happened]

## Environment
- OS: [e.g. Ubuntu 22.04]
- Rust version: [e.g. 1.75.0]
- Project version: [e.g. v0.2.0]
```

**Feature Request** (`.github/ISSUE_TEMPLATE/feature_request.md`):
```markdown
---
name: Feature request
about: Suggest an idea for this project
title: '[FEATURE] '
labels: enhancement
---

## Feature Description
[Clear description]

## Problem Statement
[What problem does this solve?]

## Proposed Solution
[How should it work?]

## Alternatives Considered
[What other approaches?]

## Willing to Contribute?
- [ ] Yes
- [ ] No
```

### PR Template

```markdown
## Description
[Brief description]

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation

## Related Issues
Fixes #123

## Testing
- [ ] Tests added/updated
- [ ] All tests passing

## Checklist
- [ ] Code follows style guidelines
- [ ] Documentation updated
- [ ] No new warnings
```

---

## Tools & Infrastructure

### Essential Tools

```bash
# rustdoc (built-in)
cargo doc --no-deps --all-features
cargo doc --open

# Doc tests
cargo test --doc

# mdBook (optional)
cargo install mdbook
mdbook serve

# Link checker
cargo install lychee
lychee docs/
```

### CI/CD Integration

**GitHub Actions** (`.github/workflows/documentation.yml`):
```yaml
name: Documentation

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        uses: actions-rust-lang/setup-rust-toolchain@v1
      - name: Build Documentation
        run: cargo doc --no-deps --all-features
      - name: Run Doc Tests
        run: cargo test --doc --all-features
      - name: Check Links
        run: lychee docs/ README.md
```

---

## Best Practices Summary

### Writing Style

✅ **DO**:
- Use active voice
- One concept per paragraph
- Short sentences (15-20 words)
- Start with WHY, then WHAT, then HOW
- Include code examples for all APIs

❌ **DON'T**:
- Use jargon unless defined
- Write walls of text
- Make assumptions about user knowledge
- Skip examples for "obvious" things

### Code Examples

✅ **DO**:
- Make examples runnable
- Show expected output
- Include error handling
- Demonstrate best practices

❌ **DON'T**:
- Use placeholder code (e.g., "do_something()")
- Skip imports
- Assume context
- Use outdated syntax

### Structure

✅ **DO**:
- Progressive difficulty (start easy, go deep)
- Clear navigation (links everywhere)
- Multiple entry points (tutorials, guides, reference)
- Consistent formatting

❌ **DON'T**:
- Bury the lead (start with most important)
- Make users read everything (provide shortcuts)
- Use inconsistent formatting
- Leave orphan pages (no links to them)

---

## Quality Checklist

### Overall Documentation

```markdown
Completeness:
- [ ] README: All sections present
- [ ] Quick Start: <5 minutes, tested
- [ ] Tutorials: Progressive, major use cases
- [ ] Guides: In-depth coverage
- [ ] API Reference: All public APIs
- [ ] Contributing: Clear workflow

Quality:
- [ ] No typos/grammar errors
- [ ] Examples tested
- [ ] Links validated
- [ ] Consistent style
- [ ] Clear language

Maintainability:
- [ ] Templates used
- [ ] Doc tests passing
- [ ] CI checks passing
- [ ] Update process defined
```

### Pre-Release Checklist

```markdown
- [ ] New APIs documented
- [ ] New features have docs
- [ ] Breaking changes have migration guide
- [ ] Examples updated
- [ ] Changelog updated
- [ ] Doc tests passing
- [ ] Links validated
- [ ] Docs built successfully
```

---

## Quick Start: Creating Documentation for Your Rust Tool

### Day 1: Foundation (2 hours)

1. **Create README.md** using template (1 hour)
2. **Write Quick Start tutorial** (30 min)
3. **Set up CI for docs** (30 min)

### Week 1: User Documentation (5 hours)

1. **Write 2-3 tutorials** (3 hours)
2. **Create 2-3 guides** (2 hours)

### Week 2: Developer Documentation (5 hours)

1. **Document all public APIs** (3 hours)
2. **Write Contributing guide** (1 hour)
3. **Create issue/PR templates** (1 hour)

### Ongoing: Maintenance (1 hour/week)

1. **Update docs for changes** (30 min)
2. **Check for broken links** (15 min)
3. **Review and improve** (15 min)

---

## Sources & Inspiration

- **[PyTorch Documentation](https://pytorch.org/docs/stable/)** - Comprehensive, multi-path learning
- **[Ollama Documentation](https://docs.ollama.com/)** - Quickstart-first, user-friendly
- **[The Rust Book](https://doc.rust-lang.org/book/)** - Progressive, concept-driven
- **[Tokio Documentation](https://tokio.rs/)** - Architecture-focused, performance-aware
- **[mdBook Documentation](https://rust-lang.github.io/mdBook/)** - Tool for building books

---

## Complete Documentation

For detailed explanations, examples, and templates, see **[DOCUMENTATION_PLAYBOOK.md](DOCUMENTATION_PLAYBOOK.md)**.

---

**Version**: 1.0
**Last Updated**: 2026-01-08
**License**: MIT OR Apache-2.0
