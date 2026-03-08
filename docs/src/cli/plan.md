# tt plan

Plan tasks in a file before starting work.

## Synopsis

```bash
tt plan [OPTIONS]
tt plan --init
```

## Description

The `plan` command lets you define tasks in `tasks.toml` before syncing them to Redis. This enables:

1. **Version control** — Check in your task plan with your code
2. **Offline planning** — Edit tasks without Redis running
3. **Review before execution** — Plan the work, then start the train

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--init` | `-i` | Create a new tasks.toml file |
| `--town <PATH>` | `-t` | Town directory (default: `.`) |

## Examples

### Initialize a Task Plan

```bash
tt plan --init
```

Creates `tasks.toml`:

```toml
[meta]
description = "Task plan for this project"

[[tasks]]
id = "example-1"
description = "Example task - replace with your own"
status = "pending"
tags = ["example"]
```

### View Current Plan

```bash
tt plan
```

Output:
```
📋 Tasks in plan (./tasks.toml):
  ⏳ [unassigned] example-1 - Example task - replace with your own
```

### Edit and Sync

```bash
# Edit tasks.toml with your editor
vim tasks.toml

# Push to Redis
tt sync push
```

## Task File Format

```toml
[meta]
description = "Sprint 1 tasks"
default_agent = "developer"  # Optional default

[[tasks]]
id = "auth-api"
description = "Build user authentication API"
agent = "backend"
status = "pending"
tags = ["auth", "api"]

[[tasks]]
id = "auth-tests"
description = "Write auth API tests"
agent = "tester"
status = "pending"
parent = "auth-api"  # Optional parent task

[[tasks]]
id = "auth-review"
description = "Review auth implementation"
status = "pending"
# No agent = unassigned
```

## Task Status Values

| Status | Icon | Meaning |
|--------|------|---------|
| `pending` | ⏳ | Not started |
| `assigned` | 📌 | Given to an agent |
| `running` | 🔄 | In progress |
| `completed` | ✅ | Done |
| `failed` | ❌ | Error |

## Workflow

1. **Plan**: `tt plan --init` → edit `tasks.toml`
2. **Review**: `tt plan` to see the plan
3. **Start**: `tt sync push` to send to Redis
4. **Execute**: Agents receive tasks
5. **Snapshot**: `tt sync pull` to save state

## Why Plan in a File?

- **Git history** — Track how the plan evolved
- **Code review** — Review task definitions in PRs
- **Templates** — Reuse task structures across projects
- **Offline** — Plan without starting Redis

Redis remains the source of truth at runtime; the file is for planning and version control.

## See Also

- [tt sync](./sync.md) — Sync tasks.toml ↔ Redis
- [tt conductor](./conductor.md) — Interactive mode
- [Tasks Concept](../concepts/tasks.md)

