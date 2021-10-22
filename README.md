# OxideOS
An attempt at a rust OS :), yes it has the same name as the oher Oxide OS, but it's outdated and I wanted to try and do it :D 

## Setup
---
Oxide needs the `nightly` version of rust, since the stable version of rust can't make custom build targets (yet).

Once you get the nightly version, you should just be able to use `cargo build`, to build the OS.

You might need to run `rustup component add llvm-tools-preview` if it doesnt work.