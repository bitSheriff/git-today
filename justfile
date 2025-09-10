
set shell := ["bash", "-uc"]

# List all recipes
default:
    just --choose

# Run checks on the code
check:
    cargo clippy

# Install the binary
install:
    cargo install --path .

# Build the release binary
release:
    # Build Linux binary
    cargo build --release

# Publish the application to crates.io
publish:
    cargo publish
