[private]
default:
    just --list

test:
    cargo check
    cargo test