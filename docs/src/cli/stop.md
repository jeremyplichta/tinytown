# tt stop

Stop the town.

## Synopsis

```bash
tt stop [OPTIONS]
```

## Description

Signals that the town should stop. Currently this is a placeholder command that logs a shutdown message. Redis cleanup happens when all connections are closed.

Note: To fully reset a town, use `tt reset` instead.

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--town <PATH>` | `-t` | Town directory (default: `.`) |
| `--verbose` | `-v` | Enable verbose logging |

## Examples

### Stop the Town

```bash
tt stop
```

Output:
```
👋 Town stopped (Redis will be cleaned up)
```

## Related Operations

| Task | Command |
|------|---------|
| Stop all agents | `tt kill --all` |
| Reset all state | `tt reset` |
| Kill specific agent | `tt kill <agent>` |

## Redis Lifecycle

Redis in Tinytown is managed per-connection:
- Starts when `tt init` runs or first connection is made
- Stays alive while any tt command is connected
- Socket-based Redis uses the `.tt/redis.sock` file

For persistent deployments, consider running Redis independently.

## See Also

- [tt start](./start.md) — Keep town alive
- [tt reset](./reset.md) — Full state reset
- [tt kill](./kill.md) — Stop specific agents

