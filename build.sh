rustup toolchain install nightly
rustup component add rust-src --toolchain nightly
cargo build -Z build-std=core