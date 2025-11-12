use super::{Command, CommandError};
use crate::utils::string_utils::tokenize;

/// Executes the echo command.
/// This function prints its argument string to stdout, as in the shell.
///
/// # Arguments
///
/// * `args` - The string to be printed to standard output.
pub fn echo_cmd(args: &str) {
    println!("{}", args);
}

/// Parses arguments for the echo command and returns a `Command::Echo`.
/// The tokenizer handles quoted and unquoted segments and collapses unquoted whitespace.
///
/// # Arguments
///
/// * `args` - The string arguments for the echo command.
///
/// # Returns
///
/// * `Ok(Command::Echo(processed_string))` where processed_string has quotes handled and whitespace collapsed.
/// * `Err(CommandError)` if parsing fails (should not occur for echo).
pub fn parse_echo_cmd(args: &str) -> Result<Command, CommandError> {
    // Use tokenizer to handle single quotes and collapse unquoted whitespace.
    let tokens = tokenize(args);
    let processed = tokens.join(" ");
    Ok(Command::Echo(processed))
}
