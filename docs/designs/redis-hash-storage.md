# Design: Redis Hash Storage for Agent and Task State

**Author:** backend-1
**Date:** 2026-03-09
**Status:** Implemented

## Overview

This design proposes refactoring Agent and Task state storage from JSON strings to Redis Hashes, following Redis best practices for atomic field updates and memory efficiency.

## Current State

Currently, Agent and Task data are stored as JSON-serialized strings:

```
tt:agent:<uuid>  →  JSON string (SET/GET)
tt:task:<uuid>   →  JSON string (SET/GET)
```

**Limitations:**
- Cannot atomically update individual fields (must read-modify-write entire object)
- Cannot use HEXPIRE for per-field TTL
- Higher memory overhead for repeated field names across keys

## Proposed Change

Convert to Redis Hash storage:

```
tt:agent:<uuid>  →  Hash (HSET/HGET/HGETALL)
tt:task:<uuid>   →  Hash (HSET/HGET/HGETALL)
```

## Benefits

| Feature | Current (String) | Proposed (Hash) |
|---------|------------------|-----------------|
| Atomic field updates | ❌ Read-modify-write | ✅ HSET single field |
| Memory efficiency | ❌ Repeated field names | ✅ Shared field names |
| Per-field expiration | ❌ Not possible | ✅ HEXPIRE (Redis 7.4+) |
| Partial reads | ❌ Full deserialize | ✅ HGET specific field |

## Scope

### In Scope
- Agent struct storage (9 fields, all flat)
- Task struct storage (11 fields, mostly flat)

### Out of Scope
- Message storage (nested MessageType enum - not suitable for Hash)
- Inbox lists (already using appropriate LIST type)

## Data Structure Mapping

### Agent Fields
| Field | Redis Hash Field | Type |
|-------|------------------|------|
| id | id | string |
| name | name | string |
| agent_type | agent_type | string (enum) |
| state | state | string (enum) |
| cli | cli | string |
| current_task | current_task | string (nullable) |
| created_at | created_at | string (ISO 8601) |
| last_heartbeat | last_heartbeat | string (ISO 8601) |
| tasks_completed | tasks_completed | string (u64) |
| rounds_completed | rounds_completed | string (u64) |

### Task Fields
| Field | Redis Hash Field | Type |
|-------|------------------|------|
| id | id | string |
| description | description | string |
| state | state | string (enum) |
| assigned_to | assigned_to | string (nullable) |
| created_at | created_at | string (ISO 8601) |
| updated_at | updated_at | string (ISO 8601) |
| started_at | started_at | string (nullable ISO 8601) |
| completed_at | completed_at | string (nullable ISO 8601) |
| result | result | string (nullable) |
| parent_id | parent_id | string (nullable) |
| tags | tags | string (JSON array) |

**Note:** `tags` must remain JSON-encoded since Redis Hashes don't support nested arrays.

## Implementation Phases

### Phase 1: Add Hash Methods (Non-breaking)
- Add `set_agent_state_hash()`, `get_agent_state_hash()` methods
- Add `set_task_hash()`, `get_task_hash()` methods
- Keep existing JSON methods for backward compatibility

### Phase 2: Migration Path
- Add migration command to convert existing JSON keys to Hash
- Update callers to use new Hash methods
- Add version marker to detect storage format

### Phase 3: Cleanup
- Deprecate JSON methods
- Remove old string keys after migration

## Migration Strategy

```rust
// Check if key is JSON string or Hash
pub async fn is_hash_storage(&self, key: &str) -> bool {
    let key_type: String = redis::cmd("TYPE").arg(key).query_async().await?;
    key_type == "hash"
}

// Migrate a single key
pub async fn migrate_to_hash(&self, key: &str) -> Result<()> {
    if self.is_hash_storage(key).await {
        return Ok(()); // Already migrated
    }
    // Read JSON, convert to hash, delete old key
}
```

## Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| Breaking existing towns | Phase 1 adds methods without removing old ones |
| Data loss during migration | Migration is idempotent; old data preserved until verified |
| Performance regression | Benchmark before/after; Hash operations are generally faster |

## References

- [Redis Best Practices - Choose the Right Data Structure](https://redis.io/docs/latest/develop/data-types/compare-data-types/)
- [Redis HSET Documentation](https://redis.io/commands/hset/)
- [Redis HEXPIRE (7.4+)](https://redis.io/commands/hexpire/)

