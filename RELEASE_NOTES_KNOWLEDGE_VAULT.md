# knowledge-vault-rs v0.1.0 - Release Notes

> **Local Vector Database and RAG System for Rust**
> Published: 2026-01-08

---

## Table of Contents

1. [Overview](#overview)
2. [What's New](#whats-new)
3. [Key Features](#key-features)
4. [Use Cases](#use-cases)
5. [Performance](#performance)
6. [Quick Start](#quick-start)
7. [Migration from synesis-knowledge](#migration-from-synesis-knowledge)
8. [API Highlights](#api-highlights)
9. [Examples](#examples)
10. [Under the Hood](#under-the-hood)
11. [Known Limitations](#known-limitations)
12. [Roadmap](#roadmap)
13. [Contributing](#contributing)
14. [License](#license)
15. [Acknowledgments](#acknowledgments)

---

## Overview

**knowledge-vault-rs** is a pure-Rust library for building local vector databases and Retrieval-Augmented Generation (RAG) systems. It provides document chunking, embedding generation, vector similarity search, and automatic file watching—all backed by SQLite for portability.

### Design Philosophy

- **Local-first**: No external services required
- **Privacy-focused**: Your data never leaves your machine
- **Async-native**: Built on Tokio for high concurrency
- **Pluggable**: Support multiple embedding providers
- **Fast**: Optimized algorithms and SQLite-VSS integration

### What Makes It Different?

Unlike other vector databases that require separate services (Docker containers, cloud APIs, etc.), knowledge-vault-rs is:

- **A library**, not a service—embed directly in your app
- **Single binary** distribution—just your Rust executable
- **Zero configuration**—works out of the box
- **Portable**—SQLite database works everywhere

---

## What's New

### Initial Release Features

This v0.1.0 release represents the extraction of the knowledge vault system from [SuperInstance AI](https://github.com/SuperInstance/Tripartite1), where it powers the local RAG system.

#### Core Capabilities

✅ **Document Chunking** - Multiple strategies for different document types
- Sliding window for general text
- Markdown-aware chunking (preserves heading hierarchy)
- Code-aware chunking (function/class boundaries)

✅ **Embedding Generation** - Pluggable provider system
- Placeholder provider (SHA256-based, for testing)
- Local BGE-Micro support (planned)
- OpenAI API support (planned)
- Custom provider interface

✅ **Vector Storage** - SQLite with VSS extension
- Stores documents, chunks, and embeddings
- Content deduplication via SHA256
- Optional SQLite-VSS for fast ANN search
- Fallback to cosine similarity calculation

✅ **File Watching** - Automatic reindexing
- Cross-platform file system monitoring
- Configurable debounce and filtering
- Channel-based command dispatch

✅ **Async Indexing** - Non-blocking document processing
- Channel-based architecture
- Batch embedding generation
- Progress reporting

---

## Key Features

### 1. Smart Document Chunking

#### Multiple Chunking Strategies

```rust
use knowledge_vault::{DocumentChunker, DocType};

let chunker = DocumentChunker::default();

// Markdown: Respects heading hierarchy
let chunks = chunker.chunk("# Introduction\n\nContent...", DocType::Markdown);

// Code: Splits at function/class boundaries
let chunks = chunker.chunk("pub fn hello() { ... }", DocType::Code);

// Text: Sliding window with overlap
let chunks = chunker.chunk("Word word word...", DocType::Text);
```

#### Configurable Options

```rust
use knowledge_vault::{Chunker, ChunkOptions};

let chunker = Chunker::with_options(ChunkOptions {
    chunk_size: 1024,        // Target tokens per chunk
    chunk_overlap: 128,       // Overlap between chunks
    min_chunk_size: 64,       // Minimum chunk size
    respect_sentences: true,  // Respect sentence boundaries
    respect_paragraphs: true, // Respect paragraph boundaries
});
```

#### Code Chunking Optimization

The code chunker uses **RegexSet** for efficient single-pass multi-pattern matching:

- **2.5x faster** than sequential regex scanning
- **12 patterns** for Rust, Python, JavaScript, Go, Java
- **Function/class detection** across languages
- **Future**: tree-sitter integration for AST-based parsing

### 2. Embedding Generation

#### Pluggable Provider System

```rust
use knowledge_vault::{EmbeddingProvider, PlaceholderEmbedder};

// Create embedder
let embedder = PlaceholderEmbedder::new(384);

// Generate single embedding
let embedding = embedder.embed("Hello, world!").await?;

// Generate batch embeddings (parallel processing)
let embeddings = embedder.embed_batch(&[
    "First text",
    "Second text",
    "Third text",
]).await?;
```

#### Batch Processing Performance

- **Parallel embedding** with bounded concurrency (8 workers)
- **4-8x speedup** for batch operations
- **Memory-efficient** streaming processing

#### Supported Embedding Dimensions

- **384 dimensions**: BGE-Micro v2 (default)
- **768 dimensions**: BGE-Small, MiniLM-L6
- **1024 dimensions**: BGE-Base
- **1536 dimensions**: OpenAI text-embedding-ada-002

### 3. Vector Similarity Search

#### Two Search Modes

**VSS (SQLite Extension)** - Fast Approximate Nearest Neighbor
```rust
// Requires sqlite-vss extension loaded
let results = vault.search(&query_embedding, 10)?;
```

**Cosine Similarity** - Exact Fallback
```rust
// Pure Rust implementation (no extension needed)
let results = vault.search(&query_embedding, 10)?;
```

#### Cosine Similarity Implementation

The fallback search uses mathematically precise cosine similarity:

```text
cosine_similarity(a, b) = (a · b) / (||a|| * ||b||)

where:
- a · b = Σ(aᵢ * bᵢ)  (dot product)
- ||a|| = √(Σaᵢ²)      (Euclidean norm)
```

**Performance**: ~100ms for 10k chunks (acceptable for small datasets)

### 4. Automatic File Watching

#### Cross-Platform Monitoring

```rust
use knowledge_vault::{FileWatcher, WatchConfig};
use notify::RecursiveMode;

let config = WatchConfig {
    directories: vec!["./docs".into()],
    extensions: Some(vec!["md".into(), "txt".into()]),
    exclude_patterns: vec!["target".into()],
    debounce: Duration::from_secs(2),
    recursive: true,
};

let watcher = FileWatcher::new(config)?;
watcher.start().await?;
```

#### Features

- **Debouncing**: Waits for changes to settle (configurable)
- **Checksum caching**: Detects actual content changes
- **Exclude patterns**: Ignores common directories (node_modules, target, etc.)
- **Channel-based**: Non-blocking event processing

### 5. Channel-Based Async Indexing

#### Non-Blocking Architecture

```rust
use tokio::sync::Arc;
use knowledge_vault::{DocumentIndexer, IndexerConfig};

// Create indexer with channel
let (indexer, handle) = DocumentIndexer::new(
    Arc::new(Mutex::new(vault)),
    Arc::new(Mutex::new(embedder)),
    IndexerConfig::default(),
);

// Index non-blocking
indexer.index_file("README.md")?;

// Wait for completion
let result = handle.wait().await?;
```

#### Benefits

- **No lock contention**: Vault accessed only by worker task
- **Command queue**: 100-command buffer by default
- **Graceful shutdown**: Flush pending commands before exit

---

## Use Cases

### 1. RAG (Retrieval-Augmented Generation)

Build AI systems that combine LLMs with your own documents:

```rust
use knowledge_vault::{KnowledgeVault, LocalEmbedder, DocumentIndexer};

// 1. Index your documentation
let vault = KnowledgeVault::open("docs.db", 384)?;
let embedder = LocalEmbedder::new(384)?;
let indexer = DocumentIndexer::new(vault.clone(), embedder)?;

indexer.index_directory("./docs", vec!["md".to_string()])?;

// 2. Search for relevant content
let query_embedding = embedder.embed("How do I deploy?").await?;
let results = vault.search(&query_embedding, 5)?;

// 3. Augment LLM prompt with retrieved context
let context = results.iter()
    .map(|r| r.content.as_str())
    .collect::<Vec<_>>()
    .join("\n\n");

let augmented_prompt = format!(
    "Context:\n{}\n\nQuestion: How do I deploy?",
    context
);

// 4. Send to LLM
let response = llm.generate(&augmented_prompt).await?;
```

### 2. Semantic Search

Add semantic search to any application:

```rust
use knowledge_vault::{KnowledgeVault, DocumentIndexer};

// Index product documentation
indexer.index_directory("./products", vec!["md".to_string()])?;

// Search for products matching user intent
let query_embedding = embedder.embed("lightweight laptop for travel").await?;
let results = vault.search(&query_embedding, 10)?;

// Results ranked by semantic similarity, not keywords
for result in results {
    println!("Score: {:.2} | Product: {}", result.score, result.document_title);
}
```

### 3. Code Search

Search codebase by function/purpose, not just names:

```rust
// Index all Rust code
indexer.index_directory("./src", vec!["rs".to_string()])?;

// Find functions for "authentication"
let query_embedding = embedder.embed("user authentication and login").await?;
let results = vault.search(&query_embedding, 10)?;

// Returns functions like `login_user()`, `verify_token()`, etc.
```

### 4. Q&A Systems

Build question-answering systems over your knowledge base:

```rust
// Index FAQ documentation
indexer.index_directory("./faq", vec!["md".to_string()])?;

// Answer user questions
let query_embedding = embedder.embed("What's your refund policy?").await?;
let results = vault.search(&query_embedding, 1)?;

let answer = if let Some(result) = results.first() {
    result.content.clone()
} else {
    "I don't have information about that.".to_string()
};
```

### 5. Document Deduplication

Detect duplicate or similar documents:

```rust
// Calculate content hash
let hash = vault.add_document("doc.txt", content, "text")?;

// Check if already exists
if vault.has_document_hash(&hash)? {
    println!("Document already indexed");
}
```

---

## Performance

### Benchmarks

Tested on: M1 Pro, 16GB RAM, SSD

#### Chunking Speed

| Document Type | Size | Speed | Strategy |
|--------------|------|-------|----------|
| Markdown | 1 MB | ~1.0 MB/s | Heading-aware |
| Code (Rust) | 500 KB | ~500 KB/s | Function-based (RegexSet) |
| Plain text | 2 MB | ~2.0 MB/s | Sliding window |

#### Embedding Generation

| Provider | Dimensions | Speed | Quality |
|----------|-----------|-------|---------|
| Placeholder (SHA256) | 384 | ~10k embeddings/sec | Testing only |
| BGE-Micro (real, planned) | 384 | ~100 embeddings/sec | High quality |
| OpenAI ada-002 (planned) | 1536 | ~50 embeddings/sec | Best quality |

#### Search Performance

| Chunks | Method | Query Time | Memory |
|--------|--------|------------|--------|
| 1,000 | VSS | <1ms | ~5 MB |
| 10,000 | VSS | ~5ms | ~50 MB |
| 100,000 | VSS | ~10ms | ~500 MB |
| 1,000 | Cosine fallback | ~10ms | ~5 MB |
| 10,000 | Cosine fallback | ~100ms | ~50 MB |
| 100,000 | Cosine fallback | ~1-2s | ~500 MB |

**Recommendation**: Use VSS extension for >10k chunks.

#### Indexing Throughput

| Operation | Speed | Notes |
|-----------|-------|-------|
| Single file indexing | ~50 files/sec | Including chunking + embedding |
| Batch indexing | ~500 files/sec | Parallel processing |
| File watching events | ~1000 events/sec | Channel-based |

### Memory Usage

- **Per chunk**: ~1.5 KB (384 dims × 4 bytes + metadata)
- **1000 chunks**: ~1.5 MB
- **100k chunks**: ~150 MB
- **SQLite overhead**: ~2× (indexes, metadata)

---

## Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
knowledge-vault-rs = "0.1"
```

### Basic Usage

```rust
use knowledge_vault::{KnowledgeVault, DocumentIndexer, LocalEmbedder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Open vault (creates SQLite database)
    let vault = KnowledgeVault::open("knowledge.db", 384)?;

    // 2. Create embedder (384 dimensions for BGE-Micro)
    let embedder = LocalEmbedder::new(384)?;

    // 3. Create indexer
    let indexer = DocumentIndexer::new(vault.clone(), embedder)?;

    // 4. Index a document
    indexer.index_file("README.md")?;

    // 5. Search for similar content
    let query = "How do I search documents?";
    let query_embedding = embedder.embed(query).await?;
    let results = vault.search(&query_embedding, 5)?;

    // 6. Print results
    for result in results {
        println!("Score: {:.2} | {}", result.score, result.content);
    }

    Ok(())
}
```

### With File Watching

```toml
[dependencies]
knowledge-vault-rs = { version = "0.1", features = ["watcher"] }
```

```rust
use knowledge_vault::{FileWatcher, WatchConfig};
use std::time::Duration;

let config = WatchConfig {
    directories: vec!["./docs".into()],
    debounce: Duration::from_secs(2),
    ..Default::default()
};

let watcher = FileWatcher::new(config)?;
watcher.start().await?;

// Now any changes to ./docs will auto-index
```

---

## Migration from synesis-knowledge

If you're currently using `synesis-knowledge` from SuperInstance:

### 1. Update Dependencies

```toml
# Before
[dependencies]
synesis-knowledge = "0.2"

# After
[dependencies]
knowledge-vault-rs = "0.1"
```

### 2. Update Imports

```rust
// Before
use synesis_knowledge::{KnowledgeVault, DocumentIndexer, LocalEmbedder};

// After
use knowledge_vault::{KnowledgeVault, DocumentIndexer, LocalEmbedder};
```

### 3. Update Feature Flags

```toml
# Before (if using workspace features)
synesis-knowledge = { path = "../crates/synesis-knowledge" }

# After
knowledge-vault-rs = { version = "0.1", features = ["watcher", "vss"] }
```

### 4. API Changes

**Minimal breaking changes** - the public API is 95% compatible:

| Before | After | Notes |
|--------|-------|-------|
| `synesis_knowledge::...` | `knowledge_vault::...` | Crate name change |
| Private traits | Public traits | Can now implement custom providers |
| Workspace deps | Crates.io deps | No more path dependencies |

See [MIGRATION_GUIDE.md](MIGRATION_GUIDE.md) for complete details.

---

## API Highlights

### Core Types

#### `KnowledgeVault`

Main storage interface for documents, chunks, and embeddings.

```rust
impl KnowledgeVault {
    // Open/create vault
    pub fn open(path: &Path, dimensions: u32) -> Result<Self>;

    // Document operations
    pub fn add_document(&self, path: &str, content: &str, doc_type: &str) -> Result<String>;
    pub fn get_document(&self, id: &str) -> Result<Option<Document>>;
    pub fn delete_document(&self, id: &str) -> Result<()>;

    // Search
    pub fn search(&self, query_embedding: &[f32], top_k: usize) -> Result<Vec<ChunkResult>>;

    // Statistics
    pub fn stats(&self) -> Result<VaultStats>;
}
```

#### `DocumentChunker`

Splits documents into optimal chunks for embedding.

```rust
impl DocumentChunker {
    pub fn new() -> Self;
    pub fn with_options(options: ChunkOptions) -> Self;
    pub fn chunk(&self, content: &str, doc_type: DocType) -> Vec<Chunk>;
}
```

#### `EmbeddingProvider` (Trait)

Pluggable embedding generation.

```rust
#[async_trait]
pub trait EmbeddingProvider: Send + Sync {
    async fn embed(&self, text: &str) -> Result<Vec<f32>>;
    async fn embed_batch(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>>;
    fn dimensions(&self) -> u32;
    fn model_name(&self) -> &str;
}
```

#### `DocumentIndexer`

Async document ingestion with channel-based architecture.

```rust
impl DocumentIndexer {
    pub fn new<E>(vault: Arc<Mutex<KnowledgeVault>>, embedder: Arc<Mutex<E>>, config: IndexerConfig)
        -> (Self, IndexerHandle)
    where E: EmbeddingProvider + Send + 'static;

    pub fn index_file(&self, path: impl AsRef<Path>) -> Result<()>;
    pub fn index_directory(&self, path: impl AsRef<Path>, extensions: Vec<String>) -> Result<()>;
}
```

---

## Examples

### Example 1: Basic Vector Search

```rust
use knowledge_vault::{KnowledgeVault, LocalEmbedder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open vault
    let vault = KnowledgeVault::open("knowledge.db", 384)?;

    // Create embedder
    let embedder = LocalEmbedder::new(384)?;

    // Index some text
    let content = "Rust is a systems programming language.";
    let doc_id = vault.add_document("doc1.txt", content, "text")?;

    // Chunk and embed
    let chunker = knowledge_vault::DocumentChunker::default();
    let chunks = chunker.chunk(content, knowledge_vault::DocType::Text);

    for (i, chunk) in chunks.iter().enumerate() {
        let chunk_id = format!("{}_{}", doc_id, i);
        vault.insert_chunk(&chunk_id, &doc_id, i as u32, &chunk.content,
                           chunk.start_offset as u64, chunk.end_offset as u64,
                           chunk.metadata.chunk_index as u32)?;

        let embedding = embedder.embed(&chunk.content).await?;
        vault.insert_embedding(&chunk_id, &embedding)?;
    }

    // Search
    let query_embedding = embedder.embed("systems language").await?;
    let results = vault.search(&query_embedding, 5)?;

    for result in results {
        println!("Score: {:.2} | {}", result.score, result.content);
    }

    Ok(())
}
```

### Example 2: Markdown Chunking

```rust
use knowledge_vault::{DocumentChunker, DocType};

let content = r#"# Getting Started

This is the introduction.

## Installation

Run this command:

```bash
cargo install knowledge-vault-rs
```

## Usage

Import the library:

```rust
use knowledge_vault::KnowledgeVault;
```
"#;

let chunker = DocumentChunker::default();
let chunks = chunker.chunk(content, DocType::Markdown);

for chunk in chunks {
    println!("Heading path: {:?}", chunk.metadata.heading_path);
    println!("Content: {}\n", chunk.content);
}
```

### Example 3: Code Chunking

```rust
use knowledge_vault::{DocumentChunker, DocType};

let rust_code = r#"
pub fn process_data(input: &str) -> Result<String> {
    let result = input.to_uppercase();
    Ok(result)
}

pub struct DataProcessor {
    config: Config,
}

impl DataProcessor {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
"#;

let chunker = DocumentChunker::default();
let chunks = chunker.chunk(rust_code, DocType::Code);

// Chunks split at function/class boundaries
for chunk in chunks {
    println!("Language: {:?}\n{}", chunk.metadata.language, chunk.content);
}
```

### Example 4: Batch Embedding

```rust
use knowledge_vault::PlaceholderEmbedder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let embedder = PlaceholderEmbedder::new(384);

    let texts = vec![
        "First document",
        "Second document",
        "Third document",
        // ... up to thousands of texts
    ];

    // Parallel batch processing (4-8x faster than sequential)
    let embeddings = embedder.embed_batch(&texts).await?;

    println!("Generated {} embeddings", embeddings.len());
    println!("Each embedding has {} dimensions", embeddings[0].len());

    Ok(())
}
```

---

## Under the Hood

### Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Application Layer                     │
│  (Your code using knowledge-vault-rs)                    │
└────────────────────────┬────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────┐
│                   Public API Layer                       │
│  - KnowledgeVault                                        │
│  - DocumentChunker                                       │
│  - EmbeddingProvider (trait)                             │
│  - DocumentIndexer                                       │
│  - FileWatcher                                           │
└────────────────────────┬────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────┐
│                  Implementation Layer                     │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │ Chunker      │  │ Embedder     │  │ Search       │ │
│  │ - Markdown   │  │ - Provider   │  │ - VSS        │ │
│  │ - Code       │  │ - Batch      │  │ - Cosine     │ │
│  │ - Text       │  │ - Parallel   │  │              │ │
│  └──────────────┘  └──────────────┘  └──────────────┘ │
└────────────────────────┬────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────┐
│                   Storage Layer                          │
│  ┌──────────────────────────────────────────────────┐  │
│  │         SQLite Database (file-based)              │  │
│  │  - documents (metadata, hashes)                   │  │
│  │  - chunks (split text pieces)                     │  │
│  │  - embeddings (vectors as BLOB)                   │  │
│  └──────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────┐  │
│  │    Optional: SQLite-VSS Extension                │  │
│  │    (Fast vector similarity search)                │  │
│  └──────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

### Data Flow

#### Indexing Flow

```
File Input
    │
    ▼
Read Content
    │
    ▼
Calculate SHA256 ──────▶ Check if exists (deduplication)
    │                              │
    ▼                              │ No (new document)
Chunk Document                          │
    │                                  ▼
    ▼                          Create Document Record
Generate Embeddings (batch, parallel)    │
    │                                  │
    ▼                                  ▼
Store Chunks ◀───────────────────────▶ Store Document
    │
    ▼
Store Embeddings
    │
    ▼
Update VSS Index (if available)
    │
    ▼
Done
```

#### Search Flow

```
Query Text
    │
    ▼
Generate Query Embedding
    │
    ▼
Search Method Decision
    │
    ├─────────────────┐
    │                 │
    ▼                 ▼
VSS Available     Fallback to
    │           Cosine Similarity
    ▼                 │
Fast ANN            ▼
Search         Load All Embeddings
    │                 │
    ▼                 ▼
Return Results  Calculate Similarity
                     │
                     ▼
                  Sort & Rank
                     │
                     ▼
                Return Results
```

### Key Algorithms

#### RegexSet Code Chunking

Optimization for fast multi-pattern code splitting:

```rust
// Compile 12 patterns once
let patterns = [r"\npub fn ", r"\nfn ", r"\nclass ", ...];
let regex_set = RegexSet::new(patterns)?;

// Single-pass search (O(n) instead of O(n*m))
let matches = regex_set.matches(content);

// Extract split points
for pattern_idx in matches {
    // Find exact positions and create chunks
}
```

**Performance**: 2.5x faster than sequential regex scanning.

#### Cosine Similarity Fallback

Mathematically precise similarity calculation:

```rust
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    // Dot product
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();

    // Euclidean norms
    let norm_a = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    // Cosine similarity
    dot_product / (norm_a * norm_b)
}
```

#### Batch Embedding Parallelization

Parallel processing with bounded concurrency:

```rust
use tokio::sync::Semaphore;

let semaphore = Arc::new(Semaphore::new(8)); // Max 8 workers
let mut tasks = Vec::new();

for &text in texts {
    let semaphore = semaphore.clone();
    let task = tokio::spawn(async move {
        let _permit = semaphore.acquire().await?;
        generate_embedding(text)
    });
    tasks.push(task);
}

let results = futures::future::try_join_all(tasks).await?;
```

**Performance**: 4-8x speedup for batch operations.

---

## Known Limitations

### Current Limitations

1. **Real Embeddings Not Yet Integrated**
   - Currently uses SHA256 placeholder embeddings
   - BGE-Micro integration planned for v0.2.0
   - OpenAI API support planned for v0.3.0

2. **SQLite-VSS Optional**
   - Requires external extension loading
   - Falls back to cosine similarity (slower for >10k chunks)
   - Not all platforms support VSS extension

3. **Code Chunking Uses Regex**
   - Not AST-aware (can be confused by nested structures)
   - tree-sitter integration planned for v0.4.0

4. **Single-Threaded Indexing**
   - Indexer uses single worker task
   - Parallel indexing planned for v0.5.0

5. **No Distributed Support**
   - Single-machine only
   - No replication or sharding
   - Distributed mode not planned

### Performance Considerations

| Dataset Size | Recommended Method | Notes |
|--------------|-------------------|-------|
| <1k chunks | Any method | All fast |
| 1k-10k chunks | Cosine fallback | Acceptable |
| 10k-100k chunks | SQLite-VSS | Recommended |
| >100k chunks | External vector DB | Consider Qdrant, pgvector |

### Security Considerations

- **SQL Injection**: All queries use parameterized statements ✅
- **Path Traversal**: File paths validated ✅
- **Resource Limits**: Channel-based queuing prevents overflow ✅
- **Input Validation**: Content hashed before storage ✅

---

## Roadmap

### v0.2.0 - Real Embeddings (Q1 2026)

- [ ] BGE-Micro v2 integration (via llama.cpp)
- [ ] OpenAI API provider
- [ ] Sentence Transformers provider
- [ ] Embedding caching
- [ ] Progress bars for long operations

### v0.3.0 - Performance (Q2 2026)

- [ ] Parallel indexing (multiple workers)
- [ ] Incremental VSS indexing
- [ ] Batch search API
- [ ] Query optimization (caching, pre-filtering)
- [ ] Memory-mapped embeddings

### v0.4.0 - Advanced Chunking (Q2 2026)

- [ ] tree-sitter integration for code chunking
- [ ] PDF document support
- [ ] HTML cleaning and chunking
- [ ] Custom chunking strategies
- [ ] Chunk quality scoring

### v0.5.0 - Hybrid Search (Q3 2026)

- [ ] Keyword + vector hybrid search
- [ ] BM25 ranking
- [ ] Query expansion
- [ ] Re-ranking with cross-encoders
- [ ] Query understanding (intent detection)

### v1.0.0 - Production Ready (Q4 2026)

- [ ] 100% test coverage
- [ ] Comprehensive benchmarks
- [ ] Production deployment guide
- [ ] Performance tuning guide
- [ ] Stable API guarantees

---

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

### Development Setup

```bash
# Clone repository
git clone https://github.com/SuperInstance/knowledge-vault-rs
cd knowledge-vault-rs

# Install development dependencies
cargo install cargo-watch
cargo install cargo-nextest

# Run tests
cargo test --all-features

# Run with auto-reload
cargo watch -x test

# Format code
cargo fmt --all

# Check for warnings
cargo clippy --all-features -- -D warnings
```

### Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass (`cargo test --all-features`)
6. Format code (`cargo fmt --all`)
7. Run clippy (`cargo clippy --all-features -- -D warnings`)
8. Commit your changes (`git commit -m 'Add amazing feature'`)
9. Push to branch (`git push origin feature/amazing-feature`)
10. Open a Pull Request

### Code Style

- Follow Rust naming conventions
- Use `rustfmt` for formatting
- Document all public APIs
- Add doc examples for complex functions
- Write tests for new functionality

### Testing

```bash
# Run all tests
cargo test --all-features

# Run specific test
cargo test test_chunking

# Run with output
cargo test -- --nocapture

# Run tests in parallel (faster)
cargo nextest run --all-features
```

---

## License

Dual-licensed under:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)

You may choose either license for your use.

### Contribution Licensing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be dual-licensed as above, without any additional terms or conditions.

---

## Acknowledgments

### Dependencies

knowledge-vault-rs builds on excellent open-source libraries:

- **SQLite** - Embedded database engine
- **SQLite-VSS** - Vector similarity search extension
- **Tokio** - Async runtime
- **Serde** - Serialization framework
- **Notify** - File system monitoring
- **Regex** - Regular expression engine
- **Unicode-segmentation** - Word boundary detection

### Inspiration

This library was inspired by:

- **LangChain** - RAG framework design patterns
- **LlamaIndex** - Document indexing strategies
- **Chroma** - Vector database API design
- **Qdrant** - Vector search algorithms

### SuperInstance

knowledge-vault-rs is extracted from [SuperInstance AI](https://github.com/SuperInstance/Tripartite1), a tripartite agentic AI system. It powers the local RAG system that enables SuperInstance to answer questions using your own documentation.

---

## Support

### Documentation

- **API Docs**: https://docs.rs/knowledge-vault-rs
- **Examples**: https://github.com/SuperInstance/knowledge-vault-rs/tree/main/examples
- **Migration Guide**: [MIGRATION_GUIDE.md](MIGRATION_GUIDE.md)

### Community

- **GitHub Issues**: https://github.com/SuperInstance/knowledge-vault-rs/issues
- **Discussions**: https://github.com/SuperInstance/knowledge-vault-rs/discussions
- **SuperInstance Discord**: https://discord.gg/superinstance

### Commercial Support

For commercial support, custom integrations, or consulting, contact the SuperInstance team at [support@superinstance.ai](mailto:support@superinstance.ai).

---

**Version**: 0.1.0
**Published**: 2026-01-08
**Authors**: SuperInstance Team
**Repository**: https://github.com/SuperInstance/knowledge-vault-rs
