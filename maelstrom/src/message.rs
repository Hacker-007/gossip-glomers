use crate::error::MaelstromError;

use serde::{Deserialize, Serialize};
use std::{io::Write, str::FromStr};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<P> {
    pub(crate) src: String,
    pub(crate) dest: String,
    pub(crate) body: MessageBody<P>,
}

impl<P> Message<P> {
    pub fn payload(&self) -> &P {
        &self.body.payload
    }
}

impl<P: Serialize> Message<P> {
    pub fn write_to(&self, output: &mut impl Write) -> Result<(), MaelstromError> {
        serde_json::to_writer(&mut *output, self).map_err(|_| MaelstromError::IOError)?;
        output
            .write_all(b"\n")
            .map_err(|_| MaelstromError::IOError)?;
        Ok(())
    }
}

impl<P: for<'a> Deserialize<'a>> FromStr for Message<P> {
    type Err = MaelstromError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).map_err(|_| MaelstromError::MessageParseError)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct MessageBody<P> {
    #[serde(rename = "msg_id")]
    pub(crate) message_id: Option<usize>,
    pub(crate) in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub(crate) payload: P,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InitializationRequest {
    Init {
        #[serde(rename = "node_id")]
        id: String,
        #[serde(rename = "node_ids")]
        neighbors: Vec<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InitializationResponse {
    InitOk,
}