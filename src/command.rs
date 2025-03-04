mod builtin;
mod command_type;

use pathsearch::find_executable_in_path;
use std::{ffi::OsString, path::PathBuf};

use crate::error::ShellError;
pub(crate) use builtin::Builtin;
pub(crate) use command_type::CommandType;

pub(crate) struct Command<'a> {
    pub(crate) command_type: CommandType,
    pub(crate) args: &'a [&'a str],
}

impl<'a> TryFrom<&'a [&'a str]> for Command<'a> {
    type Error = ShellError;

    fn try_from(from: &'a [&'a str]) -> Result<Command<'a>, Self::Error> {
        if from.is_empty() {
            return Ok(Command {
                command_type: CommandType::Unknown,
                args: from,
            });
        }
        let maybe_builtin = Builtin::try_from(from[0]).ok();
        let cmd_type = match maybe_builtin {
            Some(b) => CommandType::Builtin(b),
            None => match handle_path(from[0]) {
                Some(path) => CommandType::Executable(path),
                None => CommandType::Unknown,
            },
        };
        Ok(Command {
            command_type: cmd_type,
            args: from,
        })
    }
}

fn handle_path(cmd: &str) -> Option<String> {
    find_executable_in_path(cmd).map(|f| f.to_string_lossy().to_string())
}
