use crate::utils::string_utils::tokenize;
use super::{Command, CommandError};

pub(crate) fn echo_cmd(args: &str) {
    println!("{}", args);
}

pub(crate) fn parse_echo_cmd(args: &str) -> Result<Command, CommandError> {
    // Use tokenizer to handle single quotes and collapse unquoted whitespace.
    let tokens = tokenize(args);
    let processed = tokens.join(" ");
    Ok(Command::Echo(processed))
}
