#[allow(unused_imports)]
mod command;
mod error;

use command::{Command, CommandType};
use error::ShellError;

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
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let line = input.trim();
        let cmd_elements: Vec<&str> = line.split_whitespace().collect();
        if cmd_elements.is_empty() {
            continue;
        }
        let cmd = Command::try_from(&cmd_elements[..])?;
        match cmd.command_type {
            CommandType::Unknown => println!("{}: command not found", line),
            CommandType::Builtin(builtin) => {
                if let Some(code) = builtin.execute(cmd.args)? {
                    run = false;
                    return_code = code;
                }
            }
            CommandType::Executable(_) => {
                let mut command = std::process::Command::new(cmd.args[0]);
                cmd.args[1..].iter().for_each(|arg| {
                    command.arg(arg);
                });
                std::io::stdout().write_all(&command.output()?.stdout)?;
            }
        }
    }
    Ok(return_code)
}
