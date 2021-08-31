rustup toolchain install nightly
rustup component add rust-src --toolchain nightly
cargo build -Z build-std=core
qemu-system-x86_64.exe -drive format=raw,file=target/x86-unknown-bare_metal/debug/bootimage-oxide.bin