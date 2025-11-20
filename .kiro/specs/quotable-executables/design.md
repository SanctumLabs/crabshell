# Design Document: Quotable Executables

## Overview

This design refactors the command parsing logic in CrabShell to properly handle executable names containing special characters (spaces, quotes, backslashes). The solution leverages the existing `tokenize` function to parse the entire command line into tokens, treating the first token as the executable name and subsequent tokens as arguments.

## Architecture

The change is localized to the `Command::from()` method in `src/commands/mod.rs`. Instead of using a simple string split, we'll use the existing tokenizer to parse the input, which already handles quotes and escapes correctly.

### Current Flow
```
Input: "'exe with spaces' arg1 arg2"
  ↓
splitn(2, ' ') → cmd="'exe", args="with spaces' arg1 arg2"
  ↓
Command parsing fails (quotes not handled)
```

### New Flow
```
Input: "'exe with spaces' arg1 arg2"
  ↓
tokenize() → ["exe with spaces", "arg1", "arg2"]
  ↓
first token = executable name, rest = arguments
  ↓
Command parsing succeeds
```

## Components and Interfaces

### Modified Component: `Command::from()`

**Location:** `src/commands/mod.rs`

**Changes:**
1. Import the `tokenize` function from `utils::string_utils`
2. Replace the `splitn(2, ' ')` logic with tokenization
3. Extract the first token as the command name
4. Join remaining tokens back into an arguments string (for compatibility with existing command parsers)

**Signature:** No change to the public interface
```rust
pub fn from(input: &str) -> Result<Self, CommandError>
```

### Affected Components

**`external::parse_external_cmd()`**: No changes needed - already receives cmd and args as separate strings

**`external::external_cmd()`**: No changes needed - already uses tokenize for argument parsing

**Built-in command parsers**: No changes needed - continue to receive args as strings

## Data Models

No new data structures required. The existing `Command` enum and its variants remain unchanged.

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system-essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*


### Property Reflection

After analyzing all acceptance criteria, several properties can be consolidated:
- Properties 1.1, 1.2, 1.3 test space handling with different quoting mechanisms → combine into Property 1
- Properties 2.1, 2.2, 2.3 test quote handling with different quoting mechanisms → combine into Property 2
- Properties 3.1, 3.2 test backslash handling → combine into Property 3
- Properties 4.1, 4.2 test argument separation, where 4.2 is more comprehensive → use Property 4
- Property 4.3 tests tokenizer concatenation (already tested in tokenizer tests) → skip
- Property 5.2 is the fundamental parsing property → Property 5

### Property 1: Executable names with spaces are parsed correctly

*For any* string containing spaces, when wrapped in single quotes, double quotes, or with escaped spaces, tokenizing the command line should produce that string (without quotes/escapes) as the first token.

**Validates: Requirements 1.1, 1.2, 1.3**

### Property 2: Executable names with quotes are parsed correctly

*For any* string containing single or double quotes, when properly quoted or escaped, tokenizing the command line should produce that string (with quotes preserved as literals) as the first token.

**Validates: Requirements 2.1, 2.2, 2.3**

### Property 3: Executable names with backslashes are parsed correctly

*For any* string containing backslashes, when properly quoted or escaped, tokenizing the command line should produce that string (with backslashes handled according to quoting rules) as the first token.

**Validates: Requirements 3.1, 3.2**

### Property 4: Arguments are separated from executable names

*For any* quoted executable name and any set of arguments (quoted or unquoted), tokenizing the command line should produce the executable as the first token and each argument as subsequent separate tokens.

**Validates: Requirements 4.1, 4.2**

### Property 5: First token is treated as executable name

*For any* tokenized command input with at least one token, the Command parser should treat the first token as the executable name and all remaining tokens as arguments.

**Validates: Requirements 5.2**

## Error Handling

No changes to error handling are required. The existing error handling for:
- Unknown commands (`CommandError::NotFound`)
- PATH lookup failures
- Execution errors

...will continue to work as before. The tokenizer already handles malformed quotes and escapes gracefully.

## Testing Strategy

### Unit Tests

We will add unit tests to verify:
1. Command parsing with quoted executable names containing spaces
2. Command parsing with quoted executable names containing quotes
3. Command parsing with quoted executable names containing backslashes
4. Command parsing with quoted executables and quoted arguments
5. Backward compatibility with existing command parsing

### Property-Based Tests

We will use the `proptest` crate for property-based testing. Each property-based test will:
- Run a minimum of 100 iterations
- Generate random strings with special characters
- Verify the parsing behavior matches the correctness properties
- Be tagged with comments referencing the design document properties

**Property-based testing library:** `proptest` (Rust standard for PBT)

**Test configuration:**
```rust
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    // test implementation
}
```

**Test tagging format:**
```rust
// Feature: quotable-executables, Property 1: Executable names with spaces are parsed correctly
```

### Integration Tests

Integration tests will verify end-to-end behavior:
1. Create test executables with special characters in their names
2. Execute them through the shell
3. Verify correct execution and output

## Implementation Notes

### Compatibility Considerations

The change maintains backward compatibility because:
1. The tokenizer already handles unquoted strings correctly
2. Existing commands without special characters will parse identically
3. The arguments string reconstruction preserves the interface for built-in command parsers

### Performance Considerations

The tokenizer is already used for argument parsing in external commands, so there's no performance regression. The change actually simplifies the code by using a single consistent parsing approach.

### Edge Cases

1. **Empty input**: Already handled by existing code
2. **Whitespace-only input**: Tokenizer returns empty vector, handled by existing code
3. **Unclosed quotes**: Tokenizer handles gracefully (treats rest of input as quoted)
4. **Trailing backslashes**: Tokenizer handles according to quoting rules
