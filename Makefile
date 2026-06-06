# Makefile for opencc-cli

.PHONY: all build test fmt fmt-check clippy clean install completions help

# Default target
all: build test

help:
	@echo "========================================================================"
	@echo " opencc-cli - Makefile Common Tasks"
	@echo "========================================================================"
	@echo "Available commands:"
	@echo "  make build         - Build opencc-cli in debug mode"
	@echo "  make release-build - Compile and package release binaries using cargo-dist"
	@echo "  make test          - Run all integration tests (50+ test cases)"
	@echo "  make fmt           - Format Rust source code"
	@echo "  make fmt-check     - Check Rust source formatting without modifying files"
	@echo "  make clippy        - Run Clippy linter with strict warning-as-error"
	@echo "  make clean         - Clean all cargo target build files"
	@echo "  make install       - Install opencc-cli binary locally to cargo bin path"
	@echo "  make completions   - Generate shell completion scripts for all shells"
	@echo "========================================================================"

build:
	cargo build

release-build:
	dist build

test:
	cargo test

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

clippy:
	cargo clippy --workspace --all-targets -- -D warnings

clean:
	cargo clean

install:
	cargo install --path . --force

completions: build
	mkdir -p completions
	cargo run -- completions bash > completions/opencc-cli.bash
	cargo run -- completions zsh > completions/_opencc-cli
	cargo run -- completions fish > completions/opencc-cli.fish
	cargo run -- completions powershell > completions/opencc-cli.ps1
	cargo run -- completions elvish > completions/opencc-cli.elv
	@echo "Shell completions generated successfully in the 'completions/' directory."
