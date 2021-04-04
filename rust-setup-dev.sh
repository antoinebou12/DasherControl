curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
export PATH="$HOME/.cargo/bin:$PATH"
rustup update
rustup default nightly
# Install cmake make g++ build-essential libpq-dev libssl-dev
cargo install diesel_cli --no-default-features --features postgres
source .env
diesel migration run