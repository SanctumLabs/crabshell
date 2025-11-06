pub const BUILT_IN_COMMANDS: [&str; 3] = ["echo", "exit", "type"];

pub enum Command {
    ExitCommand,
    EchoCommand { display: String },
    TypeCommand { command_name: String},
    CommandNotFound,
}

impl Command {
    pub(crate) fn from_input(input: &str) -> Self {
        let input = input.trim();
        if input.contains("exit") {
            return Self::ExitCommand;
        };

        if let Some(pos) = input.find("echo ") {
            if pos == 0 {
                return Self::EchoCommand {
                    display: input["echo ".len()..].to_string(),
                };
            }
        }

        if let Some(pos) = input.find("type ") {
            if pos == 0 {
                return Self::TypeCommand {
                    command_name: input["type ".len()..].to_string(),
                };
            }
        }

        Self::CommandNotFound
    }
}