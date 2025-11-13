use std::env;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;


pub fn get_exec_path_string(exec: &str) -> Result<String, Error> {
    for path in env::var("PATH").unwrap().split(":") {
        let path = format!("{}/{}", path, exec);

        if std::fs::metadata(&path).is_ok() {
            return Ok(path);
        }
    }

    Err(Error::new(ErrorKind::NotFound, format!("{}: command not found", exec)))
}

/// Gets the executable path of the passed in executable file and runs it if exists
///
/// # Arguments
///
/// * `exec`: Executable path
///
/// returns: Result<PathBuf, Error>
///
/// # Examples
///
/// ```
///
/// ```
pub fn get_executable_path(exec: &str) -> Result<PathBuf, Error> {
    for path in env::var("PATH").unwrap().split(":") {
        let cmd_path = PathBuf::from(path).join(exec);

        if cmd_path.exists() {
            return Ok(cmd_path);
        }
    }

    Err(Error::new(ErrorKind::NotFound, format!("{exec}: command not found")))
}
