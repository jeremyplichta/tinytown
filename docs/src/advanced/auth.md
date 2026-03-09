# Authentication & Authorization

Townhall supports multiple authentication modes and fine-grained authorization scopes for secure remote access.

## Authentication Modes

### None (Default)

No authentication required. Only safe on loopback (`127.0.0.1`).

```toml
[townhall.auth]
mode = "none"  # Default
```

> ⚠️ **Security**: Townhall will refuse to start if binding to non-loopback addresses with `mode = "none"`.

### API Key

Secure token-based authentication using Argon2id password hashing.

```toml
[townhall.auth]
mode = "api_key"
api_key_hash = "$argon2id$v=19$m=19456,t=2,p=1$..."
api_key_scopes = ["town.read", "town.write"]  # Optional: restrict scopes
```

#### Generating an API Key

```bash
# Generate a new API key (outputs raw key and hash)
tt generate-api-key

# Output:
# API Key: abc123def456...
# Hash: $argon2id$v=19$m=19456,t=2,p=1$...
#
# Add to tinytown.toml:
# [townhall.auth]
# mode = "api_key"
# api_key_hash = "$argon2id$v=19$..."
```

**Important**: Store the raw API key securely. Only the hash is stored in config.

#### Using API Keys

Pass the key via `Authorization` header or `X-API-Key`:

```bash
# Bearer token (recommended)
curl -H "Authorization: Bearer <your-api-key>" \
  http://localhost:8787/v1/status

# X-API-Key header
curl -H "X-API-Key: <your-api-key>" \
  http://localhost:8787/v1/status
```

### OIDC (Coming Soon)

OpenID Connect authentication for enterprise deployments.

```toml
[townhall.auth]
mode = "oidc"
issuer = "https://issuer.example.com"
audience = "tinytown-api"
jwks_url = "https://issuer.example.com/.well-known/jwks.json"
required_scopes = ["tinytown:access"]
clock_skew_seconds = 60
```

## Authorization Scopes

All endpoints require specific scopes for access:

| Scope | Description | Endpoints |
|-------|-------------|-----------|
| `town.read` | Read status and agent info | `GET /v1/*`, inbox |
| `town.write` | Assign tasks, send messages | `POST /v1/tasks/*`, messages, backlog |
| `agent.manage` | Spawn/kill/restart agents | `POST /v1/agents`, recovery |
| `admin` | Full access (grants all scopes) | All endpoints |

### Configuring Scopes

For API key auth, configure allowed scopes:

```toml
[townhall.auth]
mode = "api_key"
api_key_hash = "..."
api_key_scopes = ["town.read", "town.write"]  # Read/write but no agent management
```

Empty `api_key_scopes` grants admin access (all scopes).

## Audit Logging

All mutating operations (POST, PUT, DELETE, PATCH) are logged:

```
INFO audit: operation completed request_id="abc123" principal="api_key" method="POST" path="/v1/agents" result="success"
WARN audit: operation denied request_id="def456" principal="api_key" method="POST" path="/v1/recover" result="denied"
```

Audit events include:
- **Request ID** - Unique identifier for correlation
- **Principal ID** - Who made the request
- **Scopes** - What permissions they had
- **Method/Path** - What they tried to do
- **Result** - success, denied, or error

### Security Notes

- Authorization headers are **never logged**
- API keys are stored as Argon2id hashes, never plaintext
- Auth errors use constant-time responses to prevent timing attacks

## TLS Configuration

For production deployments, enable TLS:

```toml
[townhall.tls]
enabled = true
cert_path = "/path/to/server.crt"
key_path = "/path/to/server.key"
```

### Mutual TLS (mTLS)

For service-to-service authentication:

```toml
[townhall.mtls]
enabled = true
required = true  # Reject clients without valid certs
ca_path = "/path/to/ca.crt"
```

## Security Best Practices

1. **Local development**: Use `mode = "none"` with `bind = "127.0.0.1"`
2. **Team access**: Use `mode = "api_key"` with TLS
3. **Production**: Use `mode = "oidc"` with TLS and mTLS
4. **Always** scope API keys to minimum required permissions
5. **Rotate** API keys regularly

