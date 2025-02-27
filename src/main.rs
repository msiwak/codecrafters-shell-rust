#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        match Command::from(input) {
            Command::Echo(echo) => println!("{echo}"),
            Command::Exit(return_code) => std::process::exit(return_code),
            Command::Unknown(unknown_command) => println!("{}: command not found", unknown_command),
        }
    }
}

enum Command {
    Unknown(String),
    Echo(String),
    Exit(i32),
}

impl From<String> for Command {
    fn from(input: String) -> Command {
        match input.trim() {
            "exit 0" => Command::Exit(0),
            echo if echo.starts_with("echo ") => Command::Echo(input[4..].trim().to_string()),
            other => Command::Unknown(other.to_string()),
        }
    }
}
