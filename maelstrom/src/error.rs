use std::fmt::Display;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum MaelstromError {
    IOError,
    MessageParseError,
}

impl Display for MaelstromError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError => write!(f, "[maelstrom error] - io error"),
            Self::MessageParseError => write!(f, "[maelstrom error] - failed to parse message"),
        }
    }
}

impl std::error::Error for MaelstromError {}
