use std::io::{BufRead, Read, StdoutLock};

use serde::Serialize;

use crate::{
    error::MaelstromError,
    message::{InitializationRequest, InitializationResponse, Message, MessageBody},
    node::MaelstromNode,
};

pub struct Service {
    outbox_id: usize,
    output: StdoutLock<'static>,
}

impl Service {
    pub fn new() -> Self {
        Self {
            outbox_id: 1,
            output: std::io::stdout().lock(),
        }
    }

    pub fn outbox_id(&self) -> usize {
        self.outbox_id
    }

    pub fn respond_to<T, U: Serialize>(
        &mut self,
        message: &Message<T>,
        payload: U,
    ) -> Result<(), MaelstromError> {
        let message = Message {
            src: message.dest.clone(),
            dest: message.src.clone(),
            body: MessageBody {
                message_id: Some(self.outbox_id),
                in_reply_to: message.body.message_id,
                payload,
            },
        };

        self.outbox_id += 1;
        message.write_to(&mut self.output)
    }

    pub fn peer_rpc<T: Serialize>(
        &mut self,
        src: String,
        dest: String,
        payload: T,
    ) -> Result<(), MaelstromError> {
        let message = Message {
            src,
            dest,
            body: MessageBody {
                message_id: Some(self.outbox_id),
                in_reply_to: None,
                payload,
            },
        };

        self.outbox_id += 1;
        message.write_to(&mut self.output)
    }

    pub fn run<N: MaelstromNode>(&mut self) -> Result<(), MaelstromError> {
        let mut input = std::io::stdin().lock();

        let line = input
            .by_ref()
            .lines()
            .next()
            .expect("an initialization message")
            .map_err(|_| MaelstromError::IOError)?;

        let init_message: Message<InitializationRequest> = line
            .parse()
            .map_err(|_| MaelstromError::MessageParseError)?;

        let mut node = N::new(&init_message);
        self.respond_to(&init_message, InitializationResponse::InitOk)?;

        for line in input.by_ref().lines() {
            let line = line.map_err(|_| MaelstromError::IOError)?;
            if let Ok(message) = line.parse::<Message<N::InputPayload>>() {
                if let Some(payload) = node.handle(&message, self)? {
                    self.respond_to(&message, payload)?;
                }
            } else if let Ok(message) = line.parse::<Message<N::PeerPayload>>() {
                if let Some(payload) = node.handle_peer(&message, self)? {
                    self.respond_to(&message, payload)?;
                }
            } else {
                return Err(MaelstromError::MessageParseError);
            }
        }

        Ok(())
    }
}
