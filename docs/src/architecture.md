# Architecture

## Backend (Rocket)

The HTTP server mounts route groups at:

| Prefix        | Module       | Role                          |
|---------------|--------------|-------------------------------|
| `/`           | `router`     | SPA shell, static `public/`   |
| `/tenants`    | `tenants`    | Login, JWT/cookies, users     |
| `/workspaces` | `workspaces` | Workspaces and applets (grid) |
| `/containers` | `containers` | DB records + Docker (bollard) |
| `/caddy`      | `caddy`      | Authenticated Caddyfile JSON API |

Templates (Tera) serve the main HTML shell; the dashboard UI is the compiled Vue app under `frontend/dist/public`.

## Frontend (Vue)

- **Vue 2** with Vuesax and `vue-grid-layout` for draggable tiles.
- Each tile hosts an **Applet** component (`IFrame`, `DockerManager`, `Terminal`, `CaddyConfigurator`, etc.).
- API calls use `axios`; authenticated routes send `Authorization: Bearer <token>` from the Vuex store after login.

## Data

PostgreSQL holds tenants, workspaces, applets, container metadata, and related tables. Diesel provides the ORM layer.

## Security notes

- JWT claims are validated against the database session where applicable.
- The Caddy generator only returns text; review any generated config before applying it to a production edge.
