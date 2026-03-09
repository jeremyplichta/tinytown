# tt reset

Reset all town state, clearing agents, tasks, and messages from Redis.

## Synopsis

```bash
tt reset [OPTIONS]
```

## Description

Performs a complete reset of the town's Redis state. This is useful when you want to start fresh without reinitializing the entire town, or when cleaning up after a failed run.

**⚠️ Warning:** This operation cannot be undone. All agents, tasks, and messages will be permanently deleted.

## Options

| Option | Description |
|--------|-------------|
| `--force` | Skip the confirmation prompt and proceed immediately |
| `--agents-only` | Only reset agent-related state (agents and inboxes), preserving tasks and backlog |
| `--town <PATH>` | Town directory (default: `.`) |
| `--verbose` | Enable verbose logging |

## Examples

### Full Reset (with confirmation)

```bash
tt reset
```

Output:
```
🗑️  Resetting town 'my-project'
   This will delete:
   - 3 agent(s)
   - 12 task(s)
   - 2 backlog item(s)

⚠️  This action cannot be undone!
   Run with --force to confirm: tt reset --force
```

### Full Reset (immediate)

```bash
tt reset --force
```

Output:
```
🗑️  Resetting town 'my-project'
   This will delete:
   - 3 agent(s)
   - 12 task(s)
   - 2 backlog item(s)

✅ Reset complete: deleted 47 Redis keys
   Run 'tt spawn <name>' to create new agents
```

### Agents-Only Reset

Reset just the agents while preserving tasks and backlog:

```bash
tt reset --agents-only --force
```

Output:
```
🗑️  Resetting agents in town 'my-project'
   This will delete:
   - 3 agent(s) and their inboxes
   Tasks and backlog will be preserved.

✅ Reset complete: deleted 12 Redis keys (agents only)
   Run 'tt spawn <name>' to create new agents
```

## Use Cases

| Scenario | Command |
|----------|---------|
| Start completely fresh | `tt reset --force` |
| Replace agents but keep tasks | `tt reset --agents-only --force` |
| Preview what will be deleted | `tt reset` (no --force) |

## What Gets Deleted

### Full Reset (`tt reset --force`)
- All registered agents (`tt:<town>:agent:*`)
- All agent inboxes (`tt:<town>:inbox:*`)
- All tasks (`tt:<town>:task:*`)
- All backlog items (`tt:<town>:backlog`)
- Agent activity logs

### Agents-Only Reset (`tt reset --agents-only --force`)
- All registered agents (`tt:<town>:agent:*`)
- All agent inboxes (`tt:<town>:inbox:*`)
- Agent activity logs

Tasks and backlog are preserved.

## Recovery

If you accidentally reset:
- If you previously ran `tt save`, you may be able to `tt restore` from the AOF file
- If tasks were synced to `tasks.toml`, you can `tt sync push` to recreate them

## See Also

- [tt init](./init.md) — Initialize a new town
- [tt spawn](./spawn.md) — Create agents after reset
- [tt save](./save.md) — Save state before reset
- [tt restore](./restore.md) — Restore from saved state

