use super::Command::{self, *};
use std::{process::Command as StdCommand};
use std::os::unix::fs::PermissionsExt;

pub(crate) fn external_cmd(input_command: &Command) {
    let External { args, cmd, ..  } = input_command else {
        eprintln!("Unexpected error occurred while executing external command");
        return;
    };

    let mut process = StdCommand::new(cmd)
        .args(args.split(' '))
        .spawn()
        .unwrap();

    process.wait().unwrap();
}

pub(crate) fn parse_external_cmd(cmd: &str, args: &str) -> Option<Command> {
    let Ok(path_env) = std::env::var("PATH") else {
        return None;
    };
    let paths = path_env.split(':');

    for dir in paths {
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
