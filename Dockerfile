FROM rustlang/rust:nightly-slim
RUN apt-get update
RUN apt-get install -y default-libmysqlclient-dev
WORKDIR /usr/src/myapp

RUN rustup default nightly;
RUN cargo install diesel_cli --no-default-features --features "postgrep"
RUN cargo install diesel_cli --no-default-features --features "postgrep"