# Technology Stack

## Language & Tooling

- **Language**: Rust (edition 2021, minimum version 1.80)
- **Build System**: Cargo
- **Package Manager**: Cargo

## Dependencies

- `anyhow` (1.0.68) - Error handling
- `thiserror` (1.0.38) - Custom error types
- `tracing` (0.1.41) - Logging framework
- `tracing-subscriber` (0.3.20) - Logging subscriber
- `color-eyre` (0.6.3) - Enhanced error reporting
- `bytes` (1.3.0) - Buffer management

## Common Commands

### Building
```bash
# Development build
make build
# or
cargo build

# Production build
make build PROFILE=release
# or
cargo build --release
```

### Testing
```bash
# Run tests
make test
# or
cargo test --verbose

# Run tests with release profile
make test PROFILE=release
```

### Linting
```bash
# Run clippy
make clippy
# or
cargo clippy --all --all-targets -- -D warnings
```

### Formatting
```bash
# Format code
make fmt
# or
cargo fmt --all
```

### Running
```bash
# Run the shell
./your_program.sh
# or
cargo run
```

## Code Quality

- Clippy is configured with strict warnings (`-D warnings`)
- Code formatting enforced via `cargo fmt`
- Clippy configuration in `clippy.toml`
