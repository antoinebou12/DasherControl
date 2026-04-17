# syntax=docker/dockerfile:1

FROM node:20-bookworm AS frontend
WORKDIR /build/frontend
COPY frontend/package.json ./
RUN npm install --no-audit --no-fund
COPY frontend/ ./
RUN npm run build

FROM rust:1.88-bookworm AS rust-builder
RUN apt-get update -qq && apt-get install -y -qq libpq-dev pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*
RUN cargo install diesel_cli --version 1.4.1 --no-default-features --features postgres
WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations
COPY Rocket.toml ./
COPY --from=frontend /build/frontend/dist ./frontend/dist
RUN cargo build --release --bin dasher_control --bin create_admin

FROM debian:bookworm-slim AS runtime
RUN apt-get update -qq && apt-get install -y -qq libpq5 ca-certificates curl netcat-openbsd \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/dashercontrol

COPY --from=rust-builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel
COPY --from=rust-builder /build/target/release/dasher_control ./
COPY --from=rust-builder /build/target/release/create_admin ./
COPY --from=rust-builder /build/frontend/dist ./frontend/dist/
COPY --from=rust-builder /build/migrations /usr/src/migrations/
COPY --from=rust-builder /build/Rocket.toml ./Rocket.toml
COPY certs/ ./certs/
COPY scripts/wait-for-db.sh ./wait-for-db.sh
RUN chmod +x wait-for-db.sh

HEALTHCHECK CMD curl -fsS http://127.0.0.1:8080/ >/dev/null 2>&1 || curl -fsSk https://127.0.0.1:8080/ >/dev/null 2>&1 || exit 1

CMD ["/bin/sh", "/usr/src/dashercontrol/wait-for-db.sh", "database:5432", "-q", "--", "/usr/src/dashercontrol/dasher_control"]
