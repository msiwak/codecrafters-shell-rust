use std::env;

use crate::error::ShellError;

use super::{Command, CommandType};

#[derive(Debug, PartialEq)]
pub(crate) enum Builtin {
    Echo,
    Exit,
    Pwd,
    Type,
}

impl TryFrom<&str> for Builtin {
    type Error = ();

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        match from {
            "echo" => Ok(Builtin::Echo),
            "exit" => Ok(Builtin::Exit),
            "pwd" => Ok(Builtin::Pwd),
            "type" => Ok(Builtin::Type),
            _ => Err(()),
        }
    }
}

impl Builtin {
    pub(crate) fn execute(&self, args: &[&str]) -> Result<Option<i32>, ShellError> {
        let mut result = None;
        match self {
            Builtin::Echo => println!("{}", args[1..].join(" ")),
            Builtin::Exit => result = Some(0),
            Builtin::Pwd => println!("{}", env::current_dir()?.to_string_lossy()),
            Builtin::Type => {
                let sub_cmd = Command::try_from(&args[1..])?;
                match sub_cmd.command_type {
                    CommandType::Unknown => println!("{}: not found", sub_cmd.args[0]),
                    CommandType::Builtin(_) => {
                        println!("{} is a shell builtin", sub_cmd.args[0])
                    }
                    CommandType::Executable(path) => println!("{} is {path}", sub_cmd.args[0]),
                }
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::command::Builtin;

    #[test]
    fn echo() {
        assert_eq!(Builtin::try_from("echo").unwrap(), Builtin::Echo);
    }

    #[test]
    fn exit() {
        assert_eq!(Builtin::try_from("exit").unwrap(), Builtin::Exit);
    }

    #[test]
    fn type_cmd() {
        assert_eq!(Builtin::try_from("type").unwrap(), Builtin::Type);
    }

    #[test]
    fn other() {
        assert_eq!(Builtin::try_from("whatever"), Err(()));
    }
}
