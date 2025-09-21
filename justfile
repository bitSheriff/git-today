
set shell := ["bash", "-uc"]

# List all recipes
default:
    just --choose

# Run checks on the code
check:
    # Format Code
    @cargo fmt
    # Lint Code
    @cargo clippy

# Build the debug binary
build: check
    # build the debug binary
    @cargo build

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

# Internal Recipe for running integration tests
_integration-tests:
    # Run integration tests
    @( cd test && ./execute_tests.sh )

_unit-tests:
    # Run unit tests
    @cargo test

# Run Test Suite
test: build _unit-tests _integration-tests
