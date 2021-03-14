curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
export PATH="$HOME/.cargo/bin:$PATH"
rustup default nightly
rustup update
# install cmake make g++ build-essential libpq-dev
cargo install
