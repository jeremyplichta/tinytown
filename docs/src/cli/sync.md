# tt sync

Synchronize tasks between tasks.toml and Redis.

## Synopsis

```bash
tt sync [push|pull]
```

## Description

The `sync` command moves task data between the `tasks.toml` file and Redis:

- **push**: File → Redis (deploy your plan)
- **pull**: Redis → File (snapshot current state)

## Arguments

| Argument | Description |
|----------|-------------|
| `push` | Send tasks from tasks.toml to Redis (default) |
| `pull` | Save Redis tasks to tasks.toml |

## Examples

### Push Plan to Redis

After editing `tasks.toml`:

```bash
tt sync push
```

Output:
```
⬆️  Pushed 5 tasks from tasks.toml to Redis
```

### Pull State from Redis

Save current Redis state to file:

```bash
tt sync pull
```

Output:
```
⬇️  Pulled 5 tasks from Redis to tasks.toml
```

## Workflow

```
┌─────────────────┐     push      ┌─────────────────┐
│   tasks.toml    │ ───────────►  │     Redis       │
│   (planning)    │               │   (execution)   │
│                 │ ◄───────────  │                 │
└─────────────────┘     pull      └─────────────────┘
        │                                 │
        │                                 │
        ▼                                 ▼
   Git tracked                     Fast queries
   Human readable                  Agent access
   Offline edits                   Real-time state
```

## When to Use

### Push (file → Redis)

- After editing `tasks.toml`
- At the start of a work session
- To reset task state

### Pull (Redis → file)

- Before committing to git
- To snapshot progress
- To share state with team

## Data Flow

### Push Behavior

1. Reads all tasks from `tasks.toml`
2. Creates corresponding Task objects
3. Stores each in Redis at `tt:task:<id>`
4. Tags include `plan:<id>` for tracking

### Pull Behavior

1. (Currently) Initializes empty tasks.toml if missing
2. (Future) Scans Redis for `tt:task:*` keys
3. Converts to TaskEntry format
4. Writes to tasks.toml

## See Also

- [tt plan](./plan.md) — Create and view task plans
- [Tasks Concept](../concepts/tasks.md)
- [Redis Configuration](../advanced/redis.md)

