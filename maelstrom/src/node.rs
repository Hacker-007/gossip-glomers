use serde::{Deserialize, Serialize};

use crate::{
    error::MaelstromError,
    message::{InitializationRequest, Message},
    service::Service,
};

pub trait MaelstromNode {
    type InputPayload: for<'a> Deserialize<'a>;
    type OutputPayload: Serialize;
    type PeerPayload: Serialize + for<'a> Deserialize<'a>;

    fn new(init_message: &Message<InitializationRequest>) -> Self;
    fn handle(
        &mut self,
        message: &Message<Self::InputPayload>,
        service: &mut Service,
    ) -> Result<Option<Self::OutputPayload>, MaelstromError>
    where
        Self: Sized;

    fn handle_peer(
        &mut self,
        _: &Message<Self::PeerPayload>,
        _: &mut Service,
    ) -> Result<Option<Self::PeerPayload>, MaelstromError> {
        Ok(None)
    }
}
