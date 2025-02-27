#[derive(Debug, thiserror::Error)]
pub enum ShellError {
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
}
