// Integration tests for crabshell command parsing and utils.

use crabshell::commands::error::CommandError;
use crabshell::commands::Command;
use crabshell::commands::{cd, echo, exit, typee};
use crabshell::utils::string_utils::tokenize;

#[test]
fn test_tokenize_basic() {
    assert_eq!(tokenize("foo bar"), vec!["foo", "bar"]);
    assert_eq!(tokenize("a  b\tc"), vec!["a", "b", "c"]);
    assert_eq!(tokenize("'a b' c"), vec!["a b", "c"]);
    assert_eq!(tokenize(r#""a b" c"#), vec!["a b", "c"]);
    assert_eq!(tokenize(r#"foo\ bar baz"#), vec!["foo bar", "baz"]);
    assert_eq!(tokenize(r#"foo"bar"baz"#), vec!["foobarbaz"]);
    assert_eq!(tokenize(r#"foo 'bar' baz"#), vec!["foo", "bar", "baz"]);
}

#[test]
fn test_parse_echo_cmd() {
    let parsed = echo::parse_echo_cmd("hello world").unwrap();
    match parsed {
        Command::Echo(s) => assert_eq!(s, "hello world"),
        _ => panic!("Wrong command variant"),
    }
    let with_quotes = echo::parse_echo_cmd("'hello  world' a  b").unwrap();
    match with_quotes {
        Command::Echo(s) => assert_eq!(s, "hello  world a b"),
        _ => panic!("Wrong command variant"),
    }
}

#[test]
fn test_parse_cd_cmd() {
    let parsed = cd::parse_cd_cmd("/tmp").unwrap();
    match parsed {
        Command::Cd(s) => assert_eq!(s, "/tmp"),
        _ => panic!("Wrong command variant"),
    }
}

#[test]
fn test_parse_exit_cmd() {
    assert!(matches!(exit::parse_exit_cmd(""), Ok(Command::Exit(0))));
    assert!(matches!(exit::parse_exit_cmd("1"), Ok(Command::Exit(1))));
    assert!(matches!(
        exit::parse_exit_cmd("notanumber"),
        Ok(Command::Exit(1))
    ));
}

#[test]
fn test_parse_type_cmd() {
    let parsed = typee::parse_type_cmd("echo").unwrap();
    match parsed {
        Command::Type(cmd) => assert_eq!(cmd, "echo"),
        _ => panic!("Wrong command variant"),
    }
}

#[test]
fn test_command_from_builtin_echo() {
    let c = Command::from("echo test").unwrap();
    match c {
        Command::Echo(s) => assert_eq!(s, "test"),
        _ => panic!("Wrong command variant"),
    }
}

#[test]
fn test_command_from_builtin_exit() {
    let c = Command::from("exit 0").unwrap();
    assert!(matches!(c, Command::Exit(0)));
}

#[test]
fn test_command_from_empty() {
    let c = Command::from("").unwrap();
    assert!(matches!(c, Command::Noop));
}

#[test]
fn test_command_not_found() {
    let err = Command::from("doesnotexist").err().unwrap();
    match err {
        CommandError::NotFound(s) => assert_eq!(s, "doesnotexist"),
    }
}

// Unit tests for quoted executable names with spaces
#[test]
fn test_single_quoted_executable_with_spaces() {
    let result = Command::from("'exe with spaces' arg1");
    match result {
        Ok(Command::External { cmd, args, .. }) => {
            assert_eq!(cmd, "exe with spaces");
            assert_eq!(args, "arg1");
        }
        Err(CommandError::NotFound(cmd)) => {
            assert_eq!(cmd, "exe with spaces");
        }
        _ => panic!("Expected External command or NotFound error"),
    }
}

#[test]
fn test_double_quoted_executable_with_spaces() {
    let result = Command::from("\"exe with spaces\" arg1");
    match result {
        Ok(Command::External { cmd, args, .. }) => {
            assert_eq!(cmd, "exe with spaces");
            assert_eq!(args, "arg1");
        }
        Err(CommandError::NotFound(cmd)) => {
            assert_eq!(cmd, "exe with spaces");
        }
        _ => panic!("Expected External command or NotFound error"),
    }
}

#[test]
fn test_escaped_spaces_executable() {
    let result = Command::from("exe\\ with\\ spaces arg1");
    match result {
        Ok(Command::External { cmd, args, .. }) => {
            assert_eq!(cmd, "exe with spaces");
            assert_eq!(args, "arg1");
        }
        Err(CommandError::NotFound(cmd)) => {
            assert_eq!(cmd, "exe with spaces");
        }
        _ => panic!("Expected External command or NotFound error"),
    }
}

// Unit tests for quoted executable names with quotes
#[test]
fn test_single_quoted_executable_with_double_quotes() {
    let result = Command::from("'exe with \"quotes\"' arg1");
    match result {
        Ok(Command::External { cmd, args, .. }) => {
            assert_eq!(cmd, "exe with \"quotes\"");
            assert_eq!(args, "arg1");
        }
        Err(CommandError::NotFound(cmd)) => {
            assert_eq!(cmd, "exe with \"quotes\"");
        }
        _ => panic!("Expected External command or NotFound error"),
    }
}

#[test]
fn test_double_quoted_executable_with_single_quotes() {
    let result = Command::from("\"exe with 'quotes'\" arg1");
    match result {
        Ok(Command::External { cmd, args, .. }) => {
            assert_eq!(cmd, "exe with 'quotes'");
            assert_eq!(args, "arg1");
        }
        Err(CommandError::NotFound(cmd)) => {
            assert_eq!(cmd, "exe with 'quotes'");
        }
        _ => panic!("Expected External command or NotFound error"),
    }
}

#[test]
fn test_escaped_quotes_executable() {
    let result = Command::from("exe\\ with\\ \\\"quotes\\\" arg1");
    match result {
        Ok(Command::External { cmd, args, .. }) => {
            assert_eq!(cmd, "exe with \"quotes\"");
            assert_eq!(args, "arg1");
        }
        Err(CommandError::NotFound(cmd)) => {
            assert_eq!(cmd, "exe with \"quotes\"");
        }
        _ => panic!("Expected External command or NotFound error"),
    }
}

// Unit tests for quoted executable names with backslashes
#[test]
fn test_quoted_executable_with_backslashes() {
    let result = Command::from("'exe\\with\\backslash' arg1");
    match result {
        Ok(Command::External { cmd, args, .. }) => {
            assert_eq!(cmd, "exe\\with\\backslash");
            assert_eq!(args, "arg1");
        }
        Err(CommandError::NotFound(cmd)) => {
            assert_eq!(cmd, "exe\\with\\backslash");
        }
        _ => panic!("Expected External command or NotFound error"),
    }
}

#[test]
fn test_escaped_backslashes_executable() {
    let result = Command::from("exe\\\\with\\\\backslash arg1");
    match result {
        Ok(Command::External { cmd, args, .. }) => {
            assert_eq!(cmd, "exe\\with\\backslash");
            assert_eq!(args, "arg1");
        }
        Err(CommandError::NotFound(cmd)) => {
            assert_eq!(cmd, "exe\\with\\backslash");
        }
        _ => panic!("Expected External command or NotFound error"),
    }
}

// Unit tests for quoted executables with arguments (Task 3.1)
#[test]
fn test_quoted_executable_with_unquoted_args() {
    let result = Command::from("'my exe' arg1 arg2");
    match result {
        Ok(Command::External { cmd, args, .. }) => {
            assert_eq!(cmd, "my exe");
            assert_eq!(args, "arg1 arg2");
        }
        Err(CommandError::NotFound(cmd)) => {
            assert_eq!(cmd, "my exe");
        }
        _ => panic!("Expected External command or NotFound error"),
    }
}

#[test]
fn test_quoted_executable_with_quoted_args() {
    let result = Command::from("'my exe' 'arg 1' \"arg 2\"");
    match result {
        Ok(Command::External { cmd, args, .. }) => {
            assert_eq!(cmd, "my exe");
            assert_eq!(args, "arg 1 arg 2");
        }
        Err(CommandError::NotFound(cmd)) => {
            assert_eq!(cmd, "my exe");
        }
        _ => panic!("Expected External command or NotFound error"),
    }
}

#[test]
fn test_mixed_quoting_executable_and_args() {
    let result = Command::from("\"my exe\" arg1 'arg 2' arg3");
    match result {
        Ok(Command::External { cmd, args, .. }) => {
            assert_eq!(cmd, "my exe");
            assert_eq!(args, "arg1 arg 2 arg3");
        }
        Err(CommandError::NotFound(cmd)) => {
            assert_eq!(cmd, "my exe");
        }
        _ => panic!("Expected External command or NotFound error"),
    }
}

// Backward compatibility tests (Task 4.2)
#[test]
fn test_backward_compat_echo_simple() {
    let result = Command::from("echo hello");
    match result {
        Ok(Command::Echo(s)) => assert_eq!(s, "hello"),
        _ => panic!("Expected Echo command"),
    }
}

#[test]
fn test_backward_compat_echo_multiple_args() {
    let result = Command::from("echo hello world");
    match result {
        Ok(Command::Echo(s)) => assert_eq!(s, "hello world"),
        _ => panic!("Expected Echo command"),
    }
}

#[test]
fn test_backward_compat_exit_no_args() {
    let result = Command::from("exit");
    assert!(matches!(result, Ok(Command::Exit(0))));
}

#[test]
fn test_backward_compat_exit_with_code() {
    let result = Command::from("exit 42");
    assert!(matches!(result, Ok(Command::Exit(42))));
}

#[test]
fn test_backward_compat_type_builtin() {
    let result = Command::from("type echo");
    match result {
        Ok(Command::Type(s)) => assert_eq!(s, "echo"),
        _ => panic!("Expected Type command"),
    }
}

#[test]
fn test_backward_compat_pwd() {
    let result = Command::from("pwd");
    assert!(matches!(result, Ok(Command::Pwd)));
}

#[test]
fn test_backward_compat_cd_path() {
    let result = Command::from("cd /tmp");
    match result {
        Ok(Command::Cd(s)) => assert_eq!(s, "/tmp"),
        _ => panic!("Expected Cd command"),
    }
}

#[test]
fn test_backward_compat_cd_home() {
    let result = Command::from("cd ~");
    match result {
        Ok(Command::Cd(s)) => assert_eq!(s, "~"),
        _ => panic!("Expected Cd command"),
    }
}

#[test]
fn test_backward_compat_external_simple() {
    let result = Command::from("ls");
    match result {
        Ok(Command::External { cmd, args, .. }) => {
            assert_eq!(cmd, "ls");
            assert_eq!(args, "");
        }
        Err(CommandError::NotFound(cmd)) => {
            assert_eq!(cmd, "ls");
        }
        _ => panic!("Expected External command or NotFound error"),
    }
}

#[test]
fn test_backward_compat_external_with_args() {
    let result = Command::from("ls -la /tmp");
    match result {
        Ok(Command::External { cmd, args, .. }) => {
            assert_eq!(cmd, "ls");
            assert_eq!(args, "-la /tmp");
        }
        Err(CommandError::NotFound(cmd)) => {
            assert_eq!(cmd, "ls");
        }
        _ => panic!("Expected External command or NotFound error"),
    }
}

#[test]
fn test_backward_compat_args_with_quotes() {
    let result = Command::from("echo 'hello world'");
    match result {
        Ok(Command::Echo(s)) => assert_eq!(s, "hello world"),
        _ => panic!("Expected Echo command"),
    }
}

#[test]
fn test_backward_compat_args_with_double_quotes() {
    let result = Command::from("echo \"hello world\"");
    match result {
        Ok(Command::Echo(s)) => assert_eq!(s, "hello world"),
        _ => panic!("Expected Echo command"),
    }
}

#[test]
fn test_backward_compat_args_with_escaped_spaces() {
    let result = Command::from("echo hello\\ world");
    match result {
        Ok(Command::Echo(s)) => assert_eq!(s, "hello world"),
        _ => panic!("Expected Echo command"),
    }
}

#[test]
fn test_backward_compat_mixed_args() {
    let result = Command::from("echo 'arg1' arg2 \"arg3\"");
    match result {
        Ok(Command::Echo(s)) => assert_eq!(s, "arg1 arg2 arg3"),
        _ => panic!("Expected Echo command"),
    }
}

// Property-based tests
use proptest::prelude::*;

// Feature: quotable-executables, Property 5: First token is treated as executable name
// Validates: Requirements 5.2
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    
    #[test]
    fn prop_first_token_is_executable(
        cmd_name in "[a-zA-Z][a-zA-Z0-9_-]{0,20}",
        args in prop::collection::vec("[a-zA-Z0-9_-]+", 0..5)
    ) {
        // Build a command string with the command name and arguments
        let mut input = cmd_name.clone();
        if !args.is_empty() {
            input.push(' ');
            input.push_str(&args.join(" "));
        }
        
        // Parse the command
        let result = Command::from(&input);
        
        // The command should either:
        // 1. Be recognized as a built-in (echo, exit, type, pwd, cd)
        // 2. Result in NotFound error with the correct command name
        match result {
            Ok(Command::Echo(_)) => assert_eq!(cmd_name, "echo"),
            Ok(Command::Exit(_)) => assert_eq!(cmd_name, "exit"),
            Ok(Command::Type(_)) => assert_eq!(cmd_name, "type"),
            Ok(Command::Pwd) => assert_eq!(cmd_name, "pwd"),
            Ok(Command::Cd(_)) => assert_eq!(cmd_name, "cd"),
            Ok(Command::External { cmd, .. }) => assert_eq!(cmd, cmd_name),
            Ok(Command::Noop) => panic!("Should not be Noop for non-empty input"),
            Err(CommandError::NotFound(not_found_cmd)) => assert_eq!(not_found_cmd, cmd_name),
        }
    }
}

// Feature: quotable-executables, Property 1: Executable names with spaces are parsed correctly
// Validates: Requirements 1.1, 1.2, 1.3
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    
    #[test]
    fn prop_executable_with_spaces_parsed_correctly(
        // Generate executable names with spaces
        word1 in "[a-zA-Z][a-zA-Z0-9_-]{0,10}",
        word2 in "[a-zA-Z][a-zA-Z0-9_-]{0,10}",
        word3 in "[a-zA-Z][a-zA-Z0-9_-]{0,10}",
        // Generate arguments
        args in prop::collection::vec("[a-zA-Z0-9_-]+", 0..3),
        // Choose quoting style: 0 = single quotes, 1 = double quotes, 2 = escaped spaces
        quote_style in 0..3u8
    ) {
        let exe_name = format!("{} {} {}", word1, word2, word3);
        
        // Build the command string with different quoting styles
        let input = match quote_style {
            0 => format!("'{}' {}", exe_name, args.join(" ")),  // Single quotes
            1 => format!("\"{}\" {}", exe_name, args.join(" ")), // Double quotes
            _ => format!("{} {}", exe_name.replace(' ', "\\ "), args.join(" ")), // Escaped spaces
        };
        
        // Parse the command
        let result = Command::from(input.trim());
        
        // The executable name should be parsed correctly (with spaces preserved)
        match result {
            Ok(Command::External { cmd, args: parsed_args, .. }) => {
                assert_eq!(cmd, exe_name);
                assert_eq!(parsed_args, args.join(" "));
            }
            Err(CommandError::NotFound(not_found_cmd)) => {
                assert_eq!(not_found_cmd, exe_name);
            }
            _ => panic!("Expected External command or NotFound error for executable with spaces"),
        }
    }
}

// Feature: quotable-executables, Property 2: Executable names with quotes are parsed correctly
// Validates: Requirements 2.1, 2.2, 2.3
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    
    #[test]
    fn prop_executable_with_quotes_parsed_correctly(
        // Generate executable names with quotes
        word1 in "[a-zA-Z][a-zA-Z0-9_-]{0,8}",
        word2 in "[a-zA-Z][a-zA-Z0-9_-]{0,8}",
        // Generate arguments
        args in prop::collection::vec("[a-zA-Z0-9_-]+", 0..3),
        // Choose quoting style: 0 = single quotes with double quotes inside, 1 = double quotes with single quotes inside
        quote_style in 0..2u8
    ) {
        // Build executable name with quotes inside
        let (exe_name, input) = match quote_style {
            0 => {
                // Single quotes with double quotes inside
                let exe_name = format!("{} \"{}\"", word1, word2);
                let input = format!("'{}' {}", exe_name, args.join(" "));
                (exe_name, input)
            }
            _ => {
                // Double quotes with single quotes inside
                let exe_name = format!("{} '{}'", word1, word2);
                let input = format!("\"{}\" {}", exe_name, args.join(" "));
                (exe_name, input)
            }
        };
        
        // Parse the command
        let result = Command::from(input.trim());
        
        // The executable name should be parsed correctly (with quotes preserved as literals)
        match result {
            Ok(Command::External { cmd, args: parsed_args, .. }) => {
                assert_eq!(cmd, exe_name);
                assert_eq!(parsed_args, args.join(" "));
            }
            Err(CommandError::NotFound(not_found_cmd)) => {
                assert_eq!(not_found_cmd, exe_name);
            }
            _ => panic!("Expected External command or NotFound error for executable with quotes"),
        }
    }
}

// Feature: quotable-executables, Property 3: Executable names with backslashes are parsed correctly
// Validates: Requirements 3.1, 3.2
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    
    #[test]
    fn prop_executable_with_backslashes_parsed_correctly(
        // Generate executable names with backslashes
        word1 in "[a-zA-Z][a-zA-Z0-9_-]{0,8}",
        word2 in "[a-zA-Z][a-zA-Z0-9_-]{0,8}",
        word3 in "[a-zA-Z][a-zA-Z0-9_-]{0,8}",
        // Generate arguments
        args in prop::collection::vec("[a-zA-Z0-9_-]+", 0..3),
        // Choose quoting style: 0 = single quotes, 1 = escaped backslashes
        quote_style in 0..2u8
    ) {
        // Build executable name with backslashes
        let exe_name = format!("{}\\{}\\{}", word1, word2, word3);
        
        // Build the command string with different quoting styles
        let input = match quote_style {
            0 => {
                // Single quotes preserve backslashes literally
                format!("'{}' {}", exe_name, args.join(" "))
            }
            _ => {
                // Escaped backslashes (double backslash becomes single backslash)
                let escaped_exe = exe_name.replace('\\', "\\\\");
                format!("{} {}", escaped_exe, args.join(" "))
            }
        };
        
        // Parse the command
        let result = Command::from(input.trim());
        
        // The executable name should be parsed correctly (with backslashes preserved)
        match result {
            Ok(Command::External { cmd, args: parsed_args, .. }) => {
                assert_eq!(cmd, exe_name);
                assert_eq!(parsed_args, args.join(" "));
            }
            Err(CommandError::NotFound(not_found_cmd)) => {
                assert_eq!(not_found_cmd, exe_name);
            }
            _ => panic!("Expected External command or NotFound error for executable with backslashes"),
        }
    }
}

// Feature: quotable-executables, Property 4: Arguments are separated from executable names
// Validates: Requirements 4.1, 4.2
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    
    #[test]
    fn prop_arguments_separated_from_executable(
        // Generate executable name with spaces
        exe_word1 in "[a-zA-Z][a-zA-Z0-9_-]{0,8}",
        exe_word2 in "[a-zA-Z][a-zA-Z0-9_-]{0,8}",
        // Generate arguments (some with spaces, some without)
        arg1 in "[a-zA-Z][a-zA-Z0-9_-]{0,8}",
        arg2_word1 in "[a-zA-Z][a-zA-Z0-9_-]{0,6}",
        arg2_word2 in "[a-zA-Z][a-zA-Z0-9_-]{0,6}",
        arg3 in "[a-zA-Z][a-zA-Z0-9_-]{0,8}",
        // Choose quoting styles for executable and arguments
        exe_quote_style in 0..2u8,  // 0 = single quotes, 1 = double quotes
        arg2_quote_style in 0..2u8, // 0 = single quotes, 1 = double quotes
    ) {
        // Build executable name with spaces
        let exe_name = format!("{} {}", exe_word1, exe_word2);
        
        // Build argument with spaces
        let arg2 = format!("{} {}", arg2_word1, arg2_word2);
        
        // Build the command string with quoted executable and mixed arguments
        let quoted_exe = match exe_quote_style {
            0 => format!("'{}'", exe_name),
            _ => format!("\"{}\"", exe_name),
        };
        
        let quoted_arg2 = match arg2_quote_style {
            0 => format!("'{}'", arg2),
            _ => format!("\"{}\"", arg2),
        };
        
        let input = format!("{} {} {} {}", quoted_exe, arg1, quoted_arg2, arg3);
        
        // Parse the command
        let result = Command::from(input.trim());
        
        // The executable name and arguments should be separated correctly
        match result {
            Ok(Command::External { cmd, args: parsed_args, .. }) => {
                // Executable should be parsed correctly
                assert_eq!(cmd, exe_name);
                
                // Arguments should be separated and parsed correctly
                let expected_args = format!("{} {} {}", arg1, arg2, arg3);
                assert_eq!(parsed_args, expected_args);
            }
            Err(CommandError::NotFound(not_found_cmd)) => {
                // Even if not found, the executable name should be parsed correctly
                assert_eq!(not_found_cmd, exe_name);
            }
            _ => panic!("Expected External command or NotFound error for quoted executable with arguments"),
        }
    }
}
