# List available commands
default:
    @just --list

# Run all tests
test:
    cargo test -- --test-threads=1

# Run tests for logger only
test-logger:
    cargo test -p rust-telemetry -- --test-threads=1

# Lint with clippy
lint:
    cargo clippy --all-targets

# Format code
fmt:
    cargo fmt

# Check formatting without changing
fmt-check:
    cargo fmt --check

# Lint and format
check: fmt lint

# Build debug
build:
    cargo build

# Build release
build-release:
    cargo build --release

# Run the example
run:
    cargo run -p example

# Clean build artifacts
clean:
    cargo clean

