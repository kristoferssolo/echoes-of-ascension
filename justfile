set dotenv-load

PROJECT_NAME := "echoes-of-ascension"

# List all available commands
default:
    @just --list

# Format code
fmt:
    cargo fmt

# Lint code
lint:
    cargo clippy -- -D warnings

# Run tests
test:
    cargo test

# Build the application (debug)
build:
    cargo build

# Build the application (release)
build-release:
    cargo build --release

# Run the application (debug)
run:
    cargo run

# Run the application (release)
run-release:
    cargo run --release

# Run migrations
migrate:
    cargo sqlx migrate run

# Revert migrations
migrate-revert:
    cargo sqlx migrate revert

# Create a new migration
migrate-create name:
    cargo sqlx migrate add $(name)

# Check migrations
migrate-status:
    cargo sqlx migrate status

# Watch for changes and run tests/linting/run (for development)
dev:
    cargo watch -x clippy -x test -x run | bunyan

# Build, migrate, and run (release)
deploy:
    just build-release
    just migrate
    just run-release

# Generate documentation
doc:
    cargo doc --open

# Clean the project
clean:
    cargo clean

# Analyze binary size
analyze-size:
    cargo build --release
    cargo install cargo-bloat
    cargo bloat --release --all-features --crates

# Check dependencies for security vulnerabilities
audit:
    cargo audit

# Check for outdated dependencies
outdated:
    cargo outdated
