#[allow(unused_imports)]
mod command;
mod error;

use command::{Builtin, Command, CommandType};
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
            CommandType::Builtin(builtin) => match builtin {
                Builtin::Echo => println!("{}", cmd.args[1..].join(" ")),
                Builtin::Exit => {
                    run = false;
                    return_code = 0;
                }
                Builtin::Type => {
                    let sub_cmd = Command::try_from(&cmd.args[1..])?;
                    match sub_cmd.command_type {
                        CommandType::Unknown => println!("{}: not found", sub_cmd.args[0]),
                        CommandType::Builtin(_) => {
                            println!("{} is a shell builtin", sub_cmd.args[0])
                        }
                        CommandType::Executable(path) => println!("{} is {path}", sub_cmd.args[0]),
                    }
                }
            },
            CommandType::Executable(_) => todo!(),
        }
    }
    Ok(return_code)
}
