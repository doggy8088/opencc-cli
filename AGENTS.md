# Agent Onboarding & Developer Guide (AGENTS.md)

Welcome! This document provides information on the architecture, development workflows, testing strategy, and release process of the `opencc-cli` project. If you are an AI assistant or human developer onboarding to this project, read this guide to understand how to contribute.

---

## 1. Project Overview & Architecture

`opencc-cli` is a cross-platform command-line tool written in Rust. It wraps the [`opencc-rust`](https://github.com/doggy8088/opencc-rust) library to perform rapid, memory-safe Chinese Simplified/Traditional character and phrase conversions.

### High-Level Components
- **CLI Parsing (`clap`)**: Uses `clap` (v4) with standard features. Restricts the `--from` (`-f`) and `--to` (`-t`) arguments to valid, built-in OpenCC locales via strict `value_parser` arrays.
- **Conversion Engine**: Depends on the external pure-Rust port `opencc-rust` via a Git dependency.
- **Stream Processing**: Uses `BufReader` and `BufWriter` around inputs and outputs. This allows efficient conversion of massive text files and unbounded streams line-by-line without high memory consumption.
- **Shell Completions (`clap_complete`)**: Implements a `completions` subcommand that outputs shell scripts for `bash`, `zsh`, `fish`, `powershell`, and `elvish` directly to standard output.

---

## 2. Directory Structure

```
/opencc-cli/
├── Cargo.toml            # Rust manifest with packaging metadata & release profiles
├── dist-workspace.toml   # Configurations for 'cargo-dist'
├── LICENSE               # MIT license file (Copyright Will 保哥)
├── Makefile              # Automation of common tasks (build, test, lint, format)
├── README.md             # Traditional Chinese (zh-tw) user guide
├── src/
│   └── main.rs           # Core CLI execution entry point
├── tests/
│   └── e2e.rs            # Category-Partition & Boundary value E2E test suite (51 tests)
└── .github/
    └── workflows/
        └── release.yml   # CI/CD release pipeline (managed automatically by cargo-dist)
```

---

## 3. Core Development Workflows

We use a `Makefile` to simplify common commands.

### Common Commands

- **Show Help Info**:
  ```bash
  make help
  ```
- **Build Development Binary**:
  ```bash
  make build
  ```
- **Run the E2E Test Suite**:
  ```bash
  make test
  ```
- **Format Code & Lint**:
  ```bash
  make fmt          # Formats all Rust files
  make fmt-check    # Checks formatting (dry run)
  make clippy       # Runs Clippy (treats all lints/warnings as errors)
  ```
- **Install Locally**:
  ```bash
  make install      # Installs/overwrites 'opencc-cli' to your local cargo bin path (~/.cargo/bin)
  ```
- **Generate Completions locally**:
  ```bash
  make completions  # Generates completions under 'completions/' (ignored by git)
  ```

---

## 4. Testing & Verification

The testing harness in `tests/e2e.rs` consists of **51 integration tests** spanning four distinct tiers:

1. **Tier 1: Feature Coverage**:
   Happy path validations for stdin/stdout, files, default conversion configurations, and completions subcommand.
2. **Tier 2: Boundary & Corner Cases**:
   Empty streams, read-only file handlers, non-existent directories, unknown locale parameters, and extreme input length validations.
3. **Tier 3: Feature Combinations**:
   Pairwise combinations of stream piping with explicit and implicit flags.
4. **Tier 4: Real-World Workloads**:
   Multiline Wikipedia extracts, syntax code comment blocks, diff patches, and piped round-trip conversions.

> [!NOTE]
> Always run `make test` before pushing any changes. All 51 tests must pass.

---

## 5. Distribution and Releases

`opencc-cli` is released for macOS (ARM/Intel), Linux (ARM/Intel), and Windows via **`cargo-dist`** and wraps the release binaries into an automated **npm package installer**.

### Release Flow

1. **Local Configuration**:
   - Release profiles and build metadata are in `Cargo.toml`.
   - `cargo-dist` settings are maintained in `dist-workspace.toml`.
2. **CI/CD Build & Publish**:
   - When a Git tag matching `v*` (e.g. `v0.1.0`) is pushed, GitHub Actions automatically executes the workflow defined in `.github/workflows/release.yml`.
   - It builds native binaries for the defined target triples, generates `.tar.xz`/`.zip` archives, creates a GitHub Release, and publishes the npm installer package to the registry.
3. **npm Post-install Mechanic**:
   - The generated npm package acts as a lightweight wrapper. Upon `npm install -g opencc-cli`, a post-install script downloads the correct precompiled native binary for the user's OS and CPU from the GitHub Release attachments.
