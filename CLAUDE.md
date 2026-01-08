# SuperInstance AI - Complete Development Guide

> **Current Status**: Multi-Interface Expansion Phase (2026-01-08)
> **Tests**: 298/298 passing (100%)
> **Repository**: https://github.com/SuperInstance/Tripartite1
> **Version**: v0.2.0

---

## Executive Summary

SuperInstance is a **modular, multi-interface AI platform** with a privacy-first, local-first architecture. The system uses a **tripartite consensus system** (Pathos, Logos, Ethos) and is designed as both a standalone application AND a collection of independent, reusable tools.

**Core Philosophy**: "Build once, deploy everywhere." Each component is designed to work independently or as part of the larger ecosystem.

---

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [User Interfaces](#user-interfaces)
3. [Plug-and-Play AI Tool Integration](#plug-and-play-ai-tool-integration)
4. [Independent Standalone Tools](#independent-standalone-tools)
5. [Development Methodology](#development-methodology)
6. [Current Status](#current-status)
7. [Quick Reference](#quick-reference)

---

## Architecture Overview

### System Architecture Diagram

```
┌──────────────────────────────────────────────────────────────┐
│                    USER INTERFACE LAYER                       │
│  ┌──────────┐  ┌───────────┐  ┌────────┐  ┌──────────┐  │
│  │   CLI    │  │   Web     │  │Desktop│  │  VS Code  │  │
│  │(Rust CLI)│  │(Next.js)  │ │(Tauri) │  │Extension │  │
│  └─────┬────┘  └─────┬─────┘  └──┬─────┘  └────┬─────┘  │
└────────┼───────────┼───────────┼────────┼──────────┘
         │           │           │        │
┌────────▼───────────▼───────────▼────────▼──────────▼───────┐
│              SuperInstance Core Hub (Rust)                     │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐                            │
│  │ PATHOS  │  │  LOGOS  │  │  ETHOS  │  ← Tripartite Council    │
│  │ (Intent)│  │ (Logic) │  │ (Truth) │                            │
│  └────┬────┘  └────┬────┘  └────┬────┘                            │
│       └───────────┼───────────┘                               │
│                   ▼                                              │
│         ┌─────────────────┐                                     │
│         │ Consensus Engine│ ← Weighted voting (0.85 threshold)     │
│         └────────┬────────┘                                     │
│                   │                                              │
│  ┌──────────────────┼───────────────────┐                       │
│  ▼               ▼                       ▼                       │
│  ┌────────┐   ┌────────┐           ┌───────┐                      │
│  │Privacy │   │Knowledge│           │Model  │                      │
│  │ Proxy  │   │ Vault   │           │Manager│                      │
│  └────────┘   └────────┘           └───────┘                      │
│                                                                │
│  ┌─────────────────────────────────────────────────────────────┐  │
│  │            Plug-and-Play Tool Integration Layer             │  │
│  │  ┌────────┐  ┌──────────┐  ┌──────┐  ┌─────────┐           │  │
│  │  │ Ollama │  │LM Studio │  │vLLM  │  │LocalAI  │           │  │
│  │  └────┬───┘  └────┬───────┘  └──┬───┘  └────┬─────┘           │  │
│  │     │         │            │      │        │               │  │
│  │  ┌──▼─────────▼────────────▼───▼──────▼─────────────────┐   │  │
│  │  │    OpenAI-Compatible API Layer (abstraction)       │   │  │
│  │  └──────────────────────┬────────────────────────────┘   │  │
│  │                         │                                 │  │
│  │  ┌─────────────────────▼─────────────────────────────┐   │  │
│  │  │     Core SuperInstance (synesis-core)              │   │  │
│  │  └────────────────────────────────────────────────────┘   │  │
│  └───────────────────────────────────────────────────────────────┘│
└────────────────────────────────────────────────────────────────────┘
```

### Design Principles

1. **Modularity**: Each component can work independently
2. **Pluggability**: Easy to swap backends (Ollama vs llama.cpp vs local)
3. **Privacy-First**: Sensitive data never leaves device without consent
4. **Local-First**: Full functionality without internet
5. **Open Standards**: Use OpenAI-compatible APIs where possible
6. **Cross-Platform**: Windows, macOS, Linux, iOS, Android

---

## User Interfaces

### UI Overview Matrix

| Interface | Status | Priority | Use Case | Technology Stack |
|-----------|--------|----------|----------|-----------------|
| **CLI** | ✅ Complete | P0 | Developers, power users | Rust CLI |
| **Web Dashboard** | 🔄 Planned | P1 | General users, teams | Next.js + Rust |
| **Desktop App** | 🔄 Planned | P1 | Privacy-focused users | Tauri + React |
| **VS Code Extension** | 🔄 Planned | P1 | Developers | TypeScript + WASM |
| **Mobile SDK** | 🔄 Planned | P2 | On-the-go users | Flutter |

---

## 1. CLI (Command-Line Interface) ✅ COMPLETE

**Status**: Production Ready
**Documentation**: [docs/CLI_USER_GUIDE.md](docs/CLI_USER_GUIDE.md) (NEW)

### Architecture

```
CLI Application (synesis-cli)
├── Commands/
│   ├── ask/           # Main query interface
│   ├── init/          # First-time setup
│   ├── knowledge/     # RAG management
│   ├── model/          # Model management
│   ├── cloud/          # Cloud operations
│   ├── metrics/        # Performance metrics
│   ├── config/        # Configuration
│   └── manifest/       # Hardware manifests
└── Core Library
    └── Direct calls to synesis-core (no HTTP overhead)
```

### User Guide

#### Installation

```bash
# Clone and build
git clone https://github.com/SuperInstance/Tripartite1.git
cd Tripartite1
cargo build --release

# (Optional) Add to PATH
export PATH="$PATH:$(pwd)/target/release"
```

#### Essential Commands

```bash
# First-time setup
synesis init

# Basic usage
synesis ask "Explain how Rust's ownership works"

# Knowledge base
synesis knowledge add ~/Documents/my-project/
synesis ask "How does the authentication system work?"

# Model management
synesis model list
synesis model download phi-3-mini

# System status
synesis status
```

#### Advanced Features

```bash
# Interactive mode
synesis chat --interactive

# Cloud escalation
synesis ask --cloud "Explain quantum computing deeply"

# Configuration profiles
synesis profile create developer
synesis profile use developer
synesis profile set developer.model phi-3-mini

# Batch processing
synesis batch --file queries.txt --output results/
```

#### Output Modes

```bash
# Standard output
synesis ask "query"

# JSON output (for scripting)
synesis ask "query" --format json --output result.json

# Markdown output
synesis ask "query" --format markdown --pipe-less

# Verbose mode
synesis ask "query" --verbose
```

### Features

- ✅ Hardware detection (auto-optimizes for your system)
- ✅ Tripartite consensus visualization
- ✅ Knowledge vault RAG integration
- ✅ Privacy redaction (18 built-in patterns)
- ✅ Local inference (no internet required)
- ✅ Optional cloud escalation
- ✅ Configuration profiles
- ✅ Streaming responses
- ✅ Session persistence

---

## 2. Web Dashboard 🔄 PLANNED

**Status**: Architecture Complete, Implementation Scheduled
**Priority**: P1
**Documentation**: [docs/WEB_DASHBOARD_GUIDE.md](docs/WEB_DASHBOARD_GUIDE.md) (NEW)

### Architecture

```
┌───────────────────────────────────────────────────────────┐
│                    Web Dashboard (Next.js 14)               │
│                                                                │
│  Frontend Layer                                                 │
│  ┌─────────────────────────────────────────────────────────┐  │
│  │  UI Components (shadcn/ui + Tailwind CSS)                │  │
│  │  ┌────────┐  ┌──────────┐  ┌─────────────────────────┐  │  │
│  │  │Chat UI │  │Admin     │  │ Knowledge Browser        │  │  │
│  │  │        │  │Panel    │  │                         │  │  │
│  │  └────────┘  └──────────┘  └─────────────────────────┘  │  │
│  │                                                          │  │
│  │  ┌──────────────────────────────────────────────────────┐  │ │
│  │  │    State Management (Zustand + React Query)          │  │ │
│  │  └──────────────────────┬───────────────────────────┘  │  │
│  └──────────────────────────┼───────────────────────────────┘ │
│                             │ HTTP/WebSocket              │
├─────────────────────────────┼─────────────────────────────────┤
│  Backend Web Server (Actix)    │                                 │
│  ┌────────────────────────────────────────────────────────┐   │
│  │  API Endpoints                                         │   │
│  │  POST   /api/v1/chat                                 │   │
│  │  GET    /api/v1/chat/:id/stream (SSE)               │   │
│  │  POST   /api/v1/knowledge                            │   │
│  │  GET    /api/v1/status                               │   │
│  │  WS     /api/v1/ws (WebSocket)                      │   │
│  └──────────────────────┬───────────────────────────────┘   │
│                           │                                 │
│  ┌──────────────────────▼─────────────────────────────┐   │
│  │  Core SuperInstance (synesis-core, synesis-...)     │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

### Key Features

1. **Real-Time Chat Interface**
   - Streaming responses with typing indicators
   - Agent visualization (Pathos/Logos/Ethos status)
   - Consensus meter showing live voting
   - Message history with search

2. **Knowledge Vault Browser**
   - Document listing with metadata
   - Chunk viewer with embeddings
   - Semantic search interface
   - Drag-and-drop document upload

3. **Configuration Management**
   - Visual model selection
   - Consensus threshold slider
   - Privacy pattern toggles
   - Profile management UI

4. **Admin Dashboard**
   - System metrics (CPU, GPU, memory usage)
   - Billing overview
   - User management (multi-tenant)
   - Cloud connection status

### User Guide

#### Accessing the Dashboard

```bash
# Start web server
cd web-dashboard
npm install
npm run dev

# Access at
open http://localhost:3000
```

#### Core Workflows

**1. Chat Interface**
- Enter query in chat box
- Watch agents think in real-time
- View consensus formation
- Share conversations (future feature)

**2. Knowledge Management**
- Browse indexed documents
- Upload new documents (drag-and-drop)
- Search semantic content
- View chunk breakdowns

**3. Settings**
- Switch between models
- Adjust consensus threshold
- Toggle privacy patterns
- Manage profiles

### Technology Stack

**Frontend**:
- Next.js 14 (App Router)
- TypeScript
- Tailwind CSS
- shadcn/ui components
- Zustand (state management)
- React Query (data fetching)

**Backend**:
- Actix-web 4
- Tokio (async runtime)
- SSE/WebSocket support
- JWT authentication

---

## 3. Desktop Application 🔄 PLANNED

**Status**: Architecture Complete, Implementation Scheduled
**Priority**: P1
**Documentation**: [docs/DESKTOP_APP_GUIDE.md](docs/DESKTOP_APP_GUIDE.md) (NEW)

### Architecture

```
┌──────────────────────────────────────────────────┐
│        Desktop App (Tauri 2.0)                     │
│                                                   │
│  Frontend (React + TypeScript)                      │
│  ┌────────────────────────────────────────────┐  │
│  │  (Reuses web-dashboard components)            │  │
│  │  ┌──────────┐  ┌─────────┐                     │  │
│  │  │Chat View │  │Settings │                     │  │
│  │  └──────────┘  └─────────┘                     │  │
│  └────────────────────────────────────────────┘  │
│                   │ IPC (Tauri Commands)           │
├───────────────────┼──────────────────────────────────┤
│  Backend (Rust)                                     │
│  ┌──────────────────────────────────────────────┐ │
│  │  synesis-core (embedded library)             │ │
│  │  - Direct function calls (no HTTP)           │ │
│  │  - Native performance                      │ │
│  │  - Offline-first architecture                │ │
│  └──────────────────────────────────────────────┘  │
│                                                   │
│  Native OS Features:                             │
│  - System tray icon                              │
│  - Global hotkeys (Cmd+Shift+S)                   │
│  - Native notifications                           │
│  - File drag-and-drop                             │
│  - OS keychain integration                       │
└───────────────────────────────────────────────────┘
```

### Key Features

1. **Offline-First**
   - Full functionality without internet
   - Local model inference
   - Background sync when online

2. **Native OS Integration**
   - System tray with quick actions
   - Global hotkey for quick access
   - Native notifications
   - File associations

3. **Local Resource Management**
   - Hardware acceleration UI
   - Model download manager
   - Disk space management
   - GPU memory monitoring

4. **Security**
   - OS keychain for API keys
   - Biometric unlock (TouchID/Windows Hello)
   - Encrypted local storage

### User Guide

#### Installation

```bash
# Install Desktop App
cargo install --path desktop-app

# Or download pre-built binary
# Available for: Windows, macOS, Linux
```

#### First Launch

1. **Hardware Detection**: Auto-detects CPU/GPU/RAM
2. **Model Selection**: Choose default model
3. **Profile Setup**: Create or import profile
4. **Knowledge**: Optionally index documents

#### Core Features

**Chat Panel**:
- Real-time agent visualization
- Conversation history
- Quick actions (clear, export, settings)

**Knowledge Browser**:
- Document tree view
- Chunk viewer
- Search interface

**System Tray Menu**:
- Quick chat (global hotkey)
- Knowledge vault status
- System metrics

### Distribution

- **Windows**: `.exe` installer
- **macOS**: `.dmg` installer
- **Linux**: AppImage/deb package

---

## 4. VS Code Extension 🔄 PLANNED

**Status**: Architecture Complete, Implementation Scheduled
**Priority**: P1
**Documentation**: [docs/VSCODE_EXTENSION_GUIDE.md](docs/VSCODE_EXTENSION_GUIDE.md) (NEW)

### Architecture

```
┌──────────────────────────────────────────────────┐
│        VS Code Extension (TypeScript)             │
│                                                   │
│  Extension Host (Node.js)                         │
│  ┌──────────────────────────────────────────────┐ │
│  │  Language Features                             │ │
│  │  - Inline completion                           │ │
│  │  - Code explanation                          │ │ │
│  │  - Refactoring suggestions                     │ │
│  │  - Code lens integration                     │ │
│  └──────────────────────────────────────────────┘ │
│                                                   │
│  ┌──────────────────────────────────────────────┐ │
│  │  WASM Module (synesis-wasm)                 │ │
│  │  - Compiled Rust core                        │ │
│  │  - Runs in Node.js via WASI                   │ │
│  └──────────────────────────────────────────────┘ │
│                        │                          │
│              ┌──────────────┐                       │
│              │ synesis-server│ ← Optional        │
│              │  (Rust binary)│   background      │
│              └──────────────┘                       │
└───────────────────────────────────────────────────┘
```

### Key Features

1. **Context-Aware Completions**
   - File-aware suggestions
   - Project-wide semantic search
   - Import suggestions

2. **Inline Explanations**
   - Hover over code for explanations
   - Step-by-step logic breakdown
   - Safety warnings for dangerous code

3. **Codebase Q&A**
   - "How does authentication work?"
   - "Find all usages of function X"
   - "Explain module Y architecture"

4. **Refactoring Assistant**
   - Suggest improvements via Ethos
   - Performance optimization hints
   - Bug detection

### User Guide

#### Installation

```bash
# Install from VS Code Marketplace
code --install-extension superinstance.superinstance

# Or install from VSIX
code --install-extension superinstance-superinstance.vsix
```

#### Core Features

**Inline Completion**:
- Type code, get suggestions
- Context-aware (reads open files)
- Based on indexed codebase

**Chat Panel** (Sidebar):
- Ask questions about codebase
- Get explanations
- Request refactoring help

**Commands**:
- `synesis.explain` - Explain selected code
- `synesis.refactor` - Suggest improvements
- `synesis.chat` - Open chat panel

#### Configuration

```json
// settings.json
{
  "superinstance.model": "phi-3-mini",
  "superinstance.knowledgePath": "~/Documents/my-project",
  "superinstance.backend": "local",
  "superinstance.hotkey": "cmd+shift+s"
}
```

---

## 5. Mobile SDK 🔄 PLANNED

**Status**: Architecture Complete, Implementation Scheduled
**Priority**: P2
**Documentation**: [docs/MOBILE_SDK_GUIDE.md](docs/MOBILE_SDK_GUIDE.md) (NEW)

### Architecture

```
┌──────────────────────────────────────────────────┐
│        Mobile App (Flutter)                        │
│                                                   │
│  Dart Layer                                        │
│  ┌────────────────────────────────────────────┐  │
│  │  UI Components (Flutter widgets)            │  │
│  │  - Chat interface                           │  │
│  │  - Settings screens                         │  │
│  │  - Knowledge browser                        │  │
│  └────────────────────────────────────────────┘  │
│                                                   │
│  ┌────────────────────────────────────────────┐  │
│  │  synesis_mobile (Dart package)             │  │
│  │  - SuperInstanceClient                   │  │
│  │  - Message streaming                     │  │
│  │  - Local storage                           │  │
│  └────────────────────────────────────────────┘  │
│                   │ Platform Channel              │
├─────────────────────┼──────────────────────────────────┤
│  Native Platform Layer                             │
│  ┌──────────────┐        ┌──────────────┐      │
│  │   Android    │        │     iOS      │      │
│  │   (Kotlin)   │        │   (Swift)    │      │
│  │              │        │              │      │
│  │ synesis-core │        │ synesis-core │      │
│  │  via JNI     │        │  via FFI     │      │
│  │              │        │              │      │
│  │ Local models  │        │ Local models  │      │
│  └──────────────┘        └──────────────┘      │
│                                                   │
│  Offline-First Storage                            │
└───────────────────────────────────────────────────┘
```

### Key Features

1. **Offline Mode**
   - Cache previous queries
   - Local-only mode
   - Download models for offline

2. **Privacy-First**
   - Biometric authentication
   - Encrypted local storage
   - No data leaves device

3. **Battery Optimization**
   - Adaptive inference
   - Background processing limits
   - Thermal throttling

4. **Native Integrations**
   - Share sheet integration
   - Widgets (home screen)
   - Siri shortcuts / Google Assistant

### User Guide

#### Installation

```bash
# Install from App Store
# Available for: iOS, Android

# Or build from source
flutter pub global activate
cd mobile-app
flutter run
```

#### Core Features

**Chat Interface**:
- Voice input (speech-to-text)
- Text-to-speech responses
- Conversation history
- Sync across devices

**Knowledge Vault**:
- Browse indexed documents
- Search semantic content
- View code chunks
- Upload new documents

**Settings**:
- Model selection
- Cloud sync toggle
- Privacy settings
- Profile management

---

## Plug-and-Play AI Tool Integration

### Integration Architecture

```
SuperInstance Core
       │
       ├───┬──────────────────────────────────────┐
       │   │                                        │
       │   ├────────► Ollama (local model server)    │
       │   │                                        │
       │   ├────────► LM Studio (GUI model runner)    │
         │   │                                        │
       │   ├────────► LocalAI (OpenAI-compatible)    │
       │   │                                        │
       │   ├────────► vLLM (high-performance)       │
       │   │                                        │
       │   ├────────► NeMo Agent Toolkit (NVIDIA)  │
       │   │                                        │
       │   └────────► VILA (multimodal)           │
       │                                          │
       ▼                                          ▼
   OpenAI-Compatible API Abstraction Layer
```

### Supported Tools

| Tool | Type | Integration | Status | Use Case |
|------|------|------------|--------|----------|
| **[Ollama](https://ollama.com)** | Local server | `synesis-ollama` crate | ✅ Planned | Easy model management |
| **[LM Studio](https://lmstudio.ai)** | Desktop GUI | `synesis-lmstudio` crate | ✅ Planned | GUI-first workflow |
| **[LocalAI](https://localai.io/)** | OpenAI-compatible | `synesis-localai` crate | ✅ Planned | API compatibility |
| **[vLLM](https://lmzh.uvicu.org/)** | High-performance | `synesis-vllm` crate | 🔄 Future | Production serving |
| **[NeMo](https://developer.nvidia.com/nemo-agent-toolkit)** | Agent framework | `synesis-nemo` bridge | 🔄 Future | Agent capabilities |
| **[VILA](https://github.com/NVlabs/VILA)** | Multimodal | `synesis-vision` crate | 🔄 Future | Image understanding |

---

### 1. Ollama Integration ✅

**Documentation**: [tools/OLLAMA_INTEGRATION.md](tools/OLLAMA_INTEGRATION.md) (NEW)

#### Architecture

```rust
// crates/synesis-ollama/
//! Ollama integration for SuperInstance

pub mod client;
pub mod models;
pub mod inference;

pub struct OllamaAgent {
    config: AgentConfig,
    client: OllamaClient,
    model: String,
}

#[async_trait]
impl Agent for OllamaAgent {
    async fn process(&self, input: AgentInput) -> Result<AgentResponse, AgentError> {
        let prompt = self.format_prompt(&input);
        let response = self.client.generate(&self.model, &prompt).await?;
        Ok(AgentResponse::from(response))
    }
}
```

#### Usage

```bash
# Pull and use Ollama models
synesis backend set ollama.enabled true
synesis ollama pull llama3:70b
synesis ask "Explain quantum computing" --backend ollama
```

**Benefits**:
- No compilation required
- Easy model management
- GPU acceleration
- Multi-model support

---

### 2. LM Studio Integration ✅

**Documentation**: [tools/LM_STUDIO_INTEGRATION.md](tools/LM_STUDIO_INTEGRATION.md) (NEW)

#### Architecture

```rust
// crates/synesis-lmstudio/
//! LM Studio integration (uses local API)

pub struct LMStudioClient {
    base_url: String,  // http://localhost:1234
}

impl LMStudioClient {
    pub async fn discover() -> Result<Self, LMSError> {
        let ports = [1234, 8080, 5000];
        for port in ports {
            if Self::probe(&format!("http://localhost:{}", port)).await {
                return Ok(Self { base_url: format!("http://localhost:{}", port) });
            }
        }
        Err(LMSError::NotFound)
    }

    pub async fn get_loaded_model(&self) -> Result<String, LMSError> {
        let resp = reqwest::get(&format!("{}/v1/models", self.base_url))
            .await?
            .json::<ModelsResponse>()
            .await?;

        Ok(resp.data.first().map(|m| m.id.clone()).unwrap_or_default())
    }
}
```

#### Usage

```bash
# Auto-detect LM Studio
synesis backend detect

# Use LM Studio
synesis ask "query" --backend lmstudio
```

---

### 3. LocalAI Compatibility Layer ✅

**Documentation**: [tools/LOCALAI_INTEGRATION.md](tools/LOCALAI_INTEGRATION.md) (NEW)

#### Purpose

Provides OpenAI-compatible API wrapper for multiple local inference tools.

#### Architecture

```rust
// crates/synesis-localai/
//! LocalAI compatibility layer

pub struct LocalAIAdapter {
    endpoint: String,
    api_key: Option<String>,
}

impl LocalAIAdapter {
    pub async fn chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, LocalAIError> {
        let url = format!("{}/v1/chat/completions", self.endpoint);
        let client = reqwest::Client::new();

        let resp = client.post(&url).json(&request).send().await?
            .json::<ChatCompletionResponse>().await?;

        Ok(resp)
    }
}

// Convert SuperInstance requests to OpenAI format
impl From<AgentInput> for ChatCompletionRequest {
    fn from(input: AgentInput) -> Self {
        ChatCompletionRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: input.manifest.query,
            }],
            temperature: Some(0.7),
            max_tokens: Some(2048),
        }
    }
}
```

#### Usage

```bash
# Start LocalAI server
localai-server --models-path ./models --port 8080

# Configure SuperInstance
synesis config set localai.endpoint http://localhost:8080
synesis ask "query" --backend localai
```

---

### 4. vLLM Integration 🔄 Future

**Purpose**: High-performance production serving

**Documentation**: [tools/VLLM_INTEGRATION.md](tools/VLLM_INTEGRATION.md) (NEW)

#### Use Cases

- Production API server
- Batch inference
- Multi-model serving
- Load balancing

---

### 5. NeMo Agent Toolkit 🔄 Research

**NVIDIA NeMo**: Framework for building conversational AI

**Resources**:
- [NeMo Agent Toolkit](https://developer.nvidia.com/nemo-agent-toolkit)
- [Build Custom AI Agents](https://developer.nvidia.com/blog/how-to-build-custom-ai-agents-with-nemo-agent-toolkit-open-source-library/)
- [RagaAI Catalyst Integration](https://raga.ai/resources/blogs/raga-ai-catalyst-integrates-with-nvidia-nemo-agent-toolkit-building-reliable-ai-agents-from-day-one)

#### Integration Approach

```python
# Python bridge via PyO3
# crates/synesis-nemo/

use pyo3::prelude::*;

pub struct NeMoAgent {
    python: Python,
    nemo_module: Py<PyModule>,
}

impl NeMoAgent {
    pub async fn transcribe(&self, audio: &[u8]) -> Result<String, NeMoError> {
        let python = Python::acquire_gil();
        let py = python.python();

        let result = self.nemo_module.call_method1(py, "transcribe", (audio,))?;
        Ok(result.extract(py)?)
    }
}
```

#### Use Cases

- Speech-to-text input
- Text-to-speech output
- Multimodal query processing

---

### 6. VILA Multimodal 🔄 Research

**VILA**: NVIDIA's Vision-Language model family

**Resources**:
- [VILA GitHub](https://github.com/NVlabs/VILA)
- [Visual Language Models on NVIDIA Hardware](https://developer.nvidia.cn/blog/visual-language-models-on-nvidia-hardware-with-vila/)
- [New VILA-1.5 Models](https://forums.developer.nvidia.com/t/new-vila-1-5-multimodal-vision-language-models-released-in-3b-8b-13b-40b/)

#### Architecture

```rust
// crates/synesis-vision/
//! Multimodal AI support (VILA, LLaVA, etc.)

pub struct VisionAgent {
    model: VisionModel,
    embedder: ImageEmbedder,
}

pub enum VisionInput {
    Image(Vec<u8>),
    ImageUrl(String),
    Screenshot,
}

impl VisionAgent {
    pub async fn analyze_image(
        &self,
        image: VisionInput,
        query: &str,
    ) -> Result<VisionResponse, VisionError> {
        let embedding = self.embedder.encode(&image).await?;
        let response = self.model.generate(query, embedding).await?;
        Ok(response)
    }
}
```

#### Usage

```bash
# Analyze images
synesis ask "What's in this image?" --image photo.png

# Analyze screenshots
synesis ask "What error is shown?" --screenshot

# Batch analysis
synesis vision analyze ./photos/ --query "What objects are visible?"
```

---

### 7. JEPA Architecture Research 🔄 Exploration

**JEPA** (Joint Embedding Predictive Architecture): Meta's approach to self-supervised learning

**Resources**:
- [VL-JEPA Paper](https://arxiv.org/abs/2512.10942)
- [JEPA Overview](https://medium.com/@saumya.april1/jepa-joint-embedding-predictive-architecture-5b1ee798c8b7)

#### Potential Applications

1. **Predictive Text Completion**
   - Enhanced code completion
   - Workflow prediction
   - Intent prediction (Pathos enhancement)

2. **Sequence Modeling**
   - Code understanding
   - Log analysis
   - Behavior prediction

3. **Representation Learning**
   - Better embeddings
   - Self-supervised pre-training

---

## Independent Standalone Tools

### Tool Extraction Strategy

Each major component is designed as an independent crate with:
- Clean API surface
- Standalone CLI
- Library usage
- Documentation
- Examples

---

### 1. Privacy Proxy (Redaction System) 🛡️

**Standalone Crate**: `privacy-proxy`

**Documentation**: [tools/PRIVACY_PROXY_TOOL.md](tools/PRIVACY_PROXY_TOOL.md) (NEW)

#### Purpose

Redact sensitive information (PII, secrets, credentials) before sharing data.

#### API

```rust
//! Privacy Proxy: Redact and re-inflate sensitive information

use privacy_proxy::{Redactor, RedactorConfig, TokenVault};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create vault
    let vault = TokenVault::open("vault.db")?;

    // Create redactor
    let config = RedactorConfig::default();
    let mut redactor = Redactor::new(config, vault)?;

    // Redact text
    let text = "Contact john@example.com or call 555-1234";
    let result = redactor.redact(text, "session-1").await?;

    println!("Original: {}", text);
    println!("Redacted: {}", result.redacted_text);
    // Output: Contact [EMAIL_001] or call [PHONE_001]

    Ok(())
}
```

#### CLI

```bash
# Redact text
privacy-proxy redact "Contact john@example.com" --vault vault.db

# Re-inflate (when authorized)
privacy-proxy reinflate redacted.txt --vault vault.db

# Server mode (microservice)
privacy-proxy server --port 8080
```

#### Use Cases

1. **Log Scrubbing**
   ```bash
   cat application.log | privacy-proxy redact --stream > clean.log
   ```

2. **Data Sharing**
   - Redact PII before sharing
   - Compliance with GDPR/CCPA

3. **API Gateway**
   ```bash
   # Kubernetes sidecar
   privacy-proxy server --mode gateway
   ```

4. **Developer Tool**
   ```bash
   # Pre-commit hook
   git diff | privacy-proxy redact --check-secrets
   ```

#### Deployment

- **Docker**: `privacy-proxy:latest`
- **Homebrew**: `brew install privacy-proxy`
- **npm**: `npm install @privacy-proxy/cli`
- **Python**: `pip install privacy-proxy`

---

### 2. Token Vault 🔐

**Standalone Crate**: `token-vault`

**Documentation**: [tools/TOKEN_VAULT_TOOL.md](tools/TOKEN_VAULT_TOOL.md) (NEW)

#### Purpose

Secure storage and retrieval of sensitive tokens for reversible redaction.

#### API

```rust
//! Secure token vault for reversible redaction

use token_vault::TokenVault;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create vault
    let vault = TokenVault::open("vault.db")?;

    // Store sensitive value
    let token = vault.store("EMAIL", "john@example.com", "session-1")?;
    println!("Token: {}", token);
    // Output: [EMAIL_ABC123]

    // Retrieve value
    if let Some(value) = vault.retrieve(&token)? {
        println!("Retrieved: {}", value);
        // Output: john@example.com
    }

    Ok(())
}
```

#### CLI

```bash
# Store tokens
token-vault store --category EMAIL --value "john@example.com"

# Retrieve token
token-vault retrieve --token [EMAIL_ABC123]

# Clear session
token-vault clear-session --session-id session-1

# Export/Import
token-vault export > vault_backup.json
token-vault import < vault_backup.json
```

#### Use Cases

1. **Application Secrets**
   - API key storage
   - Runtime token retrieval

2. **Data Masking**
   - Database anonymization
   - Test data generation

3. **Compliance**
   - GDPR right to be forgotten
   - Audit trails

---

### 3. Consensus Engine 🤝

**Standalone Crate**: `consensus-engine`

**Documentation**: [tools/CONSENSUS_ENGINE_TOOL.md](tools/CONSENSUS_ENGINE_TOOL.md) (NEW)

#### Purpose

Multi-agent voting system for decision-making and quality assurance.

#### API

```rust
//! Multi-agent consensus engine

use consensus_engine::{Agent, ConsensusEngine, ConsensusConfig, Vote, ConsensusResult};

struct SecurityAgent {
    id: String,
    strictness: f32,
}

#[async_trait]
impl Agent<SecurityRequest> for SecurityAgent {
    async fn evaluate(&self, input: SecurityRequest) -> Vote {
        if input.risk_score > self.strictness {
            Vote::Reject { reason: "Too risky".into() }
        } else {
            Vote::Approve { confidence: 0.9 }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let agents: Vec<Box<dyn Agent<SecurityRequest>>> = vec![
        Box::new(SecurityAgent { id: "audit".into(), strictness: 0.7 }),
        Box::new(SecurityAgent { id: "compliance".into(), strictness: 0.8 }),
        Box::new(SecurityAgent { id: "security".into(), strictness: 0.9 }),
    ];

    let config = ConsensusConfig {
        threshold: 0.85,
        max_rounds: 3,
        agent_weights: vec![0.3, 0.3, 0.4],
    };

    let mut engine = ConsensusEngine::new(agents, config);
    let result = engine.decide(SecurityRequest { risk_score: 0.75 }).await?;

    match result {
        ConsensusResult::Approved => println!("Request approved"),
        ConsensusResult::Rejected(reason) => println!("Rejected: {}", reason),
        ConsensusResult::Timeout => println!("Consensus timeout"),
    }

    Ok(())
}
```

#### CLI

```bash
# Create consensus configuration
consensus-engine init --config consensus.yaml

# Run consensus process
consensus-engine decide --config consensus.yaml --input request.json

# Add agent
consensus-engine agents add --type security --strictness 0.8
```

#### Use Cases

1. **Security Approval Systems**
   - Multi-factor authorization
   - Peer review automation
   - Compliance workflows

2. **Financial Trading**
   - Risk assessment consensus
   - Multi-strategy agreement

3. **Medical Diagnosis**
   - Second opinion automation
   - Specialist consensus

4. **Decision Support**
   - Business decisions
   - Policy approval

---

### 4. Knowledge Vault (RAG System) 📚

**Standalone Crate**: `knowledge-vault`

**Documentation**: [tools/KNOWLEDGE_VAULT_TOOL.md](tools/KNOWLEDGE_VAULT_TOOL.md) (NEW)

#### Purpose

Local-first RAG (Retrieval-Augmented Generation) system for semantic search and document understanding.

#### API

```rust
//! Local-first RAG system with vector search

use knowledge_vault::{KnowledgeBase, Embedder, DocumentMetadata};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create knowledge base
    let embedder = Box::new(PlaceholderEmbedder::new(384));
    let mut kb = KnowledgeBase::new("./knowledge.db", embedder).await?;

    // Add document
    kb.add(
        "Rust is a systems programming language",
        DocumentMetadata {
            title: "About Rust".to_string(),
            source: "README.md".to_string(),
            doc_type: "markdown".to_string(),
        }
    ).await?;

    // Search similar content
    let results = kb.search("systems programming language", 5).await?;
    for result in results {
        println!("Found: {} (score: {:.2})", result.content, result.score);
    }

    Ok(())
}
```

#### CLI

```bash
# Initialize knowledge base
knowledge-vault init ./my-knowledge

# Add documents
knowledge-vault add ./docs/ --recursive

# Search
knowledge-vault search "How does authentication work?"

# Ask question with context
knowledge-vault ask "Explain the architecture"

# Export/Import
knowledge-vault export > backup.json
knowledge-vault import < backup.json
```

#### Use Cases

1. **Documentation Search**
   - Developer docs
   - API references
   - Internal wikis

2. **Codebase Assistant**
   - Code understanding
   - Refactoring suggestions
   - Bug detection

3. **Research Assistant**
   - Literature review
   - Paper summarization
   - Citation search

4. **Personal Knowledge Management**
   - Note search
   - Email archive
   - File organization

---

### 5. Hardware Detector 🔧

**Standalone Crate**: `hw-detect`

**Documentation**: [tools/HARDWARE_DETECTOR_TOOL.md](tools/HARDWARE_DETECTOR_TOOL.md) (NEW)

#### Purpose

Cross-platform hardware detection and capability assessment.

#### API

```rust
//! Cross-platform hardware detection

use hw_detect::{HardwareDetector, HardwareInfo, Recommendation};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Detect all hardware
    let hw = HardwareDetector::detect()?;
    println!("Hardware Info:");
    println!("  CPU: {}", hw.cpu.name);
    println!("  GPU: {} ({} MB)", hw.gpu.first().map(|g|g.name.as_str()).unwrap_or("None"));
    println!("  RAM: {} GB", hw.memory.total_gb);

    // Get recommendations
    let rec = hw.recommendation()?;
    println!("  Recommended Model: {}", rec.model);
    println!("  Can Run LLM: {}", rec.can_run_llm);

    // Export as JSON
    let json = hw.to_json(&hw)?;
    println!("JSON: {}", json);

    Ok(())
}
```

#### CLI

```bash
# Detect hardware
hw-detect

# JSON output
hw-detect --json > hardware.json

# Recommendations
hw-detect recommend

# Compare systems
hw-detect compare system1.json system2.json
```

#### Use Cases

1. **Game Launchers**
   - Detect system capabilities
   - Adjust graphics settings

2. **DevOps Tools**
   - Container resource allocation
   - Kubernetes scheduling

3. **Software Installers**
   - Check requirements
   - Warn about incompatibility

4. **Benchmarking**
   - Hardware comparison
   - Performance regression detection

---

### 6. Model Registry 📦

**Standalone Crate**: `model-registry`

**Documentation**: [tools/MODEL_REGISTRY_TOOL.md](tools/MODEL_REGISTRY_TOOL.md) (NEW)

#### Purpose

Model versioning, management, and marketplace functionality.

#### API

```rust
//! Model versioning and management

use model_registry::{ModelRegistry, ModelMetadata, ModelFilters};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut registry = ModelRegistry::open("./models.db")?;

    // Register model
    let model = ModelMetadata {
        id: "phi-3-mini".to_string(),
        name: "Phi-3 Mini".to_string(),
        version: "2.6".to_string(),
        size_bytes: 2_300_000_000,
        quantization: "Q4_K_M".to_string(),
        parameters: ModelParameters {
            context_length: 2048,
            max_tokens: 2048,
            embedding_size: 384,
        },
    };

    registry.register(model).await?;

    // List models
    let filters = ModelFilters {
        min_size: Some(1_000_000_000),
        max_size: Some(5_000_000_000),
        quantization: vec!["Q4_K_M".to_string()],
        ..Default::default()
    };

    let models = registry.list(filters)?;
    for model in models {
        println!("{}: {} ({})", model.name, model.version);
    }

    Ok(())
}
```

#### CLI

```bash
# Register model
model-registry register \
  --name "Phi-3 Mini" \
  --version "2.6" \
  --size 2300000000 \
  --quantization Q4_K_M

# Download model
model-registry download phi-3-mini

# List models
model-registry list --tag "small"

# Check updates
model-registry check-updates phi-3-mini
```

#### Use Cases

1. **MLOps Platforms**
   - Model versioning
   - Experiment tracking

2. **App Stores**
   - HuggingFace alternative
   - Model marketplace

3. **Enterprise**
   - Approval workflows
   - Access control

---

### 7. QUIC Tunnel 🚇

**Standalone Crate**: `quic-tunnel`

**Documentation**: [tools/QUIC_TUNNEL_TOOL.md](tools/QUIC_TUNNEL_TOOL.md) (NEW)

#### Purpose

Secure, low-latency bidirectional streaming with QUIC protocol.

#### API

```rust
//! Secure QUIC tunnel with mTLS

use quic_tunnel::{QuicTunnel, TunnelConfig, TunnelEvent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = TunnelConfig {
        server_addr: "tunnel.superinstance.ai:443".to_string(),
        device_id: "device-123".to_string(),
        cert_path: "./cert.pem".into(),
        key_path: "./key.pem".into(),
        ..Default::default()
    };

    let mut tunnel = QuicTunnel::new(config).await?;
    tunnel.connect().await?;

    // Open stream
    let mut stream = tunnel.open_stream().await?;

    // Send message
    stream.send(b"Hello, world!").await?;

    // Receive message
    let data = stream.recv().await?;
    println!("Received: {}", String::from_utf8(&data)?);

    Ok(())
}
```

#### CLI

```bash
# Connect to server
quic-tunnel client --server tunnel.superinstance.ai:443

# Start server
quic-tunnel server --port 443 --cert cert.pem --key key.pem

# Test connection
quic-tunnel ping tunnel.superinstance.ai:443
```

#### Use Cases

1. **Secure Remote Access**
   - Alternative to VPN
   - IoT device connectivity

2. **Microservice Mesh**
   - Service-to-service communication
   - Service discovery

3. **Streaming**
   - Video streaming
   - Real-time data

4. **Gaming**
   - Low-latency multiplayer
   - UDP-friendly transport

---

### 8. Billing System 💰

**Standalone Crate**: `metered-billing`

**Documentation**: [tools/METERED_BILLING_TOOL.md](tools/METERED_BILLING_TOOL.md) (NEW)

#### Purpose

Metered usage tracking, cost calculation, and invoice generation.

#### API

```rust
//! Metered billing with local-first tracking

use metered_billing::{BillingEngine, UsageEvent, PricingTier, Money};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut billing = BillingEngine::new(
        PricingTier::Free {
            monthly_limit_cents: 100_00, // $100/month free
        }
    );

    // Record usage
    billing.record(UsageEvent {
        event_type: EventType::Query,
        tokens_used: 150,
        model: "phi-3-mini".to_string(),
    }).await?;

    // Check unbilled total
    let unbilled = billing.unbilled();
    println!("Unbilled: ${}", unbilled);

    // Generate invoice
    let invoice = billing.invoice().await?;
    println!("Invoice: ${}", invoice.total);

    Ok(())
}
```

#### CLI

```bash
# Initialize billing
metered-billing init --tier free

# Record usage
metered-billing record api-call --tokens 1000

# Check balance
metered-billing balance

# Generate invoice
metered-billing invoice --month 2024-01

# Sync with cloud
metered-billing sync
```

#### Use Cases

1. **API Billing**
   - Meter API usage
   - Tiered pricing

2. **SaaS Applications**
   - Subscription management
   - Usage-based billing

3. **Marketplaces**
   - Stripe Connect alternative
   - Multi-tenant billing

4. **Cost Monitoring**
   - Budget alerts
   - Cost optimization

---

## Development Methodology

### Core Principles

1. **Modular First** - Build independent, reusable components
2. **Plugin Architecture** - Easy to swap backends and tools
3. **Privacy by Design** - Never compromise on data protection
4. **Open Standards** - Use OpenAI-compatible APIs
5. **Documentation-Driven** - Document before, during, after
6. **Test Coverage** - 100% coverage requirement

### The Ralph Loop

```bash
while not complete:
    1. Design architecture (document in CLAUDE.md)
    2. Create stub implementation (minimal viable product)
    3. Add comprehensive tests
    4. Write user guides and examples
    5. Iterate based on feedback
    6. Commit and push to repository
```

### Quality Gates

All code must pass:
- ✅ All tests passing (100%)
- ✅ Zero compiler warnings
- ✅ Zero clippy warnings
- ✅ All public APIs documented
- ✅ Thread safety verified

---

## Current Status

### Phase 1: Local Kernel ✅ COMPLETE

**Core Features**:
- ✅ CLI with all commands working
- ✅ Tripartite consensus system
- ✅ Privacy proxy (18 redaction patterns)
- ✅ Knowledge vault (SQLite-VSS)
- ✅ Hardware detection
- ✅ Model management

**Test Results**: 298/298 passing (100%)

### Phase 2: Cloud Mesh ✅ COMPLETE

**Cloud Features**:
- ✅ QUIC tunnel with mTLS
- ✅ Cloud escalation client
- ✅ Billing system with local ledger
- ✅ LoRA upload and hot-swap
- ✅ Collaborator system
- ✅ Telemetry and heartbeat
- ✅ Binary message protocol

**Test Results**: 68/68 cloud tests passing

### Phase 3: Multi-Interface Expansion 🔄 IN PROGRESS

**Planned**:
- [ ] Web Dashboard architecture complete
- [ ] Desktop app foundation
- [ ] VS Code extension foundation
- [ ] Independent tool extraction begins
- [ ] Plug-and-play integrations start

---

## Quick Reference

### Essential Commands

```bash
# === SETUP ===
synesis init                              # First-time setup
cargo test --workspace                    # Run all tests
cargo clippy --workspace -- -D warnings    # Lint check
cargo fmt --all                          # Format code

# === DEVELOPMENT ===
cargo build --workspace --all-targets      # Build all
cargo test --package <crate>                # Test one crate
cargo doc --no-deps --workspace            # Generate docs

# === DEBUGGING ===
RUST_LOG=debug cargo test --package <crate>  # With traces
cargo test --workspace -- --nocapture       # Show output

# === RELEASE ===
cargo build --release                       # Optimized build
upx target/release/synesis               # Upload binary
```

### Project Structure

```
/mnt/c/claudesuperinstance/
├── CLAUDE.md                               ← This file
├── README.md                                ← User overview
├── Cargo.toml                               ← Workspace config
│
├── crates/                                 ← Rust workspace
│   ├── synesis-cli/                        ← CLI
│   ├── synesis-core/                       ← Core (tripartite)
│   ├── synesis-knowledge/                  ← Knowledge (RAG)
│   ├── synesis-models/                        ← Models (hardware)
│   ├── synesis-privacy/                     ← Privacy (redaction)
│   └── synesis-cloud/                        ← Cloud (QUIC)
│
├── docs/                                   ← User documentation
├── phases/                                 ← Phase roadmaps
├── status/                                 ← Status reports
├── tools/                                  ← Tool guides (NEW)
└── architecture/                          └── Architecture docs
```

---

## Commit & Push Strategy

### Before Committing

```bash
# 1. Run full test suite
cargo test --workspace

# 2. Check for warnings
cargo clippy --workspace --all-targets -- -D warnings

# 3. Format code
cargo fmt --all

# 4. Check documentation
cargo doc --no-deps --workspace 2>&1 | grep -c "warning"
```

### Commit Message Format

```
<type>: <subject>

<details>
<summary>
Brief summary of changes
</summary>

<body>
Detailed explanation of what was changed and why
</body>

<footer>
Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>
</footer>
```

### Push Strategy

```bash
# Push to main branch
git push origin main

# Or for specific branch
git push origin feature-branch
```

---

## Key Files to Reference

**Architecture**:
- `CLAUDE.md` (this file) - Complete development guide
- `README.md` - Project overview
- `ARCHITECTURE.md` - System architecture (if exists)

**Phase Planning**:
- `phases/PHASE_1_LOCAL_KERNEL.md`
- `phases/PHASE_2_DETAILED_ROADMAP.md`

**Status Reports**:
- `status/PHASE_2_COMPLETE.md`
- `status/FINAL_COMPREHENSIVE_IMPROVEMENTS_REPORT.md`

**Documentation**:
- `docs/CLI_USER_GUIDE.md`
- `docs/TROUBLESHOOTING.md`
- `docs/USAGE_EXAMPLES.md`

**Tool Guides** (NEW):
- `tools/PRIVACY_PROXY_TOOL.md`
- `tools/TOKEN_VAULT_TOOL.md`
- `tools/CONSENSUS_ENGINE_TOOL.md`
- `tools/KNOWLEDGE_VAULT_TOOL.md`
- `tools/HARDWARE_DETECTOR_TOOL.md`
- `tools/MODEL_REGISTRY_TOOL.md`
- `tools/QUIC_TUNNEL_TOOL.md`
- `tools/METERED_BILLING_TOOL.md`
- `tools/OLLAMA_INTEGRATION.md`
- `tools/LM_STUDIO_INTEGRATION.md`
- `tools/LOCALAI_INTEGRATION.md`
- `tools/VLLM_INTEGRATION.md`
- `tools/NEMO_INTEGRATION.md`
- `tools/VILA_INTEGRATION.md`

---

**Last Updated**: 2026-01-08
**Version**: v0.2.0
**Tests**: 298/298 passing (100%)
**Status**: ✅ PRODUCTION READY

**Next Phase**: Multi-Interface Implementation & Tool Extraction

---

*For detailed documentation on specific features, see the corresponding files listed above.*
