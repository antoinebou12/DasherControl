FROM jdrouet/rust-nightly:stretch-slim-20201230 as builder

WORKDIR /usr/src

# Install basic rust (musl) and postgres lib
RUN apt-get update -y -q && apt-get dist-upgrade -y -q
RUN apt-get install -y -q cmake make g++ build-essential libpq-dev pkg-config libssl-dev musl-tools curl netcat libdbus-1-dev
#RUN rustup target add x86_64-unknown-linux-musl
RUN rustup default nightly && rustup update
RUN cargo install diesel_cli --no-default-features --features postgres

# create new tmp project
#RUN USER=root cargo new dashercontrol
WORKDIR /usr/src/dashercontrol

# Copy frontend import file
COPY ./scripts/js-dev-setup.sh .
COPY ./frontend/ ./frontend
# install npm
RUN bash js-dev-setup.sh

# copy important files
COPY Cargo.toml .
COPY Cargo.lock .
COPY .env .
COPY Rocket.toml .

COPY ./src/ ./src/
COPY ./migrations/ ./migrations/
COPY ./certs/ ./certs/
COPY ./scripts/wait-for-db.sh .

# build small bin for rocket rust
#RUN cargo install --target x86_64-unknown-linux-musl --path .
RUN cargo build --release
RUN cargo build --bin create_admin

COPY ./scripts/wait-for-db.sh .

FROM rust:alpine
RUN apk update -q && apk add netcat-openbsd curl postgresql-dev build-base -q
RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /usr/src/dashercontrol

COPY --from=builder /usr/src/dashercontrol/src/ /usr/src/dashercontrol/src/

COPY --from=builder /usr/src/dashercontrol/frontend/dist/ /usr/src/dashercontrol/frontend/dist/
COPY --from=builder /usr/src/dashercontrol/migrations/ /usr/src/migrations/

COPY --from=builder /usr/src/dashercontrol/.env /usr/src/dashercontrol/.env
COPY --from=builder /usr/src/dashercontrol/wait-for-db.sh /usr/src/dashercontrol/wait-for-db.sh
COPY --from=builder /usr/src/dashercontrol/Rocket.toml /usr/src/dashercontrol/Rocket.toml
COPY --from=builder /usr/src/dashercontrol/Cargo.toml /usr/src/dashercontrol/Cargo.toml
COPY --from=builder /usr/src/dashercontrol/Cargo.lock /usr/src/dashercontrol/Cargo.lock

# certs ssl
COPY --from=builder /usr/src/dashercontrol/certs/cert.crt /usr/src/dashercontrol/certs/cert.crt
COPY --from=builder /usr/src/dashercontrol/certs/cert.key /usr/src/dashercontrol/certs/cert.key

# binary
COPY --from=builder /usr/src/dashercontrol/target/release/dasher_control /usr/src/dashercontrol/dasher_control
COPY --from=builder /usr/src/dashercontrol/target/release/create_admin /usr/src/dashercontrol/create_admin

HEALTHCHECK CMD ( curl -f https://localhost:8080/ || curl -f http://localhost:8080/ || exit 1)

CMD ["/bin/sh", "/usr/src/dashercontrol/wait-for-db.sh", "database:5432", "-q", "--", "/usr/src/dashercontrol/dasher_control"]