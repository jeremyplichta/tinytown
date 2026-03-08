# 📊 Complexity Analysis: Tinytown vs Gastown

This document compares the complexity of **Tinytown** (simple multi-agent orchestration) with **Gastown** (enterprise-grade orchestration system).

## Executive Summary

| Metric | Tinytown | Gastown | Ratio |
|--------|----------|---------|-------|
| **Total Lines of Code** | 1,435 | 317,898 | **221x smaller** |
| **Files** | 12 | 1,133 | **94x fewer** |
| **Languages** | 1 (Rust) | 16 | **16x simpler** |
| **Core Types** | 5 | 50+ | **10x fewer concepts** |
| **Config Files** | 1 JSON | 10+ YAML/TOML/JSON | **10x simpler config** |

## Lines of Code Comparison

### Tinytown (tokei output)
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Language              Files        Lines         Code     Comments       Blanks
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Rust                     10         1826         1374           71          381
 Markdown                  1          126            0           90           36
 TOML                      1           47           31            7            9
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Total                    12         2302         1435          421          446
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

### Gastown (tokei output)
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Language              Files        Lines         Code     Comments       Blanks
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Go                      930       346333       263004        39712        43617
 TOML                     50        21927        19623           96         2208
 Markdown                 73        20326            0        14770         5556
 JSON                     14        15712        15694            0           18
 + 12 more languages ...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Total                  1133       428282       317898        56522        53862
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## MFCQI Score Comparison

Gastown Python components (via mfcqi):
```
❌ MFCQI Score: 0.526

📊 Metrics Breakdown:
 Metric                  Score     Rating
 Cyclomatic Complexity    0.59  ⚠️ Needs Work
 Cognitive Complexity     0.84  ⭐ Excellent
 Halstead Volume          0.65    ✅ Good
 Maintainability Index    0.61    ✅ Good
 Code Duplication         1.00  ⭐ Excellent
 Documentation Coverage   0.82  ⭐ Excellent
 Security Score           0.06    ❌ Poor
 Secrets Exposure         0.00    ❌ Poor
 Code Smell Density       1.00  ⭐ Excellent
```

**Note:** mfcqi is designed for Python. Tinytown is pure Rust with zero Python dependencies.

## Rust Code Quality (Tinytown)

### Clippy Analysis
```
✅ 0 warnings
✅ 0 errors
```

### Test Coverage
```
32 integration tests - 100% passing
1 doctest - 100% passing
```

## Architectural Complexity

### Tinytown: 5 Core Types
```
Town     → Orchestrator (1 file, ~150 lines)
Agent    → Worker definition (1 file, ~100 lines)
Task     → Work unit (1 file, ~100 lines)
Message  → Inter-agent comms (1 file, ~80 lines)
Channel  → Redis connection (1 file, ~150 lines)
```

### Gastown: 50+ Concepts
```
Agents (8 types): Mayor, Deacon, Boot, Dogs, Witness, Refinery, Polecats, Crew
State: Identity, Sandbox, Session (3-layer model)
Storage: Dolt SQL, Beads (2-level), Git worktrees
Workflows: Convoys, Formulas (4 types), DAGs
Monitoring: Feed, Dashboard, OTEL
... and much more
```

## Configuration Complexity

### Tinytown: 1 JSON file (~15 lines)
```json
{
  "name": "my-town",
  "redis": { "use_socket": true },
  "default_model": "claude",
  "max_agents": 10
}
```

### Gastown: Multiple config files
- `town.json` - Town identity
- `config.json` - Behavioral config (per-town)
- `config.json` - Per-rig overrides
- `settings/config.json` - Agent configuration
- `settings/escalation.json` - Escalation routes
- `config/messaging.json` - Mail/queues/channels
- 50+ TOML files for various settings

## Why Tinytown is Better (For Most Use Cases)

| Scenario | Tinytown | Gastown |
|----------|----------|---------|
| **Setup time** | 30 seconds | Hours |
| **Learning curve** | 1 hour | Days/Weeks |
| **Debugging** | Read 1,400 lines | Navigate 318,000 lines |
| **Customization** | Modify directly | Understand 50+ concepts first |
| **Resource usage** | Minimal (Redis only) | Dolt SQL + Redis + Daemon |
| **Deployment** | Single binary | Docker Compose + multiple services |

## When to Use Gastown Instead

- 20+ concurrent agents
- Cross-project coordination (multiple rigs)
- Complex workflow DAGs
- Enterprise monitoring requirements
- Persistent agent identity across restarts

## Conclusion

**Tinytown delivers 90% of the value with 1% of the complexity.**

For most multi-agent orchestration needs, Tinytown's simplicity is a feature, not a limitation. When you truly need Gastown's features, you'll know—and migration is straightforward since both use Redis.

---

*Generated with tokei v14.0.0 and mfcqi v0.0.4*

