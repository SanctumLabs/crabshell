use crate::utils::path_utils;
use crate::utils::string_utils::tokenize;
use error::CommandError;

pub mod cd;
pub mod echo;
pub mod error;
pub mod exit;
pub mod external;
pub mod pwd;
pub mod typee;

pub enum Command {
    Noop,
    Exit(i32),
    Echo(String),
    Type(String),
    Pwd,
    Cd(String),
    External {
        cmd: String,
        args: String,
        path: String,
    },
}

impl Command {
    pub fn execute(&self) {
        use Command::*;
        match self {
            Noop => (),
            Echo(args) => echo::echo_cmd(args),
            Exit(code) => exit::exit_cmd(*code),
            Type(args) => typee::type_cmd(args),
            Pwd => pwd::pwd_cmd(),
            Cd(args) => cd::cd_cmd(args),
            External { .. } => external::external_cmd(self),
        }
    }

    pub fn from(input: &str) -> Result<Self, CommandError> {
        use Command::*;

        let tokens = tokenize(input.trim());
        let cmd = tokens.first().map(|s| s.as_str()).unwrap_or("");
        let args = if tokens.len() > 1 {
            tokens[1..].join(" ")
        } else {
            String::new()
        };
        let args = args.as_str();
        Ok(match cmd {
            "" => Noop,
            "echo" => echo::parse_echo_cmd(args)?,
            "exit" => exit::parse_exit_cmd(args)?,
            "type" => typee::parse_type_cmd(args)?,
            "pwd" => pwd::parse_pwd_cmd()?,
            "cd" => cd::parse_cd_cmd(args)?,
            _ => match external::parse_external_cmd(cmd, args) {
                Some(cmd) => cmd,
                None => {
                    let result = path_utils::get_executable_path(cmd);
                    match result {
                        Ok(path_buffer) => External {
                            cmd: cmd.to_string(),
                            args: args.to_string(),
                            path: path_buffer
                                .file_name()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_string(),
                        },
                        Err(_err) => Noop,
                    };
                    return Err(CommandError::NotFound(cmd.to_string()));
                }
            },
        })
    }
}
