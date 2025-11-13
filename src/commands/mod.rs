use error::CommandError;
use crate::utils::string_utils::tokenize;

pub(crate) mod cd;
pub(crate) mod constants;
pub(crate) mod echo;
pub(crate) mod error;
pub(crate) mod exit;
pub(crate) mod external;
pub(crate) mod pwd;
pub(crate) mod typee;

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

        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Ok(Noop);
        }

        // Keep simple split for built-ins to preserve original args string
        let parts = trimmed.splitn(2, ' ').collect::<Vec<&str>>();
        let head = parts.get(0).copied().unwrap_or("");
        let tail = parts.get(1).copied().unwrap_or("");

        Ok(match head {
            "echo" => echo::parse_echo_cmd(tail)?,
            "exit" => exit::parse_exit_cmd(tail)?,
            "type" => typee::parse_type_cmd(tail)?,
            "pwd" => pwd::parse_pwd_cmd()?,
            "cd" => cd::parse_cd_cmd(tail)?,
            _ => {
                // Fallback to tokenizer-based parsing to support quoted executables
                let tokens = tokenize(trimmed);
                if tokens.is_empty() {
                    Noop
                } else {
                    let cmd_tok = &tokens[0];
                    let args_joined = if tokens.len() > 1 {
                        tokens[1..].join(" ")
                    } else {
                        String::new()
                    };
                    match external::parse_external_cmd(cmd_tok, &args_joined) {
                        Some(cmd) => cmd,
                        None => return Err(CommandError::NotFound(cmd_tok.to_string())),
                    }
                }
            }
        })
    }
}
