#[allow(unused_imports)]
mod error;

use crate::error::ShellError;
use std::io::{self, Write};

fn main() {
    match run() {
        Ok(return_code) => std::process::exit(return_code),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}

fn run() -> Result<i32, ShellError> {
    let mut run = true;
    let mut return_code = 0;
    while run {
        print!("$ ");
        io::stdout().flush().map_err(ShellError::from)?;
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(ShellError::from)?;
        input = input.trim().to_string();
        let cmd_elements = cmd_line_to_cmd_list(input.clone());
        if cmd_elements.is_empty() {
            continue;
        }
        let cmd = Command::from(cmd_elements);
        match cmd.build_in_command {
            BuildInCommand::Echo => println!("{}", cmd.args.join(" ")),
            BuildInCommand::Exit => {
                run = false;
                return_code = 0;
            }
            BuildInCommand::Type => {
                let sub_cmd = BuildInCommand::from(cmd.args[0].as_str());
                match sub_cmd {
                    BuildInCommand::None => println!("{}: not found", cmd.args[0].as_str()),
                    _ => println!("{} is a shell builtin", cmd.args[0]),
                }
            }
            BuildInCommand::None => println!("{}: command not found", input),
        }
    }
    Ok(return_code)
}

fn cmd_line_to_cmd_list(cmd: String) -> Vec<String> {
    cmd.split(' ')
        .map(|e| e.trim())
        .filter(|e| !e.is_empty())
        .map(|e| e.to_string())
        .collect()
}

struct Command {
    build_in_command: BuildInCommand,
    args: Vec<String>,
}

enum BuildInCommand {
    Echo,
    Exit,
    Type,
    None,
}

impl From<&str> for BuildInCommand {
    fn from(from: &str) -> Self {
        match from.trim() {
            "echo" => BuildInCommand::Echo,
            "exit" => BuildInCommand::Exit,
            "type" => BuildInCommand::Type,
            _ => BuildInCommand::None,
        }
    }
}

impl From<Vec<String>> for Command {
    fn from(from: Vec<String>) -> Command {
        let cmd = BuildInCommand::from(from[0].as_str());
        Command {
            build_in_command: cmd,
            args: from[1..].to_vec(),
        }
    }
}
