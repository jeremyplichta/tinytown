# Redis Configuration

Tinytown uses Redis for message passing and state storage. Here's how to configure and optimize it.

## Default Setup

By default, Tinytown:
1. Starts a local Redis server
2. Uses a Unix socket at `./redis.sock`
3. Disables TCP (port 0)
4. Runs in-memory only

## Unix Socket vs TCP

### Unix Socket (Default)

```json
{
  "redis": {
    "use_socket": true,
    "socket_path": "redis.sock"
  }
}
```

**Pros:**
- ~10x faster latency (~0.1ms vs ~1ms)
- No network overhead
- No port conflicts

**Cons:**
- Local only (same machine)
- File permissions matter

### TCP Connection

```json
{
  "redis": {
    "use_socket": false,
    "host": "127.0.0.1",
    "port": 6379
  }
}
```

**Use for:**
- Remote Redis servers
- Docker containers
- Networked deployments

## Connecting to External Redis

Use an existing Redis server instead of starting one:

```json
{
  "redis": {
    "use_socket": false,
    "host": "redis.example.com",
    "port": 6379
  }
}
```

Tinytown will connect without starting a new server.

## Persistence

By default, Redis runs in-memory. Data is lost on restart.

### Enable RDB Snapshots

```bash
redis-cli -s ./redis.sock CONFIG SET save "60 1"
```

Saves every 60 seconds if at least 1 key changed.

### Enable AOF (Append Only File)

```bash
redis-cli -s ./redis.sock CONFIG SET appendonly yes
redis-cli -s ./redis.sock CONFIG SET appendfsync everysec
```

Logs every write. More durable but slower.

### Recommended Production Settings

```bash
# Save every 5 min if 1+ changes, every 1 min if 100+ changes
redis-cli CONFIG SET save "300 1 60 100"

# Enable AOF with fsync every second
redis-cli CONFIG SET appendonly yes
redis-cli CONFIG SET appendfsync everysec
```

## Memory Management

### Set Memory Limit

```bash
redis-cli CONFIG SET maxmemory 256mb
redis-cli CONFIG SET maxmemory-policy allkeys-lru
```

### Monitor Memory

```bash
redis-cli INFO memory
```

## Key Patterns

Tinytown uses these key patterns:

| Pattern | Type | Purpose |
|---------|------|---------|
| `tt:inbox:<uuid>` | List | Agent message queues |
| `tt:agent:<uuid>` | String | Agent state (JSON) |
| `tt:task:<uuid>` | String | Task state (JSON) |
| `tt:broadcast` | Pub/Sub | Broadcast channel |

## Debugging

### Connect to Redis

```bash
# Unix socket
redis-cli -s ./redis.sock

# TCP
redis-cli -h 127.0.0.1 -p 6379
```

### Useful Commands

```bash
# List all tinytown keys
KEYS tt:*

# Check inbox length
LLEN tt:inbox:550e8400-...

# View agent state
GET tt:agent:550e8400-...

# Monitor all operations
MONITOR

# Get server info
INFO
```

### Clear All Data

```bash
# Danger: Deletes everything!
redis-cli -s ./redis.sock FLUSHALL
```

## Docker Deployment

```yaml
# docker-compose.yml
version: '3'
services:
  redis:
    image: redis:8
    ports:
      - "6379:6379"
    volumes:
      - redis-data:/data
    command: redis-server --appendonly yes

volumes:
  redis-data:
```

Then configure Tinytown:
```json
{
  "redis": {
    "use_socket": false,
    "host": "localhost",
    "port": 6379
  }
}
```

## Performance Tuning

### For Low Latency

- Use Unix sockets
- Disable persistence (if acceptable)
- Use local SSD

### For Durability

- Enable AOF with `everysec`
- Use persistent storage
- Set up replication (advanced)

### For High Throughput

- Increase `tcp-backlog`
- Tune `timeout` and `tcp-keepalive`
- Use pipelining in code

