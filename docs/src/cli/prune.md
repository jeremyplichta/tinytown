# tt prune

Remove stopped or stale agents from Redis.

## Synopsis

```bash
tt prune [OPTIONS]
```

## Description

Removes agents that are in a terminal state (Stopped or Error) from Redis. This cleans up agent records after they've finished or failed. Useful for managing long-running towns where many agents have come and gone.

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--all` | | Remove ALL agents, not just stopped ones |
| `--town <PATH>` | `-t` | Town directory (default: `.`) |
| `--verbose` | `-v` | Enable verbose logging |

## Examples

### Prune Only Stopped Agents

```bash
tt prune
```

Output:
```
🗑️  Removed worker-1 (abc123...) - Stopped
🗑️  Removed worker-2 (def456...) - Error
✨ Pruned 2 agent(s)
```

### Remove All Agents

```bash
tt prune --all
```

⚠️ **Warning:** This removes ALL agents including active ones.

## Common Workflow

After recovering orphaned agents, prune them:

```bash
# First, recover any orphaned agents (marks them as stopped)
tt recover

# Then remove stopped agents from Redis
tt prune
```

## See Also

- [tt recover](./recover.md) — Detect and clean up orphaned agents
- [tt reset](./reset.md) — Full town reset
- [tt kill](./kill.md) — Stop a specific agent

