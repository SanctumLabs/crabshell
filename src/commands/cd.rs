use super::{Command, CommandError};

pub fn cd_cmd(args: &str) {
    use std::env::{set_current_dir, var};
    let args = match args {
        "~" => var("HOME").unwrap_or("/".into()),
        _ => args.into(),
    };
    match set_current_dir(&args) {
        Ok(_) => (),
        Err(_) => eprintln!("cd: {}: No such file or directory", &args),
    }
}

pub fn parse_cd_cmd(args: &str) -> Result<Command, CommandError> {
    Ok(Command::Cd(args.to_owned()))
}
