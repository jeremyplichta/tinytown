# tt task

Manage individual tasks.

## Synopsis

```bash
tt task <SUBCOMMAND> [OPTIONS]
```

## Description

Provides operations for managing specific tasks by ID: resolving the current tracked assignment, completing work, viewing details, or listing tasks.

## Subcommands

### Complete

Mark a task as completed:

```bash
tt task complete <TASK_ID> [--result <MESSAGE>]
```

### Current

Show the currently tracked task for an agent:

```bash
tt task current [AGENT]
```

If you run this inside a Tinytown agent loop, `AGENT` is optional and Tinytown resolves the current worker automatically. This is the safest way for a worker to confirm the real Tinytown task ID before running `tt task complete ...`, especially when the task description itself contains other UUID-like values such as mission IDs.

### Show

View details of a specific task:

```bash
tt task show <TASK_ID>
```

### List

List all tasks with optional filtering:

```bash
tt task list [--state <STATE>]
```

States: `pending`, `assigned`, `running`, `completed`, `failed`, `cancelled`

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--town <PATH>` | `-t` | Town directory (default: `.`) |
| `--verbose` | `-v` | Enable verbose logging |

## Examples

### Mark a Task Complete

```bash
tt task complete 550e8400-e29b-41d4-a716-446655440000 --result "Fixed the bug"
```

Output:
```
✅ Task 550e8400-e29b-41d4-a716-446655440000 marked as completed
   Description: Fix authentication bug
   Result: Fixed the bug
```

### Show the Current Tracked Task

```bash
tt task current frontend
```

Output:
```
📋 Current task for 'frontend': 550e8400-e29b-41d4-a716-446655440000
   Description: Mission c7d2e4dd-30e5-48e5-8bfc-95d5f14b13bf: implement issue #5
   State: Assigned
   Complete with: tt task complete 550e8400-e29b-41d4-a716-446655440000 --result "what was done"
```

### View Task Details

```bash
tt task show 550e8400-e29b-41d4-a716-446655440000
```

Output:
```
📋 Task: 550e8400-e29b-41d4-a716-446655440000
   Description: Fix authentication bug
   State: Running
   Assigned to: backend-1
   Created: 2025-03-09T12:00:00Z
   Updated: 2025-03-09T12:05:00Z
   Started: 2025-03-09T12:01:00Z
   Tags: backend, auth
```

### List Running Tasks

```bash
tt task list --state running
```

Output:
```
📋 Tasks (2):
   🔄 550e8400-... - Fix authentication bug [backend-1]
   🔄 660e9500-... - Update API endpoints [backend-2]
```

### List All Tasks

```bash
tt task list
```

State icons:
- ⏳ Pending
- 📌 Assigned
- 🔄 Running
- ✅ Completed
- ❌ Failed
- 🚫 Cancelled

## See Also

- [tt tasks](./tasks.md) — Overview of pending tasks
- [tt assign](./assign.md) — Assign new tasks
- [tt backlog](./backlog.md) — Manage unassigned tasks
