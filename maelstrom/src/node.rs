use std::io::Write;

use serde::{Deserialize, Serialize};

use crate::{
    error::MaelstromError,
    message::{InitializationRequest, Message},
    service::Service,
};

pub trait MaelstromNode {
    type InputPayload: for<'a> Deserialize<'a>;
    type OutputPayload: Serialize;

    fn new(init_message: &Message<InitializationRequest>) -> Self;
    fn handle(
        &mut self,
        message: &Message<Self::InputPayload>,
        service: &mut Service,
        output: &mut impl Write,
    ) -> Result<Option<Self::OutputPayload>, MaelstromError>
    where
        Self: Sized;
}
