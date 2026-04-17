# Caddy reverse proxy

DasherControl includes a **CaddyConfigurator** applet and a small API that returns a **Caddy v2 Caddyfile** from JSON you provide.

## API

- **URL:** `POST /caddy/api/generate`
- **Auth:** `Authorization: Bearer <JWT>` (same token as other authenticated APIs).
- **Body:**

```json
{
  "sites": [
    {
      "hostname": "app.example.com",
      "upstream": "127.0.0.1:8080",
      "tls_email": "you@example.com",
      "path_prefix": "api",
      "encode": true
    }
  ]
}
```

- **Response:** `{ "caddyfile": "..." }`

Fields:

| Field          | Required | Description |
|----------------|----------|-------------|
| `hostname`     | yes      | Site address in the Caddyfile. |
| `upstream`     | yes      | `host:port` or Docker service name. |
| `tls_email`    | no       | If set, adds a `tls` line for ACME email. |
| `path_prefix`  | no       | If set (e.g. `api`), wraps `reverse_proxy` in `handle_path /api/*`. |
| `encode`       | no       | If true, adds `encode zstd gzip`. |

## Dashboard

Add a new applet and choose **CaddyConfigurator**. Fill hostname and upstream, optionally TLS email and path prefix, then **Generate** and **Copy** the result into your real Caddyfile or config repo.

## After generation

1. Paste the snippet into your Caddy configuration.
2. Reload Caddy (`caddy reload` or your process manager).
3. Ensure DNS for `hostname` points to the machine running Caddy.

This tool does not deploy Caddy or obtain certificates by itself; it only formats a starting point for your homelab.
