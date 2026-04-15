# SuperInstance AI

> **Privacy-first, local-first AI with tripartite consensus**

[![CI](https://github.com/SuperInstance/Tripartite1/actions/workflows/ci.yml/badge.svg)](https://github.com/SuperInstance/Tripartite1/actions/workflows/ci.yml)
[![Documentation](https://github.com/SuperInstance/Tripartite1/actions/workflows/documentation.yml/badge.svg)](https://github.com/SuperInstance/Tripartite1/actions/workflows/documentation.yml)
[![Security](https://github.com/SuperInstance/Tripartite1/actions/workflows/security.yml/badge.svg)](https://github.com/SuperInstance/Tripartite1/actions/workflows/security.yml)
[![codecov](https://codecov.io/gh/SuperInstance/Tripartite1/branch/main/graph/badge.svg)](https://codecov.io/gh/SuperInstance/Tripartite1)
[![License](https://img.shields.io/badge/license-MIT%20%7C%20Apache--2.0-blue.svg)](LICENSE-APACHE)
[![Rust Version](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Phase](https://img.shields.io/badge/phase-2%20%7C%20Cloud%20Mesh-brightgreen.svg)](phases/PHASE_2_DETAILED_ROADMAP.md)
[![Tests](https://img.shields.io/badge/tests-298%2F298-brightgreen.svg)](https://github.com/SuperInstance/Tripartite1)

**SuperInstance AI** is a revolutionary agentic AI system that prioritizes your privacy through local processing while enabling intelligent cloud escalation when needed. Unlike traditional AI chatbots, SuperInstance uses a **tripartite consensus system** where three specialized AI agents—Pathos, Logos, and Ethos—must agree before responding.

## 🎯 What Makes SuperInstance Different?

### Tripartite Consensus System

Three specialized agents collaborate on every query:

- **Pathos** (Intent): *"What does the user actually want?"*
- **Logos** (Logic): *"How do we accomplish this?"*
- **Ethos** (Truth): *"Is this safe, accurate, and feasible?"*

**No response is emitted until all three agents agree.**

### Privacy-First Architecture

- 🔒 **All sensitive data is tokenized** before cloud processing
- 🏠 **Local-first by default**—your data stays on your machine
- 🔐 **18 built-in redaction patterns** (emails, API keys, credentials, etc.)
- 🛡️ **Local token vault**—mappings never leave your device
- 🔄 **Automatic re-inflation**—responses restored locally

### Local-First Processing

- ⚡ **Automatic hardware detection** (CPU, GPU, RAM, disk)
- 🎯 **Intelligent model selection** based on available resources
- 📚 **Local knowledge vault** with RAG capabilities
- 💾 **Works completely offline** after initial setup
- 🌐 **Optional cloud escalation** for complex tasks (Phase 2)

## 🚀 Quick Start

### Prerequisites

- **Rust** 1.75+ ([install via rustup](https://rustup.rs/))
- **C compiler** and OpenSSL headers
- **8GB RAM** minimum (16GB recommended)

### Installation

```bash
# Clone the repository
git clone https://github.com/SuperInstance/Tripartite1.git
cd Tripartite1

# Build release binary
cargo build --release

# Initialize the system
./target/release/synesis init

# Run your first query
./target/release/synesis ask "What is the capital of France?"
```

**Output:**
```
🤔 Pathos (Intent): User wants factual information about French geography
🧠 Logos (Logic): Retrieving knowledge about capital cities...
✅ Ethos (Truth): Verifying factual accuracy...

✅ Consensus reached (0.95 confidence)

The capital of France is Paris.

---
Agents: 3/3 agreed | Confidence: 95% | Time: 2.3s
```

## 📚 Usage Examples

### Basic Query

```bash
synesis ask "Explain how vector databases work"
```

### Knowledge Vault (RAG)

```bash
# Add your documents
synesis knowledge add ~/Documents/my-project/

# Query your codebase
synesis ask "How does the authentication system work?"
```

### Custom Configuration

```bash
# Adjust consensus threshold
synesis config set consensus.threshold 0.90

# Change model
synesis config set agents.pathos.model phi-3-mini
```

### View System Status

```bash
synesis status

# Output:
# ┌─────────────┬──────────────────┐
# │ Component   │ Status           │
# ├─────────────┼──────────────────┤
# │ CPU         │ 16 cores @ 3.5GHz│
# │ GPU         │ NVIDIA RTX 4090  │
# │ RAM         │ 32 GB            │
# │ Model       │ phi-3-mini       │
# └─────────────┴──────────────────┘
```

## 🏗️ Architecture

```
User Query
     │
     ▼
┌───────────────────────────────────┐
│         Privacy Proxy             │ ← Redact sensitive data
└─────────────┬─────────────────────┘
              │
┌─────────────▼─────────────────────┐
│      Tripartite Council           │
│  ┌────────┐ ┌────────┐ ┌────────┐│
│  │ Pathos │ │  Logos │ │  Ethos ││ ← Three agents
│  └───┬────┘ └───┬────┘ └───┬────┘│
│      └───────────┼──────────────┘│
│                  │               │
│         ┌────────▼────────┐      │
│         │ Consensus Engine │      │ ← Weighted voting
│         └────────┬────────┘      │
└───────────────────┼───────────────┘
                   │
         ┌─────────┴─────────┐
         ▼                   ▼
    Local Models      Cloud Escalation
   (phi-3, llama)      (Claude, GPT-4)
         │                   │
    ┌────▼────┐       ┌─────▼─────┐
    │Knowledge│       │   QUIC    │
    │  Vault  │       │  Tunnel   │
    └─────────┘       └───────────┘
```

**Learn More**: [ARCHITECTURE.md](ARCHITECTURE.md) | [Developer Guide](DEVELOPER_GUIDE.md)

## 🎓 Key Features

### Tripartite Consensus

- **Multi-agent deliberation**: Each agent brings unique perspective
- **Weighted voting**: Not all agents equal (Ethos has veto power)
- **Revision rounds**: Agents negotiate if initial consensus is low
- **Transparent**: See how each agent contributed

### Privacy & Security

- **18 redaction patterns**: Emails, API keys, phone numbers, SSNs, etc.
- **Token vault**: Local SQLite database, never transmitted
- **Re-inflation**: Only happens locally on your device
- **mTLS**: All cloud communication uses mutual TLS (Phase 2)

### Knowledge Vault (RAG)

- **SQLite-VSS**: Fast vector search on local documents
- **Automatic chunking**: Multiple strategies (paragraph, sentence, fixed)
- **Semantic search**: Find relevant information in your codebase
- **Source citation**: Responses include where information came from

### Performance

- **Parallel execution**: Agents run concurrently (25-33% latency reduction)
- **Hardware acceleration**: GPU support (NVIDIA, AMD, Apple Silicon)
- **Model caching**: First query slower, subsequent queries fast
- **Resource efficient**: Works on 8GB RAM (16GB recommended)

## 📖 Documentation

### For Users

- **[Getting Started Tutorial](docs/tutorials/getting-started.md)** - Installation and first query
- **[Your First Query](docs/tutorials/your-first-query.md)** - Understanding the tripartite system
- **[Knowledge Vault Guide](docs/tutorials/knowledge-vault.md)** - Using RAG with your documents
- **[Privacy Basics](docs/tutorials/privacy-basics.md)** - How privacy features work
- **[FAQ](docs/reference/faq.md)** - Frequently asked questions
- **[Glossary](docs/reference/glossary.md)** - Terminology and concepts

### For Developers

- **[Developer Guide](DEVELOPER_GUIDE.md)** - Contribution and development workflow
- **[Architecture Deep Dive](ARCHITECTURE.md)** - System design and internals
- **[API Documentation](https://docs.rs/synesis-core/)** - Rust API reference
- **[Examples](examples/)** - Runnable code examples
- **[Testing Guide](docs/contributing/testing-guide.md)** - How to write tests

### Phase Documentation

- **[Phase 1: Local Kernel](phases/PHASE_1_LOCAL_KERNEL.md)** ✅ Complete
- **[Phase 2: Cloud Mesh](phases/PHASE_2_DETAILED_ROADMAP.md)** 🔄 In Progress (33%)
- **[Phase 3: Marketplace](phases/PHASE_3_MARKETPLACE.md)** - Planned
- **[Phase 4: Utility](phases/PHASE_4_UTILITY.md)** - Planned

## 🛠️ CLI Commands

```bash
# Query the AI
synesis ask "Your question here"

# Knowledge management
synesis knowledge add <path>          # Add documents
synesis knowledge search "query"       # Search vault
synesis knowledge stats                # View statistics

# Configuration
synesis config list                   # List all settings
synesis config get <key>              # Get setting
synesis config set <key> <value>      # Change setting

# System information
synesis status                        # View system status
synesis metrics show                  # View performance metrics

# Model management
synesis model list                    # List available models
synesis model download <model>        # Download a model
synesis model info <model>            # Model details
```

## 💡 Use Cases

### For Developers

- **Code understanding**: "How does the authentication flow work?"
- **Bug investigation**: "Why is this function returning an error?"
- **Code review**: "What are the potential issues with this code?"
- **Documentation**: "Generate docs for this API endpoint"

### For Researchers

- **Literature review**: "Summarize recent papers on vector databases"
- **Concept explanation**: "Explain Rust ownership with examples"
- **Technical writing**: "Write a technical description of this system"

### For Writers

- **Content generation**: "Write blog post about async Rust"
- **Editing**: "Improve clarity and flow of this paragraph"
- **Ideation**: "Brainstorm features for a mobile app"

### For Everyone

- **Learning**: "Teach me about machine learning"
- **Analysis**: "Compare and contrast these two approaches"
- **Decision making**: "What are the trade-offs between SQL and NoSQL?"

## 🔧 System Requirements

### Minimum (CPU-only)

- 8 GB RAM
- 10 GB disk space
- x86_64 or ARM64 CPU

### Recommended

- 16 GB RAM
- 4 GB VRAM (NVIDIA GPU)
- 25 GB disk space
- Ubuntu 22.04+ / macOS 12+ / Windows 10+

### Optimal

- 32 GB RAM
- 8 GB VRAM (NVIDIA RTX 3060+)
- NVMe storage
- Dedicated GPU (NVIDIA, AMD, or Apple Silicon)

## 📦 Project Status

- **Version**: v0.2.0
- **Phase**: Phase 1 (Local Kernel) ✅ Complete | Phase 2 (Cloud Mesh) 🔄 33% Complete
- **Tests**: 250+ passing (100%)
- **Code Quality**: Zero warnings (all library crates)
- **Documentation**: Comprehensive (70+ markdown files)

### Completed Features (Phase 1)

- ✅ Tripartite council with three agents
- ✅ Consensus engine with multi-round negotiation
- ✅ Privacy proxy with 18 redaction patterns
- ✅ Knowledge vault with RAG and semantic search
- ✅ Hardware detection and model management
- ✅ CLI with all commands
- ✅ Comprehensive testing (250+ tests)
- ✅ Zero compiler warnings

### In Progress (Phase 2)

- 🔄 QUIC tunnel with mTLS (Sessions 2.1-2.2 complete)
- 🔄 Device telemetry and heartbeat (Session 2.3 complete)
- 🔄 Cloud escalation client (Session 2.4 in progress)
- ⏳ Billing integration (Session 2.6)
- ⏳ Cloudflare Workers deployment (Session 2.7)

## 🤝 Contributing

We welcome contributions! SuperInstance is a community-driven project.

### Good First Issues

- 📚 Improve documentation
- 🧪 Add tests
- 🐛 Fix bugs
- ✨ Add features

**See**: [CONTRIBUTING.md](CONTRIBUTING.md) | [Developer Guide](DEVELOPER_GUIDE.md)

### Development Workflow

1. Read [Developer Guide](DEVELOPER_GUIDE.md)
2. Set up development environment
3. Pick an issue or create one
4. Fork and create a branch
5. Make your changes
6. Add tests and documentation
7. Submit a pull request

## 📊 Performance

| Metric | Local (CPU) | Local (GPU) | Cloud |
|--------|------------|-------------|-------|
| First query | 5-8s | 3-5s | 2-3s |
| Subsequent | 2-3s | 1-2s | 1-2s |
| Memory usage | 4-8 GB | 6-12 GB | N/A |
| Privacy | 100% | 100% | Tokenized |

*Benchmarks on: Intel i7-12700K, 32GB RAM, NVIDIA RTX 4090*

## 🗺️ Roadmap

### Phase 1: Local Kernel ✅ COMPLETE
- Tripartite consensus system
- Privacy proxy with redaction
- Knowledge vault with RAG
- Hardware detection
- CLI interface

### Phase 2: Cloud Mesh 🔄 IN PROGRESS (33%)
- QUIC tunnel with mTLS
- Cloud escalation (Claude, GPT-4)
- Billing and metering
- LoRA hot-swap
- Collaborator system

### Phase 3: Marketplace ⏳ PLANNED
- LoRA training
- Knowledge marketplace
- Model sharing
- Monetization

### Phase 4: Utility ⏳ PLANNED
- SDKs (Python, JavaScript)
- Desktop application
- Mobile SDK
- Distributed mode

**See**: [PROJECT_ROADMAP.md](PROJECT_ROADMAP.md) for details

## 🔐 Privacy & Security

SuperInstance is designed with privacy as a core principle:

### Data Protection

- ✅ **Local processing by default**: Your data never leaves your device
- ✅ **Tokenization before cloud**: Sensitive info replaced with UUIDs
- ✅ **Local token vault**: Mappings stored locally (SQLite)
- ✅ **mTLS encryption**: All cloud communication encrypted (Phase 2)
- ✅ **Open source**: Fully auditable codebase

### Redaction Patterns

Built-in patterns for:
- Email addresses
- API keys (GitHub, AWS, OpenAI, etc.)
- Phone numbers
- Social Security Numbers
- Credit card numbers
- Passwords
- IP addresses
- And 10 more...

**See**: [Privacy Basics Tutorial](docs/tutorials/privacy-basics.md)

## 🧪 Testing

```bash
# Run all tests
cargo test --workspace

# Run specific crate tests
cargo test -p synesis-core
cargo test -p synesis-knowledge
cargo test -p synesis-privacy

# Run with output
cargo test --workspace -- --nocapture

# Test coverage
cargo test --workspace --all-features
```

**Test Results**: 250+ tests passing (100%)

## 📝 License

Licensed under either of:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT))
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

at your option.

## 🙏 Acknowledgments

Built with amazing open-source projects:

- **[llama.cpp](https://github.com/ggerganov/llama.cpp)** - Local LLM inference
- **[SQLite](https://sqlite.org/)** + **[SQLite-VSS](https://github.com/asg017/sqlite-vss)** - Vector database
- **[Tokio](https://tokio.rs/)** - Async runtime
- **[Quinn](https://github.com/quinn-rs/quinn)** - QUIC implementation
- **[Cloudflare Workers](https://workers.cloudflare.com/)** - Edge compute (Phase 2)

## 📞 Contact & Support

### Getting Help

- **[Documentation](docs/)** - Start here
- **[FAQ](docs/reference/faq.md)** - Common questions
- **[Troubleshooting](TROUBLESHOOTING.md)** - Solve problems
- **[GitHub Issues](https://github.com/SuperInstance/Tripartite1/issues)** - Report bugs
- **[GitHub Discussions](https://github.com/SuperInstance/Tripartite1/discussions)** - Ask questions

### Community

- **GitHub**: [SuperInstance/Tripartite1](https://github.com/SuperInstance/Tripartite1)
- **Star** ⭐ us if you find SuperInstance useful!
- **Watch** 👀 to track progress
- **Fork** 🍴 to contribute

---

**SuperInstance AI** - *Your AI, your way, your privacy.*

**Version**: 0.2.0 | **Status**: Production-Ready (Phase 1) | **Tests**: 250+ Passing ✅

*Last Updated: 2026-01-07*

---

## 📐 Tripartite Protocol Architecture

### The Three-Agent Model

The tripartite consensus system draws inspiration from Aristotle's rhetorical triangle. Each agent embodies a distinct reasoning modality:

```
                    ┌─────────────────────────────┐
                    │      User Query             │
                    └──────────────┬──────────────┘
                                   │
              ┌────────────────────┼────────────────────┐
              │                    │                    │
       ┌──────▼──────┐     ┌──────▼──────┐     ┌──────▼──────┐
       │   PATHOS    │     │   LOGOS     │     │   ETHOS     │
       │  (Intent)   │     │  (Logic)    │     │  (Truth)    │
       ├─────────────┤     ├─────────────┤     ├─────────────┤
       │ • Goal      │     │ • Method    │     │ • Safety     │
       │ • Context   │     │ • Steps     │     │ • Accuracy   │
       │ • Nuance    │     │ • Feasibility│    │ • Ethics     │
       │ • Emotion   │     │ • Logic     │     │ • Constraints│
       └──────┬──────┘     └──────┬──────┘     └──────┬──────┘
              │                   │                    │
              └───────────────────┼────────────────────┘
                                  │
                        ┌─────────▼─────────┐
                        │ Consensus Engine  │
                        │ Weighted Voting   │
                        │ Revision Rounds   │
                        │ Confidence Score  │
                        └─────────┬─────────┘
                                  │
                    ┌─────────────▼──────────────┐
                    │ Unified Response           │
                    │ + Agent Contributions      │
                    │ + Confidence Level         │
                    │ + Processing Time          │
                    └────────────────────────────┘
```

### Mathematical Foundation

#### Consensus Scoring Algorithm

The consensus engine uses a weighted voting mechanism with veto power:

```
C = (w_P · S_P + w_L · S_L + w_E · S_E) / (w_P + w_L + w_E)

Where:
  C  = consensus score [0, 1]
  S_P = Pathos score (intent alignment)
  S_L = Logos score (logical coherence)
  S_E = Ethos score (safety & accuracy)
  w_P = 0.3 (Pathos weight)
  w_L = 0.4 (Logos weight — logic dominant)
  w_E = 0.5 (Ethos weight — has veto power)
```

**Veto Mechanism**: If `S_E < threshold_veto` (default: 0.5), the response is blocked regardless of other scores. This ensures safety-first behavior.

#### Multi-Round Negotiation

When initial consensus `C < threshold` (default: 0.90), agents enter revision rounds:

```
Round 1: Independent assessment → C₁ = 0.72 (below threshold)
Round 2: Agents see each other's assessments → C₂ = 0.85
Round 3: Targeted revision of disagreements → C₃ = 0.93 ✓ CONSENSUS
```

Maximum revision rounds: 5 (configurable). Falls back to safest partial response if consensus not reached.

#### Confidence Calibration

```
Final Confidence = C_consensus × (1 - decay(t)) × quality_factor

Where:
  decay(t)   = 1 - e^(-λt)     (time-based freshness)
  quality     = token_precision × source_reliability
  λ          = 0.001            (decay constant)
```

### Rust Crate Architecture

```
tripartite-rs/
├── crates/
│   ├── synesis-core/        # Core types, consensus engine, agent trait
│   │   ├── src/agent.rs     # Agent trait definition & implementations
│   │   ├── src/consensus.rs # Weighted voting, revision rounds
│   │   ├── src/manifest.rs  # Hardware manifest & capability detection
│   │   └── src/error.rs     # Error types
│   ├── synesis-privacy/     # Tokenization, redaction, vault
│   ├── synesis-knowledge/   # RAG, embeddings, vector search, chunking
│   ├── synesis-models/      # Model loading, inference abstraction
│   ├── synesis-cloud/       # QUIC tunnel, mTLS, cloud escalation (Phase 2)
│   └── synesis-cli/         # CLI binary (ask, knowledge, config, status)
├── tools/
│   └── privox/              # Standalone privacy proxy library
├── knowledge-vault-rs/      # Standalone knowledge vault library
├── cloud/                   # Cloudflare Workers for cloud mesh (Phase 2)
└── manifests/               # Hardware capability manifests
    ├── minimal.json          # 8GB RAM, CPU-only
    ├── standard.json         # 16GB RAM, CPU-only
    ├── apple-m2.json         # Apple Silicon M2
    ├── rtx-4050-laptop.json  # NVIDIA RTX 4050 Laptop
    └── jetson-orin-nano.json # NVIDIA Jetson Orin Nano
```

### Key API Reference

```rust
use synesis_core::{Agent, ConsensusEngine, Council};

// Create a tripartite council
let mut council = Council::new(config);

// Process a query
let result = council.process(query).await?;

// Inspect individual agent contributions
println!("Pathos: {:?}", result.agent_contributions.pathos);
println!("Logos:  {:?}", result.agent_contributions.logos);
println!("Ethos:  {:?}", result.agent_contributions.ethos);
println!("Consensus: {:.2}", result.consensus_score);
println!("Rounds: {}", result.revision_rounds);

// Access the knowledge vault
use synesis_knowledge::Vault;
let vault = Vault::open("./data/vault.db")?;
vault.add_document("./my-project/")?;
let results = vault.search("How does auth work?")?;
```

---

<img src="callsign1.jpg" width="128" alt="callsign">
