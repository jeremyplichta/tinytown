# Agent Coordination

How agents work together and decide when tasks are complete.

## The Simple Model

Tinytown keeps coordination simple:

1. **Conductor** orchestrates (spawns agents, assigns tasks)
2. **Workers** do the work
3. **Reviewer** decides when work is done
4. **Conductor** monitors and coordinates handoffs

## The Reviewer Pattern

Always include a reviewer agent. They're your quality gate:

```
┌────────────┐     work      ┌────────────┐
│  Conductor │ ────────────► │   Worker   │
└─────┬──────┘               └─────┬──────┘
      │                            │
      │                            │ completes
      │                            ▼
      │    review request    ┌────────────┐
      │ ────────────────────►│  Reviewer  │
      │                      └─────┬──────┘
      │                            │
      │◄───────────────────────────┘
      │      approve / reject
      │
      ▼
   Done (or assign fixes)
```

## Why a Reviewer?

Without a reviewer, who decides "done"?

| Approach | Problem |
|----------|---------|
| Worker decides | "I'm done" but is it good? |
| Conductor decides | Conductor may not understand domain |
| User decides | User has to check everything |
| **Reviewer decides** | ✓ Separation of concerns |

The reviewer pattern is used everywhere: code review, QA, editing. It works.

## How It Works in Practice

### 1. Conductor Spawns Team

```bash
tt spawn backend
tt spawn frontend
tt spawn reviewer  # Always include!
```

### 2. Workers Work

```bash
tt assign backend "Build the API"
tt assign frontend "Build the UI"
```

### 3. Conductor Requests Review

When `tt status` shows workers are idle:

```bash
tt assign reviewer "Review the API implementation. Check security, error handling, tests. Approve or list what needs fixing."
```

### 4. Reviewer Responds

The reviewer either:
- **Approves**: "LGTM, API is solid"
- **Requests changes**: "Password hashing uses weak algorithm, fix needed"

### 5. Conductor Acts

- If approved → task is done
- If changes needed → `tt assign backend "Fix: use bcrypt instead of md5"`

## Messages Between Agents

Agents can send messages directly via their inboxes:

```rust
// In code (for custom integrations)
let msg = Message::new(worker_id, reviewer_id, MessageType::Custom {
    kind: "ready_for_review".into(),
    payload: r#"{"files": ["src/api.rs"]}"#.into(),
});
channel.send(&msg).await?;
```

But for simplicity, the **conductor handles coordination**. Agents don't need to message each other directly—the conductor assigns review tasks when workers are done.

## Keeping It Simple

Tinytown deliberately avoids:

- ❌ Complex state machines
- ❌ Automatic dependency resolution
- ❌ Event-driven triggers

Instead:

- ✅ Conductor checks `tt status`
- ✅ Conductor assigns next task
- ✅ Reviewer is the quality gate

This is explicit and easy to understand. You always know what's happening.

## Comparison with Gastown

| Aspect | Gastown | Tinytown |
|--------|---------|----------|
| Coordination | Mayor + Witness + Hooks | Conductor + Reviewer |
| Completion | Complex bead states | Reviewer approves |
| Automation | Event-driven | Conductor-driven |
| Complexity | High | Low |

Gastown automates more but is harder to understand. Tinytown is explicit.

