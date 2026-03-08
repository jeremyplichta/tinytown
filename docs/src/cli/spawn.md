# tt spawn

Create a new agent.

## Synopsis

```bash
tt spawn <NAME> [OPTIONS]
```

## Description

Spawns a new worker agent in the town. The agent is registered in Redis and ready to receive tasks.

## Arguments

| Argument | Description |
|----------|-------------|
| `<NAME>` | Human-readable agent name (e.g., `worker-1`, `backend`, `reviewer`) |

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--model <MODEL>` | `-m` | AI model to use (default: `claude`) |
| `--town <PATH>` | `-t` | Town directory (default: `.`) |
| `--verbose` | `-v` | Enable verbose logging |

## Built-in Models

| Model | Command | Description |
|-------|---------|-------------|
| `claude` | `claude --print` | Anthropic Claude |
| `auggie` | `augment` | Augment Code |
| `codex` | `codex` | OpenAI Codex |
| `gemini` | `gemini` | Google Gemini |
| `copilot` | `gh copilot` | GitHub Copilot |
| `aider` | `aider` | Aider |
| `cursor` | `cursor` | Cursor |

## Examples

### Spawn with Default Model

```bash
tt spawn worker-1
```

### Spawn with Specific Model

```bash
tt spawn backend --model auggie
tt spawn reviewer --model codex
```

### Spawn Multiple Agents

```bash
tt spawn frontend --model claude
tt spawn backend --model auggie
tt spawn tester --model codex
tt spawn reviewer --model claude
```

## Output

```
🤖 Spawned agent 'backend' using model 'auggie'
   ID: 550e8400-e29b-41d4-a716-446655440000
```

## Agent Naming

Choose descriptive names:

| Good Names | Why |
|------------|-----|
| `backend` | Describes the work area |
| `worker-1` | Simple numbered workers |
| `reviewer` | Describes the role |
| `alice` | Personality names work too |

| Avoid | Why |
|-------|-----|
| `agent` | Too generic |
| `a` | Not descriptive |
| Spaces | Use hyphens instead |

## Agent State After Spawn

New agents start in `Starting` state, then transition to `Idle`:

```
Starting → Idle (ready for work)
```

Check state with:
```bash
tt list
```

## Errors

### Town Not Initialized

```
Error: Town not initialized at . Run 'tt init' first.
```

**Solution:** Run `tt init` or specify `--town` path.

### Agent Already Exists

Agents are tracked by name. Spawning the same name creates a new agent with a new ID.

## See Also

- [tt init](./init.md) — Initialize a town
- [tt assign](./assign.md) — Assign tasks to agents
- [tt list](./list.md) — List all agents
- [Agents Concept](../concepts/agents.md)

