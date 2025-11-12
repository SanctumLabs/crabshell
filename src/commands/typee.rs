use super::{Command, CommandError};

pub fn type_cmd(command: &str) {
    use Command::*;
    if command.is_empty() {
        eprint!("");
        return;
    }
    match Command::from(command) {
        Ok(External { path, .. }) => {
            println!("{} is {}", command, path);
        }
        Ok(_) => {
            println!("{} is a shell builtin", command);
        }
        Err(CommandError::NotFound(..)) => {
            eprintln!("{}: not found", command)
        }
    };
}

pub fn parse_type_cmd(args: &str) -> Result<Command, CommandError> {
    let args = args.trim();
    Ok(Command::Type(args.to_owned()))
}
