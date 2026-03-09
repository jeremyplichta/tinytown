# tt auth

Authentication management for townhall.

## Synopsis

```bash
tt auth <SUBCOMMAND>
```

## Description

Manages authentication credentials for the townhall REST API and MCP servers.

## Subcommands

### gen-key

Generate a new API key and its hash:

```bash
tt auth gen-key
```

## Examples

### Generate API Key

```bash
tt auth gen-key
```

Output:
```
🔐 Generated new API key

API Key (store securely, shown only once):
tt_abc123def456...

API Key Hash (add to tinytown.toml):
$argon2id$v=19$m=19456,t=2,p=1$...

Add to your tinytown.toml:

  [townhall.auth]
  mode = "api_key"
  api_key_hash = "$argon2id$v=19$..."

Then use the API key with townhall:
  curl -H 'Authorization: Bearer tt_abc12...' http://localhost:8080/v1/status
```

## Configuration

After generating a key, add to `tinytown.toml`:

```toml
[townhall]
bind = "127.0.0.1"
rest_port = 8787

[townhall.auth]
mode = "api_key"
api_key_hash = "$argon2id$v=19$m=19456,t=2,p=1$..."
```

## Using the API Key

### With curl

```bash
curl -H "Authorization: Bearer tt_abc123..." http://localhost:8787/v1/status
```

### In scripts

```bash
export TINYTOWN_API_KEY="tt_abc123..."
curl -H "Authorization: Bearer $TINYTOWN_API_KEY" http://localhost:8787/v1/agents
```

## Security Best Practices

1. **Never commit API keys** — Add to `.env` or secrets manager
2. **Use environment variables** — Don't hardcode in scripts
3. **Rotate keys periodically** — Generate new keys with `tt auth gen-key`
4. **Consider OIDC** — For production, use OIDC authentication

## See Also

- [Authentication & Authorization](../advanced/auth.md) — Full auth guide
- [Townhall Control Plane](../advanced/townhall.md) — REST API reference

