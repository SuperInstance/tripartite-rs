# SuperInstance CLI - Complete User Guide

> **Interface**: Command-Line Interface (Rust)
> **Status**: ✅ Production Ready
> **Target Users**: Developers, power users, DevOps engineers

---

## Table of Contents

1. [Installation](#installation)
2. [First-Time Setup](#first-time-setup)
3. [Core Commands](#core-commands)
4. [Knowledge Management](#knowledge-management)
5. [Model Management](#model-management)
6. [Cloud Integration](#cloud-integration)
7. [Advanced Usage](#advanced-usage)
8. [Configuration](#configuration)
9. [Troubleshooting](#troubleshooting)

---

## Installation

### Prerequisites

- **Rust**: 1.70+ (for building from source)
- **Git**: For cloning the repository
- **System**: Windows, macOS, or Linux

### Install from Source

```bash
# Clone the repository
git clone https://github.com/SuperInstance/Tripartite1.git
cd Tripartite1

# Build release binary (optimized, ~5-10 min)
cargo build --release

# (Optional) Install globally
sudo cp target/release/synesis /usr/local/bin/
# On Windows: Add target\release to PATH
```

### Install via Cargo (Future)

```bash
cargo install synesis-cli
```

### Verify Installation

```bash
synesis --version
# Output: synesis 0.2.0

synesis --help
# Output: Command-line interface for SuperInstance AI
```

---

## First-Time Setup

### Initialize SuperInstance

```bash
synesis init
```

This command:
1. Creates `~/.synesis/` directory structure
2. Downloads default models (Phi-3, Llama-3.2-3B)
3. Generates hardware manifest
4. Creates default configuration

### Directory Structure

```
~/.synesis/
├── config.toml           # Main configuration
├── models/               # Downloaded models
│   ├── phi-3/
│   └── llama-3.2-3B/
├── knowledge/            # Vector database
│   └── vault.db
├── manifests/            # Hardware profiles
├── logs/                 # Activity logs
└── credentials/          # Cloud API keys
```

### Interactive Setup

```bash
$ synesis init
✓ Detecting hardware...
  CPU: Apple M2 (8 cores)
  RAM: 16 GB
  GPU: Not available

✓ Creating directory structure...
  ~/.synesis/
  ~/.synesis/models/
  ~/.synesis/knowledge/

✓ Downloading models...
  [====================] 100% phi-3 (2.3 GB)
  [====================] 100% llama-3.2-3B (2.8 GB)

✓ Generating hardware manifest...
  Tier: MEDIUM (CPU-only, 16GB RAM)

✓ Creating configuration...
  ~/.synesis/config.toml

✓ Setup complete!

Run 'synesis status' to verify everything is working.
```

---

## Core Commands

### Check System Status

```bash
synesis status
```

**Output**:
```
SuperInstance AI v0.2.0

System Status:
  State: ✓ Operational
  Uptime: 2h 34m

Hardware:
  Tier: MEDIUM (CPU-only, 16GB RAM)
  CPU: Apple M2 @ 3.5 GHz
  RAM: 14.2 GB / 16 GB used
  GPU: Not available

Models:
  ✓ phi-3 (3.8B params, 2.3 GB)
  ✓ llama-3.2-3B (3.0B params, 2.8 GB)

Knowledge Vault:
  Indexes: 3
  Chunks: 1,247
  Embeddings: 1,247

Council:
  Agents: 3/3 active
  Consensus Rate: 94.2%
  Avg Latency: 2.3s
```

### Ask Questions

```bash
# Simple question
synesis ask "Explain Rust's ownership system"

# With context from knowledge base
synesis ask "How does the authentication module work?"

# With max tokens
synesis ask --max-tokens 2048 "Write a detailed comparison of Rust and Go"

# With tone
synesis ask --tone technical "Explain zero-cost abstractions"

# With verbosity
synesis ask --verbosity concise "What is a closure?"
```

### View Metrics

```bash
synesis metrics
```

**Output**:
```
SuperInstance Metrics (Last 24h)

Queries:
  Total: 1,247
  Success Rate: 98.3%
  Avg Latency: 2.3s
  P50: 1.8s | P95: 4.2s | P99: 6.1s

Council Performance:
  Consensus Rounds: 1.2 avg
  Timeout Rate: 1.7%
  Pathos Participation: 100%
  Logos Participation: 100%
  Ethos Participation: 100%

Resource Usage:
  CPU: 45% avg
  RAM: 8.2 GB avg
  Model Loading Time: 1.2s

Knowledge Vault:
  Queries: 342
  Avg Results: 4.2
  Hit Rate: 67.3%
```

---

## Knowledge Management

### Add Knowledge

```bash
# Add entire directory
synesis knowledge add ~/projects/my-app/

# Add specific file
synesis knowledge add ~/docs/ARCHITECTURE.md

# Add with metadata
synesis knowledge add ~/docs/ \
  --doc-type "markdown" \
  --source "internal-docs"

# Add with custom chunk size
synesis knowledge add ~/src/ \
  --chunk-size 1000 \
  --overlap 200
```

### List Knowledge

```bash
synesis knowledge list
```

**Output**:
```
Knowledge Vault Indexes

┌─────────────────────────────┬─────────┬────────┬──────────┐
│ Index Name                  │ Chunks  │ Source │ Updated  │
├─────────────────────────────┼─────────┼────────┼──────────┤
│ my-app-rust-code            │ 847     │ local  │ 2m ago   │
│ internal-docs               │ 234     │ local  │ 1h ago   │
│ architecture-docs           │ 166     │ local  │ 3d ago   │
└─────────────────────────────┴─────────┴────────┴──────────┘

Total: 1,247 chunks across 3 indexes
```

### Search Knowledge

```bash
# Direct search (bypassing council)
synesis knowledge search "authentication"

# Top-k results
synesis knowledge search "database schema" --top-k 10

# Filter by source
synesis knowledge search "API" --source my-app
```

**Output**:
```
Top 5 Results for "authentication":

1. [0.94] src/auth/mod.rs
   /// Authentication module using JWT tokens
   /// Supports refresh tokens and revocation

2. [0.89] docs/api/auth.md
   # Authentication API
   All endpoints require Bearer token...

3. [0.87] src/auth/jwt.rs
   pub struct Claims {
       pub sub: String,
       pub exp: usize,
   }

4. [0.82] src/middleware/auth.rs
   pub async fn auth_middleware(
       req: ServiceRequest,
   ) -> Result<ServiceResponse, Error>

5. [0.78] docs/auth/flows.md
   ## Authentication Flow
   1. Client sends credentials...
```

### Remove Knowledge

```bash
# Remove by index name
synesis knowledge remove my-app-rust-code

# Clear all knowledge
synesis knowledge clear --confirm
```

### Rebuild Indexes

```bash
# Rebuild all indexes
synesis knowledge rebuild

# Rebuild specific index
synesis knowledge rebuild my-app-rust-code
```

---

## Model Management

### List Available Models

```bash
synesis model list
```

**Output**:
```
Available Models

┌──────────────────┬──────────┬─────────┬────────┬────────┐
│ Model            │ Params   │ Size    │ Type   │ Status │
├──────────────────┼──────────┼─────────┼────────┼────────┤
│ phi-3            │ 3.8B     │ 2.3 GB  │ Quant.│ ✓ Act. │
│ llama-3.2-3B     │ 3.0B     │ 2.8 GB  │ Quant.│ ✓ Act. │
│ mistral-7B       │ 7.0B     │ 5.4 GB  │ Quant.│ ✗ Inv. │
├──────────────────┴──────────┴─────────┴────────┴────────┤
│ Legend: Quant.=Quantized, Act.=Active, Inv.=Installed  │
└──────────────────────────────────────────────────────────┘
```

### Download Models

```bash
# Download specific model
synesis model download mistral-7B

# Download with quantization
synesis model download llama-3.2-3B --quantization q4

# List available for download
synesis model available
```

### Remove Models

```bash
# Remove model
synesis model remove mistral-7B

# Force remove (even if active)
synesis model remove phi-3 --force
```

### Set Default Model

```bash
# Set default for specific tier
synesis model default set phi-3

# Show current default
synesis model default show
```

---

## Cloud Integration

### Cloud Login

```bash
# Interactive login
synesis cloud login

# With API key
synesis cloud login --api-key sk-xxxxx

# Device code flow
synesis cloud login --device
```

### Cloud Status

```bash
synesis cloud status
```

**Output**:
```
Cloud Account Status

┌──────────────────────────────────────────┐
│ Status: ✓ Connected                       │
│ Account: casey@example.com                │
│ Plan: Pay-as-you-go                       │
│ Balance: $24.50                           │
│ This Month: $12.30 used                   │
├──────────────────────────────────────────┤
│ Endpoint: api.superinstance.ai            │
│ Region: us-west-1                         │
│ Latency: 45ms                             │
└──────────────────────────────────────────┘
```

### Escalate to Cloud

```bash
# Auto-select model
synesis cloud ask "Explain quantum computing"

# Specify model
synesis cloud ask --model sonnet "Help debug this issue"

# Stream response
synesis cloud ask --stream "Write a Rust async runtime"

# With tone
synesis cloud ask --tone casual "What's new in AI?"
```

### Upload LoRA

```bash
synesis cloud push \
  --file ./my-lora.gguf \
  --name "my-custom-lora" \
  --base-model "llama-3.2-3B" \
  --description "Fine-tuned on medical data"
```

---

## Advanced Usage

### Hardware Manifests

```bash
# Show current manifest
synesis manifest show

# Detect hardware
synesis manifest detect

# Generate new manifest
synesis manifest generate

# Apply manifest
synesis manifest apply custom-manifest.toml
```

### Configuration

```bash
# Show config
synesis config show

# Edit config
synesis config edit

# Set value
synesis config set council.threshold 0.9

# Reset to default
synesis config reset
```

### Logging

```bash
# View logs
synesis logs

# Follow logs
synesis logs --follow

# Filter by component
synesis logs --filter council

# Last N lines
synesis logs --tail 100
```

---

## Configuration

### Config File Location

```
~/.synesis/config.toml
```

### Example Configuration

```toml
[council]
threshold = 0.85
max_rounds = 3
timeout_secs = 30

[council.agents]
pathos_weight = 0.34
logos_weight = 0.33
ethos_weight = 0.33

[knowledge]
chunk_size = 500
overlap = 50
max_results = 5
embedding_model = "bge-micro"

[models]
default = "phi-3"
download_path = "~/.synesis/models"
cache_size_mb = 2048

[cloud]
enabled = true
endpoint = "api.superinstance.ai"
timeout_secs = 30
auto_escalate = false

[logging]
level = "info"
file = "~/.synesis/logs/synesis.log"
max_size_mb = 100
```

### Environment Variables

```bash
# Override config path
export SYNESIS_CONFIG=/custom/path/config.toml

# Override models directory
export SYNESIS_MODELS=/custom/models

# Override knowledge directory
export SYNESIS_KNOWLEDGE=/custom/knowledge

# Enable debug logging
export SYNESIS_LOG=debug

# Cloud API key (alternative to login)
export SYNESIS_API_KEY=sk-xxxxx
```

---

## Troubleshooting

### Model Not Loading

**Problem**: Model fails to load with "out of memory" error.

**Solutions**:
```bash
# Check RAM usage
synesis status

# Use smaller model
synesis model default set phi-3

# Reduce model cache
synesis config set models.cache_size_mb 1024

# Close other applications
# On Linux: free -h
# On macOS: vm_stat
# On Windows: Task Manager
```

### Knowledge Index Slow

**Problem**: Knowledge queries are slow (>5s).

**Solutions**:
```bash
# Rebuild indexes
synesis knowledge rebuild

# Reduce chunk size for faster retrieval
synesis config set knowledge.chunk_size 300

# Limit results
synesis knowledge search "query" --top-k 3

# Check disk I/O
# On Linux: iotop
# On macOS: iostat
```

### Council Timeout

**Problem**: Council frequently times out.

**Solutions**:
```bash
# Increase timeout
synesis config set council.timeout_secs 60

# Reduce threshold
synesis config set council.threshold 0.80

# Reduce max rounds
synesis config set council.max_rounds 2

# Check which agent is slow
synesis metrics
```

### Cloud Connection Failed

**Problem**: Cannot connect to cloud.

**Solutions**:
```bash
# Check credentials
synesis cloud status

# Re-authenticate
synesis cloud login

# Check network
ping api.superinstance.ai

# Test tunnel
synesis cloud ping
```

### Enable Debug Logging

```bash
# Temporary (current session)
synesis --log-level debug ask "test query"

# Persistent (config file)
synesis config set logging.level debug

# Via environment variable
export SYNESIS_LOG=debug
synesis ask "test query"
```

### Reset Everything

```bash
# WARNING: Deletes all data
synesis reset --hard

# Preserves models
synesis reset --keep-models

# Backup first
cp -r ~/.synesis ~/.synesis.backup
synesis reset --hard
```

---

## Performance Tips

### 1. Use Quantized Models

```bash
# Q4 quantization: ~4x smaller, ~95% quality
synesis model download llama-3.2-3B --quantization q4
```

### 2. Optimize Knowledge Chunks

```bash
# Smaller chunks = faster retrieval
synesis config set knowledge.chunk_size 300
synesis config set knowledge.overlap 30
```

### 3. Cache Models in RAM

```bash
# Increase cache (if you have RAM to spare)
synesis config set models.cache_size_mb 4096
```

### 4. Use Streaming for Long Responses

```bash
# Stream reduces perceived latency
synesis cloud ask --stream "explain quantum computing"
```

### 5. Parallel Knowledge Indexing

```bash
# Index multiple sources in parallel
synesis knowledge add ~/src/ &
synesis knowledge add ~/docs/ &
wait
```

---

## Keyboard Shortcuts

### Interactive Mode

```bash
synesis ask --interactive
```

**Shortcuts**:
- `Ctrl+C`: Cancel query
- `Ctrl+D`: End multi-line input
- `Ctrl+L`: Clear screen
- `↑/↓`: Navigate history
- `Ctrl+R`: Search history
- `Ctrl+C` (twice): Force exit

---

## Examples

### Development Workflow

```bash
# 1. Initialize project
synesis init

# 2. Index codebase
synesis knowledge add ~/projects/my-rust-app/

# 3. Ask questions
synesis ask "How does the authentication work?"
synesis ask "Find all functions that call the database"

# 4. Generate documentation
synesis ask "Write API documentation for the auth module"

# 5. Debug with help
synesis ask "Why is this Rust code not compiling?" < code.rs
```

### Daily Usage

```bash
# Morning status check
synesis status

# Add new documentation
synesis knowledge add ~/docs/new-feature.md

# Query throughout the day
synesis ask "Summarize the new feature"

# Evening metrics
synesis metrics
```

---

## Getting Help

### Built-in Help

```bash
synesis --help
synesis ask --help
synesis knowledge --help
```

### Version Info

```bash
synesis --version
# Output: synesis 0.2.0

synesis --version --verbose
# Output:
# synesis 0.2.0
# Build: 2026-01-08
# Commit: 528c53b
# Rust: 1.75.0
```

### Report Issues

- **GitHub Issues**: https://github.com/SuperInstance/Tripartite1/issues
- **Discussions**: https://github.com/SuperInstance/Tripartite1/discussions
- **Documentation**: https://docs.superinstance.ai

---

**Last Updated**: 2026-01-08
**CLI Version**: 0.2.0
**Guide Version**: 1.0
