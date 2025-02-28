use super::builtin::Builtin;

pub(crate) enum CommandType {
    Unknown,
    Builtin(Builtin),
    Executable(String),
}
