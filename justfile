# List available commands
default:
    @just --list

# Operations
test:
    cargo test --workspace -- --test-threads=1
lint:
    cargo clippy --workspace --all-targets
fmt:
    cargo fmt --all
build:
    cargo build --workspace
fix:
    cargo fix --workspace --all-targets --allow-dirty
release:
    cargo build --release

# Meta commands
ci: lint fmt build test

