FROM rustlang/rust:nightly as builder

WORKDIR /usr/src

# Install basic rust (musl) and postgres lib
RUN apt-get update -y -q && apt-get dist-upgrade -y -q
RUN apt-get install -y -q cmake make g++ build-essential libpq-dev pkg-config libssl-dev musl-tools curl
RUN rustup target add x86_64-unknown-linux-musl
# Install orm diesel
RUN cargo install diesel_cli --no-default-features --features postgres
# create new tmp project
RUN USER=root cargo new dashercontrol
WORKDIR /usr/src/dashercontrol

# copy important files
COPY Cargo.toml .
COPY Cargo.lock .
COPY .env .
COPY Rocket.toml .
COPY ./scripts/js-dev-setup.sh .
COPY ./frontend/ ./frontend

# install npm
RUN bash js-dev-setup.sh && cd ..
# build small bin for rocket rust
RUN rustup target install x86_64-unknown-linux-musl
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM rustlang/rust:nightly
WORKDIR /usr/src/dashercontrol
COPY --from=builder /usr/src/dashercontrol/target/release .
COPY --from=builder /usr/src/dashercontrol/frontend/dist .
COPY --from=builder /usr/src/dashercontrol/.env .
RUN source .env
RUN cargo install diesel_cli --no-default-features --features postgres
USER 1000

CMD ["bash", "./script/wait-for-db.sh", "database:5432", "-q", "--", "diesel", "setup", "&&", "diesel migration run", "&&", "ROCKET_ENV=production cargo run"]