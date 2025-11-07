#[allow(unused_imports)]
use std::io::{self, Write};
mod commands;
mod utils;
use commands::Command;

fn main() {
    utils::logger::setup().expect("Unable to setup logger");

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        match Command::from(input.trim()) {
            Ok(cmd) => cmd.execute(),
            Err(err) => eprintln!("{err}"),
        };

        io::stdout().flush().unwrap();
        io::stderr().flush().unwrap();
    }
}
