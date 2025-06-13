set shell := ["bash", "-cu"]
set windows-shell := ["powershell"]

# Default action
_:
    just lint
    just fmt
    just test

# Setup
setup:
    brew install ls-lint typos-cli

# Lint code
lint:
    ls-lint
    typos
    cargo check
    cargo clippy
    cargo test -p jder_axum -- --nocapture

# Format code
fmt:
    cargo fmt

# Run tests
test:
    cargo test -p tests -- --nocapture

# Run test server
server:
    cargo run -p tests

# Clean
clean:
    cargo clean
