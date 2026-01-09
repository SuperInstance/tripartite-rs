# Contributing to privox

Thank you for your interest in contributing to privox! This document provides guidelines and instructions for contributing.

## Development Setup

```bash
# Clone the repository
git clone https://github.com/SuperInstance/privox.git
cd privox

# Install development dependencies
cargo install cargo-watch cargo-tarpaulin

# Run tests
cargo test

# Run with auto-reload
cargo watch -x test

# Check code style
cargo fmt --all -- --check

# Run linter
cargo clippy --all-features -- -D warnings
```

## Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_email_detection

# Run with coverage
cargo tarpaulin --out Html
```

## Code Style

- Use `cargo fmt` for formatting
- Follow Rust naming conventions
- Add doc comments to all public APIs
- Keep functions focused and small
- Write tests for new features

## Submitting Changes

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes
4. Run tests: `cargo test`
5. Run linter: `cargo clippy`
6. Commit your changes: `git commit -am 'Add my feature'`
7. Push to branch: `git push origin feature/my-feature`
8. Open a Pull Request

## Areas Where We'd Love Help

- Additional PII patterns (international formats)
- Performance optimizations
- Documentation improvements
- Integration examples with popular frameworks
- Bug fixes

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
