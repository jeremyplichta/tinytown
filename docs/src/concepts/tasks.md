# Tasks

A **Task** is a unit of work that can be assigned to an agent.

## Task Properties

| Property | Type | Description |
|----------|------|-------------|
| `id` | UUID | Unique identifier |
| `description` | String | What needs to be done |
| `state` | Enum | Current lifecycle state |
| `assigned_to` | Option | Agent working on this |
| `created_at` | DateTime | When created |
| `updated_at` | DateTime | Last modification |
| `completed_at` | Option | When finished |
| `result` | Option | Output or error message |
| `parent_id` | Option | For hierarchical tasks |
| `tags` | Vec | Labels for filtering |

## Task States

```
┌─────────┐
│ Pending │ ── Created, waiting for assignment
└────┬────┘
     │ assign()
     ▼
┌──────────┐
│ Assigned │ ── Given to an agent
└────┬─────┘
     │ start()
     ▼
┌─────────┐
│ Running │ ── Agent is working on it
└────┬────┘
     │
     ├─── complete() ──► ┌───────────┐
     │                   │ Completed │ ✓
     │                   └───────────┘
     │
     ├─── fail() ──────► ┌────────┐
     │                   │ Failed │ ✗
     │                   └────────┘
     │
     └─── cancel() ────► ┌───────────┐
                         │ Cancelled │ ⊘
                         └───────────┘
```

## Creating Tasks

### CLI

```bash
# Assign a task directly to an agent
tt assign worker-1 "Implement user authentication"

# Check pending tasks
tt tasks
```

### Using tasks.toml (Recommended)

Define tasks in a file for version control and batch assignment:

```toml
[[tasks]]
id = "auth-api"
description = "Implement user authentication"
agent = "backend"
status = "pending"
tags = ["auth", "api"]

[[tasks]]
id = "auth-tests"
description = "Write tests for auth API"
agent = "tester"
status = "pending"
parent = "auth-api"
```

Then sync to Redis:
```bash
tt sync push
```

See [tt plan](../cli/plan.md) for the full task DSL.

## Task Lifecycle

Tasks move through states automatically as agents work on them:

1. **Pending** → Created, waiting for assignment
2. **Assigned** → Given to an agent via `tt assign`
3. **Running** → Agent is actively working
4. **Completed/Failed/Cancelled** → Terminal states

Check task state with:
```bash
tt tasks
```

Or inspect directly in Redis:
```bash
redis-cli -s ./redis.sock GET "tt:<town_name>:task:<uuid>"
```

## Task Storage in Redis

Tasks are stored as JSON using town-isolated keys:

```
tt:<town_name>:task:<uuid>  →  JSON serialized Task struct
```

This allows multiple towns to share the same Redis instance. You can inspect tasks directly:
```bash
redis-cli -s ./redis.sock GET "tt:<town_name>:task:550e8400-e29b-41d4-a716-446655440000"
```

See [tt migrate](../cli/migrate.md) for upgrading from older key formats.

## Hierarchical Tasks

Create parent-child relationships in `tasks.toml`:

```toml
[[tasks]]
id = "user-system"
description = "User Management System"
agent = "architect"
status = "pending"

[[tasks]]
id = "signup"
description = "User signup flow"
parent = "user-system"
agent = "backend"
status = "pending"

[[tasks]]
id = "login"
description = "User login flow"
parent = "user-system"
agent = "backend"
status = "pending"
```

## Task Tags

Use tags to categorize and filter:

```toml
[[tasks]]
id = "fix-xss"
description = "Fix XSS vulnerability"
status = "pending"
tags = ["security", "bug", "P0"]
```

## Comparison with Gastown Beads

| Feature | Tinytown Tasks | Gastown Beads |
|---------|----------------|---------------|
| Storage | Redis (JSON) | Dolt SQL |
| Hierarchy | Optional parent_id | Full graph |
| Metadata | Tags array | Full schema |
| Persistence | Redis persistence | Git-backed |
| Complexity | Simple | Complex |

Tinytown tasks are intentionally simpler. If you need Gastown's bead features (dependency graphs, git versioning, SQL queries), consider using Gastown or building on top of Tinytown.

