# tt list

List all agents in the town.

## Synopsis

```bash
tt list [OPTIONS]
```

## Description

Shows all agents registered in the town with their current state.

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--town <PATH>` | `-t` | Town directory (default: `.`) |
| `--verbose` | `-v` | Enable verbose logging |

## Examples

### List All Agents

```bash
tt list
```

## Output

```
Agents:
  backend (550e8400-e29b-41d4-a716-446655440000) - Working
  frontend (6ba7b810-9dad-11d1-80b4-00c04fd430c8) - Idle
  reviewer (6ba7b811-9dad-11d1-80b4-00c04fd430c9) - Idle
```

### No Agents

```
No agents. Run 'tt spawn <name>' to create one.
```

## Agent States

| State | Meaning |
|-------|---------|
| `Starting` | Agent is initializing |
| `Idle` | Ready for work |
| `Working` | Executing a task |
| `Paused` | Temporarily stopped |
| `Stopped` | Terminated |
| `Error` | Something went wrong |

## See Also

- [tt status](./status.md) — Detailed status including pending messages
- [tt spawn](./spawn.md) — Create new agents
- [Agents Concept](../concepts/agents.md)

