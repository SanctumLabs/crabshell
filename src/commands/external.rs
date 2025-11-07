use super::Command::{self, *};
use std::{io, io::Write, process::Command as StdCommand};

pub(crate) fn external_cmd(cmd: &Command) {
    let External { args, path, .. } = cmd else {
        eprintln!("Unexpected error occurred while executing external command");
        return;
    };
    let output = StdCommand::new(path)
        .args(args.split(' '))
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

pub(crate) fn parse_external_cmd(cmd: &str, args: &str) -> Option<Command> {
    let Ok(path_env) = std::env::var("PATH") else {
        return None;
    };
    path_env
        .split(':')
        .map(|path| {
            let full_path = format!("{}/{}", path, cmd);
            std::fs::metadata(&full_path).map(|_| full_path)
        })
        .find_map(Result::ok)
        .map(|path| External {
            cmd: cmd.to_string(),
            args: args.to_string(),
            path,
        })
}
