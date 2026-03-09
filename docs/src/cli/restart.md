# tt restart

Restart a stopped agent with fresh rounds.

## Synopsis

```bash
tt restart <AGENT> [OPTIONS]
```

## Description

Restarts an agent that is in a terminal state (Stopped or Error). Resets the agent's state to Idle, clears any stop flags, and spawns a new agent process with fresh rounds.

The agent must already exist and be stopped. To create a new agent, use `tt spawn`.

## Arguments

| Argument | Description |
|----------|-------------|
| `AGENT` | Name of the agent to restart |

## Options

| Option | Description |
|--------|-------------|
| `--rounds <N>` | Maximum rounds for restarted agent (default: 10) |
| `--foreground` | Run in foreground instead of backgrounding |
| `--town <PATH>` | Town directory (default: `.`) |
| `--verbose` | Enable verbose logging |

## Examples

### Basic Restart

```bash
tt restart worker-1
```

Output:
```
🔄 Restarting agent 'worker-1'...
   Rounds: 10
   Log: .tt/logs/worker-1.log

✅ Agent 'worker-1' restarted
```

### Restart with More Rounds

```bash
tt restart worker-1 --rounds 20
```

### Restart in Foreground

```bash
tt restart worker-1 --foreground
# Agent runs in terminal, you see all output
```

### Error: Agent Still Active

```bash
tt restart worker-1
```

Output:
```
❌ Agent 'worker-1' is still active (Working)
   Use 'tt kill worker-1' to stop it first
```

### Error: Agent Not Found

```bash
tt restart nonexistent
```

Output:
```
❌ Agent 'nonexistent' not found
```

## Restart vs Spawn

| Command | Use Case |
|---------|----------|
| `tt restart` | Revive existing stopped agent (keeps ID) |
| `tt spawn` | Create brand new agent (new ID) |

## Common Workflow

After agent exhausts its rounds:

```bash
# Check status
tt status
# Shows: worker-1 (Stopped) - completed 10/10 rounds

# Restart with more rounds
tt restart worker-1 --rounds 15
```

## See Also

- [tt spawn](./spawn.md) — Create new agents
- [tt kill](./kill.md) — Stop agents gracefully
- [tt recover](./recover.md) — Mark orphaned agents as stopped

