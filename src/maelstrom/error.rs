use std::fmt::Display;

#[derive(Debug)]
pub enum MaelstromError {
    IOError,
    MessageParseError,
    InitializationMessageMissing,
}

impl Display for MaelstromError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError => write!(f, "[maelstrom error] - io error"),
            Self::MessageParseError => write!(f, "[maelstrom error] - failed to parse message"),
            Self::InitializationMessageMissing => {
                write!(f, "[maelstrom error] - initialization message missing")
            }
        }
    }
}

impl std::error::Error for MaelstromError {}
