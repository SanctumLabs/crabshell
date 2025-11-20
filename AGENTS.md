# AGENTS GUIDE

This document is for all human or AI agents collaborating on this Rust project. It explains what the project is, how to build and test it, how we write code, how we test it, and the security posture we expect contributors to maintain.


## 1. Project Overview

- Name: crabshell
- Purpose: A minimal Unix-like shell implemented in Rust. It reads commands from stdin, parses them, runs built-ins, or spawns external processes.
- Minimum Supported Rust Version (MSRV): 1.80 (per Cargo.toml). CI/buildpack may use Rust 1.88; using latest stable (>= 1.88) locally is recommended.
- Edition: 2021

### Key Features
- Built-ins: echo, exit, pwd, cd, type
- External commands: Resolved via PATH and executed using std::process::Command
- Tokenization: Shell-like tokenization for quotes and simple escapes (utils::string_utils)
- Logging and diagnostics: tracing + color-eyre

### High-Level Architecture
- src/main.rs
  - Initializes logging (utils::logger::setup).
  - REPL loop prints "$ ", reads a line, parses to Command, executes, and flushes stdout/stderr.
- src/commands
  - mod.rs: Defines Command enum and the dispatcher (execute and from parsing).
  - echo.rs, exit.rs, cd.rs, pwd.rs, typee.rs: Implement built-ins (parse_* + *_cmd patterns).
  - external.rs: PATH lookup and execution of external programs using std::process::Command; checks execute permission bits.
  - error.rs: CommandError for parse/dispatch errors.
- src/utils
  - string_utils.rs: tokenize() for splitting commands into arguments honoring quotes and basic escapes.
  - path_utils.rs: Helpers for PATH resolution for executables.
  - logger.rs: tracing subscriber setup and fdbg! macro for contextual debug strings.
- tests
  - tests/commands/*.rs and tests/utils/*.rs: Integration tests for parsing and utilities.

### Conventions When Adding Functionality
- Parsing vs execution:
  - Use parse_<name>_cmd(args: &str) -> Result<Command, CommandError> to parse input.
  - Implement <name>_cmd(args: &T) (or similar) for the execution path.
- Extend Command enum with a new variant when introducing a new built-in, and update Command::execute and Command::from accordingly.
- Prefer returning CommandError variants instead of printing from parsers. Printing/side-effects belong to execution.


## 2. Build and Test Commands

Pre-requisites:
- Rust toolchain (rustup) with stable rustc/cargo.
  - MSRV: 1.80 (Cargo.toml). Recommended: stable >= 1.88 to match CI buildpack.
- Unix-like environment for spawning external commands.

Common commands (Makefile shortcuts are provided):

- Format
  - make fmt
  - cargo fmt --all

- Lint (deny warnings)
  - make clippy
  - cargo clippy --all --all-targets -- -D warnings

- Build
  - make build PROFILE=dev
  - cargo build --profile dev --verbose
  - cargo build --release --verbose

- Test
  - make test PROFILE=dev
  - cargo test --verbose

- Run
  - cargo run --profile dev
  - ./your_program.sh (local run)

Environment notes:
- PATH influences external command resolution. Tests and runs may depend on your system’s PATH contents.


## 3. Code Style Guidelines

- Formatting
  - Use rustfmt. Always run make fmt or cargo fmt --all before committing.

- Linting & Static Analysis
  - Clippy is enforced with -D warnings (see Makefile). Code must be clippy-clean on all targets: cargo clippy --all --all-targets -- -D warnings.
  - Prefer smaller, readable functions; avoid needless allocations; heed clippy suggestions.

- Error Handling
  - Use anyhow for application-level error propagation and thiserror for domain-specific error enums (e.g., CommandError).
  - Avoid unwrap()/expect() outside test code or one-off program setup. Instead, return Result and bubble errors up.
  - Use color-eyre for rich error reports in main, already wired in utils::logger::setup().

- Logging & Diagnostics
  - Use tracing for logs. Favor structured logs over println!/eprintln! for non-user-facing output.
  - The fdbg! macro is available for consistent file:line context in debug strings.

- Modules & Naming
  - commands::<name>.rs provides two functions: parse_<name>_cmd and <name>_cmd when appropriate.
  - Add a Command enum variant and handle it in Command::execute and Command::from.
  - utils::string_utils::tokenize is the canonical way to split arguments; do not roll your own tokenization.

- API & Docs
  - Document public functions with concise rustdoc comments explaining behavior and edge cases.
  - Keep visibility minimal (pub(crate) where reasonable). Re-export only what needs to be public to integration tests.

- Safety
  - No unsafe code. If you must use unsafe, document the invariants exhaustively and add tests proving them.

- Style Consistency
  - Follow Rust 2021 idioms and prefer standard library traits and types over custom ones.


## 4. Testing Instructions

- Run all tests:
  - cargo test --verbose

- Run a specific test module or file:
  - cargo test --test commands
  - cargo test --test utils

- Run a specific test by name (substring match):
  - cargo test tokenize_basic_cases

- Test organization:
  - Integration tests live under tests/. They import the crate as crabshell (e.g., use crabshell::commands::Command).
  - Prefer integration tests for end-to-end parsing and command dispatch. Add focused unit tests where appropriate.

- Adding tests for a new built-in:
  1) Create tests/<area>/<feature>.rs.
  2) Test parse_<name>_cmd for success and failure cases.
  3) Add command-from input parsing tests via Command::from("<cmd> <args>").
  4) If execution has observable effects (stdout/stderr), consider capturing output or structure execution for testability.

- CI & Coverage:
  - CI runs build, lint, and tests; coverage is reported to Codecov (see badge in README).
  - Local coverage is optional; if needed, you may use cargo-tarpaulin (install separately) but it’s not required for contribution.
  - Find the CI plan in the .github/workflows folder

- Flakiness & Environment
  - Tests that rely on PATH or the presence of external commands should be written defensively. Prefer deterministic behavior and skip tests when prerequisites aren’t met.

- Add or update tests for the code you change, even if nobody asked.
- Fix any test or type errors until the whole suite is green.

## 5. Security Considerations

- Threat Model (High-Level)
  - The shell reads untrusted input from stdin and may execute programs based on PATH. Protect against surprises from malformed input and environment manipulation.

- Unsafe Code
  - The project does not use unsafe. Do not introduce unsafe without a strong, documented justification and accompanying tests.

- External Command Execution
  - Resolution: external.rs searches PATH and checks execute permission bits (0o111) before constructing a Command::External.
  - Invocation: std::process::Command is used directly (no shell like sh -c), which reduces injection risk. Continue to pass arguments as a vector (tokenize()) rather than concatenated strings.
  - Do not interpolate untrusted data into shell strings. Never invoke a shell unless absolutely necessary.

- Input Handling
  - Use utils::string_utils::tokenize for argument parsing. Be cautious when changing tokenization rules to avoid introducing injection vectors.
  - Avoid panics on malformed input. Return CommandError or Err variants and handle gracefully.

- Logging & Secrets
  - Do not log sensitive data (e.g., tokens, passwords, environment secrets). Keep logs at appropriate levels (info/debug) and avoid echoing raw user input at error level.

- Environment & PATH
  - Be aware that PATH is attacker-controlled in many contexts. Avoid assuming specific binaries. Consider canonicalizing resolved paths when needed (external.rs already canonicalizes on success).

- Dependencies & Supply Chain
  - Keep dependencies minimal and pinned via Cargo.lock. Review updates regularly.
  - Run cargo audit (install via cargo install cargo-audit) to detect known vulnerabilities before release.

- Privileges & Filesystem
  - The shell should run as a non-privileged user. Do not add features that modify sensitive files or escalate privileges.
  - When adding file operations, validate inputs, use safe path joins, and avoid TOCTOU hazards.

- Denial of Service
  - Be mindful of unbounded input growth. Where feasible, limit line lengths or handle large inputs incrementally.


## PR instructions

- Title format: <Title> <changes made>
- Always run `cargo fmt --all` and `cargo test` before committing

---

If you have questions or propose changes to these guidelines, open a GitHub issue or pull request. Aligning on these practices helps all agents collaborate effectively on this Rust project.