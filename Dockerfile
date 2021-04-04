FROM rustlang/rust:nightly
RUN apt-get update -y -q
RUN apt-get install -y -q cmake make g++ build-essential libpq-dev libssl-dev nodejs musl-gcc
WORKDIR /usr/src/app
RUN cargo install diesel_cli --no-default-features --features "postgrep"
RUN source .env
RUN diesel migration run
RUN cd frontend && npm run build && cd ..
RUN rustup target install x86_64-unknown-linux-musl
RUN cargo build --release --target=x86_64-unknown-linux-musl
CMD ["cargo", "run"]