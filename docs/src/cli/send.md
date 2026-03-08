# tt send

Send a message to an agent.

## Synopsis

```bash
tt send <TO> <MESSAGE> [OPTIONS]
```

## Description

Sends a custom message to an agent's inbox. The agent will receive it on their next inbox check.

**With `--urgent`**: Message goes to priority inbox, processed before regular messages!

Use this for:
- Agent-to-agent communication
- Conductor instructions
- Custom coordination
- **Urgent**: Interrupt agents with priority messages

## Arguments

| Argument | Description |
|----------|-------------|
| `<TO>` | Target agent name |
| `<MESSAGE>` | Message content |

## Options

| Option | Description |
|--------|-------------|
| `--urgent` | Send as urgent (processed first at start of next round) |

## Examples

### Send a Regular Message

```bash
tt send backend "The API spec is ready in docs/api.md"
```

Output:
```
📤 Sent message to 'backend': The API spec is ready in docs/api.md
```

### Send an URGENT Message

```bash
tt send backend --urgent "STOP! Security vulnerability found. Do not merge."
```

Output:
```
🚨 Sent URGENT message to 'backend': STOP! Security vulnerability found. Do not merge.
```

The agent will see this at the start of their next round, before processing regular inbox.

### Coordination Between Agents

```bash
# Developer finishes, notifies reviewer
tt send reviewer "Implementation complete. Please review src/auth.rs"

# Critical bug found - urgent interrupt
tt send developer --urgent "Critical: SQL injection in login. Fix immediately."
```

## How It Works

### Regular Messages
1. Goes to `tt:inbox:<id>` (Redis list)
2. Processed in order with other messages
3. Agent sees it when they check inbox

### Urgent Messages
1. Goes to `tt:urgent:<id>` (separate priority queue)
2. Agent checks urgent queue FIRST at start of each round
3. Urgent messages injected into agent's prompt with 🚨 marker
4. Processed before regular inbox

## See Also

- [tt inbox](./inbox.md) — Check agent's inbox
- [tt assign](./assign.md) — Assign tasks (more structured)
- [Coordination](../concepts/coordination.md)

