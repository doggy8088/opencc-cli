# Makefile for opencc-cli

.PHONY: all build test fmt fmt-check clippy clean install uninstall completions help

# Use the sibling core during multiarch development. Release builds resolve the
# versioned registry dependency after the core crate has been published.
CARGO := cargo
ifneq ($(wildcard ../opencc-rust/Cargo.toml),)
CORE_PATCH := --config 'patch.crates-io.opencc-rs-tw2.path="../opencc-rust"'
endif

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
	@echo "  make uninstall     - Uninstall opencc-cli binary from cargo bin path"
	@echo "  make completions   - Generate shell completion scripts for all shells"
	@echo "========================================================================"

build:
	$(CARGO) build $(CORE_PATCH)

release-build:
	dist build

test:
	$(CARGO) test $(CORE_PATCH)

fmt:
	$(CARGO) fmt --all

fmt-check:
	$(CARGO) fmt --all -- --check

clippy:
	$(CARGO) clippy $(CORE_PATCH) --workspace --all-targets -- -D warnings

clean:
	$(CARGO) clean

install:
	$(CARGO) install $(CORE_PATCH) --path . --force

uninstall:
	$(CARGO) uninstall opencc-cli

completions: build
	mkdir -p completions
	$(CARGO) run $(CORE_PATCH) -- completions bash > completions/opencc-cli.bash
	$(CARGO) run $(CORE_PATCH) -- completions zsh > completions/_opencc-cli
	$(CARGO) run $(CORE_PATCH) -- completions fish > completions/opencc-cli.fish
	$(CARGO) run $(CORE_PATCH) -- completions powershell > completions/opencc-cli.ps1
	$(CARGO) run $(CORE_PATCH) -- completions elvish > completions/opencc-cli.elv
	@echo "Shell completions generated successfully in the 'completions/' directory."
