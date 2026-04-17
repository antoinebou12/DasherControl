# Install

These steps match the main [README](https://github.com/antoinebou13/DasherControl/blob/main/README.md) (tested on Ubuntu 20.04).

## Prerequisites

- Rust toolchain
- Node.js and npm (for the Vue frontend)
- PostgreSQL
- `diesel_cli` with Postgres: `cargo install diesel_cli --no-default-features --features postgres`

## Database

1. Copy or create `.env` and set `DATABASE_URL` for your PostgreSQL instance.
2. Adjust `Rocket.toml` if needed.
3. Run migrations: `diesel migration run`

## Admin user

```bash
cargo run --bin create_admin
```

## Frontend build

```bash
cd frontend && npm install && npm run build && cd ..
```

## Run the server

```bash
cargo run
```

## Docker

```bash
DOCKER_BUILDKIT=1 docker build -t antoinebou13/dashercontrol .
```

Or with Compose:

```bash
DOCKER_BUILDKIT=1 docker-compose up -d --build
```
