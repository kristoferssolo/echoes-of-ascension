export RUSTC_WRAPPER:="sccache"
# Li st all available commands
default:
    @just --list

# Install required tools and dependencies
setup:
    rustup toolchain install nightly
    rustup default nightly
    rustup target add wasm32-unknown-unknown
    cargo install cargo-leptos
    cargo install cargo-watch
    cargo install just

# Development Commands

# Start development server with hot reload
dev: kill-server
    RUST_BACKTRACE=full cargo leptos watch | bunyan

# Run cargo check on both native and wasm targets
check:
    cargo check --all-targets
    cargo check --all-targets --target wasm32-unknown-unknown

# Run tests
test:
    cargo test --all-targets
    cargo test --all-targets --target wasm32-unknown-unknown

# Format code
fmt:
    cargo fmt --all

# Run clippy lints
lint:
    cargo clippy --all-targets -- -D warnings
    cargo clippy --all-targets --target wasm32-unknown-unknown -- -D warnings

# Clean build artifacts
clean:
    cargo clean
    rm -rf dist
    rm -rf target

# Build Commands

# Build for development
build-dev:
    cargo leptos build

# Build for production
build-prod:
    cargo leptos build --release

# Build WASM only
build-wasm:
    cargo leptos build-only-wasm

# Build server only
build-server:
    cargo leptos build-only-server

# Database Commands (add these when you set up your database)
db-setup:
    echo "Add your database setup commands here"

db-migrate:
    echo "Add your database migration commands here"

# Deployment Commands (customize based on your deployment strategy)
deploy:
    echo "Add your deployment commands here"

# Combined commands
check-all: fmt lint check test

# Start production server
serve-prod:
    cargo leptos serve --release

kill-server:
    #!/usr/bin/env sh
    pkill -f "target/debug/server" || true
    pkill -f "cargo-leptos" || true

