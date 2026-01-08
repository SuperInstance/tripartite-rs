# Naming Strategy - At a Glance

**Visual summary of independent tools naming recommendations**

---

## Quick Reference Table

| # | Tool | Recommended Name | Type | Priority | Status |
|---|------|------------------|------|----------|--------|
| 1 | **Privacy Proxy** | `privox` | Creative | 🔴 P1 | 🔴 Check availability |
| 2 | **Token Vault** | `token-keeper` | Descriptive | 🟢 P3 | 🟡 Check availability |
| 3 | **Consensus Engine** | `tripartite-rs` | Creative | 🔴 P1 | 🟢 Likely available |
| 4 | **Knowledge Vault** | `knowledgr` | Creative | 🟡 P2 | 🟢 Likely available |
| 5 | **Hardware Detector** | `hwscan-rs` | Technical | 🟡 P2 | 🟢 Likely available |
| 6 | **Model Registry** | `model-registry` | Descriptive | 🟢 P3 | 🟡 Check availability |
| 7 | **QUIC Tunnel** | `quicunnel` | Creative | 🔴 P1 | 🟢 Likely available |
| 8 | **Metered Billing** | `usemeter` | Creative | 🔴 P1 | 🟢 Likely available |

**Legend**:
- 🔴 P1 = Immediate priority (first-mover opportunity)
- 🟡 P2 = Short-term priority (market gap)
- 🟢 P3 = Medium-term priority (incremental value)

---

## Visual Naming Map

```
┌─────────────────────────────────────────────────────────────────┐
│                    SUPERINSTANCE ECOSYSTEM                      │
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │   PRIVOX     │  │  TRIPARTITE  │  │   QUICUNNEL  │  Tier 1   │
│  │  (Privacy)   │  │  (Consensus) │  │   (Tunnel)   │  (Brand)  │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
│         ↓                  ↓                  ↓                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │  KNOWLEDGR   │  │   USEMETER   │  │  HWSCAN-RS   │  Tier 2   │
│  │ (Knowledge)  │  │  (Billing)   │  │  (Hardware)  │  (Tech)   │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐                              │
│  │TOKEN-KEEPER  │  │MODEL-REGISTRY│         Tier 3              │
│  │   (Vault)    │  │  (Models)    │      (Functional)           │
│  └──────────────┘  └──────────────┘                              │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Decision Tree: Which Name Style?

```
                    ┌─────────────────┐
                    │  Is this tool   │
                    │ brand-critical? │
                    └────────┬────────┘
                             │
                    ┌────────┴────────┐
                    │                 │
                   YES               NO
                    │                 │
                    ▼                 ▼
            ┌───────────────┐   ┌───────────────┐
            │ Is it the     │   │ Is it         │
            │ first-mover? │   │ technical?    │
            └───────┬───────┘   └───────┬───────┘
                    │                   │
            ┌───────┴───────┐   ┌───────┴───────┐
            │               │   │               │
           YES             NO  YES            NO
            │               │   │               │
            ▼               ▼   ▼               ▼
      ┌──────────┐   ┌──────────┐ ┌──────────┐ ┌──────────┐
      │Creative  │   │Hybrid    │ │Technical │ │Descriptive│
      │(privox)  │   │(synesis-)│ │(-rs)     │ │(-function)│
      └──────────┘   └──────────┘ └──────────┘ └──────────┘
```

**Apply to tools:**
- `privox` = Creative (brand-critical, first-mover)
- `tripartite-rs` = Creative (first-mover advantage)
- `quicunnel` = Creative (memorable)
- `hwscan-rs` = Technical (developer tool)
- `usemeter` = Creative (distinctive billing)
- `knowledgr` = Creative (brandable)
- `token-keeper` = Descriptive (functional)
- `model-registry` = Descriptive (functional)

---

## Competitive Landscape

### Privacy Tools
```
Competition:     data_privacy, hanzo-guard, cleansh
Our Position:   privox (distinctive brand)
Market Gap:     No dominant "privacy redaction" brand
Opportunity:    🔴 HIGH (LLM boom increasing demand)
```

### Consensus Tools
```
Competition:    f1-nexus-mcp, hope_agents, agentic-flow
Our Position:   tripartite-rs (first-mover on "tripartite")
Market Gap:     No tripartite-specific crate exists
Opportunity:    🔴 CRITICAL (unique concept ownership)
```

### Knowledge Tools
```
Competition:    reasonkit-mem, helix-db, rig
Our Position:   knowledgr (creative "vault" brand)
Market Gap:     No "knowledge vault" branding
Opportunity:    🟡 MEDIUM (RAG market exploding)
```

### Hardware Tools
```
Competition:    hardware_query, gdt-cpus, usbwatch-rs
Our Position:   hwscan-rs (technical, AI-focused)
Market Gap:     No AI hardware profiler
Opportunity:    🟡 MEDIUM (niche but valuable)
```

### Model Tools
```
Competition:    cargo-verctl, crm (generic tools)
Our Position:   model-registry (descriptive)
Market Gap:     No AI model-specific registry
Opportunity:    🟡 MEDIUM (growing need)
```

### QUIC Tools
```
Competition:    quinn, tokio-quiche, quic-reverse
Our Position:   quicunnel (high-level tunnel)
Market Gap:     No "tunnel" branded crate
Opportunity:    🔴 HIGH (QUIC adoption growing)
```

### Billing Tools
```
Competition:    metered, usage-tracker, ratelimit_meter
Our Position:   usemeter (billing-focused, not metrics)
Market Gap:     No billing calculation crate
Opportunity:    🔴 HIGH (usage-based pricing trend)
```

### Vault Tools
```
Competition:    rust-keyvault, rusty-vault, novovault
Our Position:   token-keeper (lightweight, AI-focused)
Market Gap:     No AI tokenization vault
Opportunity:    🟢 LOW (less competitive space)
```

---

## Name Availability Status

### 🔴 Verify Immediately (This Week)
- [ ] `privox` - Creative brand, high priority
- [ ] `tripartite-rs` - First-mover, critical
- [ ] `quicunnel` - Distinctive, high priority

### 🟡 Verify This Week
- [ ] `model-registry` - Descriptive, check if taken
- [ ] `token-keeper` - Descriptive, may be conflicts

### 🟢 Verify Soon (This Month)
- [ ] `knowledgr` - Creative, likely available
- [ ] `usemeter` - Creative, likely available
- [ ] `hwscan-rs` - Technical, likely available

**Action**: Run `./check-crate-availability.sh` to verify all names

---

## Implementation Timeline

```
Week 1: Name Reservation
├─ Run availability checker
├─ Create GitHub repos (top 3)
├─ Publish placeholder crates
└─ Purchase domains

Week 2-3: Brand Development
├─ Design logos (consistent system)
├─ Set up documentation sites
├─ Create README templates
└─ Establish social presence

Week 4-6: Code Extraction (Tool 1-3)
├─ Extract tripartite-rs
├─ Extract privox
├─ Extract quicunnel
└─ Publish v0.1.0

Week 7-8: Launch & Promotion
├─ Publish blog posts
├─ Submit to Reddit/HN
├─ Add to awesome lists
└─ Community engagement

Month 3-6: Remaining Tools
├─ Extract knowledgr
├─ Extract usemeter
├─ Extract hwscan-rs
├─ Extract model-registry
├─ Extract token-keeper
└─ Publish v0.1.0

Month 6-12: Ecosystem Growth
├─ Cross-promotion
├─ Enterprise adoption
├─ Conference talks
└─ Community building
```

---

## Naming Convention Checklist

### ✅ Do's
- ✅ Use lowercase letters
- ✅ Use hyphens for multi-word names
- ✅ Keep under 30 characters
- ✅ Make it pronounceable
- ✅ Check for unintended meanings
- ✅ Verify trademark conflicts
- ✅ Ensure .crate name matches repo name

### ❌ Don'ts
- ❌ Use underscores (use hyphens)
- ❌ Use CamelCase
- ❌ Start with numbers
- ❌ Use special characters
- ❌ Use trademarked terms
- ❌ Make it too long
- ❌ Use abbreviations (unless well-known)

---

## Alternative Names (Backup Options)

### Privacy
```
1. pii-guardian
2. sanctum-rs
3. datum-safe
4. sentinel-pii
5. cloak-data
```

### Consensus
```
1. consensus-council
2. accord-rs
3. trinity-vote
4. unanimity
5. agent-vote
```

### Knowledge
```
1. rag-vault
2. memoria-rs
3. vector-cortex
4. cortex-vault
5. mem-brane
```

### Hardware
```
1. hardware-probe
2. sys-cap
3. device-detect
4. hardware-insight
5. sys-profile
```

### Models
```
1. model-keep
2. model-version
3. model-catalog
4. version-keeper
5. model-hub
```

### QUIC
```
1. quic-tunnel
2. quic-stream
3. quic-bridge
4. quic-pipe
5. quic-channel
```

### Billing
```
1. usage-meter
2. consumeter
3. bill-meter
4. metered-billing
5. usage-bill
```

### Vault
```
1. secure-token
2. token-fortress
3. keystore-rs
4. token-crypt
5. vaultis
```

---

## Success Metrics

### Track These Metrics

| Metric | 3 Month | 6 Month | 12 Month |
|--------|---------|---------|----------|
| Tools Published | 3 | 8 | 8 |
| Total Downloads | 3K | 40K | 160K |
| GitHub Stars | 150 | 1.6K | 4K |
| External Users | 6 | 80 | 400 |
| Blog Posts | 3 | 8 | 12 |

### Quality Gates

Before publishing each tool:
- ✅ All tests passing (100%)
- ✅ Zero compiler warnings
- ✅ All public APIs documented
- ✅ README with examples
- ✅ At least 1 working example
- ✅ LICENSE file included

---

## Key Takeaways

1. **First-Mover Advantage** - `tripartite-rs` is uniquely positioned
2. **Brand Building** - Creative names (`privox`) outperform generic ones
3. **Market Timing** - AI boom, QUIC adoption, RAG growth create opportunities
4. **Quality Matters** - Documentation and examples drive adoption
5. **Community** - Responsive maintainer = successful tool

---

## Quick Links

**Strategy Documents:**
- [Full Strategy](./INDEPENDENT_TOOLS_NAMING_STRATEGY.md) - 24KB comprehensive guide
- [Quick Reference](./NAMING_QUICK_REFERENCE.md) - 6.8KB lookup guide
- [Competitive Analysis](./COMPETITIVE_ANALYSIS_NAMING.md) - 14KB market analysis
- [Extraction Guide](./TOOL_EXTRACTION_GUIDE.md) - 14KB implementation
- [Executive Summary](./NAMING_STRATEGY_SUMMARY.md) - 12KB overview

**Utilities:**
- [Availability Checker](./check-crate-availability.sh) - Automated verification

---

## Next Steps

**Today:**
1. Read this document (done ✅)
2. Read [Executive Summary](./NAMING_STRATEGY_SUMMARY.md)
3. Run availability checker

**This Week:**
4. Reserve top 3 names
5. Create GitHub repos
6. Set up documentation sites

**This Month:**
7. Extract first tool
8. Publish v0.1.0
9. Launch marketing campaign

---

*Version: 1.0*
*Created: 2026-01-08*
*Author: SuperInstance AI Team*
*Status: Ready for Implementation*
