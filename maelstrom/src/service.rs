use std::io::{BufRead, Write};

use crate::{
    error::MaelstromError,
    message::{InitializationRequest, InitializationResponse, Message, MessageBody},
    node::MaelstromNode,
};

pub struct Service {
    outbox_id: usize,
}

impl Service {
    pub fn new() -> Self {
        Self { outbox_id: 1 }
    }

    pub fn outbox_id(&self) -> usize {
        self.outbox_id
    }

    pub fn respond_to<T, U>(&mut self, message: &Message<T>, payload: U) -> Message<U> {
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
        message
    }

    pub fn send_to<T>(&mut self, src: String, dest: String, payload: T) -> Message<T> {
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
        message
    }

    pub fn run<N>(
        &mut self,
        input: &mut impl BufRead,
        output: &mut impl Write,
    ) -> Result<(), MaelstromError>
    where
        N: MaelstromNode,
    {
        let line = input
            .lines()
            .next()
            .expect("an initialization message")
            .map_err(|_| MaelstromError::IOError)?;

        let init_message: Message<InitializationRequest> = line
            .parse()
            .map_err(|_| MaelstromError::MessageParseError)?;

        let mut node = N::new(&init_message);
        self.respond_to(&init_message, InitializationResponse::InitOk)
            .write_to(output)?;

        for line in input.lines() {
            let line = line.map_err(|_| MaelstromError::IOError)?;
            let message = line
                .parse::<Message<N::InputPayload>>()
                .map_err(|_| MaelstromError::MessageParseError)?;

            if let Some(payload) = node.handle(&message, self, output)? {
                self.respond_to(&message, payload).write_to(output)?;
            }
        }

        Ok(())
    }
}
