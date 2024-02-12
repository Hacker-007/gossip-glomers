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

#[derive(Debug)]
pub struct MessageBuilder<P = ()> {
    pub(crate) message: Message<P>,
}

impl<P: Default> MessageBuilder<P> {
    pub fn new(src: String, dest: String) -> Self {
        Self {
            message: Message {
                src,
                dest,
                body: MessageBody {
                    message_id: None,
                    in_reply_to: None,
                    payload: P::default(),
                },
            },
        }
    }
}

impl<P> MessageBuilder<P> {
    pub(crate) fn message_id(mut self, message_id: Option<usize>) -> MessageBuilder<P> {
        self.message.body.message_id = message_id;
        self
    }

    pub(crate) fn in_reply_to(mut self, reply_to: Option<usize>) -> MessageBuilder<P> {
        self.message.body.in_reply_to = reply_to;
        self
    }

    pub fn with_payload<T>(self, payload: T) -> MessageBuilder<T> {
        MessageBuilder {
            message: Message {
                src: self.message.src,
                dest: self.message.dest,
                body: MessageBody {
                    message_id: self.message.body.message_id,
                    in_reply_to: self.message.body.in_reply_to,
                    payload,
                },
            },
        }
    }

    pub fn build(self) -> Message<P> {
        self.message
    }
}

pub type InitializationMessage = Message<InitializationPayload>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InitializationPayload {
    Init {
        #[serde(rename = "node_id")]
        id: String,
        #[serde(rename = "node_ids")]
        neighbors: Vec<String>,
    },
    InitOk,
}
