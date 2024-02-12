use serde::Serialize;

use crate::maelstrom::message::{InitializationMessage, InitializationPayload};

use super::{
    error::MaelstromError,
    message::{Message, MessageBuilder},
};

#[derive(Debug, Serialize)]
pub struct Node {
    id: String,
    neighbors: Vec<String>,
    message_id: usize,
}

impl Node {
    pub fn new(init_message: &InitializationMessage) -> Result<Self, MaelstromError> {
        match &init_message.body.payload {
            InitializationPayload::Init { id, neighbors } => Ok(Self {
                id: id.clone(),
                neighbors: neighbors.clone(),
                message_id: 0,
            }),
            InitializationPayload::InitOk => Err(MaelstromError::InitializationMessageMissing),
        }
    }

    pub fn respond_to<T>(&mut self, message: &Message<T>) -> MessageBuilder {
        let builder = MessageBuilder::new(self.id.clone(), message.src.clone())
            .message_id(Some(self.message_id))
            .in_reply_to(message.body.message_id);

        self.message_id += 1;
        builder
    }
}
