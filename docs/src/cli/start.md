# tt start

Start the town (keep Redis alive).

## Synopsis

```bash
tt start [OPTIONS]
```

## Description

Connects to an existing town and keeps the process running until Ctrl+C. This is useful for:

1. Keeping the Redis server alive when no agents are running
2. Maintaining a persistent connection for debugging
3. Ensuring the town stays online during development

Note: Towns automatically start Redis when you run `tt init` or any command that connects. This command is mainly for explicitly keeping the connection open.

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--town <PATH>` | `-t` | Town directory (default: `.`) |
| `--verbose` | `-v` | Enable verbose logging |

## Examples

### Keep Town Running

```bash
tt start
```

Output:
```
🚀 Town started
^C
👋 Shutting down...
```

### With Specific Town

```bash
tt start --town ~/git/my-project
```

## When to Use

Most operations don't require `tt start` because:
- `tt init` starts Redis automatically
- `tt spawn` connects and stays alive
- `tt status` connects temporarily

Use `tt start` when you want to:
- Keep Redis running without spawning agents
- Debug connection issues
- Manually control the town lifecycle

## See Also

- [tt stop](./stop.md) — Stop the town
- [tt init](./init.md) — Initialize a new town
- [tt status](./status.md) — Check town status

