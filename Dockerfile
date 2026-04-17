# syntax=docker/dockerfile:1.4
# Build with BuildKit: DOCKER_BUILDKIT=1 docker compose build
# Cache mounts speed repeat builds; cargo fetch layer caches deps until Cargo.lock changes.

FROM node:20-bookworm AS frontend
WORKDIR /build/frontend
COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci --no-audit --no-fund
COPY frontend/ ./
RUN npm run build

FROM rust:1.88-bookworm AS rust-builder
RUN apt-get update -qq && apt-get install -y -qq libpq-dev pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*
RUN cargo install diesel_cli --version 1.4.1 --no-default-features --features postgres \
    && strip /usr/local/cargo/bin/diesel 2>/dev/null || true

WORKDIR /build
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

COPY src ./src
COPY migrations ./migrations
COPY Rocket.toml ./
COPY certs ./certs
# JWT HS256 key is compile-time embedded (`include_bytes!`); repo ships no secret.key.
RUN set -eu; \
    if [ ! -s certs/secret.key ]; then head -c16 /dev/urandom > certs/secret.key; fi
COPY --from=frontend /build/frontend/dist ./frontend/dist

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build --release --bin dasher_control --bin create_admin

FROM debian:bookworm-slim AS runtime
RUN apt-get update -qq && apt-get install -y -qq --no-install-recommends \
    libpq5 ca-certificates curl netcat-openbsd bash \
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

HEALTHCHECK CMD curl -fsS http://127.0.0.1:8080/ >/dev/null || exit 1

CMD ["/bin/sh", "/usr/src/dashercontrol/wait-for-db.sh", "database:5432", "-q", "--", "/usr/src/dashercontrol/dasher_control"]
