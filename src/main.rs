mod command;

#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;
use command::{Command, BUILT_IN_COMMANDS};

fn main() {
    let mut user_input = String::new();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        user_input.clear();
        let _ = io::stdin().read_line(&mut user_input);
        let command = Command::from_input(&user_input);

        match command {
            Command::ExitCommand => {
                let cmd_parts: Vec<&str> = user_input.split(" ").collect();
                if cmd_parts[1] == "0" {
                    process::exit(0);
                }
                break
            },
            Command::EchoCommand {display} => println!("{}", display),
            Command::TypeCommand {command_name} => {
                if BUILT_IN_COMMANDS.contains(&command_name.as_str()) {
                    println!("{} is a shell builtin", command_name);
                } else {
                    println!("{}: not found", command_name);
                }
            }
            Command::CommandNotFound => println!("{}: command not found", user_input.trim()),
        }
    }
}
