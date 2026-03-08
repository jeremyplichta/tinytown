# Core Concepts Overview

Tinytown has exactly **5 core types**. That's it. No more, no less.

```
┌─────────────────────────────────────────┐
│           Your Application              │
└──────────────┬──────────────────────────┘
               │
        ┌──────▼───────┐
        │    Town      │  ← Orchestrator
        └──────┬───────┘
               │
        ┌──────▼──────────────────┐
        │   Channel (Redis)       │  ← Message passing
        └──────┬──────────────────┘
               │
    ┌──────────┼──────────────┐
    ▼          ▼              ▼
┌────────┐ ┌────────┐    ┌────────┐
│ Agent  │ │ Agent  │ .. │ Agent  │  ← Workers
└───┬────┘ └───┬────┘    └───┬────┘
    │          │              │
    ▼          ▼              ▼
  Tasks      Tasks          Tasks      ← Work units
```

## The 5 Core Types

| Type | What It Is | Redis Key Pattern |
|------|------------|-------------------|
| **[Town](./towns.md)** | The orchestrator that manages everything | N/A (local) |
| **[Agent](./agents.md)** | A worker that executes tasks | `tt:agent:<id>` |
| **[Task](./tasks.md)** | A unit of work with lifecycle | `tt:task:<id>` |
| **[Message](./messages.md)** | Communication between agents | Transient |
| **[Channel](./channels.md)** | Redis-based message transport | `tt:inbox:<id>` |

## How They Work Together

### 1. Town Orchestrates

The **Town** is your control center. It:
- Starts and manages Redis
- Spawns and tracks agents
- Provides the API for coordination

```rust
let town = Town::init("./mytown", "my-project").await?;
```

### 2. Agents Execute

**Agents** are workers. Each agent has:
- A unique ID
- A name (human-readable)
- A model (claude, auggie, codex, etc.)
- A state (starting, idle, working, stopped)

```rust
let agent = town.spawn_agent("worker-1", "claude").await?;
```

### 3. Tasks Represent Work

**Tasks** are what agents work on. Each task has:
- A description
- A state (pending → assigned → running → completed/failed)
- An assigned agent

```rust
let task = Task::new("Implement the login API");
agent.assign(task).await?;
```

### 4. Messages Coordinate

**Messages** are how agents communicate. They carry:
- Sender and recipient
- Message type (TaskAssign, TaskDone, StatusRequest, etc.)
- Priority (Low, Normal, High, Urgent)

```rust
let msg = Message::new(from, to, MessageType::TaskAssign { task_id });
channel.send(&msg).await?;
```

### 5. Channel Transports

The **Channel** is the Redis connection that moves messages. It provides:
- Priority queues (urgent messages go first)
- Blocking receive (agents wait efficiently)
- State persistence (survives restarts)

## Mental Model

Think of it like a small company:

| Tinytown | Company Analogy |
|----------|-----------------|
| Town | The office building |
| Agent | An employee |
| Task | A work ticket |
| Message | An email/Slack message |
| Channel | The email/Slack system |

## What's NOT in Tinytown

Deliberately excluded to keep things simple:

- ❌ **Workflow DAGs** — Just assign tasks directly
- ❌ **Recovery daemons** — Redis persistence handles crashes
- ❌ **Multi-tier databases** — One Redis instance
- ❌ **Complex agent hierarchies** — All agents are peers

If you need these, you can build them on top of Tinytown's primitives, or use a more complex system like Gastown.

## Next Steps

Deep dive into each type:
- [Towns](./towns.md)
- [Agents](./agents.md)
- [Tasks](./tasks.md)
- [Messages](./messages.md)
- [Channels](./channels.md)

