# Independent Tools Naming - Quick Reference

**Quick lookup guide for recommended names**

---

## Top Recommendations Summary

| Tool | Internal Name | Publishing Name | Brand | Status |
|------|--------------|-----------------|-------|--------|
| **Privacy Proxy** | `synesis-privacy` | `privox` | Creative | 🔴 Check |
| **Consensus Engine** | `synesis-core` | `tripartite-rs` | Creative | 🟢 Likely Avail |
| **Knowledge Vault** | `synesis-knowledge` | `knowledgr` | Creative | 🟢 Likely Avail |
| **Hardware Detector** | `synesis-models` | `hwscan-rs` | Technical | 🟢 Likely Avail |
| **Model Registry** | `synesis-models` | `model-registry` | Descriptive | 🟡 Check |
| **QUIC Tunnel** | `synesis-cloud` | `quicunnel` | Creative | 🟢 Likely Avail |
| **Metered Billing** | `synesis-cloud` | `usemeter` | Creative | 🟢 Likely Avail |
| **Token Vault** | `synesis-privacy` | `token-keeper` | Descriptive | 🟡 Check |

---

## Decision Matrix

### When to use each naming approach

| Scenario | Recommended Style | Example |
|----------|-------------------|---------|
| **Brand-building tool** | Creative/Memorable | `privox`, `knowledgr` |
| **Core infrastructure** | Hybrid (synesis-*) | `synesis-consensus` |
| **Developer utility** | Technical | `hwscan-rs`, `quicunnel` |
| **Enterprise-focused** | Descriptive | `model-registry`, `token-keeper` |

---

## Naming Pattern Generator

### Formula: `{brand}` + `{function}` + `{suffix}`

#### Brand Options
- `privox` - Privacy tools
- `synesis` - Core ecosystem
- `tripartite` - Multi-agent systems
- No brand - Purely descriptive

#### Function Options
- `redact` - PII redaction
- `vault` - Secure storage
- `consensus` - Voting/agreement
- `mem` - Memory/knowledge
- `hw` - Hardware
- `model` - AI models
- `tunnel` - Networking
- `meter` - Billing/usage

#### Suffix Options
- `-rs` - Rust implementation
- No suffix - Modern/brandable
- `-kit` - Toolkit/framework
- `-engine` - Core component

---

## Alternative Names by Category

### Privacy Tools
```
Creative:
  privox         pii-guardian    sanctum-rs
  datum-safe     sentinel-pii    cloak-data

Descriptive:
  pii-redact     privacy-shield  auto-redact
  data-sanitize  redact-stream
```

### Consensus Tools
```
Creative:
  tripartite-rs  accord-rs       trinity-vote
  unanimity      tri-agent

Descriptive:
  consensus-council  agent-vote     council-engine
  agent-consensus
```

### Knowledge Tools
```
Creative:
  knowledgr      memoria-rs      mem-brane
  vector-cortex  cortex-vault

Descriptive:
  rag-vault      rag-store       knowledge-base
  retrieval-engine
```

### Hardware Tools
```
Technical:
  hwscan-rs      sys-cap         cpu-gpu-rs
  hardware-probe sys-profile

Descriptive:
  device-detect  hardware-insight  machine-probe
```

### Model Management
```
Descriptive:
  model-registry  model-version   model-catalog
  model-track     version-keeper

Creative:
  model-keep      model-vault     model-hub
```

### QUIC/Networking
```
Creative:
  quicunnel       quic-pipe       quic-pass

Descriptive:
  quic-tunnel     quic-stream     quic-bridge
  quic-channel    tunnel-quic
```

### Billing/Metering
```
Creative:
  usemeter        consumeter      bill-meter

Descriptive:
  usage-meter     metered-billing  usage-bill
  meter-track     billing-track
```

### Vault/Storage
```
Descriptive:
  token-keeper    secure-token    keystore-rs
  token-vault     vault-lock

Creative:
  vaultis         token-fortress  token-crypt
  token-warden
```

---

## Crates.io Naming Rules

### Requirements
- ✅ Lowercase letters only
- ✅ Numbers allowed (but not at start)
- ✅ Hyphens allowed as separators
- ❌ No underscores (use hyphens instead)
- ❌ No spaces
- ❌ No special characters

### Best Practices
- Keep under 30 characters
- Use hyphens to separate words
- Avoid `-rs` suffix (modern style)
- Make it pronounceable
- Avoid trademarked terms

### Examples
```
✅ good-name
✅ privox
✅ tripartite-rs
✅ knowledgr
❌ Bad_Name (underscores)
❌ CamelCase (mixed case)
❌ starts-with-number
❌ trademark! (special chars)
```

---

## Brand Consistency Guidelines

### If using "privox" brand:
```
privox-redact    (Core PII redaction)
privox-vault     (Token storage - future)
privox-stream    (Stream redaction - future)
privox-cli       (CLI tool - future)
```

### If using "synesis" ecosystem:
```
synesis-privacy     (Current)
synesis-consensus   (Refactored from core)
synesis-knowledge   (Current)
synesis-hardware    (Refactored from models)
synesis-tunnel      (Refactored from cloud)
synesis-billing     (Refactored from cloud)
```

### If using mixed approach:
```
privox             (Privacy - standalone brand)
tripartite-rs      (Consensus - standalone)
hwscan-rs          (Hardware - technical)
quicunnel          (Tunnel - technical)
usemeter           (Billing - technical)
synesis-knowledge  (Keep current - ecosystem)
model-registry     (Registry - descriptive)
token-keeper       (Vault - descriptive)
```

---

## SEO & Discoverability

### Keywords to include in description/crates.io metadata:
```
Privacy:
  pii redaction sanitization sensitive data protection

Consensus:
  multi-agent voting coordination agreement tripartite

Knowledge:
  rag vector search retrieval embeddings memory

Hardware:
  detection gpu cpu profiling capabilities cross-platform

Models:
  registry version management llm ai tracking

Tunnel:
  quic streaming tunneling networking cloud

Billing:
  metering usage tracking billing cost
```

---

## Quick Checklist

Before publishing a name, verify:

- [ ] Checked crates.io availability (404 response)
- [ ] Checked GitHub org availability
- [ ] Searched Google for existing projects
- [ ] Checked trademarks (USPTO database)
- [ ] Verified domain availability (optional)
- [ ] Confirmed no conflicts with major brands
- [ ] Tested pronunciation (say it out loud)
- [ ] Checked for unintended meanings
- [ ] Verified follows Rust naming conventions
- [ ] Confirmed team alignment on name

---

## Final Recommendation

### Phase 1: Immediate (This Week)
1. **`tripartite-rs`** - First-mover advantage
2. **`privox`** - Strong brand potential
3. **`quicunnel`** - Memorable technical name

### Phase 2: Short-term (Next 2 Weeks)
4. **`hwscan-rs`** - Technical utility
5. **`usemeter`** - Clear billing tool
6. **`knowledgr`** - Creative knowledge brand

### Phase 3: Medium-term (Next Month)
7. **`model-registry`** (if available) or **`model-keep`**
8. **`token-keeper`** - Descriptive vault tool

---

## Contact & Resources

- **Naming Strategy Doc**: `INDEPENDENT_TOOLS_NAMING_STRATEGY.md`
- **Availability Script**: `check-crate-availability.sh`
- **crates.io**: https://crates.io
- **Naming Guidelines**: https://doc.rust-lang.org/cargo/reference/manifest.html#name

---

*Version: 1.0*
*Last Updated: 2026-01-08*
*SuperInstance AI Team*
