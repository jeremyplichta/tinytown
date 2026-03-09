# MCP Interface

Tinytown exposes a full [Model Context Protocol (MCP)](https://modelcontextprotocol.io/) interface, allowing AI tools like Claude Desktop, Cursor, and other MCP clients to directly orchestrate agent towns.

## Quick Start

### Claude Desktop Integration

Add to `~/.config/claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "tinytown": {
      "command": "townhall",
      "args": ["mcp-stdio", "--town", "/path/to/your/project"]
    }
  }
}
```

Restart Claude Desktop. You can now ask Claude to manage your agent town!

### HTTP/SSE Mode

For browser-based MCP clients:

```bash
townhall mcp-http --port 8788
# MCP endpoint: http://localhost:8788
```

## Available Tools

MCP tools provide programmatic access to all Tinytown operations:

### Read-Only Tools (town.read scope)

| Tool | Description |
|------|-------------|
| `town.get_status` | Get town status including all agents |
| `agent.list` | List all agents with current status |
| `backlog.list` | List all tasks in the backlog |

### Write Tools (town.write scope)

| Tool | Description |
|------|-------------|
| `task.assign` | Assign a task to an agent |
| `task.complete` | Mark a task as completed |
| `message.send` | Send a message to an agent |
| `backlog.add` | Add a task to the backlog |
| `backlog.claim` | Claim a backlog task for an agent |
| `backlog.assign_all` | Assign all backlog tasks to an agent |

### Agent Management Tools (agent.manage scope)

| Tool | Description |
|------|-------------|
| `agent.spawn` | Spawn a new agent |
| `agent.kill` | Kill (stop) an agent gracefully |
| `agent.restart` | Restart a stopped agent |
| `recovery.recover_agents` | Recover orphaned agents |
| `recovery.reclaim_tasks` | Reclaim tasks from dead agents |

## Resources

MCP resources provide read-only data access:

| Resource URI | Description |
|--------------|-------------|
| `tinytown://town/current` | Current town state |
| `tinytown://agents` | List of all agents |
| `tinytown://agents/{name}` | Specific agent details |
| `tinytown://backlog` | Current backlog |
| `tinytown://tasks/{id}` | Specific task details |

## Prompts

MCP prompts provide templated interactions:

| Prompt | Description |
|--------|-------------|
| `conductor.startup_context` | Context for conductor startup |
| `agent.role_hint` | Role hints for agents |

## Example Conversation

With MCP configured, you can have natural conversations with Claude:

**You:** "Spawn two backend agents and assign them bug fixes"

**Claude:** I'll create two backend agents and assign tasks to them.
*[Uses agent.spawn tool twice, then task.assign tool]*
Done! I've spawned `backend-1` and `backend-2` and assigned bug fix tasks to each.

**You:** "What's the status of the town?"

**Claude:** *[Uses town.get_status tool]*
Your town has 2 agents running:
- `backend-1`: Working, 3 tasks completed
- `backend-2`: Working, 2 tasks completed

## Tool Response Format

All tools return JSON with consistent structure:

```json
{
  "success": true,
  "data": {
    "agent_id": "abc123...",
    "name": "worker-1",
    "cli": "claude"
  }
}
```

On error:

```json
{
  "success": false,
  "error": "Agent 'worker-99' not found"
}
```

## Transports

| Transport | Command | Best For |
|-----------|---------|----------|
| stdio | `townhall mcp-stdio` | Claude Desktop, IDE extensions |
| HTTP/SSE | `townhall mcp-http` | Browser clients, remote access |

### stdio Transport

Used by most desktop applications. The MCP server reads JSON-RPC from stdin and writes to stdout.

### HTTP/SSE Transport

Uses Server-Sent Events for server-to-client messages and HTTP POST for client-to-server messages.

```bash
# Start on custom port
townhall mcp-http --bind 0.0.0.0 --port 8788
```

