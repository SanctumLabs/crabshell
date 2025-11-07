use super::Command::{self, *};
use std::{io, io::Write, process::Command as StdCommand};
use std::os::unix::fs::PermissionsExt;

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

    for dir in path_env.split(':') {
        let full_path = format!("{}/{}", dir, cmd);
        let path = std::path::Path::new(&full_path);
        if path.exists() {
            if let Ok(metadata) = path.metadata() {
                if metadata.permissions().mode() & 0o111 != 0 {
                    if let Ok(canonical_path) = path.canonicalize() {
                        return Some(External {
                            cmd: cmd.to_string(),
                            args: args.to_string(),
                            path: canonical_path.display().to_string(),
                        });
                    }
                }
            }
        }
    }

    None

    // path_env
    //     .split(':')
    //     .map(|path| {
    //         let full_path = format!("{}/{}", path, cmd);
    //         std::fs::metadata(&full_path).map(|_| full_path)
    //     })
    //     .find_map(Result::ok)
    //     .map(|path| External {
    //         cmd: cmd.to_string(),
    //         args: args.to_string(),
    //         path,
    //     })
}
