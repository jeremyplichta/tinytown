# Concept Mapping: Gastown → Tinytown

A detailed translation guide for Gastown users.

## Agent Taxonomy

### Gastown's Agent Zoo

Gastown has **8 agent types** across two levels:

**Town-Level:**
| Agent | Role | Tinytown Equivalent |
|-------|------|---------------------|
| Mayor | Global coordinator | Your orchestration code |
| Deacon | Daemon, health monitoring | Your process + monitoring |
| Boot | Deacon watchdog | External health check |
| Dogs | Infrastructure helpers | Background tasks |

**Rig-Level:**
| Agent | Role | Tinytown Equivalent |
|-------|------|---------------------|
| Witness | Monitors polecats | Status polling loop |
| Refinery | Merge queue processor | CI/CD integration |
| Polecats | Workers | Agents |
| Crew | Human workspaces | N/A (you're the human) |

### Tinytown's Simplicity

Tinytown has **2 agent types**:

| Agent | Role |
|-------|------|
| Supervisor | Well-known ID for coordination |
| Worker | Does the actual work |

Everything else? You write it explicitly.

## Work Tracking

### Gastown Beads

Beads are git-backed structured records:

```
ID: gt-abc12
Type: task
Title: Implement login API
Status: in_progress
Priority: P1
Created: 2024-03-01
Assigned: gastown/polecats/Toast
Parent: gt-xyz99 (epic)
Dependencies: [gt-def34, gt-ghi56]
```

Features:
- Stored in Dolt SQL
- Version controlled
- Two-level (Town + Rig)
- Rich schema with dependencies
- Prefix-based namespacing

### Tinytown Tasks

Tasks are simpler JSON objects:

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "description": "Implement login API",
  "state": "running",
  "assigned_to": "6ba7b810-...",
  "created_at": "2024-03-01T10:00:00Z",
  "parent_id": null,
  "tags": ["auth", "api"]
}
```

Features:
- Stored in Redis
- Simple JSON
- Single level
- Minimal schema
- Tags for organization

### Translation

| Beads Feature | Tinytown Approach |
|---------------|-------------------|
| Priority (P0-P4) | Use tags: `["P1"]` |
| Type (task/bug/feature) | Use tags: `["bug"]` |
| Dependencies | Manual coordination |
| Parent/child | `parent_id` field |
| Status history | Not built-in (log it yourself) |

## Coordination Mechanisms

### Gastown: Convoys

Convoys track batches of related work:

```bash
gt convoy create "User Auth Feature" gt-abc12 gt-def34 gt-ghi56
gt convoy status hq-cv-xyz
```

Features:
- Auto-created by Mayor
- Tracks multiple beads
- Lifecycle: OPEN → LANDED → CLOSED
- Event-driven completion detection

### Tinytown: Manual Grouping

Use parent tasks or your own tracking:

```rust
// Option 1: Parent tasks
let feature = Task::new("User Auth Feature");
let login = Task::new("Login flow").with_parent(feature.id);
let signup = Task::new("Signup flow").with_parent(feature.id);

// Option 2: Tags
let tasks = vec![
    Task::new("Login").with_tags(["auth-feature"]),
    Task::new("Signup").with_tags(["auth-feature"]),
];

// Option 3: Your own tracking
struct Convoy {
    name: String,
    tasks: Vec<TaskId>,
}
```

### Gastown: Hooks

Hooks are the assignment mechanism:

```
Polecat has hook → Hook has pinned bead → Polecat MUST work on it
```

The "GUPP Principle": If work is on your hook, you run it immediately.

### Tinytown: Inboxes

Messages go to agent inboxes (Redis lists):

```
Agent has inbox → Messages queued → Agent polls/blocks for messages
```

You control when and how agents process work.

## Communication

### Gastown: Mail Protocol

Messages are beads of type `message`:

```bash
# Check mail
gt mail check

# Types: POLECAT_DONE, MERGE_READY, REWORK_REQUEST, etc.
```

Complex routing through beads system.

### Tinytown: Direct Messages

Messages are transient, stored in Redis:

```rust
let msg = Message::new(from, to, MessageType::TaskDone {
    task_id: "abc".into(),
    result: "Done!".into(),
});
channel.send(&msg).await?;
```

Direct, simple, explicit.

## State Persistence

### Gastown: Multi-Layer

1. **Git worktrees** - Sandbox persistence
2. **Beads ledger** - Work state (Dolt SQL)
3. **Hooks** - Work assignment
4. **State files** - Runtime state (JSON)

### Tinytown: Redis

Everything in Redis:
- `tt:agent:<id>` - Agent state
- `tt:task:<id>` - Task state
- `tt:inbox:<id>` - Message queues

Enable Redis persistence (RDB/AOF) for durability.

## Recovery

### Gastown: Automatic

- Witness patrols detect stalled polecats
- Deacon monitors system health
- Boot watches Deacon
- Hooks ensure work resumes on restart

### Tinytown: Manual

You implement recovery:

```rust
// Check agent health
if agent.state == AgentState::Error {
    // Respawn
    town.spawn_agent(&agent.name, &agent.model).await?;
}

// Retry failed tasks
if task.state == TaskState::Failed {
    new_agent.assign(task.clone()).await?;
}
```

## When Tinytown Falls Short

Gastown features you might miss:

| Feature | Why It's Useful | Tinytown Workaround |
|---------|-----------------|---------------------|
| Automatic recovery | Hands-off operation | Write recovery loops |
| Git-backed history | Audit trail | Log to files |
| Dependency graphs | Complex workflows | Manual ordering |
| Cross-rig work | Multi-repo coordination | Run multiple towns |
| Dashboard | Visual monitoring | CLI + custom tooling |

If you find yourself building these features, consider whether Gastown's complexity is justified for your use case.

