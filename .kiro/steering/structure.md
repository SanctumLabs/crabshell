# Project Structure

## Directory Layout

```
crabshell/
├── src/
│   ├── main.rs           # Entry point with REPL loop
│   ├── lib.rs            # Library root exposing public modules
│   ├── commands/         # Command implementations
│   │   ├── mod.rs        # Command enum and parsing logic
│   │   ├── error.rs      # Command error types
│   │   ├── echo.rs       # Echo command
│   │   ├── exit.rs       # Exit command
│   │   ├── typee.rs      # Type command
│   │   ├── pwd.rs        # Pwd command
│   │   ├── cd.rs         # Cd command
│   │   └── external.rs   # External command execution
│   └── utils/            # Utility modules
│       ├── mod.rs        # Utils module root
│       ├── logger.rs     # Logging setup
│       ├── path_utils.rs # PATH resolution utilities
│       └── string_utils.rs # String parsing utilities
├── tests/                # Integration tests
│   ├── commands_tests.rs
│   ├── utils_tests.rs
│   ├── commands/
│   └── utils/
└── docs/                 # Documentation
```

## Architecture Patterns

### Command Pattern
- All commands implement the `Command` enum in `src/commands/mod.rs`
- Each command has its own module with:
  - `parse_*_cmd()` - Parses arguments and returns `Result<Command, CommandError>`
  - `*_cmd()` - Executes the command logic
- Commands are executed via `Command::execute()` method

### Module Organization
- `commands/` - Each built-in command gets its own file
- `utils/` - Shared utilities for logging, path resolution, and string parsing
- Public API exposed through `lib.rs`

### Error Handling
- Custom error types using `thiserror` in `commands/error.rs`
- `anyhow` for general error handling
- `color-eyre` for enhanced error reporting

### Code Conventions
- Use descriptive function names with `_cmd` suffix for command implementations
- Document public functions with doc comments (`///`)
- Keep command parsing separate from execution logic
- Use `Result<T, CommandError>` for fallible operations
