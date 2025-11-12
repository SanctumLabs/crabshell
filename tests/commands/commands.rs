// Integration tests for crabshell command parsing and utils.

use crabshell::commands::Command;
use crabshell::commands::{cd, echo, exit, typee};
use crabshell::commands::error::CommandError;
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
    assert!(matches!(exit::parse_exit_cmd("notanumber"), Ok(Command::Exit(1))));
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
        _ => panic!("Expected NotFound"),
    }
}