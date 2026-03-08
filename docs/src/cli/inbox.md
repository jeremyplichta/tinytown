# tt inbox

Check an agent's message inbox.

## Synopsis

```bash
tt inbox <AGENT>
```

## Description

Shows how many messages are waiting in an agent's inbox.

Messages are added by:
- `tt assign` — Creates a task and sends TaskAssign message
- `tt send` — Sends a custom message

## Arguments

| Argument | Description |
|----------|-------------|
| `<AGENT>` | Agent name to check |

## Examples

```bash
tt inbox backend
```

Output:
```
📬 Inbox for 'backend': 3 messages
```

## See Also

- [tt send](./send.md) — Send messages to agents
- [tt assign](./assign.md) — Assign tasks (sends TaskAssign message)
- [Messages Concept](../concepts/messages.md)

