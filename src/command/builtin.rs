#[derive(Debug, PartialEq)]
pub(crate) enum Builtin {
    Echo,
    Exit,
    Type,
}

impl TryFrom<&str> for Builtin {
    type Error = ();

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        match from {
            "echo" => Ok(Builtin::Echo),
            "exit" => Ok(Builtin::Exit),
            "type" => Ok(Builtin::Type),
            _ => Err(()),
        }
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
