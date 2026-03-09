# tt migrate

Migrate Redis keys from old format to town-isolated format.

## Synopsis

```bash
tt migrate [OPTIONS]
```

## Description

The `migrate` command handles backward compatibility when upgrading to a version of Tinytown that uses town-isolated Redis keys. Older versions stored keys in the format `tt:type:id`, while newer versions use `tt:<town_name>:type:id` to support multiple towns sharing the same Redis instance.

This migration is:
- **Safe**: Preview with `--dry-run` before committing
- **Idempotent**: Running multiple times has no effect after initial migration
- **Atomic**: Each key is renamed atomically

## Options

| Option | Description |
|--------|-------------|
| `--dry-run` | Preview migration without making changes |
| `--force` | Skip confirmation prompt |

## Key Formats

### Old Format (pre-isolation)
```
tt:agent:<uuid>
tt:inbox:<uuid>
tt:urgent:<uuid>
tt:task:<uuid>
tt:activity:<uuid>
tt:stop:<uuid>
tt:backlog
```

### New Format (town-isolated)
```
tt:<town_name>:agent:<uuid>
tt:<town_name>:inbox:<uuid>
tt:<town_name>:urgent:<uuid>
tt:<town_name>:task:<uuid>
tt:<town_name>:activity:<uuid>
tt:<town_name>:stop:<uuid>
tt:<town_name>:backlog
```

## Examples

### Preview Migration (Dry Run)

```bash
tt migrate --dry-run
```

Output:
```
🔍 Migration Preview (dry run)
   Town: my-project

   Keys to migrate:
   tt:agent:abc123 → tt:my-project:agent:abc123
   tt:inbox:abc123 → tt:my-project:inbox:abc123
   tt:task:def456 → tt:my-project:task:def456

   Total: 3 key(s) would be migrated

   Run 'tt migrate' (without --dry-run) to perform migration.
```

### Perform Migration (Interactive)

```bash
tt migrate
```

Prompts for confirmation before proceeding.

### Perform Migration (Force)

```bash
tt migrate --force
```

Skips the confirmation prompt. Useful for automation/CI.

## When to Use

You need to run migration if:

1. **Upgrading from an older version**: If you're upgrading from a version that didn't have town isolation, your existing keys need migration.

2. **Seeing "no migration needed" message**: If the command reports no migration needed, your keys are already in the new format.

3. **Sharing Redis between towns**: Town isolation allows multiple Tinytown projects to use the same Redis instance without key conflicts.

## Recovery

If migration fails partway through:

1. Check which keys failed in the output
2. Investigate the specific errors
3. Run `tt migrate` again (idempotent - already-migrated keys won't be re-migrated)

## See Also

- [tt init](./init.md) - Initialize a new town
- [tt reset](./reset.md) - Reset all town state
- [Towns concept](../concepts/towns.md) - Understanding towns

