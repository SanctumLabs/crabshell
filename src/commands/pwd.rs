use super::{error::CommandError, Command};

pub(crate) fn pwd_cmd() {
    use std::env;
    let dir = env::current_dir().expect("failed to get current directory");
    println!("{}", dir.display());
}

pub(crate) fn parse_pwd_cmd() -> Result<Command, CommandError> {
    Ok(Command::Pwd)
}
