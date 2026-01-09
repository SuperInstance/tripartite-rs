# SuperInstance Build Orchestrator - Ralph Style

> **Orchestration Strategy**: Ralph Wiggum Continuous Loops
> **Active Agents**: Always 4 agents working in parallel
> **Mode**: Continuous iteration until `<promise>TASK_COMPLETE</promise>`
> **Auto-Accept**: Enabled for all agent spawning
> **Focus**: Highest-value tasks first, no sequential waiting

---

## What is Ralph-Style Orchestration?

You are a **Ralph Orchestrator**. Your job is to:

1. **Always keep 4 agents working** on the highest-value tasks
2. **Let agents iterate continuously** until they output `<promise>TASK_COMPLETE</promise>`
3. **Spawn new agents immediately** when one finishes (no waiting)
4. **Focus on value**, not predetermined sequences
5. **Let agents see their own work** and improve iteratively (self-referential)

### Ralph Loop Pattern

Each agent runs in a **continuous loop**:
```
while :; do
  work on task
  modify files
  run tests
  if task_complete:
    output <promise>TASK_COMPLETE</promise>
    break
  else:
    see previous work
    iterate and improve
  fi
done
```

The "self-reference" comes from agents seeing their own previous work in files and git history, not from feeding output back as input.

---

## Core Principles

### 🎯 Value-Driven Execution
- ✅ **Always 4 agents working** - never wait for rounds
- ✅ **Spawn next task immediately** when an agent finishes
- ✅ **Focus on highest-value work** - what provides the most impact?
- ✅ **Let agents iterate** - they see their work and improve
- ✅ **Completion promises** - agents output `<promise>TASK_COMPLETE</promise>` when done

### 🔄 Continuous Workflow (Not Sequential!)

**Old Way (Sequential Rounds - ❌)**:
```
Round 1 → wait → Round 2 → wait → Round 3 → wait → ...
```

**New Way (Ralph Continuous - ✅)**:
```
Spawn 4 agents on highest-value tasks
As soon as one completes → spawn next highest-value task
Always 4 agents working
No waiting, no rounds
```

### 📦 Repository Strategy

**When to Create a New Repo:**
- ✅ Tool is independently useful (others could use it)
- ✅ API is stable and well-documented
- ✅ Tests pass (100% pass rate)
- ✅ Zero warnings
- ✅ Has README that converts in 10 seconds
- ✅ Has examples showing usage

**Repo Requirements:**
- Complete README.md with badges
- LICENSE file (MIT OR Apache-2.0)
- CONTRIBUTING.md
- CI/CD workflows
- Examples directory (3+ examples)
- Comprehensive documentation
- Cross-references to related tools

**Pushing to GitHub:**
1. Create repo on GitHub (https://github.com/SuperInstance)
2. Add remote: `git remote add <tool> https://github.com/SuperInstance/<tool>.git`
3. Push: `git push <tool> main`
4. Create GitHub release (v0.1.0)
5. Publish to crates.io (if Rust crate)
6. Update Tripartite1 to use the new crate/repo
7. Update ecosystem documentation with cross-references

---

## High-Value Task Backlog

These are the tasks to work on, **ordered by value** (highest first). Agents should work through these in order.

### 🔥 IMMEDIATE HIGH VALUE (Do These First)

1. **Complete Privox Publishing** (if not already done)
   - Verify all 37 tests pass
   - Zero compiler warnings
   - Zero clippy warnings
   - Create GitHub repo
   - Publish to crates.io
   - Output: `<promise>PRIVOX_PUBLISHED</promise>`

2. **Complete Tripartite-RS Publishing** (if not already done)
   - Verify extraction is complete
   - All tests passing
   - Create GitHub repo
   - Publish to crates.io
   - Output: `<promise>TRIPARTITE_PUBLISHED</promise>`

3. **Extract Knowledge-Vault as Independent Crate**
   - Extract vault and indexing logic from synesis-knowledge
   - Remove SuperInstance dependencies
   - Create embedder trait for pluggable models
   - Design chunking strategy interface
   - Write comprehensive README
   - Create 5+ examples
   - Build CI/CD workflows
   - All tests passing
   - Create GitHub repo
   - Publish to crates.io
   - Output: `<promise>KNOWLEDGE_VAULT_PUBLISHED</promise>`

4. **Extract Hardware Detection (hwscan)**
   - Extract hardware detection logic
   - Create platform-specific modules (Linux, macOS, Windows)
   - Design tier calculation API
   - Build CLI tool
   - Write comprehensive README
   - Create examples
   - Build CI/CD
   - Create GitHub repo
   - Publish to crates.io
   - Output: `<promise>HWSCAN_PUBLISHED</promise>`

### 🎯 HIGH VALUE (Do These After Top 4)

5. **Build Model-Registry Tool**
   - Extract model registry logic
   - Create version tracking
   - Build metadata storage
   - Design download API
   - Create CLI tool
   - Write docs and examples
   - Output: `<promise>MODEL_REGISTRY_PUBLISHED</promise>`

6. **Build Token-Vault Tool**
   - Extract vault core logic
   - Add encryption-at-rest support
   - Build secure token storage
   - Design session isolation
   - Create CLI tools (server, client, admin)
   - Security audit
   - Output: `<promise>TOKEN_VAULT_PUBLISHED</promise>`

7. **Build QUIC-Tunnel Tool (quicunnel)**
   - Extract QUIC tunnel logic
   - Create connection management
   - Build stream handling
   - Add mTLS authentication
   - Build heartbeat and reconnection
   - Performance benchmarks
   - Output: `<promise>QUICUNNEL_PUBLISHED</promise>`

8. **Build Metered-Billing Tool (usemeter)**
   - Build core metering logic
   - Create event tracking
   - Design aggregation system
   - Build billing engine
   - Integration with other tools
   - Output: `<promise>USEMETER_PUBLISHED</promise>`

### 📚 MEDIUM VALUE (Documentation & Examples)

9. **Build Integration Examples - Basic**
   - CLI integration examples (privox, tripartite-rs, knowledge-vault, hwscan)
   - Web service integration examples
   - Library integration examples
   - Write integration guide
   - Output: `<promise>BASIC_INTEGRATIONS_COMPLETE</promise>`

10. **Build Integration Examples - Advanced**
    - Full-stack application example
    - Multi-agent system example
    - Distributed system example
    - Performance optimization examples
    - Output: `<promise>ADVANCED_INTEGRATIONS_COMPLETE</promise>`

### 🎨 LOWER VALUE (Polish & Ecosystem)

11. **Unified Documentation Site (mdBook)**
    - Set up mdBook structure
    - Migrate all READMEs
    - Create visual assets and diagrams
    - Set up GitHub Pages deployment
    - Output: `<promise>DOCS_SITE_COMPLETE</promise>`

12. **Ecosystem CI/CD**
    - Standardize CI workflows
    - Create shared GitHub Actions
    - Set up Dependabot
    - Build release automation
    - Output: `<promise>CICD_COMPLETE</promise>`

13-25: *(Additional polish, testing, marketing, launch tasks - see appendix)*

---

## How to Spawn Ralph-Style Agents

Use the `/ralph-loop` command to spawn agents that will iterate continuously:

```bash
/ralph-loop "Task description here. Output <promise>TASK_NAME</promise> when complete."
```

### Example: Spawning 4 Agents

```bash
# Agent 1: Complete Privox Publishing
/ralph-loop "Complete the Privox publishing. Verify all 37 tests pass, zero warnings, create GitHub repo at https://github.com/SuperInstance/privox, publish to crates.io. Output <promise>PRIVOX_PUBLISHED</promise> when done."

# Agent 2: Complete Tripartite-RS Publishing
/ralph-loop "Complete the Tripartite-RS publishing. Verify extraction is complete, all tests passing, create GitHub repo at https://github.com/SuperInstance/tripartite-rs, publish to crates.io. Output <promise>TRIPARTITE_PUBLISHED</promise> when done."

# Agent 3: Extract Knowledge-Vault
/ralph-loop "Extract Knowledge-Vault as an independent crate. Extract vault and indexing logic from synesis-knowledge, remove SuperInstance dependencies, create embedder trait, design chunking strategy, write comprehensive README, create 5+ examples, build CI/CD, create GitHub repo, publish to crates.io. Output <promise>KNOWLEDGE_VAULT_PUBLISHED</promise> when done."

# Agent 4: Extract Hardware Detection
/ralph-loop "Extract Hardware Detection (hwscan) as an independent crate. Extract hardware detection logic, create platform-specific modules for Linux/macOS/Windows, design tier calculation API, build CLI tool, write comprehensive README, create examples, build CI/CD, create GitHub repo, publish to crates.io. Output <promise>HWSCAN_PUBLISHED</promise> when done."
```

### Monitoring Agent Progress

Agents will iterate continuously. Check their progress by:

1. **Reading their output files** (if they create status files)
2. **Checking git commits** to see what they've done
3. **Running tests** to verify quality
4. **Looking for completion promises** in their outputs

### When an Agent Completes

When you see `<promise>TASK_NAME</promise>` from an agent:

1. **Mark the task as complete** in your tracking
2. **Verify the deliverables** (tests pass, repo exists, docs complete)
3. **Spawn the next highest-value task** from the backlog
4. **Always keep 4 agents working**

---

## Quality Standards

All agents must meet these standards before outputting completion promises:

### Code Quality
- ✅ All tests pass (100% pass rate)
- ✅ Zero compiler warnings
- ✅ Zero clippy warnings
- ✅ Code formatted (`cargo fmt`)
- ✅ Commits pushed to git

### Documentation
- ✅ README converts visitors in 10 seconds
- ✅ 3+ examples in `examples/`
- ✅ API documentation complete
- ✅ Getting started guide
- ✅ LICENSE file (MIT OR Apache-2.0)
- ✅ CONTRIBUTING.md

### CI/CD
- ✅ GitHub Actions workflow (multi-platform)
- ✅ Tests pass on Linux, macOS, Windows
- ✅ Security scanning configured
- ✅ Dependabot configured

### Publishing
- ✅ GitHub repo created at https://github.com/SuperInstance/<tool>
- ✅ Code pushed to main branch
- ✅ Release v0.1.0 created with release notes
- ✅ Published to crates.io (if Rust crate)
- ✅ Cross-references in ecosystem docs

---

## Task Tracking

Maintain this status section at the bottom of CLAUDE.md:

```markdown
## Active Agent Status

**Agent 1**: [Task name] - [Status]
- Spawned: [Timestamp]
- Last seen: [Timestamp]
- Progress: [Brief update]

**Agent 2**: [Task name] - [Status]
- Spawned: [Timestamp]
- Last seen: [Timestamp]
- Progress: [Brief update]

**Agent 3**: [Task name] - [Status]
- Spawned: [Timestamp]
- Last seen: [Timestamp]
- Progress: [Brief update]

**Agent 4**: [Task name] - [Status]
- Spawned: [Timestamp]
- Last seen: [Timestamp]
- Progress: [Brief update]

## Completed Tasks

- [x] Task 1 - Completed at [Timestamp]
- [x] Task 2 - Completed at [Timestamp]
- [ ] Task 3 - Next up
- [ ] Task 4 - In queue
```

---

## Appendix: Full Task List (Rounds 13-25)

For reference, here are the remaining lower-priority tasks:

13. Build Integration Examples - Basic
14. Build Integration Examples - Advanced
15. Create Unified Documentation Site (mdBook)
16. Build Ecosystem CI/CD
17. Build Testing Infrastructure
18. Create Contribution Guides
19. Build Marketing Materials
20. Build Performance Benchmarking Suite
21. Build Security Auditing
22. Create Migration Guides
23. Build Training Materials
24. Build Enterprise Features
25. Final Polish & Launch

---

## Current Status

**Orchestrator Mode**: Ralph Wiggum Continuous Loops
**Active Agents**: 0 (spawn 4 now)
**Next Action**: Spawn 4 agents on highest-value tasks using `/ralph-loop`
**Focus**: Keep 4 agents working continuously on highest-value tasks

🚀 **SPAWN 4 RALPH-LOOP AGENTS NOW**
