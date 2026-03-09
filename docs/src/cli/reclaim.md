# tt reclaim

Recover orphaned tasks from dead agents.

## Synopsis

```bash
tt reclaim [OPTIONS]
```

## Description

Finds tasks assigned to agents in terminal states (Stopped/Error) and moves them elsewhere. This prevents work from being lost when agents crash.

Without destination flags, lists orphaned tasks. With flags, moves them.

## Options

| Option | Description |
|--------|-------------|
| `--to-backlog` | Move orphaned tasks to the global backlog |
| `--to <AGENT>` | Move orphaned tasks to a specific agent |
| `--from <AGENT>` | Reclaim only from a specific dead agent |
| `--town <PATH>` | Town directory (default: `.`) |
| `--verbose` | Enable verbose logging |

## Examples

### Preview Orphaned Tasks

```bash
tt reclaim
```

Output:
```
🔄 Reclaiming orphaned tasks...
   worker-1 (Stopped): 3 message(s)
      task: 550e8400-e29b-41d4-a716-446655440000
      task: Fix authentication bug in login endpoint
      task: Update database schema

📋 Found 3 orphaned task(s)
   Use --to-backlog or --to <agent> to reclaim them
```

### Move Tasks to Backlog

```bash
tt reclaim --to-backlog
```

Output:
```
🔄 Reclaiming orphaned tasks...
   worker-1 (Stopped): 3 message(s)
      → backlog: 550e8400-e29b-41d4-a716-446655440000
      → backlog: 660e9500-e29b-41d4-a716-446655440111
      → backlog: 770f0600-e29b-41d4-a716-446655440222

✅ Moved 3 task(s) to backlog
```

### Reassign to Another Agent

```bash
tt reclaim --to worker-2
```

Output:
```
🔄 Reclaiming orphaned tasks...
   worker-1 (Stopped): 3 message(s)
      → worker-2: 550e8400-e29b-41d4-a716-446655440000
      → worker-2: Fix authentication bug in login endpoint
      → worker-2: Update database schema

✅ Moved 3 task(s) to 'worker-2'
```

### Reclaim from Specific Agent

```bash
tt reclaim --from worker-1 --to-backlog
```

## Common Workflow

After a crash:

```bash
# 1. Recover orphaned agents first
tt recover

# 2. Reclaim their tasks to backlog
tt reclaim --to-backlog

# 3. Clean up stopped agents
tt prune

# 4. Restart or spawn new agents
tt restart worker-1
# or
tt spawn worker-3

# 5. Let agents claim from backlog
```

## See Also

- [tt recover](./recover.md) — Mark orphaned agents as stopped
- [tt prune](./prune.md) — Remove stopped agents
- [tt backlog](./backlog.md) — Manage the backlog
- [Error Handling & Recovery](../tutorials/recovery.md) — Recovery tutorial

