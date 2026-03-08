# tt send

Send a message to an agent.

## Synopsis

```bash
tt send <TO> <MESSAGE>
```

## Description

Sends a custom message to an agent's inbox. The agent will receive it on their next inbox check.

Use this for:
- Agent-to-agent communication
- Conductor instructions
- Custom coordination

## Arguments

| Argument | Description |
|----------|-------------|
| `<TO>` | Target agent name |
| `<MESSAGE>` | Message content |

## Examples

### Send a Message

```bash
tt send backend "The API spec is ready in docs/api.md"
```

Output:
```
📤 Sent message to 'backend': The API spec is ready in docs/api.md
```

### Coordination Between Agents

```bash
# Developer finishes, notifies reviewer
tt send reviewer "Implementation complete. Please review src/auth.rs"

# Reviewer responds
tt send developer "LGTM! One small fix: add error handling in line 42"
```

## How It Works

1. Creates a `Message` with type `Custom`
2. Sender is the supervisor (conductor)
3. Message goes to agent's inbox in Redis (`mt:inbox:<id>`)
4. Agent receives it on next loop iteration

## See Also

- [tt inbox](./inbox.md) — Check agent's inbox
- [tt assign](./assign.md) — Assign tasks (more structured)
- [Coordination](../concepts/coordination.md)

