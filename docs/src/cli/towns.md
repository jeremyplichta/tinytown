# tt towns

List all registered towns.

## Synopsis

```bash
tt towns [OPTIONS]
```

## Description

Displays all towns registered in `~/.tt/towns.toml`. Towns are automatically registered when initialized with `tt init`. For each town, shows:
- Town name
- Connection status
- Agent count (if online)
- Path on disk

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--verbose` | `-v` | Enable verbose logging |

## Examples

### List Registered Towns

```bash
tt towns
```

Output:
```
🏘️  Registered Towns (3):

   my-project - [OK] 3 agents (2 active)
      📂 /Users/me/git/my-project

   feature-branch - [OK] 1 agents (1 active)
      📂 /Users/me/git/my-project

   old-project - [OFFLINE]
      📂 /Users/me/git/old-project
```

### Status Indicators

| Status | Meaning |
|--------|---------|
| `[OK] N agents (M active)` | Town online with Redis connection |
| `[OFFLINE]` | Town exists but Redis not running |
| `⚠️ (no config)` | Path exists but no `tinytown.toml` |
| `❌ (path not found)` | Directory no longer exists |

### No Towns Registered

```bash
tt towns
```

Output:
```
📍 No towns registered yet.
   Run 'tt init' in a directory to register a town.
```

## Registration

Towns are automatically registered when created:

```bash
cd ~/git/my-new-project
tt init --name my-new-project
# Town is now registered in ~/.tt/towns.toml
```

## Registry File

Towns are tracked in `~/.tt/towns.toml`:

```toml
[[towns]]
name = "my-project"
path = "/Users/me/git/my-project"

[[towns]]
name = "feature-branch"
path = "/Users/me/git/my-project"
```

## See Also

- [tt init](./init.md) — Initialize and register a new town
- [tt status](./status.md) — Detailed status of current town

