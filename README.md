# DasherControl

Configurable dashboard for homelab web apps: **Vue 3 + TypeScript** frontend (Vite, Pinia), **Rust (Rocket)** API, iframe applets, workspaces, Docker basics, optional OAuth, and auth.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Tests](https://img.shields.io/github/actions/workflow/status/antoinebou13/DasherControl/tests.yml?branch=main&label=tests)](https://github.com/antoinebou13/DasherControl/actions/workflows/tests.yml)
[![GitHub release](https://img.shields.io/github/v/release/antoinebou13/DasherControl)](https://github.com/antoinebou13/DasherControl/releases)

## Why this exists

Most homelab tools are separate web apps in Docker. DasherControl is one place to open Sonarr, Jellyfin, and the rest instead of many browser tabs, with saved layouts like a simple “Vista-style” widget board backed by PostgreSQL.

Reverse proxies and TLS are still painful with Portainer or raw Docker alone, so the direction of the project is more dashboard widgets (“applets”) for day-to-day tasks, not only links.

## Preview

- v0.1.5 demo: [demo gif](https://raw.githubusercontent.com/antoinebou13/DasherControl/main/images/demo%20dashercontrol.gif)
- v0.1.2 screenshot: [DasherControl.png](https://raw.githubusercontent.com/antoinebou13/DasherControl/main/images/DasherControl.png)

## Known limitation

Embedded sites use iframes. Some apps block iframes or break login (CSRF, `X-Frame-Options`, cookie rules). Those services are better opened in a normal tab or behind your own reverse proxy.

---

## Get the code

```bash
git clone https://github.com/antoinebou13/DasherControl.git
cd DasherControl
```

Or download a [release zip](https://github.com/antoinebou13/DasherControl/releases) and unpack it, then `cd` into the folder.

---

## Run locally (development)

### Prerequisites

- **PostgreSQL** running with an empty database (example below uses database name `diesel`).
- **Rust** toolchain ([rustup](https://rustup.rs/)): this repo targets **Rust 1.75+** (`rust-version` in `Cargo.toml`, `edition = "2021"`).
- **Node.js**: use a **current LTS** (18+); the frontend uses Vite 6 and TypeScript 5.7.

**Windows (native build):** install PostgreSQL client dev libraries so `libpq` links (e.g. PostgreSQL installer with “command line tools”, or vcpkg). WSL2 with Ubuntu is often simpler than MSVC + `libpq`.

### 1. Environment

```bash
cp .env.example .env
# Edit DATABASE_URL to match your Postgres user, password, host, port, and database name.
```

`DATABASE_URL` should match your Postgres instance. The example file also documents:

- `CSRF_TOKEN_KEY` and `ROCKET_SECRET_KEY` (required for cookies / CSRF in dev and prod).
- Optional **OAuth** (`OAUTH_PUBLIC_APP_URL`, Google/GitHub client IDs and secrets).
- Optional **Web SSH** allowlist: `TERMINAL_SSH_ALLOWLIST` (comma-separated `host:port`; when unset, only loopback / private ranges are allowed after DNS resolution).

Example URL:

`postgres://postgres:change-me@localhost:5432/diesel`

PowerShell (session only):

```powershell
$env:DATABASE_URL = 'postgres://postgres:change-me@localhost:5432/diesel'
```

### 2. Optional helper scripts (Linux / Git Bash)

```bash
bash scripts/rust-setup-dev.sh
bash scripts/js-dev-setup.sh
```

### 3. Frontend build

```bash
cd frontend
npm install
npm run build
cd ..
```

For day-to-day UI work, you can run the API and Vite together: start `cargo run` (Rocket on **8080** by default), then in another terminal `cd frontend && npm run dev`. Vite serves the SPA on **https://localhost:9999** and proxies `/tenants`, `/workspaces`, `/containers`, and `/caddy` to Rocket (see `frontend/vite.config.ts`). Set `OAUTH_PUBLIC_APP_URL` to match how the browser reaches the app (see comments in `.env.example`).

### 4. Database schema

Install Diesel CLI (once):

```bash
cargo install diesel_cli --no-default-features --features postgres
```

Apply migrations (from repo root, with `DATABASE_URL` set in `.env` or the environment):

```bash
diesel migration run
```

### 5. Create the first admin user

With **no arguments**, `create_admin` creates the default **development** account:

| Field    | Value            |
|----------|------------------|
| Email    | `admin@localhost` |
| Username | `admin`          |
| Password | `admin`          |

```bash
cargo run --bin create_admin
```

Custom credentials (all three required):

```bash
cargo run --bin create_admin -- 'you@example.com' 'myuser' 'strong-password-here'
```

**Do not use the default `admin` / `admin` on the public internet.** Change password or create a new admin before exposing the app.

### 6. Start the server

```bash
cargo run
```

Open **http://localhost:8080** (see `Rocket.toml` for `address` / `port`). Log in with the username and password you created.

---

## Docker

Build:

```bash
DOCKER_BUILDKIT=1 docker build -t antoinebou13/dashercontrol .
```

Compose (includes Postgres; app receives `DATABASE_URL` in the container):

```bash
DOCKER_BUILDKIT=1 docker compose up -d --build
docker compose logs -f
```

After the stack is up, run migrations and seed admin **inside** the app container (or from a one-off container on the same network), for example:

```bash
docker compose exec dashercontrol diesel migration run
docker compose exec dashercontrol /usr/src/dashercontrol/create_admin
```

Adjust paths if your image layout differs. Ensure `.env` used at image build time matches the compose database URL.

---

## Tests

```bash
cargo test
```

The frontend `package.json` does not define a separate test script; use `npm run build` (and `vue-tsc` if you add a typecheck script) to validate the UI bundle.

---

## Roadmap

More detail: [Architecture](docs/src/architecture.md) (stack and route prefixes), [Install](docs/src/install.md), [Caddy](docs/src/caddy.md). To preview the mdBook locally (requires [mdBook](https://rust-lang.github.io/mdBook/); config in `book.toml`): run `mdbook serve` from the repository root. GitHub Actions [`.github/workflows/docs.yml`](.github/workflows/docs.yml) builds the book for GitHub Pages when enabled on the repository.

### Done

Delivered capabilities, grouped by theme. Paths are from the repository root.

#### Dashboard and layout

- [x] **Applets with iframe** — Draggable grid tiles embed homelab URLs in iframes (`IFrame`, borderless variant). Frontend: `frontend/components/applets/IFrame/`, shared shell `frontend/components/applets/Applet.vue`, `frontend/components/GridItemApplet.vue`. See [Known limitation](#known-limitation) for framing caveats.
- [x] **Workspaces (save / switch)** — Named layouts per tenant, ordered in Postgres (`workspaces`). API under `/workspaces` (see `src/workspaces/`). Pinia-driven appearance and switching in the dashboard shell (`frontend/stores/workspaceAppearance.ts`).
- [x] **Applet management** — Create, resize, and place applets; CRUD via workspace routes and `applets` rows (JSON payload in DB). Editor UI: `frontend/components/applets/Editor/`, create flow `frontend/components/applets/CreateNew/`.
- [x] **Workspace theme and background** — Presets, colors, optional background image, and icon fields on workspace rows; applied client-side via `workspaceAppearance` store and `frontend/utils/workspaceTheme.ts`.
- [x] **Floating applets** — Detach tiles to floating, draggable panels (`frontend/components/FloatingAppletPanel.vue`).

#### Applets beyond iframes

- [x] **Canvas applet** — Three.js-based tile (`frontend/components/applets/Canvas/`).
- [x] **Web SSH terminal** — Admin terminal over WebSocket (`frontend/components/applets/Terminal/`, Rocket under `/terminal`). Configure allowlists and env in `.env.example`.
- [x] **Caddy configurator** — Dashboard applet and `/caddy` API (`frontend/components/applets/CaddyConfigurator/`, `src/caddy/`).

#### Docker and inventory

- [x] **Basic Docker start / stop / manage** — Live container list and admin actions via [Bollard](https://github.com/fussybeaver/bollard) (`src/containers/docker.rs`). HTTP: `/containers/api/docker_list`, restart/stop/create routes in `src/containers/handler.rs`. Dashboard: `frontend/components/applets/DockerManager/`.
- [x] **Persist docker-compose / container metadata** — Snapshot table `docker_host_containers` (Compose project/service labels and JSON names/labels). Refresh: `POST /containers/api/docker_snapshot` (Admin, requires Docker socket). Read last snapshot: `GET /containers/api/docker_snapshot`. Migration: `migrations/2026-04-16-180000_add_docker_host_containers/`.

#### Access, automation, and packaging

- [x] **User auth** — Login, JWT bearer + session checks, roles (e.g. Admin for Docker mutations). `src/tenants/`; token helpers `src/tenants/token.rs`. First user: [Create the first admin user](#5-create-the-first-admin-user).
- [x] **OAuth login (Google + GitHub)** — Authorization-code flow and account linking (`src/tenants/oauth.rs`). Configure provider keys and `OAUTH_PUBLIC_APP_URL` in `.env`.
- [x] **CI/CD** — GitHub Actions workflow [`.github/workflows/tests.yml`](.github/workflows/tests.yml) runs migrations and `cargo test` against Postgres.
- [x] **Install app via Docker / Compose** — [Docker](#docker) section above; image build and compose bring up the app with `DATABASE_URL`. Companion docs: [Install](docs/src/install.md).
- [x] **Structured server-side logging** — `tracing` + `tracing-subscriber` (`RUST_LOG`, default `dasher_control=info,rocket=warn`) initialized in [`src/main.rs`](src/main.rs); per-request access log and `X-Request-Id` in [`src/fairing.rs`](src/fairing.rs) (`RequestLogging`); Docker client messages use `tracing` instead of stderr prints in [`src/containers/docker.rs`](src/containers/docker.rs). See [`.env.example`](.env.example).
- [x] **Tests (expanded coverage)** — Bearer guard checks in [`src/tests/test_auth_http.rs`](src/tests/test_auth_http.rs); JSON smoke checks for `/json`, `/ip`, and containers list/snapshot in [`src/tests/test_main.rs`](src/tests/test_main.rs) and [`src/tests/test_containers.rs`](src/tests/test_containers.rs); Caddy validation and Caddyfile generation cases in [`src/caddy/generator.rs`](src/caddy/generator.rs).

### Ideas

- [ ] Export / import containers and workspaces
- [ ] More applet types and polish
